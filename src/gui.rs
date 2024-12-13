use eframe::egui;

pub fn start_app() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Windows GUI Example",
        options,
        Box::new(|_cc| Box::new(MyApp::default())),
    )
}

struct MyApp {
    view_input: String,
    controller_input: String,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            view_input : String::new(),
            controller_input : String::new(),
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Hello, Windows!");

            // Input field
            ui.horizontal(|ui| {
                ui.label("Enter something:");
                ui.text_edit_singleline(&mut self.controller_input);
            });

            if ui.button("Send").clicked() {
                self.view_input = self.controller_input.clone();
            }

            // Display the current string
            ui.label(format!("Entry: {}", self.view_input));
        });
    }
}
