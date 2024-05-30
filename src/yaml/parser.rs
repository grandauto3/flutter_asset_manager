use std::{fs, io};
use yaml_rust::{Yaml, YamlLoader};

pub struct YamlParser;

impl YamlParser {
    pub fn read_file() -> io::Result<Yaml>{
        let content = fs::read_to_string("pubspec.yaml")?;
        println!("{content}");

        let docs = YamlLoader::load_from_str(content.as_str()).unwrap();
        let doc = &docs[0];
        let assets = &doc["flutter"]["assets"];

        println!("{:?}", doc);
        println!("Assets: {:?}", assets);

        Ok(assets.to_owned())
    }
}
