use crate::{context::Context, rom_reader};
use byteorder::{BigEndian, ByteOrder, LittleEndian, ReadBytesExt, WriteBytesExt};

pub fn interpret(c: &mut Context, opcode: u8) -> Result<(), ()> {
    let mut bytes_count = 1;
    println!("opcode: {:x}", opcode);
    match opcode {
        0x00 => (),
        0x03 => {
            c.write_bc_register(c.read_bc_register() + 1);
            
            c.reset_flags_register(); // c.write_af_register(c.read_af_register() & 0xFF00);

            c.write_z_flag(c.read_bc_register() == 0); // af |= ((c.read_bc_register() == 0) as u16) << 6;
            c.write_n_flag(false); // af |= 0 << 5;
            c.write_h_flag(c.read_c_register()==0); // af |= ((c.read_c_register() == 0) as u16) << 4;

        }
        0x18 => {
            // JR e8
            //bytes_count = 2;

            let bytes = rom_reader::read_n_bytes_at_offset(c.get_rom_file(), c.get_pc() as u64, 2)
                .expect("could not read bytes");
            
            c.add_pc_signed(bytes[1] as i8);

            println!("{:x}, opcode: {:x}, jump de {}", c.get_pc(), opcode, bytes[1] as i8);

            return Ok(());
        },
        0x1F => todo!(),
        0x28 =>
        // JR Z, e8
        {
            bytes_count = 2;
            let bytes = rom_reader::read_n_bytes_at_offset(c.get_rom_file(), c.get_pc() as u64, 2)
                .expect("could not read bytes");

            if c.get_z_flag()
            {
                c.add_pc_signed(bytes[1] as i8);
                return Ok(())
            }
        }
        0x3E =>
        // LD A, n8
        {
            bytes_count = 2;
            let bytes = rom_reader::read_n_bytes_at_offset(c.get_rom_file(), c.get_pc() as u64, 2)
                .expect("could not read bytes");
            
            println!("{:x}, opcode : {:x}, writing {} in A", c.get_pc(), opcode, bytes[1]);
            
            c.write_a_register(bytes[1]);
        },
        0xAF =>
        // XOR A,A
        {

            c.write_a_register(0);

            c.reset_flags_register();

            c.write_z_flag(true);
            c.write_n_flag(false);
            c.write_h_flag(false);
            c.write_c_flag(false);

            println!("{:x}, opcode : {:x}, reset de a", c.get_pc(), opcode);
        },
        0xC3 => {
            // JP a16

            // bytes_count = 3;
            let bytes = rom_reader::read_n_bytes_at_offset(c.get_rom_file(), c.get_pc() as u64, 3)
                .expect("could not read bytes");

            //println!("{:02X?}", bytes);

            c.attr_pc(LittleEndian::read_u16(&bytes[1..=2]));
            return Ok(());
        },
        0xCD =>
        // CALL n16
        {
            // push current pc onto the stack
            // jump to n16 adress
            // execution then when RET instruction called, return to previous pc

            // bytes_count = 3;

            let mut bytes: Vec<u8> = Vec::new();
            bytes.write_u16::<LittleEndian>(c.get_pc()).expect("error in parsing pc register");
            
            c.write_in_stack(2, bytes);
            
            let bytes = rom_reader::read_n_bytes_at_offset(c.get_rom_file(), c.get_pc() as u64, 3)
                .expect("could not read bytes");
            
            c.attr_pc(LittleEndian::read_u16(&bytes[1..=2]));

            return Ok(())

            
        },
        0xCF => todo!(),
        0xE0 =>
        // LDH [a8], A
        {
            bytes_count = 2;
            let bytes = rom_reader::read_n_bytes_at_offset(c.get_rom_file(), c.get_pc() as u64, 2)
                .expect("could not read bytes");
            
            c.write_in_memory(1, vec!(c.read_a_register()), 0xFF00 | bytes[1] as u16);
            
            println!("{:x}, opcode: {:x}, writing at adress ram[{:x}] the value in A : {}", c.get_pc(), opcode, 0xFF00 | bytes[1] as u16, c.read_a_register());
        },
        0xEA => 
        // LD [a16], A
        {
            bytes_count = 3;
            let bytes = rom_reader::read_n_bytes_at_offset(c.get_rom_file(), c.get_pc() as u64, 3)
                .expect("could not read bytes");

            c.write_in_memory(1, vec!(c.read_a_register()), LittleEndian::read_u16(&bytes[1..=2]));
            
            println!("{:x}, opcode: {:x}, writing at adress ram[{:x}] the value in A : {}", c.get_pc(), opcode, LittleEndian::read_u16(&bytes[1..=2]), c.read_a_register());
        },
        0xF0 => {
            bytes_count = 2;

            let bytes = rom_reader::read_n_bytes_at_offset(c.get_rom_file(), c.get_pc() as u64, 2)
                .expect("could not read bytes");
        
            c.write_a_register(c.read_byte_in_memory_at_offset(0xFF00 | bytes[1] as u16));
            // println!("{:?}", bytes);
            println!("{:x}, opcode: {:x}, writing in register A the value at ram[{:x}] : {:x}", c.get_pc(), opcode, 0xFF00 | bytes[1] as u16, c.read_byte_in_memory_at_offset(0xFF00 | bytes[1] as u16))
        }
        0xF3 => {
            c.set_iem_flag(false);
        }
        0xFE => {

            bytes_count = 2;
            let bytes = rom_reader::read_n_bytes_at_offset(c.get_rom_file(), c.get_pc() as u64, 2)
                .expect("could not read bytes");

            let res: i16 = bytes[1] as i16 - c.read_a_register() as i16;

            c.write_z_flag(res==0);
            c.write_n_flag(true);
            c.write_h_flag(res < 0);
        }
        _ => panic!("Missing OpCode : {:x}", opcode),
    }
    c.add_pc(bytes_count as u16);
    Ok(())
}
