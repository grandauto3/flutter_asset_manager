use std::{fs, io};
use yaml_rust::YamlLoader;
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod ui;

use crate::{
    ui::window,
};

fn main() -> io::Result<()> {
    let content = fs::read_to_string("pubspec.yaml")?;
    println!("{content}");

    let docs = YamlLoader::load_from_str(content.as_str()).unwrap();
    let doc = &docs[0];
    let assets = &doc["flutter"]["assets"];

    println!("{:?}", doc);
    println!("Assets: {:?}", assets);
    let native_options = eframe::NativeOptions::default();
    eframe::run_native("Window title",
                       native_options,
                       Box::new(|cc| Box::new(window::Ui::new(cc))),
    ).unwrap();

    Ok(())
}

