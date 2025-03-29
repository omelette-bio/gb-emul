use std::fs::File;
use std::io::Result;
use std::path::Path;

mod rom_reader;

fn main() -> Result<()>{
    
    let rom_path = Path::new("roms/Pokemon_Blue_USA.gb");
    let rom_file = File::open(rom_path)?;
    
    // 100-103 : nop puis jump 0x150 donc 00 C3 05 01 
    // 104-133 : logo nintendo encode
    // 134-143 : titre du jeu
    // 144-145 : new licencee code

    let res_bytes = rom_reader::read_n_bytes_at_offset(&rom_file, 0x134, 10);


    match res_bytes {
        Ok(bytes) => 
            {
                //println!("Reading first opcode... : {:02X}", bytes[0]);
                //println!("Reading second opcode... : {:02X}", bytes[1]);
                println!("Entry point bytes (0x100-0x103): {:02X?}", bytes);
                println!("{}", std::str::from_utf8(&bytes).unwrap());
                /*println!("Byte at 0x{:X}: 0x{:02X}", 0x100, bytes[0]);
                println!("Byte at 0x{:X}: 0x{:02X}", 0x101, bytes[1]);
                println!("Byte at 0x{:X}: 0x{:02X}", 0x102, bytes[2]);
                println!("Byte at 0x{:X}: 0x{:02X}", 0x103, bytes[3]);*/
            }
        Err(e) => {eprintln!("Error reading rom: {}", e);}
    }

    //let mut file = File::open("~/00-projects/gb-emul/roms/Pokemon_Green_USA.gb")?;
    //let mut contents = String::new();
    //file.read_to_string(&mut contents);
    //println!("{}", contents.bytes().nth(0x100).unwrap());
    Ok(())
}
