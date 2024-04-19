use std::collections::{HashMap, HashSet};
use eframe::egui::{self, Vec2};

const HEIGHT: f32 = 400.0;
const WIDTH: f32 = 256.0;

fn main() {
    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size(Vec2::new(WIDTH, HEIGHT)).with_resizable(false),
        ..Default::default()
    };
    eframe::run_native("Simple Rate Calc", native_options, Box::new(|cc| Box::new(RateCalcApp::new(cc)))).unwrap();
}

#[derive(PartialEq, Default)]
enum SelectedTab {
    #[default]
    Rates,
    Editing
}

#[derive(Default)]
struct RateCalcApp {
    known_ingredients: Vec<Ingredient>,
    known_recipes: HashMap<Ingredient, Recipe>,
    selected_tab: SelectedTab,

    // For rate calculations
    desired_output_rate: f32,
    desired_output_ingredient: Ingredient,
    // required_rates: HashMap<Ingredient, f32>,
    
    // For adding ingredients/recipes
    add_ingredient_text: String,
    add_recipe_craft_time: f32,
    add_recipe_output_ingredient: IngredientWithCount,
    add_recipe_input_ingredients: Vec<IngredientWithCount>,
    add_recipe_used_ingredients: HashSet<Ingredient>,
}

impl RateCalcApp {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self {
            desired_output_rate: 1.0,
            ..Self::default()
        }
    }
}

impl eframe::App for RateCalcApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            ui.columns(2, |cols| {
                if cols[0].selectable_label(self.selected_tab == SelectedTab::Rates, "Rates").clicked() {
                    self.selected_tab = SelectedTab::Rates
                };
                if cols[1].selectable_label(self.selected_tab == SelectedTab::Editing, "Edit Recipes").clicked() {
                    self.selected_tab = SelectedTab::Editing
                };
            })
        });
        egui::CentralPanel::default().show(ctx, |ui| { match self.selected_tab {
            SelectedTab::Rates => {
            // Main settings
            ui.horizontal(|ui| {
                ui.label("Output");
                let dropdown = egui::ComboBox::from_id_source("output").selected_text(&self.desired_output_ingredient.name);
                dropdown.show_ui(ui, |ui| {
                    for ingredient in &self.known_ingredients {
                        let current_selection = *ingredient == self.desired_output_ingredient;
                        if ui.selectable_label(
                            current_selection,
                            &ingredient.name
                        ).clicked() && !current_selection {
                            self.desired_output_ingredient = ingredient.clone();
                        }
                    }
                });
                ui.label("Rate");
                ui.add(egui::DragValue::new(&mut self.desired_output_rate)
                    .clamp_range(0.0..=f32::MAX)
                    // .max_decimals(2)
                    // .speed(0.05)
                    .suffix("/s")
                );
            });
            ui.separator();

            // Recursive ingredient list
            ui.columns(3, |cols| {
                // cols[0].label("");
                cols[1].label("Producers");
                cols[2].label("Rate");
            });
            display_rates_info(ui, &self.desired_output_ingredient, self.desired_output_rate, &self.known_recipes);
            
            },
            SelectedTab::Editing => {
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
                ui.horizontal(|ui| {
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
                });

                // Craft time
                ui.horizontal(|ui| {
                    let dragval = egui::DragValue::new(&mut self.add_recipe_craft_time).clamp_range(0.0..=f32::MAX).max_decimals(1);
                    ui.label("Craft time ");
                    ui.add(dragval);
                });
                
                // Inputs
                ui.label("Inputs");
                fn filter_available_ingredients(
                    output_ingredient: &Ingredient,
                    input_ingredient: &Ingredient,
                    used_ingredients: &mut HashSet<Ingredient>,
                    known_ingredients: &Vec<Ingredient>,
                    known_recipes: &HashMap<Ingredient, Recipe>
                ) -> Vec<Ingredient> {
                    let mut available_ingredients = Vec::with_capacity(known_ingredients.len());
                    if !input_ingredient.name.is_empty() { available_ingredients.push(input_ingredient.clone()) }

                    available_ingredients.extend(known_ingredients.iter().filter_map(|ing| {
                        let legal = !used_ingredients.contains(ing)
                        && !detect_cyclical_recipe(output_ingredient, ing, known_recipes);
                        legal.then_some(ing.clone())
                    }));
                
                    available_ingredients
                }

                let mut remove_input = None;
                for (i, input_ing) in self.add_recipe_input_ingredients.iter_mut().enumerate() {
                    let available_ingredients = filter_available_ingredients(
                        &self.add_recipe_output_ingredient.0,
                        &input_ing.0,
                        &mut self.add_recipe_used_ingredients,
                        &self.known_ingredients,
                        &self.known_recipes
                    );
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

            // Validity check is only partial here; just checks for whether each input and output are non-zero
            // The rest of validity checking is done by relying on only valid ingredients being picked for inputs
            // earlier. If this isn't the case, it might get fucked up.
            let valid_recipe =
            !(
                ingredient_is_empty(&self.add_recipe_output_ingredient)
                || self.add_recipe_input_ingredients.iter().any(ingredient_is_empty)
            );
            let add_recipe_button = egui::Button::new("Add Recipe");
            if ui.add_enabled(valid_recipe, add_recipe_button).clicked() {
                let recipe = Recipe {
                    craft_time: self.add_recipe_craft_time,
                    output_num: self.add_recipe_output_ingredient.1,
                    input: self.add_recipe_input_ingredients.clone()
                };
                self.known_recipes.insert(self.add_recipe_output_ingredient.0.clone(), recipe);
            }}
            }
        });
    }
}

fn display_rates_info(
    ui: &mut egui::Ui,
    output_ingredient: &Ingredient,
    output_rate: f32,
    known_recipes: &HashMap<Ingredient, Recipe>
) {
    fn info_display(ui: &mut egui::Ui, name: &String, producers: f32, rate: f32) {
        ui.columns(3, |cols| {
            cols[0].label(name);
            if producers > 0.0 {
                cols[1].label(format!("{:.1}", producers));
            }
            cols[2].label(format!("{:.2}", rate));
        });
    }

    let (num_producers, input_rates) = compute_required_rates(output_ingredient, output_rate, known_recipes);
    info_display(ui, &output_ingredient.name, num_producers, output_rate);
    if let Some(rates) = input_rates {
        if !rates.is_empty() {
            let header = egui::CollapsingHeader::new("").id_source(output_ingredient);
            header.default_open(true).show_unindented(ui, |ui| {
                for (ing, rate) in rates {
                    display_rates_info(ui, &ing, rate, known_recipes);
                }
            });
        }
    }
}

fn compute_required_rates(
    output_ingredient: &Ingredient,
    output_rate: f32,
    // input_rates: &mut HashMap<Ingredient, f32>,
    known_recipes: &HashMap<Ingredient, Recipe>
) -> (f32, Option<Vec<(Ingredient, f32)>>) {
    if let Some(recipe) = known_recipes.get(output_ingredient) {
        let cycles_per_sec = output_rate/(recipe.output_num as f32);
        let num_producers = cycles_per_sec*recipe.craft_time;

        let mut required_input_rates = Vec::with_capacity(recipe.input.len());
        for input_ing in recipe.input.iter() {
            let input_rate = (input_ing.1 as f32)*cycles_per_sec;
            required_input_rates.push((input_ing.0.clone(),input_rate));
        }
        (num_producers, Some(required_input_rates))
    } else {
        (0.0, None)
    }
}

fn detect_cyclical_recipe(output_ingredient: &Ingredient, input_ingredient: &Ingredient, known_recipes: &HashMap<Ingredient, Recipe>) -> bool {
    if let Some(recipe) = known_recipes.get(input_ingredient) {
        for (input_ing, _) in &recipe.input {
            if *input_ing == *output_ingredient {
                return true;
            } else {
                return detect_cyclical_recipe(output_ingredient, input_ing, known_recipes);
            }
        }
    }
    false
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

#[derive(Default, Hash, Clone, PartialEq, Eq, Debug)]
struct Ingredient {
    pub name: String
}
type IngredientWithCount = (Ingredient, i32);
struct Recipe {
    craft_time: f32,
    output_num: i32,
    input: Vec<IngredientWithCount>
}