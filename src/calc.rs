use std::collections::HashMap;

#[derive(Default, Hash, Clone, PartialEq, Eq)]
pub struct Ingredient {
    pub name: String,
}
#[derive(Default, Hash, Clone, PartialEq, Eq)]
pub struct IngredientWithCount{
    pub ing: Ingredient,
    pub count: i32
}
pub struct Recipe {
    pub craft_time: f32,
    pub output_num: i32,
    pub inputs: Vec<IngredientWithCount>,
}


#[derive(Default)]
pub struct Calculator {
    pub known_ingredients: Vec<Ingredient>,
    pub known_recipes: HashMap<Ingredient, Recipe>,
}

