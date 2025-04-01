use serde::{Serialize, Deserialize};
use std::io::ErrorKind;
use std::io::Error;
use crate::context::Context;

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

impl Instruction
{
    pub fn interpret(&self, c: &mut Context, bytes: &Vec<u8>)
    {
        match self.mnemonic.as_str()
        {
            "CP" => 
                {
                    let val = match self.operands[1].name.as_str()
                        {
                            "n8" => bytes[1],
                            _ => 0,
                        };
                    let res: i16 = val as i16 - c.read_a_register() as i16;
                    let mut af = c.read_af_register();
                    af |= ((res != 0) as u16) << 6;
                    af |= 1 << 5;
                    af |= ((res < 0) as u16) << 4;
                    c.write_af_register(af);
                }
            _ => println!("c'est autre chose ! {} {}", self.mnemonic, self.operands.iter().map(|op| op.name.as_str()).collect::<Vec<_>>().join(", ")),

        }
        c.add_pc(bytes.len() as u16);
    }
}

