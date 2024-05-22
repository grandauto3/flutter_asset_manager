#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod ui;
mod yaml;

use std::io;

use crate::{
    ui::window,
    yaml::parser::{YamlParser},
};


fn main() -> io::Result<()> {
    YamlParser::read_file()?;

    let native_options = eframe::NativeOptions::default();
    eframe::run_native("Window title",
                       native_options,
                       Box::new(|cc| Box::new(window::Ui::new(cc))),
    ).unwrap();

    Ok(())
}

