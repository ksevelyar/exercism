use rand::Rng;
use std::cell::RefCell;
use std::collections::HashSet;

thread_local!(static ID: RefCell<HashSet<String>> = RefCell::new(HashSet::new()));

#[derive(Debug)]
pub struct Robot {
    name: String,
}

impl Default for Robot {
    fn default() -> Self {
        Self::new()
    }
}

impl Robot {
    fn random_name() -> String {
        let mut rng = rand::thread_rng();
        let letter_1: char = rng.gen_range(b'A'..=b'Z') as char;
        let letter_2: char = rng.gen_range(b'A'..=b'Z') as char;
        let number: u32 = rng.gen_range(0..999);

        format!("{}{}{:03}", letter_1, letter_2, number)
    }

    fn new_name() -> String {
        let name = Self::random_name();

        ID.with(|set| {
            let is_name_in_use = set.borrow().contains(&name);

            match is_name_in_use {
                true => Self::new_name(),
                false => {
                    (set.borrow_mut()).insert(name.clone());
                    name
                }
            }
        })
    }

    pub fn new() -> Self {
        Self {
            name: Self::new_name(),
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn reset_name(&mut self) {
        self.name = Self::new_name()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_name() {
        assert_eq!(Robot::new().name().len(), 5);
    }

    #[test]
    fn test_many_different_robots_have_different_names() {
        use std::collections::HashSet;

        // In 3,529 random robot names, there is ~99.99% chance of a name collision
        let vec: Vec<_> = (0..3529).map(|_| Robot::new()).collect();

        let set: HashSet<_> = vec.iter().map(|robot| robot.name()).collect();

        let number_of_collisions = vec.len() - set.len();

        assert_eq!(number_of_collisions, 0);
    }
}
