#[derive(Debug, PartialEq, Eq)]
pub struct CustomSet<T> {
    state: Vec<T>,
}

impl<T: std::clone::Clone + std::cmp::PartialEq + std::cmp::Ord> CustomSet<T> {
    pub fn new(input: &[T]) -> Self {
        let mut state = input.to_vec();
        state.sort_unstable();
        CustomSet { state }
    }

    pub fn contains(&self, element: &T) -> bool {
        self.state.contains(element)
    }

    pub fn add(&mut self, element: T) {
        if !self.state.contains(&element) {
            self.state.push(element);
            self.state.sort_unstable();
        }
    }

    pub fn is_subset(&self, other: &Self) -> bool {
        self.state.iter().all(|item| other.contains(item))
    }

    pub fn is_empty(&self) -> bool {
        self.state.is_empty()
    }

    pub fn is_disjoint(&self, other: &Self) -> bool {
        self.state.iter().all(|item| !other.contains(item))
    }

    #[must_use]
    pub fn intersection(&self, _other: &Self) -> Self {
        unimplemented!();
    }

    #[must_use]
    pub fn difference(&self, _other: &Self) -> Self {
        unimplemented!();
    }

    #[must_use]
    pub fn union(&self, _other: &Self) -> Self {
        unimplemented!();
    }
}

#[test]
fn add_existing_element() {
    let mut set = CustomSet::new(&[1, 2, 3]);

    set.add(3);

    assert_eq!(set, CustomSet::new(&[1, 2, 3]));
}

#[test]
fn set_contained_in_other_set_is_a_subset() {
    let set1 = CustomSet::new(&[1, 2, 3]);

    let set2 = CustomSet::new(&[4, 1, 2, 3]);

    assert!(set1.is_subset(&set2));
}
