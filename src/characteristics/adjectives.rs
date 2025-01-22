use std::io;
use crate::core::character::Character;
use crate::core::characteristics::Characteristic;

pub struct Adjectives;

impl Characteristic for Adjectives {
    fn get_header(&self) -> String {
        "Adjectives:".to_string()
    }

    fn get_traits(&self, character: &Character) -> io::Result<String> {
        Ok(character.adjectives.join("\n"))
    }
} 