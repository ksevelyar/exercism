#[derive(Debug)]
pub struct Allergies;

#[derive(Debug, PartialEq, Eq)]
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

        dbg!(allergens.map(|allergen| allergen as u32));

        Self
    }

    pub fn is_allergic_to(&self, allergen: &Allergen) -> bool {
        unimplemented!(
            "Determine if the patient is allergic to the '{:?}' allergen.",
            allergen
        );
    }

    pub fn allergies(&self) -> Vec<Allergen> {
        unimplemented!("Return the list of allergens contained within the score with which the Allergies struct was made.");
    }
}
