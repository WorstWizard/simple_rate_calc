use serde::{de::Visitor, Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Default, Hash, Clone, PartialEq, Eq, Debug, PartialOrd, Ord)]
pub struct Ingredient {
    pub name: String,
}
impl Serialize for Ingredient {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.name)
    }
}
impl<'de> Deserialize<'de> for Ingredient {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_str(IngredientVisitor)
    }
}

struct IngredientVisitor;
impl Visitor<'_> for IngredientVisitor {
    type Value = Ingredient;
    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("expecting string")
    }
    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(Ingredient {
            name: v.to_string(),
        })
    }
}

#[derive(Default, Clone, PartialEq, Serialize, Deserialize, Debug)]
pub struct IngredientWithCount {
    pub ing: Ingredient,
    pub count: f32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Recipe {
    pub craft_time: f32,
    pub output_num: f32,
    pub inputs: Vec<IngredientWithCount>,
}

#[derive(Default, Serialize, Deserialize)]
pub struct RecipeDB {
    pub known_ingredients: Vec<Ingredient>,
    pub known_recipes: HashMap<Ingredient, Recipe>,
}
