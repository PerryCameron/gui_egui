#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use eframe::egui;

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

            if ui.add(egui::RadioButton::new(radio_select(&self.radio_position,1), "First")).clicked() {
                self.radio_position = Enum::First;
                println!("{:?} Clicked",&self.radio_position);
            }
            if ui.add(egui::RadioButton::new(radio_select(&self.radio_position,2), "Second")).clicked() {
                self.radio_position = Enum::Second;
                println!("{:?} Clicked",&self.radio_position);
            }
            if ui.add(egui::RadioButton::new(radio_select(&self.radio_position,3), "Third")).clicked() {
                self.radio_position = Enum::Third;
                println!("{:?} Clicked",&self.radio_position);
            }

            ui.separator();
            ui.horizontal(|ui| {
                ui.label("Radio Selected: ");
                ui.text_edit_singleline(&mut radio_text(&self.radio_position));
            });
            ui.separator();
            ui.add(egui::Slider::new(&mut self.age, 0..=120).text("age"));
            ui.separator();
            if ui.button("Click each year").clicked() {
                self.age += 1;
            }
            ui.separator();
            ui.label(format!("Hello '{}', age {}", self.name, self.age));
            ui.separator();
            ui.spinner();
            ui.separator();
            if ui.add(egui::Checkbox::new(&mut self.happy, "Checked")).clicked() {
                if self.happy == true { self.happy = false; }
                else { self.happy = true; }
                println!("I am happy= {}",self.happy);
            }

        });
    }
}
