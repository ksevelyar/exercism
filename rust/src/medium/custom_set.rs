#[derive(Debug, PartialEq, Eq)]
pub struct CustomSet<T> {
    state: Vec<T>,
}

impl<T: std::clone::Clone + std::cmp::PartialEq + std::cmp::Ord> CustomSet<T> {
    pub fn new(input: &[T]) -> Self {
        let mut state = input.to_vec();
        state.sort_unstable();
        state.dedup();

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

    pub fn intersection(&self, other: &Self) -> Self {
        Self::new(
            &self
                .state
                .iter()
                .filter(|item| other.contains(item))
                .cloned()
                .collect::<Vec<_>>(),
        )
    }

    pub fn difference(&self, other: &Self) -> Self {
        Self::new(
            &self
                .state
                .iter()
                .filter(|item| !other.contains(item))
                .cloned()
                .collect::<Vec<_>>(),
        )
    }

    pub fn union(&self, other: &Self) -> Self {
        Self::new(
            &self
                .state
                .iter()
                .chain(other.state.iter())
                .cloned()
                .collect::<Vec<_>>(),
        )
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

#[test]
fn intersection_of_two_sets_with_shared_elements_is_a_set_of_the_shared_elements() {
    let set1 = CustomSet::new(&[1, 2, 3, 4]);

    let set2 = CustomSet::new(&[3, 2, 5]);

    assert_eq!(set1.intersection(&set2), CustomSet::new(&[2, 3]));

    assert_eq!(set2.intersection(&set1), CustomSet::new(&[2, 3]));
}
