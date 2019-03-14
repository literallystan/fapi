extern crate yaml_rust;
use std::fs;
use yaml_rust::YamlLoader;

fn main() {
    let contents = fs::read_to_string("./src/test.yaml")
        .expect("Couldn't read the file");

    let docs = &YamlLoader::load_from_str(&contents).unwrap()[0];

    println!("{}",docs["foo"][0].as_str().unwrap());
}
