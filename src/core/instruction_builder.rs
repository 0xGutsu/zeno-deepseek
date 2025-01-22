use std::fs;
use std::io;
use super::character::Character;
use super::characteristics::Characteristics;

pub struct InstructionBuilder {
    instructions: String,
}

impl InstructionBuilder {
    pub fn new() -> Self {
        Self {
            instructions: String::new(),
        }
    }

    pub fn load_character(character_name: &str) -> io::Result<Character> {
        let path = format!("./characters/{}/character.json", character_name);
        let data = fs::read_to_string(&path)?;
        serde_json::from_str(&data).map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))
    }

    pub fn build_instructions(&mut self, character_name: &str) -> io::Result<()> {
        self.instructions.clear();
        
        let character = Self::load_character(character_name)?;
        
        // Add base instructions
        self.add_instruction(&character.instructions.base);

        // Add characteristics
        let characteristics = Characteristics::build_characteristics_instructions(&character);
        self.add_instruction(&characteristics);

        // Add suffix instructions
        self.add_instruction(&character.instructions.suffix);

        Ok(())
    }

    // Add instruction to the internal buffer
    pub fn add_instruction(&mut self, instruction: &str) {
        self.instructions.push_str(instruction);
    }

    // Add multiple instructions (array equivalent)
    pub fn add_instructions(&mut self, instructions: Vec<String>) {
        for instruction in instructions {
            self.add_instruction(&instruction);
        }
    }

    // Get the complete instructions
    pub fn get_instructions(&self) -> &str {
        &self.instructions
    }
}
