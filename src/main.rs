#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod ui;
mod yaml;

use anyhow::{anyhow, Result};
use crate::{
    ui::window,
    yaml::parser::{YamlParser},
};


fn main() -> Result<()> {
    YamlParser::read_file()?;

    let native_options = eframe::NativeOptions::default();
    eframe::run_native("Window title",
                       native_options,
                       Box::new(|cc| Box::new(window::Ui::new(cc))),
    ).map_err(|error: eframe::Error| anyhow!(error.to_string()))?;

    Ok(())
}

