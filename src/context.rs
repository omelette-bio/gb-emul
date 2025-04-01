pub struct Context
{
    af: u16,
    bc: u16,
    de: u16,
    hl: u16,
    sp: u16,
    pc: u16,
}

impl Context
{
    pub fn init() -> Self { Context {af: 0, bc: 0, de: 0, hl: 0, sp: 0, pc: 0} }

    pub fn read_af_register(&self) -> u16 { self.af }

    pub fn write_af_register(&mut self, val: u16) { self.af = val; }

    pub fn read_a_register(&self) -> u8 { (self.af >> 8) as u8 }

    pub fn read_flags_register(&self) -> u8 { self.af as u8 }

    pub fn read_bc_register(&self) -> u16 { self.bc }

    pub fn read_b_register(&self) -> u8 { (self.bc >> 8) as u8 }

    pub fn read_c_register(&self) -> u8 { self.bc as u8 }

    pub fn write_bc_register(&mut self, val: u16) { self.bc = val; }

    pub fn add_pc(&mut self, val: u16) { self.pc += val }

    pub fn incr_pc(&mut self) { self.add_pc(1); }

    pub fn get_pc(&self) -> u16 {self.pc}
}
