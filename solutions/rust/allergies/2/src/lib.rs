pub struct Allergies(u32);

// Using pedantic clippy lints

use self::Allergen::{Cats, Chocolate, Eggs, Peanuts, Pollen, Shellfish, Strawberries, Tomatoes};

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
#[repr(u32)]
pub enum Allergen {
    Eggs = 1 << 0,
    Peanuts = 1 << 1,
    Shellfish = 1 << 2,
    Strawberries = 1 << 3,
    Tomatoes = 1 << 4,
    Chocolate = 1 << 5,
    Pollen = 1 << 6,
    Cats = 1 << 7,
}

const ALLERGENS: [Allergen; 8] = [
    Eggs,
    Peanuts,
    Shellfish,
    Strawberries,
    Tomatoes,
    Chocolate,
    Pollen,
    Cats,
];

impl Allergies {
    #[must_use]
    pub const fn new(score: u32) -> Self {
        Self(score)
    }

    #[must_use]
    pub const fn is_allergic_to(&self, allergen: &Allergen) -> bool {
        let allergen = *allergen as u32;
        self.0 & allergen == allergen
    }

    #[must_use]
    pub fn allergies(&self) -> Vec<Allergen> {
        ALLERGENS
            .iter()
            .filter(|a| self.is_allergic_to(a))
            .copied()
            .collect()
    }
}
