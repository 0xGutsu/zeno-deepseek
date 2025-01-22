use std::io;
use crate::core::character::Character;
use crate::core::characteristics::Characteristic;

pub struct Topics;

impl Characteristic for Topics {
    fn get_header(&self) -> String {
        "Topics:".to_string()
    }

    fn get_traits(&self, character: &Character) -> io::Result<String> {
        Ok(character.topics.join("\n"))
    }
} 