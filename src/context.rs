use std::fs::File;

pub struct Context
{
    af: u16,
    bc: u16,
    de: u16,
    hl: u16,
    sp: u16,
    pc: u16,
    rom_path: File,
    ram: [u8; 65_536],
    iem: bool,
}

impl Context
{
    pub fn init(rom_file: File) -> Self { Context {af: 0, bc: 0, de: 0, hl: 0, sp: 0xDFFF, pc: 0, rom_path: rom_file, ram: [0; 65_536], iem: false} }



    pub fn read_af_register(&self) -> u16 { self.af }

    pub fn read_a_register(&self) -> u8 { (self.af >> 8) as u8 }

    pub fn read_flags_register(&self) -> u8 { self.af as u8 }

    pub fn reset_flags_register(&mut self) { self.af &= 0xFF00 }
    
    pub fn get_iem_flag(&self) -> bool {self.iem}

    pub fn set_iem_flag(&mut self, bit:bool) {self.iem = bit}
    
    pub fn write_af_register(&mut self, val: u16) { self.af = val; }


    pub fn write_z_flag(&mut self, bit: bool) { self.af |= (bit as u16) << 7; }

    pub fn get_z_flag(&self) -> bool { (self.af >> 7) & 1 == 1 }

    pub fn write_n_flag(&mut self, bit: bool) { self.af |= (bit as u16) << 6; }

    pub fn get_n_flag(&self) -> bool { (self.af >> 6) & 1 == 1 }

    pub fn write_h_flag(&mut self, bit: bool) { self.af |= (bit as u16) << 5; }

    pub fn get_h_flag(&self) -> bool { (self.af >> 5) & 1 == 1 }

    pub fn write_c_flag(&mut self, bit: bool) { self.af |= (bit as u16) << 4; }

    pub fn get_c_flag(&self) -> bool { (self.af >> 4) & 1 == 1 }

    pub fn write_a_register(&mut self, val: u8) { self.af = (val as u16) << 8 | self.read_flags_register() as u16 }


    pub fn read_bc_register(&self) -> u16 { self.bc }

    pub fn read_b_register(&self) -> u8 { (self.bc >> 8) as u8 }

    pub fn read_c_register(&self) -> u8 { self.bc as u8 }

    pub fn write_bc_register(&mut self, val: u16) { self.bc = val; }

    pub fn add_pc(&mut self, val: u16) { self.pc += val }

    pub fn add_pc_signed(&mut self, val: i8) { self.pc = (self.pc as i32 + val as i32) as u16 }

    pub fn attr_pc(&mut self, val: u16) { self.pc = val }

    pub fn incr_pc(&mut self) { self.add_pc(1); }

    pub fn get_pc(&self) -> u16 {self.pc}

    pub fn get_rom_file(&self) -> &File {&self.rom_path}

    pub fn read_byte_in_memory_at_offset(&self, offset: u16) -> u8
    {
        self.ram[offset as usize]
    }



    fn write_byte_in_memory(&mut self, byte: u8, offset: u16)
    {
        self.ram[offset as usize] = byte; 
    }

    pub fn write_in_memory(&mut self, bytes_count: u8, bytes: Vec<u8>, offset: u16)
    {
        for i in 0..bytes_count
        {
            self.write_byte_in_memory(bytes[i as usize], offset+i as u16);
            // self.ram[(offset+ (i as u16)) as usize] = bytes[i as usize];
        }
    }

    pub fn write_in_stack(&mut self, bytes_count: u8, bytes: Vec<u8>)
    {
        for i in 0..bytes_count
        {
            // println!("{:x} : {:x}", bytes[i as usize], self.sp);
            self.write_byte_in_memory(bytes[i as usize], self.sp);
            self.sp -= 1;
        }
    }

    pub fn print_stack(&self)
    {
        let mut show_ram: bool = false;

        println!("\n====RAM====");

        for i in 0xD000..=0xDFFF
        {
            if self.sp == i as u16 
            {
                println!("{:02X} : {:02X} <= stack pointer", i, self.ram[i]);
                show_ram = true;
            }
            else if self.ram[i] != 0 || show_ram
            {
                println!("{:02X} : {:02X}", i , self.ram[i]);
            }
        }

        println!("====RAM====\n");
    }

    pub fn print_state(&self)
    {
        println!("\n====CPU-STATE====");
        println!("A: {:02X}({}), F: {:b}", self.read_a_register(), self.read_a_register(), self.read_flags_register());
        println!("B: {:02X}, C: {:02X}", self.read_b_register(), self.read_c_register());
        println!("PC: {:02X}", self.pc);
        println!("====CPU-STATE====\n");

    }
}
