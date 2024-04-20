use std::collections::HashSet;
use crate::calc::*;

#[derive(Default)]
pub struct RecipeBuilder {
    pub craft_time: f32,
    pub output_ingredient: IngredientWithCount,
    pub input_ingredients: Vec<IngredientWithCount>,
    pub used_ingredients: HashSet<Ingredient>,
}