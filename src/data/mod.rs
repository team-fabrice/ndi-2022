use std::collections::HashMap;
use std::fs::File;
use std::path::Path;
use crate::data::card_definition::CardDefinition;

pub mod card_definition;

#[derive(Debug)]
struct Data {
    cards: HashMap<String, CardDefinition>,
}

impl Data {
    pub fn load() -> Self {
        let mut cards = HashMap::new();

        for file in std::fs::read_dir("data/cards").unwrap() {
            let file = file.unwrap();
            let id = file.file_name().into_string().unwrap();
            let file = std::fs::read(file.path()).unwrap();
            cards.insert(id, toml::from_slice::<CardDefinition>(&file).unwrap());
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
