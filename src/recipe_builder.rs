use std::collections::HashSet;
use crate::data::*;

#[derive(Default)]
pub struct RecipeBuilder {
    pub craft_time: f32,
    pub output_ingredient: IngredientWithCount,
    pub input_ingredients: Vec<IngredientWithCount>,
    pub used_ingredients: HashSet<Ingredient>,
    pub available_ingredients: Vec<Ingredient>
}
impl RecipeBuilder {
    pub fn recompute_available_ingredients(&mut self, rdb: &RecipeDB) {
        let mut new_vec =
            Vec::with_capacity(rdb.known_ingredients.len());
        // if !input_ingredient.name.is_empty() {
        //     self.available_ingredients.push(input_ingredient.clone())
        // }

        new_vec.extend(rdb.known_ingredients.iter().filter_map(
            |ing| {
                let legal = !self.used_ingredients.contains(ing)
                    && !self.detect_cyclical_recipe(rdb, ing);
                legal.then_some(ing.clone())
            }
        ));

        self.available_ingredients = new_vec;
    }

    pub fn remove_input(&mut self, index: usize) {
        let to_remove = self.input_ingredients.get(index);
        if let Some(ing_c) = to_remove {
            self.used_ingredients.remove(&ing_c.ing);
            self.input_ingredients.remove(index);
        }
    }

    fn detect_cyclical_recipe(&self, rdb: &RecipeDB, input_ingredient: &Ingredient) -> bool {
        if let Some(recipe) = rdb.known_recipes.get(input_ingredient) {
            for input in &recipe.inputs {
                if input.ing == self.output_ingredient.ing
                    || self.detect_cyclical_recipe(rdb, &input.ing)
                {
                    return true;
                }
            }
        }
        false
    }
}