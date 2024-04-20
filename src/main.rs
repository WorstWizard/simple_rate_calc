use eframe::egui::{self, Vec2};
use std::collections::HashMap;

mod data;
mod calc;
mod recipe_builder;
use data::*;
use recipe_builder::*;

const HEIGHT: f32 = 400.0;
const WIDTH: f32 = 256.0;

fn main() {
    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size(Vec2::new(WIDTH, HEIGHT))
            .with_resizable(false),
        ..Default::default()
    };
    eframe::run_native(
        "Simple Rate Calc",
        native_options,
        Box::new(|cc| Box::new(RateCalcApp::new(cc))),
    )
    .unwrap();
}

#[derive(PartialEq, Default)]
enum SelectedTab {
    #[default]
    Rates,
    Editing,
}

#[derive(Default)]
struct RateCalcApp {
    recipe_db: RecipeDB,
    selected_tab: SelectedTab,

    // For rate calculations
    desired_output_rate: f32,
    desired_output_ingredient: Ingredient,

    // For adding ingredients/recipes
    add_ingredient_text: String,
    recipe_builder: RecipeBuilder,
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
                if cols[0]
                    .selectable_label(self.selected_tab == SelectedTab::Rates, "Rates")
                    .clicked()
                {
                    self.selected_tab = SelectedTab::Rates
                };
                if cols[1]
                    .selectable_label(self.selected_tab == SelectedTab::Editing, "Edit Recipes")
                    .clicked()
                {
                    self.selected_tab = SelectedTab::Editing
                };
            })
        });
        egui::CentralPanel::default().show(ctx, |ui| {
            match self.selected_tab {
                SelectedTab::Rates => {
                    // Main settings
                    ui.horizontal(|ui| {
                        ui.label("Output");
                        let dropdown = egui::ComboBox::from_id_source("output")
                            .selected_text(&self.desired_output_ingredient.name);
                        dropdown.show_ui(ui, |ui| {
                            for ingredient in &self.recipe_db.known_ingredients {
                                let current_selection =
                                    *ingredient == self.desired_output_ingredient;
                                if ui
                                    .selectable_label(current_selection, &ingredient.name)
                                    .clicked()
                                    && !current_selection
                                {
                                    self.desired_output_ingredient = ingredient.clone();
                                }
                            }
                        });
                        ui.label("Rate");
                        ui.add(
                            egui::DragValue::new(&mut self.desired_output_rate)
                                .clamp_range(0.0..=f32::MAX)
                                // .max_decimals(2)
                                // .speed(0.05)
                                .suffix("/s"),
                        );
                    });
                    ui.separator();

                    // Recursive ingredient list
                    ui.columns(3, |cols| {
                        // cols[0].label("");
                        cols[1].label("Producers");
                        cols[2].label("Rate");
                    });
                    display_rates_info(
                        ui,
                        &self.desired_output_ingredient,
                        self.desired_output_rate,
                        &self.recipe_db.known_recipes,
                    );
                }
                SelectedTab::Editing => {
                    // Add Ingredients
                    ui.horizontal(|ui| {
                        let add_ingredient_edit =
                            egui::TextEdit::singleline(&mut self.add_ingredient_text)
                                .hint_text("Add ingredient");
                        let button_clicked = ui.button("Add").clicked();
                        let text_response = ui.add(add_ingredient_edit);

                        if !self.add_ingredient_text.is_empty()
                            && (text_response.lost_focus()
                                && ui.input(|i| i.key_pressed(egui::Key::Enter))
                                || button_clicked)
                        {
                            let new_ing = Ingredient {
                                name: self.add_ingredient_text.clone(),
                            };
                            if !self.recipe_db.known_ingredients.contains(&new_ing) {
                                self.recipe_db.known_ingredients.push(new_ing)
                            }
                        }
                    });

                    ui.separator();

                    // Add Recipes
                    ui.add_enabled_ui(!self.recipe_db.known_ingredients.is_empty(), |ui| {
                        // Output dropdown
                        ui.horizontal(|ui| {
                            ui.label("Output");
                            output_ingredient_selector(ui, &self.recipe_db, &mut self.recipe_builder)
                        });

                        // Craft time
                        ui.horizontal(|ui| {
                            let dragval = egui::DragValue::new(&mut self.recipe_builder.craft_time)
                                .clamp_range(0.0..=f32::MAX)
                                .max_decimals(1);
                            ui.label("Craft time ");
                            ui.add(dragval);
                        });

                        // Inputs
                        ui.label("Inputs");
                        input_ingredient_selectors(ui, &self.recipe_db, &mut self.recipe_builder);

                        if ui.button("+").clicked() {
                            self.recipe_builder.add_blank_input();
                        }
                    });

                    ui.separator();

                    // Add recipe to system

                    // Validity check is only partial here; just checks for whether each input and output are non-zero
                    // The rest of validity checking is done by the recipe builder on click,
                    // saves frame-by-frame computation, although this would likely be trivial anyway
                    fn ingredient_is_empty(ing_w_c: &IngredientWithCount) -> bool {
                        ing_w_c.count == 0 || ing_w_c.ing.name.is_empty()
                    }
                    let valid_recipe = !(ingredient_is_empty(&self.recipe_builder.output_ingredient)
                        || self.recipe_builder.inputs()
                            .any(ingredient_is_empty));
                    let add_recipe_button = egui::Button::new("Add Recipe");
                    if ui.add_enabled(valid_recipe, add_recipe_button).clicked() {
                        match self.recipe_builder.build_recipe(&mut self.recipe_db) {
                            Ok(_) => (),
                            Err(_) => eprintln!("Broken recipe detected, not adding")
                        }
                    }
                }
            }
        });
    }
}

fn display_rates_info(
    ui: &mut egui::Ui,
    output_ingredient: &Ingredient,
    output_rate: f32,
    known_recipes: &HashMap<Ingredient, Recipe>,
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

    let (num_producers, input_rates) =
        compute_required_rates(output_ingredient, output_rate, known_recipes);
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
    known_recipes: &HashMap<Ingredient, Recipe>,
) -> (f32, Option<Vec<(Ingredient, f32)>>) {
    if let Some(recipe) = known_recipes.get(output_ingredient) {
        let cycles_per_sec = output_rate / (recipe.output_num as f32);
        let num_producers = cycles_per_sec * recipe.craft_time;

        let mut required_input_rates = Vec::with_capacity(recipe.inputs.len());
        for input_ing in recipe.inputs.iter() {
            let input_rate = (input_ing.count as f32) * cycles_per_sec;
            required_input_rates.push((input_ing.ing.clone(), input_rate));
        }
        (num_producers, Some(required_input_rates))
    } else {
        (0.0, None)
    }
}

fn input_ingredient_selectors(
    ui: &mut egui::Ui,
    rdb: &RecipeDB,
    recipe_builder: &mut RecipeBuilder,
) {
    recipe_builder.recompute_available_ingredients(rdb);

    let mut remove_input = None;
    for i in 0..recipe_builder.num_inputs() {

        if let Some(current) = recipe_builder.get_input(i) {
            let dropdown = egui::ComboBox::from_id_source(i).selected_text(&current.ing.name);
            ui.horizontal(|ui| {
                {
                    let dragval = egui::DragValue::new(recipe_builder.get_input_count_mut(i))
                        .clamp_range(0..=100)
                        .max_decimals(0);
                    ui.add(dragval);
                }
                dropdown.show_ui(ui, |ui| {
                    for ing in recipe_builder.available_ingredients().clone() {
                        if ui
                            .selectable_label(false, &ing.name)
                            .clicked()
                        {
                            recipe_builder.change_input_ingredient(i, ing);
                        }
                    }
                });
                if ui.button("X").clicked() {
                    remove_input = Some(i);
                }
            });
        }

    }
    if let Some(i) = remove_input {
        recipe_builder.remove_input(i);
    }
}

fn output_ingredient_selector(ui: &mut egui::Ui, rdb: &RecipeDB, recipe_builder: &mut RecipeBuilder) {
    let dropdown = egui::ComboBox::from_id_source("add_ingredient_output")
        .selected_text(&recipe_builder.output_ingredient.ing.name);
    ui.horizontal(|ui| {
        {
            let dragval = egui::DragValue::new(&mut recipe_builder.output_ingredient.count)
                .clamp_range(0..=100)
                .max_decimals(0);
            ui.add(dragval);
        }
        dropdown.show_ui(ui, |ui| {
            for ing in &rdb.known_ingredients {
                if *ing != recipe_builder.output_ingredient.ing && ui
                    .selectable_label(false, &ing.name)
                    .clicked()
                {
                    recipe_builder.change_output_ingredient(ing.clone());
                }
            }
        });
    });
}