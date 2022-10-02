#[derive(Debug)]
pub struct Allergies(Vec<Allergen>);

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum Allergen {
    Eggs = 1,
    Peanuts = 2,
    Shellfish = 4,
    Strawberries = 8,
    Tomatoes = 16,
    Chocolate = 32,
    Pollen = 64,
    Cats = 128,
}

impl Allergies {
    pub fn new(score: u32) -> Self {
        use Allergen::*;

        let allergens = [
            Cats,
            Pollen,
            Chocolate,
            Tomatoes,
            Strawberries,
            Shellfish,
            Peanuts,
            Eggs,
        ];

        let allergies: Vec<Allergen> = allergens
            .iter()
            .filter(|allergen| (**allergen as u32 & score) > 0)
            .copied()
            .collect();

        Self(allergies)
    }

    pub fn is_allergic_to(&self, allergen: &Allergen) -> bool {
        self.0.iter().any(|allergy| allergy == allergen)
    }

    pub fn allergies(&self) -> Vec<Allergen> {
        self.0.clone()
    }
}
