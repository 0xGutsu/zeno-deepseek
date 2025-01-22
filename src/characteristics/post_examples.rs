use std::io;
use crate::core::character::Character;
use crate::core::characteristics::Characteristic;

pub struct PostExamples;

impl Characteristic for PostExamples {
    fn get_header(&self) -> String {
        "Post Examples:".to_string()
    }

    fn get_traits(&self, character: &Character) -> io::Result<String> {
        Ok(character.post_style_examples.join("\n"))
    }
} 