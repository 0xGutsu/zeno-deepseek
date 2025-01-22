use std::io;
use crate::core::character::Character;
use crate::core::characteristics::Characteristic;

pub struct Lore;

impl Characteristic for Lore {
    fn get_header(&self) -> String {
        "Lore:".to_string()
    }

    fn get_traits(&self, character: &Character) -> io::Result<String> {
        Ok(character.lore.join("\n"))
    }
} 