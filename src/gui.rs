

use std::{collections::HashMap, fmt::Error};

use eframe::egui;

use crate::{process::get_vars, ErrorState};


pub fn start_app() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Your Calculator",
        options,
        Box::new(|_cc| Box::new(MyApp::default())),
    )
}

struct MyApp {
    equation_input: String,
    error_state: ErrorState,
    vars: HashMap<String, String>,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            equation_input : String::new(),
            error_state: ErrorState::NoneState,
            vars: HashMap::new(),
        }
    }
}

impl MyApp {
    fn clear_error(&mut self){
        self.error_state = ErrorState::NoneState;
    }
    fn set_error(&mut self, state: ErrorState){
        self.error_state = state;
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Welcome to your calculator!");

            // Input field for eq
            ui.horizontal(|ui| {
                ui.label("Enter an equation:");
                ui.text_edit_singleline(&mut self.equation_input);
            });

            if ui.button("Process Equation").clicked() {
                // process variables
                match get_vars(&self.equation_input){
                    Ok(vars) => {self.vars = vars; self.clear_error();},
                    Err(()) => {self.set_error(ErrorState::IncorrectEquation); self.vars.clear();}
                }
            }

            if self.error_state == ErrorState::IncorrectEquation  {
                // Display error message appropriately
                ui.label(format!("Error! Equation '{}' is not a valid equation.", self.equation_input));
            } 
            else {     
                if self.vars.len() > 1 {
                    for (var, val) in self.vars.iter_mut() {
                        ui.horizontal(|ui| {
                            ui.label(format!("Variable {}: ", var));
                            ui.text_edit_singleline(val);
                        }); 
                    }
                }

                // solve equation given values
                if ui.button("Solve").clicked() {
                    
                    // see if one to solve for:
                    if self.vars.values().filter(|v| !v.is_empty()).count() == self.vars.len() - 1 {

                        // clear error state
                        self.clear_error();

                        // empty out inputs
                        for (_, v) in self.vars.iter_mut() {
                            v.clear();
                        }
                    } else {
                        self.set_error(ErrorState::ImproperNumberOfValues);
                    }

                }
            }

            if self.error_state == ErrorState::ImproperNumberOfValues {
                ui.label("Error! Number of values entered is not sufficient to solve for a single variable.");
            }

        });
    }
}

