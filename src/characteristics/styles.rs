use std::io;
use crate::core::character::Character;
use crate::core::characteristics::Characteristic;

pub struct Styles;

impl Characteristic for Styles {
    fn get_header(&self) -> String {
        "Styles:".to_string()
    }

    fn get_traits(&self, character: &Character) -> io::Result<String> {
        Ok(character.styles.join("\n"))
    }
} 