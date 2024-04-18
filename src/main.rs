use std::collections::HashMap;
use eframe::egui::{self, Vec2};
use egui::Response;

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
                
                if text_response.lost_focus()
                && ui.input(|i| i.key_pressed(egui::Key::Enter))
                || button_clicked {
                    self.known_ingredients.push(Ingredient { name: self.add_ingredient_text.clone() });
                }
            });

            ui.separator();

            // Add Recipes
            ui.add_enabled_ui(self.known_ingredients.len() > 0, |ui| {
                

                // Output dropdown
                ui.label("Output");
                ingredient_selector(ui,
                    &self.known_ingredients,
                    &mut self.add_recipe_output_ingredient
                );
                ui.label("Inputs");
            });

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

fn ingredient_selector(ui: &mut egui::Ui, ingredient_list: &[Ingredient], selection: &mut IngredientWithCount) {
    let selected_ing = selection.0.clone();
    let dropdown = egui::ComboBox::from_id_source(&selected_ing).selected_text(&selected_ing.name);
    let dragval = egui::DragValue::new(&mut selection.1).clamp_range(0..=100).max_decimals(0);
    ui.horizontal(|ui| {
        ui.add(dragval);
        if ingredient_list.is_empty() {
            dropdown.show_ui(ui, |ui| {ui.label("- - -")});
        } else {
            dropdown.show_ui(ui, |ui| {
                for ingredient in ingredient_list {
                    if ui.selectable_label(
                        *ingredient == selected_ing,
                        &ingredient.name
                    ).clicked() {
                        selection.0 = ingredient.clone();
                    }
                }
            });
        }
    });
}

#[derive(Default, Hash, Clone, PartialEq)]
struct Ingredient {
    pub name: String
}
type IngredientWithCount = (Ingredient, i32);
struct Recipe {
    craft_time: f32,
    output: IngredientWithCount,
    input: Vec<IngredientWithCount>
}