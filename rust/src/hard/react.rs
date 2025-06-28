/// `InputCellId` is a unique identifier for an input cell.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct InputCellId(usize);
/// `ComputeCellId` is a unique identifier for a compute cell.
/// Values of type `InputCellId` and `ComputeCellId` should not be mutually assignable,
/// demonstrated by the following tests:
///
/// ```compile_fail
/// let mut r = react::Reactor::new();
/// let input: react::ComputeCellId = r.create_input(111);
/// ```
///
/// ```compile_fail
/// let mut r = react::Reactor::new();
/// let input = r.create_input(111);
/// let compute: react::InputCellId = r.create_compute(&[react::CellId::Input(input)], |_| 222).unwrap();
/// ```

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ComputeCellId(usize);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CallbackId(usize);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CellId {
    Input(InputCellId),
    Compute(ComputeCellId),
}

#[derive(Debug, PartialEq, Eq)]
pub enum RemoveCallbackError {
    NonexistentCell,
    NonexistentCallback,
}

pub struct Reactor<'a, T> {
    values: Vec<(CellId, T)>,
    computed_values: Vec<(CellId, T, Box<dyn Fn(&[T]) -> T + 'a>, Vec<CellId>)>,
    callbacks: Vec<(CallbackId, Box<dyn FnMut(T) + 'a>)>,
    next_cell_id: usize,
    next_callback_id: usize,
}

impl<'a, T: Copy + PartialEq> Default for Reactor<'a, T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a, T: Copy + PartialEq> Reactor<'a, T> {
    pub fn new() -> Self {
        Self {
            callbacks: Vec::new(),
            values: Vec::new(),
            computed_values: Vec::new(),
            next_cell_id: 1,
            next_callback_id: 1,
        }
    }

    // Creates an input cell with the specified initial value, returning its ID.
    pub fn create_input(&mut self, initial: T) -> InputCellId {
        let input = InputCellId(self.next_cell_id);

        self.values.push((CellId::Input(input), initial));
        self.next_cell_id += 1;

        input
    }

    // Creates a compute cell with the specified dependencies and compute function.
    // The compute function is expected to take in its arguments in the same order as specified in
    // `dependencies`.
    // You do not need to reject compute functions that expect more arguments than there are
    // dependencies (how would you check for this, anyway?).
    //
    // If any dependency doesn't exist, returns an Err with that nonexistent dependency.
    // (If multiple dependencies do not exist, exactly which one is returned is not defined and
    // will not be tested)
    //
    // Notice that there is no way to *remove* a cell.
    // This means that you may assume, without checking, that if the dependencies exist at creation
    // time they will continue to exist as long as the Reactor exists.
    pub fn create_compute<F: Fn(&[T]) -> T + 'a>(
        &mut self,
        dependencies: &[CellId],
        compute_func: F,
    ) -> Result<ComputeCellId, CellId> {
        let input = ComputeCellId(self.next_cell_id);
        let values: Result<Vec<T>, CellId> = dependencies
            .iter()
            .map(|&dep_id| self.value(dep_id).ok_or(dep_id))
            .collect();

        let computed = compute_func(&values?);

        self.computed_values.push((
            CellId::Compute(input),
            computed,
            Box::new(compute_func),
            dependencies.to_owned(),
        ));
        self.next_cell_id += 1;

        Ok(input)
    }

    // Retrieves the current value of the cell, or None if the cell does not exist.
    pub fn value(&self, id: CellId) -> Option<T> {
        match id {
            CellId::Input(_) => {
                self.values.iter().find_map(
                    |(cell_id, value)| {
                        if *cell_id == id { Some(*value) } else { None }
                    },
                )
            }
            CellId::Compute(_) => {
                self.computed_values
                    .iter()
                    .find_map(
                        |(cell_id, value, _fun, _deps)| {
                            if *cell_id == id { Some(*value) } else { None }
                        },
                    )
            }
        }
    }

    // Sets the value of the specified input cell.
    // Returns false if the cell does not exist.
    pub fn set_value(&mut self, id: InputCellId, new_value: T) -> bool {
        let cell = self
            .values
            .iter_mut()
            .find(|(cell_id, _value)| *cell_id == CellId::Input(id));

        if let Some(cell) = cell {
            *cell = (CellId::Input(id), new_value);

            self.update_computed(CellId::Input(id));

            true
        } else {
            false
        }
    }

    fn update_computed(&mut self, id: CellId) {
        let updates: Vec<(usize, T)> = self
            .computed_values
            .iter()
            .enumerate()
            .filter(|(_, (_cell_id, _val, _func, deps))| deps.contains(&id))
            .map(|(index, (_cell_id, _val, compute_func, deps))| {
                let values: Vec<_> = deps
                    .iter()
                    .map(|&dep_id| self.value(dep_id).unwrap())
                    .collect();
                let new_value = compute_func(&values);
                (index, new_value)
            })
            .collect();

        for (index, new_value) in updates {
            self.computed_values[index].1 = new_value;
            let cell_id = self.computed_values[index].0;

            self.update_computed(cell_id);
        }
    }

    // Adds a callback to the specified compute cell.
    //
    // Returns the ID of the just-added callback, or None if the cell doesn't exist.
    //
    // Callbacks on input cells will not be tested.
    //
    // The semantics of callbacks (as will be tested):
    // For a single set_value call, each compute cell's callbacks should each be called:
    // * Zero times if the compute cell's value did not change as a result of the set_value call.
    // * Exactly once if the compute cell's value changed as a result of the set_value call.
    //   The value passed to the callback should be the final value of the compute cell after the
    //   set_value call.
    pub fn add_callback<F: FnMut(T) + 'a>(
        &mut self,
        id: ComputeCellId,
        callback: F,
    ) -> Option<CallbackId> {
        self.computed_values
            .iter()
            .find(|(cell_id, _val, _func, _deps)| *cell_id == CellId::Compute(id))?;

        let callback_id = CallbackId(self.next_callback_id);
        self.next_callback_id += 1;

        self.callbacks.push((callback_id, Box::new(callback)));

        Some(callback_id)
    }

    // Removes the specified callback, using an ID returned from add_callback.
    //
    // Returns an Err if either the cell or callback does not exist.
    //
    // A removed callback should no longer be called.
    pub fn remove_callback(
        &mut self,
        cell: ComputeCellId,
        callback: CallbackId,
    ) -> Result<(), RemoveCallbackError> {
        self.computed_values
            .iter()
            .position(|(id, _, _, _)| *id == CellId::Compute(cell))
            .ok_or(RemoveCallbackError::NonexistentCell)?;

        let index = self
            .callbacks
            .iter()
            .position(|(id, _)| *id == callback)
            .ok_or(RemoveCallbackError::NonexistentCallback)?;

        self.callbacks.remove(index);

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn input_cells_have_a_value() {
        let mut reactor = Reactor::new();

        let input = reactor.create_input(10);

        assert_eq!(reactor.value(CellId::Input(input)), Some(10));
    }

    #[test]
    fn an_input_cells_value_can_be_set() {
        let mut reactor = Reactor::new();

        let input = reactor.create_input(4);

        assert!(reactor.set_value(input, 20));

        assert_eq!(reactor.value(CellId::Input(input)), Some(20));
    }

    #[test]
    fn error_setting_a_nonexistent_input_cell() {
        let mut dummy_reactor = Reactor::new();

        let input = dummy_reactor.create_input(1);

        assert!(!Reactor::new().set_value(input, 0));
    }

    #[test]
    fn compute_cells_calculate_initial_value() {
        let mut reactor = Reactor::new();

        let input = reactor.create_input(1);

        let output = reactor
            .create_compute(&[CellId::Input(input)], |v| v[0] + 1)
            .unwrap();

        assert_eq!(reactor.value(CellId::Compute(output)), Some(2));
    }

    #[test]
    fn compute_cells_take_inputs_in_the_right_order() {
        let mut reactor = Reactor::new();

        let one = reactor.create_input(1);

        let two = reactor.create_input(2);

        let output = reactor
            .create_compute(&[CellId::Input(one), CellId::Input(two)], |v| {
                v[0] + v[1] * 10
            })
            .unwrap();

        assert_eq!(reactor.value(CellId::Compute(output)), Some(21));
    }

    #[test]
    fn error_creating_compute_cell_if_input_doesnt_exist() {
        let mut dummy_reactor = Reactor::new();

        let input = dummy_reactor.create_input(1);

        assert_eq!(
            Reactor::new().create_compute(&[CellId::Input(input)], |_| 0),
            Err(CellId::Input(input))
        );
    }

    #[test]
    fn do_not_break_cell_if_creating_compute_cell_with_valid_and_invalid_input() {
        let mut dummy_reactor = Reactor::new();

        let _ = dummy_reactor.create_input(1);

        let dummy_cell = dummy_reactor.create_input(2);

        let mut reactor = Reactor::new();

        let input = reactor.create_input(1);

        assert_eq!(
            reactor.create_compute(&[CellId::Input(input), CellId::Input(dummy_cell)], |_| 0),
            Err(CellId::Input(dummy_cell))
        );

        assert!(reactor.set_value(input, 5));

        assert_eq!(reactor.value(CellId::Input(input)), Some(5));
    }

    #[test]
    fn compute_cells_update_value_when_dependencies_are_changed() {
        let mut reactor = Reactor::new();

        let input = reactor.create_input(1);

        let output = reactor
            .create_compute(&[CellId::Input(input)], |v| v[0] + 1)
            .unwrap();

        assert_eq!(reactor.value(CellId::Compute(output)), Some(2));

        assert!(reactor.set_value(input, 3));

        assert_eq!(reactor.value(CellId::Compute(output)), Some(4));
    }

    #[test]
    fn compute_cells_can_depend_on_other_compute_cells() {
        let mut reactor = Reactor::new();

        let input = reactor.create_input(1);

        let times_two = reactor
            .create_compute(&[CellId::Input(input)], |v| v[0] * 2)
            .unwrap();

        let times_thirty = reactor
            .create_compute(&[CellId::Input(input)], |v| v[0] * 30)
            .unwrap();

        let output = reactor
            .create_compute(
                &[CellId::Compute(times_two), CellId::Compute(times_thirty)],
                |v| v[0] + v[1],
            )
            .unwrap();

        assert_eq!(reactor.value(CellId::Compute(output)), Some(32));

        assert!(reactor.set_value(input, 3));

        assert_eq!(reactor.value(CellId::Compute(output)), Some(96));
    }
}
