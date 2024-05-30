use eframe::egui::Context;
use eframe::{egui, Frame};
use yaml_rust::Yaml;

pub struct Ui {
    yaml: Yaml,
}

impl eframe::App for Ui {
    fn update(&mut self, ctx: &Context, frame: &mut Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Hello World!");
            if let Yaml::Array(arr) = &self.yaml {
                arr.iter().for_each(|element| {
                    ui.label(element.as_str().unwrap());
                });
            };
        });
    }
}

impl Default for Ui {
    fn default() -> Self {
        Self { yaml: Yaml::Null }
    }
}

impl Ui {
    pub fn new(cc: &eframe::CreationContext<'_>, yaml: Yaml) -> Self {
        Self { yaml }
    }
}
