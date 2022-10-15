use rand::Rng;

#[derive(Debug)]
pub struct Robot {
    name: String,
}

impl Robot {
    fn new_name() -> String {
        let mut rng = rand::thread_rng();
        let letter_1: char = rng.gen_range(b'A'..b'Z') as char;
        let letter_2: char = rng.gen_range(b'A'..b'Z') as char;
        let number: u32 = rng.gen_range(0..999);

        format!("{}{}{:03}", letter_1, letter_2, number)
    }

    pub fn new() -> Self {
        Self { name: Self::new_name() }
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
}
