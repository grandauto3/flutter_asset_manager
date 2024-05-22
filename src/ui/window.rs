use eframe::egui::Context;
use eframe::{egui, Frame};

#[derive(Default)]
pub struct Ui {}

impl eframe::App for Ui {
     fn update(&mut self, ctx: &Context, frame: &mut Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Hello World!");
        });
    }
}

impl Ui {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        Self::default()
    }
}
