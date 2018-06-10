#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate toml;

use std::path::Path;
use std::fs;
use std::error::Error;

#[derive(Deserialize)]
pub struct CargoToml {
    pub package: Package,
}

#[derive(Deserialize)]
pub struct Package {
    pub name: String,
    pub version: String,
    pub license: Option<String>,
    pub description: Option<String>,
}

pub fn read_cargo_toml(path: impl AsRef<Path>) -> Result<CargoToml, Box<Error>> {
    let string_representation = fs::read_to_string(path)?;
    toml::from_str(&string_representation).map_err(|e| e.into())
}

