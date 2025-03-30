use std::fs::File;
use std::io::Result;
use std::path::Path;
use byteorder::{ByteOrder, BigEndian, LittleEndian, ReadBytesExt};
use serde::{Serialize, Deserialize};
mod rom_reader;
use std::io::Read;
use std::fmt::LowerHex;
mod instructions;
use instructions::InstructionSet;
fn main() -> Result<()>{
    
    let rom_path = Path::new("roms/Pokemon_Blue_USA.gb");
    let instr_path = Path::new("Opcodes.json");
    let rom_file = File::open(rom_path)?;
    let mut instr_file = File::open(instr_path)?;

    let mut contents = String::new();
    instr_file.read_to_string(&mut contents)?;

    let instruction_set: InstructionSet = serde_json::from_str(&contents)?;
    //println!("{:?}", instruction_set.unprefixed.get("0x00"));

    // 100-103 : nop puis jump 0x150 donc 00 C3 50 01 
    // 104-133 : logo nintendo encode
    // 134-143 : titre du jeu
    // 144-145 : new licencee code

    /*
    let res_bytes = rom_reader::read_n_bytes_at_offset(&rom_file, 0x100, 4);


    match res_bytes {
        Ok(bytes) => 
            {
                println!("Entry point bytes (0x100-0x103): {:02X?}", bytes);
                println!("{:?}", instruction_set.unprefixed.get(&format!("{:x}",bytes[0])));
                println!("{:?}", &bytes[2..=3]); 
                println!("{}", LittleEndian::read_u16(&bytes[2..=3]));
            }
        Err(e) => {eprintln!("Error reading rom: {}", e);}
    }
    */
    let pc = 0x150;
    let mut instruction_bytes_count = 1;
    let res_bytes = rom_reader::read_byte_at_offset(&rom_file, pc);
    match res_bytes {
        Ok(byte) => 
            {
                let cur_inst = instruction_set.get_instruction(byte)?;
                instruction_bytes_count = cur_inst.bytes;
            }
        Err(e) => {eprintln!("Error reading rom: {}", e);}
    }
    let res_bytes = rom_reader::read_n_bytes_at_offset(&rom_file, pc, instruction_bytes_count as usize)?;
    println!("{:02X?}",res_bytes);
    /*
    match res_bytes {
        Ok(bytes) =>
            {
                let mut i: u16 = 0;
                while (true)
                {
                    let cur_inst = instruction_set.unprefixed.get(&format!("0x{:X}",bytes[i as usize])).unwrap();
                    println!("0x{:x}: {} {}", i+0x150, cur_inst.mnemonic, cur_inst.operands.iter().map(|op| op.name.as_str()).collect::<Vec<_>>().join(", "));
                    i += cur_inst.bytes as u16;
                    if (i >= 10) { break; }
                }
            }
        Err(e) => {eprintln!("Error reading rom: {}", e);}
    }
    */
    Ok(())
}
