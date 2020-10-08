use mongodb::bson;
use serde::{self, Deserialize};
use std::fmt::{Display, Formatter, Result};

#[derive(Deserialize)]
pub struct Recipe {
    name: String,
    ingredients: Vec<Ingredient>,
    #[serde(default)]
    rating: f32,
    #[serde(default)]
    reviews: Vec<Review>,
}

impl Display for Recipe {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(
            f,
            "Recipe: {}, Rating: {} ({})",
            self.name,
            self.rating,
            self.reviews
                .iter()
                .map(|i| i.rating.to_string())
                .collect::<Vec<String>>()
                .join(", "),
        )
    }
}

#[derive(Debug, Deserialize)]
pub struct Ingredient {
    name: String,
}

#[derive(Debug, Deserialize)]
pub struct Review {
    when: bson::DateTime,
    rating: u8,
}

impl Display for Review {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", self.rating)
    }
}
