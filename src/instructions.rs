use serde::{Serialize, Deserialize};
use std::io::ErrorKind;
use std::io::Error;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Operand {
    pub name: String,
    #[serde(default)]
    pub immediate: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bytes: Option<u8>
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Flags {
    pub Z: String,
    pub N: String,
    pub H: String,
    pub C: String
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Instruction {
    pub immediate: bool, 
    pub operands: Vec<Operand>,
    pub cycles: Vec<u8>,
    pub bytes: u8,
    pub mnemonic: String,
    pub flags: Flags,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InstructionSet {
    pub unprefixed: std::collections::HashMap<String, Instruction>,
}


impl InstructionSet
{
    pub fn get_instruction(&self, opcode: u8) -> std::io::Result<Instruction>
    {
        self.unprefixed
            .get(&format!("0x{:X}",opcode))
            .ok_or_else(|| Error::new(
                ErrorKind::NotFound,
                format!("Opcode 0x{:02X} not found.", opcode)
            )).cloned()
    }
}
