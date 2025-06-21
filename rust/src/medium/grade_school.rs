use std::collections::{BTreeMap, BTreeSet};

#[allow(clippy::new_without_default)]
pub struct School {
    students: BTreeMap<u32, BTreeSet<String>>,
}

impl Default for School {
    fn default() -> Self {
        Self::new()
    }
}

impl School {
    pub fn new() -> School {
        School {
            students: BTreeMap::new(),
        }
    }

    pub fn add(&mut self, grade: u32, student: &str) {
        let entry = self.students.entry(grade).or_default();
        entry.insert(student.to_owned());
    }

    pub fn grades(&self) -> Vec<u32> {
        self.students.keys().cloned().collect()
    }

    pub fn grade(&self, grade: u32) -> Vec<String> {
        match self.students.get(&grade) {
            Some(vec) => vec.iter().cloned().collect(),
            None => Vec::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_grade_when_no_students_have_that_grade() {
        let mut s = School::new();

        s.add(7, "Logan");

        assert_eq!(s.grade(1), Vec::<String>::new());
    }
}
