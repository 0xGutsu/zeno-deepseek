use std::io;
use crate::core::character::Character;
use crate::core::characteristics::Characteristic;

pub struct Bio;

impl Characteristic for Bio {
    fn get_header(&self) -> String {
        "Bio:".to_string()
    }

    fn get_traits(&self, character: &Character) -> io::Result<String> {
        Ok(format!("Headline: {}\nKey Traits:\n{}", 
            character.bio.headline,
            character.bio.key_traits.join("\n")
        ))
    }
} 