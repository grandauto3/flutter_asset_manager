#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use anyhow::Result;
use iced::{
    Application,
    Settings,
};

use crate::{
    core::app::App,
    yaml::parser::YamlParser,
};

mod ui;
mod yaml;
mod core;

fn main() -> Result<()> {
    let asset_list = YamlParser::read_file("./pubspec.yaml".as_ref())?;

    App::run(Settings::default())?;

    Ok(())
}

