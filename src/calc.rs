use crate::data::*;
use std::collections::HashMap;

#[derive(Default)]
pub struct Calculator {
    pub output_rate: f32,
    pub output_ingredient: Ingredient,
}
impl Calculator {
    pub fn compute_required_rates(
        output_ingredient: &Ingredient,
        output_rate: f32,
        known_recipes: &HashMap<Ingredient, Recipe>,
    ) -> (f32, Option<Vec<(Ingredient, f32)>>) {
        if let Some(recipe) = known_recipes.get(output_ingredient) {
            let cycles_per_sec = output_rate / recipe.output_num;
            let num_producers = cycles_per_sec * recipe.craft_time;

            let mut required_input_rates = Vec::with_capacity(recipe.inputs.len());
            for input_ing in recipe.inputs.iter() {
                let input_rate = input_ing.count * cycles_per_sec;
                required_input_rates.push((input_ing.ing.clone(), input_rate));
            }
            (num_producers, Some(required_input_rates))
        } else {
            (0.0, None)
        }
    }
    pub fn compute_aggregate_rates(
        &self,
        known_recipes: &HashMap<Ingredient, Recipe>,
    ) -> Vec<(Ingredient, f32, f32)> {
        fn aggregate_inputs(
            ingredient: Ingredient,
            required_rate: f32,
            visit_num: &mut u32,
            agg_map: &mut HashMap<Ingredient, (f32, f32, u32)>,
            recipes: &HashMap<Ingredient, Recipe>,
        ) {
            let (producers, inputs) =
                Calculator::compute_required_rates(&ingredient, required_rate, recipes);
            agg_map
                .entry(ingredient)
                .and_modify(|(prod, rate, visit)| {
                    *prod += producers;
                    *rate += required_rate;
                    *visit = *visit_num
                })
                .or_insert((producers, required_rate, *visit_num));

            if let Some(inputs) = inputs {
                for (ing, rate) in inputs {
                    *visit_num += 1;
                    aggregate_inputs(ing, rate, visit_num, agg_map, recipes)
                }
            }
        }
        let mut aggregate_rates = HashMap::new();
        let mut visit_num = 0;
        aggregate_inputs(
            self.output_ingredient.clone(),
            self.output_rate,
            &mut visit_num,
            &mut aggregate_rates,
            known_recipes,
        );

        let mut unsorted_rates: Vec<(u32, (Ingredient, f32, f32))> = aggregate_rates
            .drain()
            .map(|(ing, (prod, rate, visit))| (visit, (ing, prod, rate)))
            .collect();

        unsorted_rates.sort_by_key(|(visit, _)| *visit);
        unsorted_rates.into_iter().map(|(_, val)| val).collect()
    }
}
