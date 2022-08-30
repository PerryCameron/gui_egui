#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use eframe::egui;


const STRING_OPTIONS: [&str; 16] = [
    "Option 1", "Option 2", "Option 3", "Option 4", "Option 5", "Option 6", "Option 7", "Option 8",
    "Option 9", "Option 10", "Option 11", "Option 12", "Option 13", "Option 14", "Option 15", "Option 16",
];

fn main() {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Perry's Test Application", // this sets the name in the title bar
        options,
        Box::new(|_cc| Box::new(MyApp::default())),
    );
}

struct MyApp {
    name: String,
    age: u32,
    radio_position: Enum,
    string_option: String,
    happy: bool,
}

#[derive(PartialEq)]
#[derive(Debug)]
enum Enum {
    First,
    Second,
    Third,
}

fn radio_select(s: &Enum, i: u16) -> bool {
    match s {
        Enum::First if i == 1 => true,
        Enum::Second if i == 2 => true,
        Enum::Third if i ==3 => true,
        _ => false,
    }
}

fn check_select(b: bool) -> bool {
    if b { false }
    else { true }
}

fn radio_text(s: &Enum) -> String {
    match s {
        Enum::First => String::from("First"),
        Enum::Second => String::from("Second"),
        Enum::Third => String::from("Third"),
    }
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            name: "Arthur".to_owned(),
            age: 42,
            radio_position: Enum::First,
            string_option: String::from(""),
            happy: true,
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("This is a heading");
            ui.horizontal(|ui| {
                ui.label("Your name: ");
                ui.text_edit_singleline(&mut self.name);
            });
            ui.separator();
            let cmd = "cargo install puffin_viewer && puffin_viewer --url 127.0.0.1:8585";
            ui.horizontal(|ui| {
                ui.monospace(cmd);
                if ui.small_button("📋").clicked() {
                    ui.output().copied_text = cmd.into();
                }
            });
            ui.separator();
            ui.with_layout(egui::Layout::left_to_right(egui::Align::TOP), |ui| {
                if ui.add(egui::RadioButton::new(radio_select(&self.radio_position, 1), "First")).clicked() {
                    self.radio_position = Enum::First;
                }
                if ui.add(egui::RadioButton::new(radio_select(&self.radio_position, 2), "Second")).clicked() {
                    self.radio_position = Enum::Second;
                }
                if ui.add(egui::RadioButton::new(radio_select(&self.radio_position, 3), "Third")).clicked() {
                    self.radio_position = Enum::Third;
                }
            });
            ui.separator();
            ui.horizontal(|ui| {
                ui.spacing_mut().item_spacing.x = 2.0; // remove spacing between widgets
                ui.label("Radio Selected: ");
                ui.text_edit_singleline(&mut radio_text(&self.radio_position));
                ui.text_edit_singleline(&mut radio_text(&self.radio_position));
            });
            ui.separator();
            ui.add(egui::Slider::new(&mut self.age, 0..=120).text("age"));
            ui.separator();
            ui.horizontal(|ui| {
                if ui.button("Add 1 to year").clicked() {
                    self.age += 1;
                }
                if ui.button("Add 2 to year").clicked() {
                    self.age += 2;
                }

                // lets throw a combobox in here for the hell of it
                egui::ComboBox::from_label(format!(
                    "Currently selected string: {}",
                    self.string_option
                ))
                    .selected_text(self.string_option.clone())
                    .show_ui(ui, |ui| {
                        for option in STRING_OPTIONS {
                            // Selectable values can be anything: enums, strings or integers - as long as they can be compared and have a text repersentation
                            ui.selectable_value(&mut self.string_option, option.into(), option);
                        }
                    });

            });
            ui.separator();
            ui.label(format!("Hello '{}', age {}", self.name, self.age));
            // ui.separator();
            // ui.spinner();
            ui.separator();
            if ui.add(egui::Checkbox::new(&mut self.happy, "Checked")).clicked() {
                check_select(self.happy);
                println!("I am happy= {}",self.happy);
            }
            ui.separator();
            ui.hyperlink("https://eaglecreeksailing.com");
            ui.separator();
            let mut my_f32:f32 = 2.0;
            ui.collapsing("Click to see what is hidden!", |ui| {
                ui.label("Not much, as it turns out");
                ui.add(egui::DragValue::new(&mut my_f32).speed(0.1));
            });
            ui.separator();
        });
    }
}
