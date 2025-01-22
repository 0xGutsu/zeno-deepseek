use serde::Deserialize;

#[derive(Deserialize)]
pub struct CharacterBio {
    pub headline: String,
    pub key_traits: Vec<String>,
}

#[derive(Deserialize)]
pub struct CharacterInstructions {
    pub base: String,
    pub suffix: String,
}

#[derive(Deserialize)]
pub struct Character {
    pub instructions: CharacterInstructions,
    pub adjectives: Vec<String>,
    pub bio: CharacterBio,
    pub lore: Vec<String>,
    pub styles: Vec<String>,
    pub topics: Vec<String>,
    pub post_style_examples: Vec<String>,
} 