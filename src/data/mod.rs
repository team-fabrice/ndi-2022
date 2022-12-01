use std::collections::HashMap;
use std::fs::File;
use std::path::Path;
use walkdir::WalkDir;
use crate::data::card_definition::CardDefinition;

pub mod card_definition;

#[derive(Debug)]
struct Data {
    cards: HashMap<String, CardDefinition>,
}

impl Data {
    pub fn load() -> Self {
        let mut cards = HashMap::new();

        for file in WalkDir::new("data/cards") {
            let file = file.unwrap();
            if file.metadata().unwrap().is_file() {
                let id = file.path().strip_prefix("data/cards/").unwrap().to_str().unwrap().strip_suffix(".toml").unwrap().to_string().replace("/", "_");
                let file = std::fs::read(file.path()).unwrap();
                cards.insert(id, toml::from_slice::<CardDefinition>(&file).unwrap());
            }
        }

        Self {
            cards,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load() {
        dbg!(Data::load());
    }
}
