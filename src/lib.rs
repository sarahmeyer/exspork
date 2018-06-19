#[macro_use]
extern crate nom;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate toml;

pub mod parser;

use std::path::Path;
use std::fs;
use std::error::Error;

// todo
// generate doc stub per "export function" statement in wasm_project.d.ts
//  - do we have access to anything more descriptive than the typescript file?
// export generated readme to wasm_project directory
//

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

pub fn generate_markdown() -> Result<(), Box<Error>> {
    let config = read_cargo_toml("Cargo.toml")?;

    let mut markdown = String::new();

    markdown.push_str(&format!("# {}\n", config.package.name));
    markdown.push_str(&format!("### Version: {}\n", config.package.version));

    if let Some(description) = config.package.description {
        markdown.push_str(&description);
        markdown.push('\n');
    }

    if let Some(license) = config.package.license {
        markdown.push_str(&format!("## License\n{}\n", license));
    }

    fs::write("generated_readme.md", &markdown)?;

    Ok(())
}
