use std::ops::{BitOr, BitAnd};

use super::integer_casting::CastWithNegation;
use super::interconnect::Interconnect;
use super::opcode::*;
use super::opcode::Op::*;
use enum_primitive::FromPrimitive;

#[derive(Copy, Clone)]
pub struct CPUStatus {
    carry: bool,
    zero: bool,
    irq_disable: bool,
    decimal: bool, //not used by the 2A03
    overflow: bool,
    negative: bool,
}

impl From<u8> for CPUStatus {
    fn from(val: u8) -> CPUStatus {
        CPUStatus {
            carry: (val&0x1) > 0,
            zero: (val&0x2) > 0,
            irq_disable: (val&0x4) > 0,
            decimal: (val&0x8) > 0,
            overflow: (val&0x10) > 0,
            negative: (val&0x20) > 0,
        }
    }
}

impl BitOr<u8> for CPUStatus {
    type Output = CPUStatus;

    fn bitor(self, val: u8) -> CPUStatus {
        CPUStatus::from(self.to_u8() | val)
    }
}

impl BitAnd<u8> for CPUStatus {
    type Output = CPUStatus;

    fn bitand(self, val: u8) -> CPUStatus {
        CPUStatus::from(self.to_u8() & val)
    }
}

impl CPUStatus {
    pub fn new() -> CPUStatus {
        CPUStatus {
            carry: false,
            zero: false,
            irq_disable: false,
            decimal: false,
            overflow: false,
            negative: false,
        }
    }

    pub fn set_carry(&mut self, val: bool) {
        self.carry = val;
    }
    pub fn set_zero(&mut self, val: bool) {
        self.zero = val;
    }
    pub fn set_irq_disable(&mut self, val: bool) {
        self.irq_disable = val;
    }
    pub fn set_decimal(&mut self, val: bool) {
        self.decimal = val;
    }
    pub fn set_overflow(&mut self, val: bool) {
        self.overflow = val;
    }
    pub fn set_negative(&mut self, val: bool) {
        self.negative = val;
    }

    pub fn to_u8(&self) -> u8 {
        let mut val = self.carry as u8;
        val += (self.zero as u8) << 1;
        val += (self.irq_disable as u8) << 2;
        val += (self.decimal as u8) << 3;
        val += (self.overflow as u8) << 6;
        val += (self.negative as u8) << 7;

        val
    }
}

pub struct NESCpu {
    a: u8, //accumulator
    x: u8, //x-index
    y: u8, //y-index
    s: u8, //stack-pointer

    pc: u16, //program counter

    p: CPUStatus, //cpu status
}

impl NESCpu {
    pub fn new() -> NESCpu {
        NESCpu {
            a: 0,
            x: 0,
            y: 0,
            s: 0,

            pc: 0x4020, //for now just make this the start of PRG ROM in the cart

            p: CPUStatus::new(),
        }
    }

    pub fn a(&self) -> u8 {
        self.a
    }
    pub fn x(&self) -> u8 {
        self.x
    }
    pub fn y(&self) -> u8 {
        self.y
    }
    pub fn s(&self) -> u8 {
        self.s
    }
    pub fn p(&self) -> CPUStatus {
        self.p
    }

    pub fn set_a(&mut self, val: u8) {
        self.a = val;

        if self.a == 0 {
            self.p.set_zero(true);
        } else {
            self.p.set_zero(false);
        }

        if (self.a & 0x80) > 1 {
            self.p.set_negative(true);
        } else {
            self.p.set_negative(false);
        }
    }
    pub fn set_x(&mut self, val: u8) {
        self.x = val;

        if self.x == 0 {
            self.p.set_zero(true);
        } else {
            self.p.set_zero(false);
        }

        if (self.x & 0x80) > 1 {
            self.p.set_negative(true);
        } else {
            self.p.set_negative(false);
        }
    }
    pub fn set_y(&mut self, val: u8) {
        self.y = val;

        if self.y == 0 {
            self.p.set_zero(true);
        } else {
            self.p.set_zero(false);
        }

        if (self.y & 0x80) > 1 {
            self.p.set_negative(true);
        } else {
            self.p.set_negative(false);
        }
    }
    pub fn set_s(&mut self, val: u8) {
        self.s = val;
    }

    pub fn set_p_u8(&mut self, val: u8) {
        self.p = CPUStatus::from(val);
    }
    pub fn set_p(&mut self, val: CPUStatus) {
        self.p = val;
    }

    pub fn offset_a(&mut self, val: u8) {
        self.a += val;
    }
    pub fn offset_x(&mut self, val: u8) {
        self.x += val;
    }
    pub fn offset_y(&mut self, val: u8) {
        self.y += val;
    }
    pub fn offset_s(&mut self, val: u8) {
        self.s += val;
    }
    pub fn offset_pc(&mut self, val: u16) {
        self.pc = self.pc.wrapping_add(val);
    }

    pub fn do_instruction(&mut self, interconnect: &mut Interconnect) -> bool{

        //Read 3 bytes (1st is opcode, 2nd is first operand (if any), 3rd is second operand (if any))
        let op = interconnect.read_mem(self.pc as usize) as u32;
        let imm1 = (interconnect.read_mem((self.pc + 1) as usize) as u32) << 8;
        let imm2 = (interconnect.read_mem((self.pc + 2) as usize) as u32) << 16;

        let opcode = Opcode::new(op | imm1 | imm2);

        match Op::from_i32(opcode.op() as i32) {
            Some(op) => {
                print!("{:?} ", op);
                
                match op {

                    //Branch if Plus (adds to the program counter if negative flag is clear)
                    BPLRelative => {
                        if self.p.negative == false {
                            println!("0x{:04X}", opcode.imm1().cast_with_neg());
                            self.offset_pc(opcode.imm1().cast_with_neg());
                            self.offset_pc(1);
                        } else {
                            self.offset_pc(2);
                        }
                    }

                    //Set Interrupt Disable (Sets the I flag to true)
                    SEIImplied => {
                        self.p.set_irq_disable(true);

                        self.offset_pc(1);
                    }

                    //Store Accumulator (Stores a into memory with absolute addressing)
                    STAAbsolute => {
                        interconnect.write_absolute(opcode.abs_addr(), self.a);

                        self.offset_pc(3);
                    }

                    //Transfers x-index into stack pointer
                    TXSImplied => {
                        self.s = self.x;

                        self.offset_pc(1);
                    }

                    //Loads operand into x-index (modifies zero and negatives flags)
                    LDXImmediate => {
                        self.set_x(opcode.imm1());

                        self.offset_pc(2);
                    }

                    //Loads operand into accumulator (modifies zero and negatives flags)
                    LDAImmediate => {
                        self.set_a(opcode.imm1());

                        self.offset_pc(2);
                    }

                    //Loads operand into accumulator using absolute addressing
                    //(modifies zero and negatives flags)
                    LDAAbsolute => {
                        let val = interconnect.read_absolute(opcode.abs_addr());

                        self.set_a(val);

                        self.offset_pc(3);
                    }

                    //Clear Decimal Mode (Sets the D flag to false)
                    CLDImplied => {
                        self.p.set_decimal(false);

                        self.offset_pc(1);
                    }

                    _ => {
                        println!("encountered unimplemented opcode: {:?}", op);
                        return false
                    }
                }
            }
            
            None => {
                println!("unknown {:?}, pc: 0x{:04X}", opcode, self.pc);
                return false
            }
        }
        
        println!("{:?}, pc: 0x{:04X}", opcode, self.pc);

        true
    }
}