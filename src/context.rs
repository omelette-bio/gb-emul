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
}

impl Context
{
    pub fn init(rom_file: File) -> Self { Context {af: 0, bc: 0, de: 0, hl: 0, sp: 0xDFFF, pc: 0, rom_path: rom_file, ram: [0; 65_536]} }

    pub fn read_af_register(&self) -> u16 { self.af }

    pub fn write_af_register(&mut self, val: u16) { self.af = val; }

    pub fn read_a_register(&self) -> u8 { (self.af >> 8) as u8 }

    pub fn read_flags_register(&self) -> u8 { self.af as u8 }

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

    pub fn write_in_memory(&mut self, bytes_count: u8, bytes: Vec<u8>, offset: u16)
    {
        for i in 0..bytes_count
        {
            self.ram[(offset+ (i as u16)) as usize] = bytes[i as usize];
        }
    }
}
