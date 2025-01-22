use std::io;
use super::character::Character;
use crate::characteristics::{
    bio::Bio,
    lore::Lore,
    adjectives::Adjectives,
    post_examples::PostExamples,
    styles::Styles,
    topics::Topics, 
};

pub trait Characteristic {
    fn get_header(&self) -> String;
    fn get_traits(&self, character: &Character) -> io::Result<String>;
}

pub struct Characteristics;

impl Characteristics {
    pub fn get_characteristics() -> Vec<Box<dyn Characteristic>> {
        vec![
            Box::new(Bio),
            Box::new(Lore),
            Box::new(PostExamples),
            Box::new(Adjectives),
            Box::new(Styles),
            Box::new(Topics),
        ]
    }

    pub fn build_characteristics_instructions(character: &Character) -> String {
        let mut chars_instruction = String::new();
        let characteristics = Self::get_characteristics();

        for characteristic in characteristics {
            chars_instruction += &characteristic.get_header();
            chars_instruction += "\n";
            chars_instruction += &characteristic.get_traits(character).unwrap();
            chars_instruction += "\n";
        }

        chars_instruction
    }
}
