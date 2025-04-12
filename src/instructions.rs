use crate::{context::Context, rom_reader};
use byteorder::{BigEndian, ByteOrder, LittleEndian, ReadBytesExt};

pub fn interpret(c: &mut Context, opcode: u8) -> Result<(), ()> {
    let mut bytes_count = 1;
    println!("opcode: {:x}", opcode);
    match opcode {
        0x00 => (),
        0x03 => {
            c.write_bc_register(c.read_bc_register() + 1);
            c.write_af_register(c.read_af_register() & 0xFF00);
            let mut af = c.read_af_register();

            af |= ((c.read_bc_register() == 0) as u16) << 6;
            af |= 0 << 5;
            af |= ((c.read_c_register() == 0) as u16) << 4;

            c.write_af_register(af);
        }
        0x18 => {
            // JR e8
            //bytes_count = 2;
            let bytes = rom_reader::read_n_bytes_at_offset(c.get_rom_file(), c.get_pc() as u64, 2)
                .expect("could not read bytes");
            //println!("{:b}", bytes[1]);
            c.add_pc_signed(bytes[1] as i8);
            println!("{:b}, opcode: {:b}, jump de {}", c.get_pc(), opcode, bytes[1] as i8);
            return Ok(());
        },
        0x1F => todo!(),
        0x28 =>
        // JR Z, e8
        {
            bytes_count = 2;
            let bytes = rom_reader::read_n_bytes_at_offset(c.get_rom_file(), c.get_pc() as u64, 2)
                .expect("could not read bytes");
            let f = c.read_flags_register();
            let z = (f >> 7) & 1;
            if z == 1 
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
            //println!("{:x?}", bytes);
            println!("{:x}, opcode : {:x}, writing {} in A", c.get_pc(), opcode, bytes[1]);
            c.write_af_register((c.read_af_register() & 0x00FF) | (bytes[1] as u16) << 4);
        },
        0xAF =>
        // XOR A,A
        {
            let mut af = c.read_af_register();

            af |= 1 << 6;
            af |= 0 << 5;
            af |= 0 << 4;
            af |= 0 << 3;

            c.write_af_register(af & 0x00FF);
            println!("{:b}, opcode : {:b}, reset de a", c.get_pc(), opcode);
        },
        0xC3 => {
            // bytes_count = 3;
            let bytes = rom_reader::read_n_bytes_at_offset(c.get_rom_file(), c.get_pc() as u64, 3)
                .expect("could not read bytes");
            let flags: u8 = c.read_flags_register();
            let z = flags >> 7;
            if z == 0 {
                c.attr_pc(LittleEndian::read_u16(&bytes[1..=2]));
            }
            return Ok(());
        },
        0xCF => todo!(),
        0xEA => 
        // LD [a16], A
        {
            bytes_count = 3;
            let bytes = rom_reader::read_n_bytes_at_offset(c.get_rom_file(), c.get_pc() as u64, 3)
                .expect("could not read bytes");
            c.write_in_memory(1, vec!(c.read_a_register()), LittleEndian::read_u16(&bytes[1..=2]));
            println!("{:x}, opcode: {:x}, writing at adress ram[{:x}] the value in A : {}", c.get_pc(), opcode, LittleEndian::read_u16(&bytes[1..=2]), c.read_a_register());
        },
        0xFE => {
            bytes_count = 2;
            let bytes = rom_reader::read_n_bytes_at_offset(c.get_rom_file(), c.get_pc() as u64, 2)
                .expect("could not read bytes");
            let val = bytes[1];
            let res: i16 = val as i16 - c.read_a_register() as i16;
            let mut af = c.read_af_register();
            af |= ((res == 0) as u16) << 6;
            af |= 1 << 5;
            af |= ((res < 0) as u16) << 4;
            c.write_af_register(af);
        }
        _ => println!("Missing OpCode : {:x}", opcode),
    }
    c.add_pc(bytes_count as u16);
    Ok(())
}
