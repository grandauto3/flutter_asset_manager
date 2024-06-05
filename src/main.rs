#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use anyhow::Result;
use iced::{Command, font, Sandbox, Settings};

use crate::{
    core::app::App,
    yaml::parser::YamlParser
};

mod ui;
mod yaml;
mod core;

fn main() -> Result<()> {
    let asset_list = YamlParser::read_file()?;

    let _ = Command::batch(vec![
        font::load(iced_aw::BOOTSTRAP_FONT_BYTES)
    ]);
    App::run(Settings::default())?;

    Ok(())
}

