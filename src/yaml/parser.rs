use std::{
    fs,
    io,
    path::Path,
};
use yaml_rust::{
    Yaml,
    YamlLoader,
};

pub struct YamlParser;

impl YamlParser {
    pub fn read_file(yaml_path: &Path) -> io::Result<Yaml> {
        let content = fs::read_to_string(yaml_path)?;
        println!("{content}");

        let docs = YamlLoader::load_from_str(content.as_str()).unwrap();
        let doc = &docs[0];
        let assets = &doc["flutter"]["assets"];

        println!("{:?}", doc);
        println!("Assets: {:?}", assets);

        Ok(assets.to_owned())
    }
}
