use std::collections::HashSet;
use crate::data::*;

#[derive(Default)]
pub struct RecipeBuilder {
    pub craft_time: f32,
    pub output_ingredient: IngredientWithCount,
    pub used_ingredients: HashSet<Ingredient>,
    input_ingredients: Vec<IngredientWithCount>,
    available_ingredients: Vec<Ingredient>
}
impl RecipeBuilder {
    pub fn recompute_available_ingredients(&mut self, rdb: &RecipeDB) {
        let mut available =
            Vec::with_capacity(rdb.known_ingredients.len());

        available.extend(rdb.known_ingredients.iter().filter_map(
            |ing| {
                let legal = !self.used_ingredients.contains(ing)
                    && !self.detect_cyclical_recipe(rdb, ing);
                legal.then_some(ing.clone())
            }
        ));
        self.available_ingredients = available
    }
    pub fn available_ingredients(&self) -> &Vec<Ingredient> {
        &self.available_ingredients
    }

    pub fn build_recipe(&self, rdb: &mut RecipeDB) -> Result<(), ()> {
        if self.is_recipe_valid(rdb) {
            let recipe = Recipe {
                craft_time: self.craft_time,
                output_num: self.output_ingredient.count,
                inputs: self.input_ingredients.clone()
            };
            rdb.known_recipes.insert(self.output_ingredient.ing.clone(), recipe);
            Ok(())
        } else {
            Err(())
        }
    }

    pub fn inputs(&self) -> std::slice::Iter<IngredientWithCount> {
        self.input_ingredients.iter()
    }
    pub fn num_inputs(&self) -> usize {
        self.input_ingredients.len()
    }
    pub fn get_input(&self, index: usize) -> Option<&IngredientWithCount> {
        self.input_ingredients.get(index)
    }
    pub fn change_input_ingredient(&mut self, index: usize, value: Ingredient) {
        if let Some(ing_c) = self.input_ingredients.get_mut(index) {
            self.used_ingredients.remove(&ing_c.ing);
            self.used_ingredients.insert(value.clone());
            ing_c.ing = value;
        }
    }
    pub fn change_output_ingredient(&mut self, value: Ingredient) {
        self.used_ingredients.remove(&self.output_ingredient.ing);
        self.used_ingredients.insert(value.clone());
        self.output_ingredient.ing = value;
        for input_ing in &mut self.input_ingredients {
            if input_ing.ing == self.output_ingredient.ing {
                input_ing.ing = Ingredient::default()
            }
        }
    }
    pub fn get_input_count_mut(&mut self, index: usize) -> &mut i32 {
        &mut self.input_ingredients[index].count
    }
    pub fn add_blank_input(&mut self) {
        self.input_ingredients.push(IngredientWithCount::default());
    }
    pub fn remove_input(&mut self, index: usize) {
        if let Some(ing_c) = self.input_ingredients.get(index) {
            self.used_ingredients.remove(&ing_c.ing);
            self.input_ingredients.remove(index);
        }
    }

    fn is_recipe_valid(&self, rdb: &RecipeDB) -> bool {
        for ing_w_count in &self.input_ingredients {
            if self.detect_cyclical_recipe(rdb, &ing_w_count.ing) {
                eprintln!("cycle");
                return false
            }
            if self.input_ingredients.iter().filter(|other_ing_c| {
                other_ing_c.ing.name == ing_w_count.ing.name
            }).count() > 1 {
                eprintln!("duplicate inputs");
                return false
            }
            if ing_w_count.count < 1 {
                eprintln!("zero input/output");
                return false
            }
        }
        true
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