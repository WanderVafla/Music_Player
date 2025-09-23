// Import necessary stuff from the eframe crate
use eframe::egui;

// Main entry point of the program
fn main() -> Result<(), eframe::Error> {
    // Default window settings
    let options = eframe::NativeOptions::default();

    // Start the egui application!
    eframe::run_native(
        "egui Demo", // Window title
        options,     // Window settings
        // This creates our app state. Don't worry too much about Box::new for now.
        Box::new(|_cc| Ok(Box::new(MyApp::default()))),
    )
}

// This struct holds our application's data (state)
// `#[derive(Default)]` makes it easy to create a starting instance
#[derive(Default)]
struct MyApp {
    label: String, // For a text input field
    value: f32,    // For a slider
    // We will add more fields here later!
}

// Implement the eframe::App trait for MyApp, telling eframe how to run it
impl eframe::App for MyApp {
    // The `update` method is called every frame to draw the UI
    // `&mut self` allows this method to modify MyApp's fields
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Show a central panel where we'll put our UI
        egui::CentralPanel::default().show(ctx, |ui| {
            // `ui` is our tool to add widgets

            // Add a heading
            ui.heading("My egui Application");

            // Arrange items horizontally
            ui.horizontal(|ui| {
                ui.label("Write something: ");
                // Text input linked to `self.label`
                // `&mut self.label` lets the widget change the `label` field
                ui.text_edit_singleline(&mut self.label);
            });

            // Add a slider linked to `self.value`
            ui.add(egui::Slider::new(&mut self.value, 0.0..=10.0).text("value"));

            // Add a button
            if ui.button("Increment").clicked() {
                // If clicked, increase `self.value`
                self.value += 1.0;
            }

            // Display the current state in a label
            ui.label(format!("Hello '{}', value: {}", self.label, self.value));
        });
    }
}
