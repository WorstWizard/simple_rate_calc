use std::collections::{HashMap, HashSet};
use eframe::egui::{self, Vec2};

const HEIGHT: f32 = 300.0;
const WIDTH: f32 = 300.0;

fn main() {
    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size(Vec2::new(WIDTH, HEIGHT)).with_resizable(false),
        ..Default::default()
    };
    eframe::run_native("Simple Rate Calc", native_options, Box::new(|cc| Box::new(RateCalcApp::new(cc)))).unwrap();
}

#[derive(Default)]
struct RateCalcApp {
    desired_output_rate: f32,
    add_ingredient_text: String,
    add_recipe_output_ingredient: IngredientWithCount,
    add_recipe_input_ingredients: Vec<IngredientWithCount>,
    add_recipe_used_ingredients: HashSet<Ingredient>,
    known_ingredients: Vec<Ingredient>,
    known_recipes: HashMap<String, Recipe>
}

impl RateCalcApp {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        Self {
            desired_output_rate: 1.0,
            ..Self::default()
        }
    }
}

impl eframe::App for RateCalcApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {

            // Add Ingredients
            ui.horizontal(|ui| {
                let add_ingredient_edit = egui::TextEdit::singleline(&mut self.add_ingredient_text)
                    .hint_text("Add ingredient");
                let button_clicked = ui.button("Add").clicked();
                let text_response = ui.add(add_ingredient_edit);
                
                if !self.add_ingredient_text.is_empty()
                && (text_response.lost_focus()
                && ui.input(|i| i.key_pressed(egui::Key::Enter))
                || button_clicked)
                {
                    let new_ing = Ingredient { name: self.add_ingredient_text.clone() };
                    if !self.known_ingredients.contains(&new_ing) { self.known_ingredients.push(new_ing) }
                }
            });

            ui.separator();

            // Add Recipes
            ui.add_enabled_ui(self.known_ingredients.len() >= 1, |ui| {

                // Output dropdown
                ui.label("Output");
                if ingredient_selector(ui, 0,
                    self.known_ingredients.iter(),
                    &mut self.add_recipe_output_ingredient,
                    &mut self.add_recipe_used_ingredients
                ) {
                    for input_ing in self.add_recipe_input_ingredients.iter_mut() {
                        if input_ing.0 == self.add_recipe_output_ingredient.0 {
                            *input_ing = IngredientWithCount::default();
                        }
                    }
                }
                
                // Inputs
                ui.label("Inputs");
                let mut remove_input = None;
                for (i, input_ing) in self.add_recipe_input_ingredients.iter_mut().enumerate() {
                    let mut available_ingredients = Vec::with_capacity(self.known_ingredients.len());
                    if !input_ing.0.name.is_empty() { available_ingredients.push(input_ing.0.clone()) }
                    available_ingredients.extend(self.known_ingredients.iter().filter_map(|ing|
                        if self.add_recipe_used_ingredients.contains(&ing) { None } else { Some(ing.clone()) }
                    ));
                    ui.horizontal(|ui| {
                        ingredient_selector(ui, i+1,
                            available_ingredients.iter(),
                            input_ing,
                            &mut self.add_recipe_used_ingredients
                        );
                        if ui.button("X").clicked() {
                            remove_input = Some(i);
                            self.add_recipe_used_ingredients.remove(&input_ing.0);
                        }
                    });
                }
                if let Some(i) = remove_input {
                    self.add_recipe_input_ingredients.remove(i);
                }


                if ui.button("+").clicked() {
                    self.add_recipe_input_ingredients.push((Ingredient::default(), 0));
                }
            });

            ui.separator();

            // Add recipe to system

            let valid_recipe =
            !(
                ingredient_is_empty(&self.add_recipe_output_ingredient)
                || self.add_recipe_input_ingredients.iter().any(ingredient_is_empty)
            );
            let add_recipe_button = egui::Button::new("Add Recipe");
            if ui.add_enabled(valid_recipe, add_recipe_button).clicked() {
                todo!("add recipe");
            }

            // // Main settings
            // ui.horizontal(|ui| {
            //     ui.label("Output");
            //     // ui.add(egui::ComboBox::)
            //     ui.label("Desired output rate");
            //     ui.add(egui::DragValue::new(&mut self.desired_output_rate)
            //         .clamp_range(0.0..=f32::MAX)
            //         .max_decimals(2)
            //         .speed(0.05)
            //         .suffix("/s")
            //     );
            // });
            // ui.separator();

            // // Recursive ingredient list
            // ui.vertical(|ui| {
            //     ui.label("Some stuff");
            //     ui.label("Some more stuff");
            //     ui.label("Even more stuff");
            // });

            for ingredient in &self.known_ingredients {
                ui.label(&ingredient.name);
            }
        });
    }
}

fn ingredient_selector<'a, I>(ui: &mut egui::Ui, id_source: impl std::hash::Hash, ingredient_list: I, selection: &mut IngredientWithCount, used_ingredients: &mut HashSet<Ingredient>) -> bool
    where I: Iterator<Item = &'a Ingredient>,
{
    let dropdown = egui::ComboBox::from_id_source(id_source).selected_text(&selection.0.name);
    let dragval = egui::DragValue::new(&mut selection.1).clamp_range(0..=100).max_decimals(0);
    let selected_ing = selection.0.clone();
    let mut changed_value = false;
    ui.horizontal(|ui| {
        // if ingredient_list. {
        //     dropdown.show_ui(ui, |ui| {ui.label("- - -")});
        // } else {
            ui.add(dragval);
            dropdown.show_ui(ui, |ui| {
                for ingredient in ingredient_list {
                    let current_selection = *ingredient == selected_ing;
                    if ui.selectable_label(
                        current_selection,
                        &ingredient.name
                    ).clicked() && !current_selection {
                        used_ingredients.remove(&selected_ing);
                        used_ingredients.insert(ingredient.clone());
                        selection.0 = ingredient.clone();
                        changed_value = true;
                    }
                }
            });
        // }
    });
    changed_value
}

fn ingredient_is_empty(ing: &IngredientWithCount) -> bool {
    ing.1 == 0 || ing.0.name.is_empty()
}

#[derive(Default, Hash, Clone, PartialEq, Eq)]
struct Ingredient {
    pub name: String
}
type IngredientWithCount = (Ingredient, i32);
struct Recipe {
    craft_time: f32,
    output: IngredientWithCount,
    input: Vec<IngredientWithCount>
}