use std::ops::{BitOr, BitAnd};

use super::integer_casting::CastWithNegation;
use super::interconnect::Interconnect;
use super::opcode::*;
use super::opcode::Op::*;
use enum_primitive::FromPrimitive;

#[derive(Copy, Clone, Debug)]
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

            pc: 0x8000, //for now just make this the start of PRG ROM in the cart

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

    pub fn shift_left(&mut self, val: u8) -> u8 {
        let result = val << 1;

        if (val&0x80) > 0 {
            self.p.set_carry(true);
        } else {
            self.p.set_carry(false);
        }

        if (result&0x80) > 0 {
            self.p.set_negative(true);
        } else {
            self.p.set_negative(false);
        }

        if val == 0 {
            self.p.set_zero(true);
        } else {
            self.p.set_zero(false);
        }

        result
    }

    pub fn shift_right(&mut self, val: u8) -> u8 {
        let result = val >> 1;

        if (val&1) > 0 {
            self.p.set_carry(true);
        } else {
            self.p.set_carry(false);
        }

        if (result&0x80) > 0 {
            self.p.set_negative(true);
        } else {
            self.p.set_negative(false);
        }

        if val == 0 {
            self.p.set_zero(true);
        } else {
            self.p.set_zero(false);
        }

        result
    }

    pub fn rol(&mut self, val: u8) -> u8 {
        let result = (val << 1) | self.p.carry as u8;

        if (val&0x80) > 0 {
            self.p.set_carry(true);
        } else {
            self.p.set_carry(false);
        }

        if (result&0x80) > 0 {
            self.p.set_negative(true);
        } else {
            self.p.set_negative(false);
        }

        if val == 0 {
            self.p.set_zero(true);
        } else {
            self.p.set_zero(false);
        }

        result
    }

    pub fn ror(&mut self, val: u8) -> u8 {
        let result = (val >> 1) | ((self.p.carry as u8) << 7);

        if (val&0x1) > 0 {
            self.p.set_carry(true);
        } else {
            self.p.set_carry(false);
        }

        if (result&0x80) > 0 {
            self.p.set_negative(true);
        } else {
            self.p.set_negative(false);
        }

        if val == 0 {
            self.p.set_zero(true);
        } else {
            self.p.set_zero(false);
        }

        result
    }

    pub fn set_pc(&mut self, val: u16) {
        self.pc = val;
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
        self.s = val % 0xFF;
    }

    pub fn set_p_u8(&mut self, val: u8) {
        self.p = CPUStatus::from(val);
    }
    pub fn set_p(&mut self, val: CPUStatus) {
        self.p = val;
    }

    pub fn offset_a(&mut self, val: u8) {
        self.a = self.a.wrapping_add(val);

        if (self.a&0x80) > 0 {
            self.p.set_negative(true);
        } else {
            self.p.set_negative(false);
        }

        if self.a == 0 {
            self.p.set_zero(true);
        } else {
            self.p.set_zero(false);
        }
    }
    pub fn offset_x(&mut self, val: u8) {
        self.x = self.x.wrapping_add(val);

        if (self.x&0x80) > 0 {
            self.p.set_negative(true);
        } else {
            self.p.set_negative(false);
        }

        if self.x == 0 {
            self.p.set_zero(true);
        } else {
            self.p.set_zero(false);
        }
    }
    pub fn offset_y(&mut self, val: u8) {
        self.y = self.y.wrapping_add(val);

        if (self.y&0x80) > 0 {
            self.p.set_negative(true);
        } else {
            self.p.set_negative(false);
        }

        if self.y == 0 {
            self.p.set_zero(true);
        } else {
            self.p.set_zero(false);
        }
    }
    pub fn offset_s(&mut self, val: u8) {
        self.s = (self.s.wrapping_add(val) as u16 % 0x100) as u8;
    }
    pub fn offset_pc(&mut self, val: u16) {
        self.pc = self.pc.wrapping_add(val);
    }

    pub fn add_with_carry(&mut self, lhs: u8, rhs: u8) -> u8 {
        if self.p.decimal {
            //return self.add_carry_decimal(left, right);
        }

        let carry = self.p.carry;
        let mut result = (lhs as u16) + (rhs as u16);

        if carry {
            result = result + 1;
        }
        
        if ((result as u8)&0x80) == 0x80 {
            self.p.set_negative(true);
        }
        else {
            self.p.set_negative(false);
        }
        if (result as u8) == 0 {
            self.p.set_zero(true);
        }
        else {
            self.p.set_zero(false);
        }

        if (result as i16) < -128 || (result as i16) > 127 {
            self.p.set_overflow(true);
        } else {
            self.p.set_overflow(false);
        }

        if result > 255 {
            self.p.set_carry(true);
        } else {
            self.p.set_carry(false);
        }
        
        result as u8
    }

    fn add(&mut self, lhs: u8, rhs: u8) -> u8 {
        let mut val = lhs.wrapping_sub(rhs);
        
        if val < lhs {
            self.p.set_carry(true);
        } else {
            self.p.set_carry(false);
        }
        if val == 0 {
            self.p.set_zero(true);
        } else {
            self.p.set_zero(false);
        }
        if (val&0x80) > 0 {
            self.p.set_negative(true);
        } else {
            self.p.set_negative(false);
        }

        val
    }

    pub fn subtract_with_carry(&mut self, lhs: u8, rhs: u8) -> u8 {
        let mut val = (lhs as u16).wrapping_sub((rhs as u16));
        
        if !self.p.carry {
            val = val.wrapping_sub(1);
        }

        if val > 255 {
            self.p.set_carry(true);
        } else {
            self.p.set_carry(false);
        }
        if val == 0 {
            self.p.set_zero(true);
        } else {
            self.p.set_zero(false);
        }
        if (val&0x80) > 0 {
            self.p.set_negative(true);
        } else {
            self.p.set_negative(false);
        }

        (val&0xFF) as u8
    }

    pub fn subtract(&mut self, lhs: u8, rhs: u8) -> u8 {
        let val = lhs.wrapping_sub(rhs);
        
        if lhs >= rhs {
            self.p.set_carry(true);
        } else {
            self.p.set_carry(false);
        }
        if val == 0 {
            self.p.set_zero(true);
        } else {
            self.p.set_zero(false);
        }
        if (val&0x80) > 0 {
            self.p.set_negative(true);
        } else {
            self.p.set_negative(false);
        }

        val
    }

    pub fn and(&mut self, lhs: u8, rhs: u8) -> u8 {
        let val = lhs & rhs;
        
        if val == 0 {
            self.p.set_zero(true);
        } else {
            self.p.set_zero(false);
        }
        if (val&0x80) > 0 {
            self.p.set_negative(true);
        } else {
            self.p.set_negative(false);
        }

        val
    }

    pub fn or(&mut self, lhs: u8, rhs: u8) -> u8 {
        let val = lhs | rhs;
        
        if val == 0 {
            self.p.set_zero(true);
        } else {
            self.p.set_zero(false);
        }
        if (val&0x80) > 0 {
            self.p.set_negative(true);
        } else {
            self.p.set_negative(false);
        }

        val
    }

    pub fn eor(&mut self, lhs: u8, rhs: u8) -> u8 {
        let val = lhs ^ rhs;
        
        if val == 0 {
            self.p.set_zero(true);
        } else {
            self.p.set_zero(false);
        }
        if (val&0x80) > 0 {
            self.p.set_negative(true);
        } else {
            self.p.set_negative(false);
        }

        val
    }

    pub fn push_stack(&mut self, interconnect: &mut Interconnect, val: u8) {
        interconnect.write_mem(0x100 + self.s as usize, val);
        self.offset_s(0xFF);
    }

    pub fn pop_stack(&mut self, interconnect: &mut Interconnect) -> u8 {
        self.offset_s(1);
        let val = interconnect.read_mem(0x100 + self.s as usize);

        val
    }

    pub fn do_nmi(&mut self, interconnect: &mut Interconnect) {
        let return_point = self.pc;

        let p = self.p.to_u8();
        self.push_stack(interconnect, ((return_point&0xFF00) >> 8) as u8);
        self.push_stack(interconnect, (return_point&0xFF) as u8);
        self.push_stack(interconnect, p);

        let addr_lo = interconnect.read_absolute(0xFFFA) as u16;
        let addr_hi = interconnect.read_absolute(0xFFFB) as u16;

        self.pc = (addr_hi << 8) | addr_lo;
    }

    //6502 opcode info http://obelisk.me.uk/6502/reference.html
    pub fn do_instruction(&mut self, interconnect: &mut Interconnect) -> bool {
        //Read 3 bytes (1st is opcode, 2nd is first operand (if any), 3rd is second operand (if any))
        let op = interconnect.read_mem(self.pc as usize) as u32;
        let imm1 = (interconnect.read_mem((self.pc.wrapping_add(1)) as usize) as u32) << 8;
        let imm2 = (interconnect.read_mem((self.pc.wrapping_add(2)) as usize) as u32) << 16;

        let opcode = Opcode::new(op | imm1 | imm2);

        println!("pc: 0x{:04X}", self.pc);

        //using a nifty crate that can convert integers to enums
        //to make pattern matching nicer
        match Op::from_i32(opcode.op() as i32) {
            Some(op) => {
                print!("{:?} ", op);
                
                match op {

                    //IMPLIED
                    Op::BRKImmediate => {
                        if self.p.irq_disable == false {
                            let return_point = self.pc.wrapping_add(1);

                            let p = self.p.to_u8();
                            self.push_stack(interconnect, ((return_point&0xFF00) >> 8) as u8);
                            self.push_stack(interconnect, (return_point&0xFF) as u8);
                            self.push_stack(interconnect, p);

                            let addr_lo = interconnect.read_absolute(0xFFFE) as u16;
                            let addr_hi = interconnect.read_absolute(0xFFFF) as u16;

                            self.pc = (addr_hi << 8) | addr_lo;
                        } else {
                            self.offset_pc(1);
                        }

                        return false;
                    }

                    Op::ORAIndirectX => {
                        let mut val = interconnect.read_indexed_indirect_x(opcode.imm1() as usize, self.x as usize);
                        let a = self.a;
                        val = self.or(a, val);

                        self.a = val;

                        self.offset_pc(2);
                    }

                    //ORs the accumulator with memory
                    //(modifies zero and negative flag)
                    Op::ORAZeroPage => {
                        let mut val = interconnect.read_zero_page(opcode.imm1() as usize);
                        let a = self.a;
                        val = self.or(a, val);

                        self.a = val;

                        self.offset_pc(2);
                    }

                    Op::ORAImmediate => {
                        let a = self.a;
                        let val = self.or(a, opcode.imm1());

                        self.a = val;

                        self.offset_pc(2);
                    }

                    Op::ASLZeroPage => {
                        let mut val = interconnect.read_zero_page(opcode.imm1() as usize);
                        val = self.shift_left(val);

                        interconnect.write_zero_page(opcode.imm1() as usize, val);

                        self.offset_pc(2);
                    }

                    Op::ASLAccumulator => {
                        let a = self.a;

                        self.a = self.shift_left(a);

                        self.offset_pc(1);
                    }

                    //Branch if Plus (adds to the program counter if negative flag is clear)
                    Op::BPLRelative => {
                        if self.p.negative == false {
                            print!("Branching from 0x{:04X}", self.pc);
                            self.offset_pc(opcode.imm1().cast_with_neg());
                            println!(" to 0x{:04X} (PC + 0x{:02X})", self.pc + 2, opcode.imm1().cast_with_neg());
                        }

                        self.offset_pc(2);
                    }

                    //Clears the Carry flag
                    Op::CLCImplied => {
                        self.p.set_carry(false);

                        self.offset_pc(1);
                    }

                    Op::ORAAbsoluteY => {
                        let mut val = interconnect.read_absolute_indexed_y(opcode.abs_addr(), self.y as usize);
                        let a = self.a;

                        val = self.or(a, val);

                        self.a = val;

                        self.offset_pc(3);
                    }

                    Op::ORAAbsoluteX => {
                        let mut val = interconnect.read_absolute_indexed_x(opcode.abs_addr(), self.x as usize);
                        let a = self.a;

                        val = self.or(a, val);

                        self.a = val;

                        self.offset_pc(3);
                    }

                    //Pushes return point onto stack (pc + 3), then sets pc to absolute address
                    Op::JSRAbsolute => {
                        print!("Jumping from 0x{:04X} --- ", self.pc);

                        let return_point = self.pc.wrapping_add(2);
                        let addr = opcode.abs_addr() as u16;

                        self.push_stack(interconnect, ((return_point&0xFF00) >> 8) as u8);
                        self.push_stack(interconnect, (return_point&0xFF) as u8);
                        self.pc = addr;

                        println!("Jumping to 0x{:04X}", self.pc);
                    }
                    
                    Op::BITZeroPage => {
                        let val = interconnect.read_zero_page(opcode.imm1() as usize);
                        let a = self.a;

                        let _ = self.and(a, val);
                        
                        self.p.set_overflow((val&0x40) > 0);
                        self.p.set_negative((val&0x80) > 0);

                        self.offset_pc(2);
                    }

                    Op::ANDZeroPage => {
                        let mut val = interconnect.read_zero_page(opcode.imm1() as usize);
                        let a = self.a;

                        let val = self.and(a, val);

                        self.a = val;

                        self.offset_pc(2);
                    }

                    Op::ROLZeroPage => {
                        let mut val = interconnect.read_zero_page(opcode.imm1() as usize);
                        val = self.rol(val);

                        interconnect.write_zero_page(opcode.imm1() as usize, val);

                        self.offset_pc(2);
                    }

                    //AND Accumulator with immediate
                    Op::ANDImmediate => {
                        let a = self.a;
                        let val = self.and(a, opcode.imm1());

                        self.a = val;

                        self.offset_pc(2);
                    }

                    Op::ROLAccumulator => {
                        let a = self.a;
                        self.a = self.rol(a);

                        self.offset_pc(1);
                    }

                    Op::BITAbsolute => {
                        let val = interconnect.read_absolute(opcode.abs_addr() as usize);
                        let a = self.a;

                        let _ = self.and(a, val);
                        
                        self.p.set_overflow((val&0x40) > 0);
                        self.p.set_negative((val&0x80) > 0);

                        self.offset_pc(3);
                    }

                    Op::ROLAbsolute => {
                        let mut val = interconnect.read_absolute(opcode.abs_addr() as usize);
                        val = self.rol(val);

                        interconnect.write_absolute(opcode.abs_addr(), val);

                        self.offset_pc(3);
                    }

                    Op::BMIRelative => {
                        if self.p.negative == true {
                            print!("Branching from 0x{:04X}", self.pc);
                            self.offset_pc(opcode.imm1().cast_with_neg());
                            println!(" to 0x{:04X} (PC + 0x{:02X})", self.pc + 2, opcode.imm1().cast_with_neg());
                        }

                        self.offset_pc(2);
                    }

                    Op::ANDAbsoluteY => {
                        let mut val = interconnect.read_absolute_indexed_y(opcode.abs_addr(), self.y as usize);
                        let a = self.a;

                        let val = self.and(a, val);

                        self.a = val;

                        self.offset_pc(3);
                    }

                    Op::ANDAbsoluteX => {
                        let mut val = interconnect.read_absolute_indexed_x(opcode.abs_addr(), self.x as usize);
                        let a = self.a;

                        let val = self.and(a, val);

                        self.a = val;

                        self.offset_pc(3);
                    }

                    Op::SECImplied => {
                        self.p.set_carry(true);

                        self.offset_pc(1);
                    }

                    Op::RTIImplied => {
                        let p = self.pop_stack(interconnect) as u8;
                        let lo = self.pop_stack(interconnect) as u16;
                        let hi = self.pop_stack(interconnect) as u16;
                        let ret = ((hi << 8) | lo).wrapping_sub(1);

                        self.p = CPUStatus::from(p);
                        self.pc = ret;

                        self.offset_pc(1);
                    }

                    Op::LSRZeroPage => {
                        let mut val = interconnect.read_zero_page(opcode.imm1() as usize);
                        val = self.shift_right(val);

                        interconnect.write_zero_page(opcode.imm1() as usize, val);

                        self.offset_pc(2);
                    }

                    Op::EORZeroPage => {
                        let mut val = interconnect.read_zero_page(opcode.imm1() as usize);
                        let a = self.a;
                        val = self.eor(a, val);

                        self.a = val;

                        self.offset_pc(2);
                    }

                    Op::PHAImplied => {
                        let a = self.a;
                        self.push_stack(interconnect, a);

                        self.offset_pc(1);
                    }

                    Op::LSRAccumulator => {
                        let a = self.a;
                        self.a = self.shift_right(a);

                        self.offset_pc(1);
                    }

                    Op::JMPAbsolute => {
                        self.pc = opcode.abs_addr() as u16;
                    }

                    Op::EORAbsolute => {
                        let val = interconnect.read_absolute(opcode.abs_addr() as usize);
                        let a = self.a;

                        let _ = self.eor(a, val);

                        self.offset_pc(3);
                    }

                    Op::BVCRelative => {
                        if self.p.overflow == false {
                            print!("Branching from 0x{:04X}", self.pc);
                            self.offset_pc(opcode.imm1().cast_with_neg());
                            println!(" to 0x{:04X} (PC + 0x{:02X})", self.pc + 2, opcode.imm1().cast_with_neg());
                        }

                        self.offset_pc(2);
                    }

                    Op::RTSImplied => {
                        print!("Returning from 0x{:04X} sp: 0x{:04X} -- ", self.pc, self.s);

                        let lo = self.pop_stack(interconnect) as u16;
                        let hi = self.pop_stack(interconnect) as u16;
                        let ret = (hi << 8) | lo;

                        self.pc = ret;

                        self.offset_pc(1);

                        //println!("S: {:04X}", self.s);
                        println!("Returning to 0x{:04X} - sp: 0x{:04X}", self.pc, self.s);
                    }

                    Op::ADCZeroPage => {
                        let mut val = interconnect.read_zero_page(opcode.imm1() as usize);
                        let a = self.a;
                        val = self.add_with_carry(a, val);

                        self.a = val;

                        self.offset_pc(2);
                    }

                    Op::RORZeroPage => {
                        let mut val = interconnect.read_zero_page(opcode.imm1() as usize);
                        
                        val = self.ror(val);

                        interconnect.write_zero_page(opcode.imm1() as usize, val);

                        self.offset_pc(2);
                    }

                    Op::PLAImplied => {
                        let val = self.pop_stack(interconnect);

                        self.set_a(val);

                        self.offset_pc(1);
                    }

                    Op::ADCImmediate => {
                        let a = self.a;
                        let val = self.add_with_carry(a, opcode.imm1());

                        self.a = val;

                        self.offset_pc(2);
                    }

                    Op::ADCAbsolute => {
                        let mut val = interconnect.read_absolute(opcode.abs_addr());
                        let a = self.a;
                        val = self.add_with_carry(a, val);

                        self.a = val;

                        self.offset_pc(3);
                    }

                    Op::RORAbsolute => {
                        let mut val = interconnect.read_absolute(opcode.abs_addr());
                        
                        val = self.ror(val);

                        interconnect.write_absolute(opcode.abs_addr(), val);

                        self.offset_pc(3);
                    }

                    //Set Interrupt Disable (Sets the I flag to true)
                    Op::SEIImplied => {
                        self.p.set_irq_disable(true);

                        self.offset_pc(1);
                    }

                    Op::ADCAbsoluteY => {
                        let mut val = interconnect.read_absolute_indexed_y(opcode.abs_addr(), self.y as usize);
                        let a = self.a;
                        val = self.add_with_carry(a, val);

                        self.a = val;

                        self.offset_pc(3);
                    }

                    Op::ADCAbsoluteX => {
                        let mut val = interconnect.read_absolute_indexed_x(opcode.abs_addr(), self.x as usize);
                        let a = self.a;
                        val = self.add_with_carry(a, val);

                        self.a = val;

                        self.offset_pc(3);
                    }

                    Op::RORAbsoluteX => {
                        let mut val = interconnect.read_absolute_indexed_x(opcode.abs_addr(), self.x as usize);
                        
                        val = self.ror(val);

                        interconnect.write_absolute_indexed_x(opcode.abs_addr(), self.x as usize, val);

                        self.offset_pc(3);
                    }

                    Op::JMPAbsIndirect => {
                        let addr = interconnect.read_absolute(opcode.abs_addr()) as u16 | ((interconnect.read_absolute(opcode.abs_addr() + 1) as u16) << 8);

                        self.pc = addr;
                    }

                    Op::STYZeroPage => {
                        interconnect.write_zero_page(opcode.imm1() as usize, self.y);

                        self.offset_pc(2);
                    }

                    Op::STAZeroPage => {
                        interconnect.write_zero_page(opcode.imm1() as usize, self.a);

                        self.offset_pc(2);
                    }

                    Op::STXZeroPage => {
                        interconnect.write_zero_page(opcode.imm1() as usize, self.x);

                        self.offset_pc(2);
                    }

                    Op::DEYImplied => {
                        self.offset_y(0xFF);

                        self.offset_pc(1);
                    }

                    Op::TXAImplied => {
                        let x = self.x;
                        self.set_a(x);

                        self.offset_pc(1);
                    }

                    Op::STYAbsolute => {
                        interconnect.write_absolute(opcode.abs_addr(), self.y);

                        self.offset_pc(3);
                    }

                    Op::STXAbsolute => {
                        interconnect.write_absolute(opcode.abs_addr(), self.x);

                        self.offset_pc(3);
                    }

                    //Store Accumulator (Stores a into memory with absolute addressing)
                    Op::STAAbsolute => {
                        interconnect.write_absolute(opcode.abs_addr(), self.a);

                        self.offset_pc(3);
                    }

                    Op::BCCRelative => {
                        if self.p.carry == false {
                            print!("Branching from 0x{:04X}", self.pc);
                            self.offset_pc(opcode.imm1().cast_with_neg());
                            println!(" to 0x{:04X} (PC + 0x{:02X})", self.pc + 2, opcode.imm1().cast_with_neg());
                        }

                        self.offset_pc(2);
                    }

                    Op::STAIndirectY => {
                        interconnect.write_indexed_indirect_y(opcode.imm1() as usize, self.y as usize, self.a);

                        self.offset_pc(2);
                    }

                    //Store Accumulator (Stores into memory with zero paged x addressing)
                    Op::STAZeroPageX => {
                        interconnect.write_zero_paged_indexed_x(opcode.imm1() as usize, self.x as usize, self.a);

                        self.offset_pc(2);
                    }

                    Op::TYAImplied => {
                        let y = self.y;
                        self.set_a(y);

                        self.offset_pc(1);
                    }

                    Op::STAAbsoluteY => {
                        interconnect.write_absolute_indexed_y(opcode.abs_addr(), self.y as usize, self.a);

                        self.offset_pc(3);
                    }

                    //Transfers x-index into stack pointer
                    Op::TXSImplied => {
                        self.s = self.x;

                        //println!("TXS sp: 0x{:04X}", self.s);

                        self.offset_pc(1);
                    }

                    Op::STAAbsoluteX => {
                        interconnect.write_absolute_indexed_x(opcode.abs_addr(), self.x as usize, self.a);

                        self.offset_pc(3);
                    }

                    //Loads operand into y-index (modifies zero and negatives flags)
                    Op::LDYImmediate => {
                        self.set_y(opcode.imm1());

                        self.offset_pc(2);
                    }

                    //Loads operand into x-index (modifies zero and negatives flags)
                    Op::LDXImmediate => {
                        self.set_x(opcode.imm1());

                        self.offset_pc(2);
                    }

                    Op::LDYZeroPage => {
                        let val = interconnect.read_zero_page(opcode.imm1() as usize);

                        self.set_y(val);

                        self.offset_pc(2);
                    }

                    Op::LDXZeroPage => {
                        let val = interconnect.read_zero_page(opcode.imm1() as usize);

                        self.set_x(val);

                        self.offset_pc(2);
                    }

                    //Transfer A to Y-index
                    Op::TAYImplied => {
                        let a = self.a;
                        self.set_y(a);

                        self.offset_pc(1);
                    }

                    //Loads operand into accumulator (modifies zero and negatives flags)
                    Op::LDAImmediate => {
                        self.set_a(opcode.imm1());

                        self.offset_pc(2);
                    }

                    Op::TAXImplied => {
                        let a = self.a;
                        self.set_x(a);

                        self.offset_pc(1);
                    }

                    Op::LDYAbsolute => {
                        let val = interconnect.read_absolute(opcode.abs_addr());

                        self.set_y(val);

                        self.offset_pc(3);
                    }


                    Op::LDXAbsolute => {
                        let val = interconnect.read_absolute(opcode.abs_addr());

                        self.set_x(val);

                        self.offset_pc(3);
                    }

                    Op::LDAZeroPage => {
                        let val = interconnect.read_zero_page(opcode.imm1() as usize);

                        self.set_a(val);

                        self.offset_pc(2);
                    }

                    //Loads operand into accumulator using absolute addressing
                    //(modifies zero and negatives flags)
                    Op::LDAAbsolute => {
                        let val = interconnect.read_absolute(opcode.abs_addr());

                        self.set_a(val);

                        self.offset_pc(3);
                    }

                    //Branch if carry set (adds to the program counter if carry flag is set)
                    Op::BCSRelative => {
                        if self.p.carry == true {
                            print!("Branching from 0x{:04X}", self.pc);
                            self.offset_pc(opcode.imm1().cast_with_neg());
                            println!(" to 0x{:04X} (PC + 0x{:02X})", self.pc + 2, opcode.imm1().cast_with_neg());
                        }

                        self.offset_pc(2);
                    }

                    Op::LDAIndirectY => {
                        let val = interconnect.read_indexed_indirect_y(opcode.imm1() as usize, self.y as usize);

                        self.set_a(val);

                        self.offset_pc(2);
                    }

                    //Loads value into y-index from zero page x memory
                    Op::LDYZeroPageX => {
                        let val = interconnect.read_zero_paged_indexed_x(opcode.imm1() as usize, self.x as usize);

                        self.set_y(val);

                        self.offset_pc(2);
                    }

                    //Loads value into accumulator from zero page x memory
                    Op::LDAZeroPageX => {
                        let val = interconnect.read_zero_paged_indexed_x(opcode.imm1() as usize, self.x as usize);

                        self.set_a(val);

                        self.offset_pc(2);
                    }

                    Op::LDAAbsoluteY => {
                        let val = interconnect.read_absolute_indexed_y(opcode.abs_addr(), self.y as usize);

                        self.set_a(val);

                        self.offset_pc(3);
                    }

                    //Loads operand into accumulator using absolute indexed addressing
                    //(modifies zero and negatives flags)
                    Op::LDAAbsoluteX => {
                        let val = interconnect.read_absolute_indexed_x(opcode.abs_addr(), self.x as usize);
                        
                        self.set_a(val);

                        self.offset_pc(3);
                    }

                    Op::LDXAbsoluteY => {
                        let val = interconnect.read_absolute_indexed_y(opcode.abs_addr(), self.y as usize);
                        
                        self.set_x(val);

                        self.offset_pc(3);
                    }

                    //Compare y-index with operand
                    //(modifies carry, zero, and negative flags)
                    Op::CPYImmediate => {
                        let y = self.y;
                        let _ = self.subtract(y, opcode.imm1());

                        self.offset_pc(2);
                    }

                    Op::CMPIndirectX => {
                        let val = interconnect.read_indexed_indirect_x(opcode.imm1() as usize, self.x as usize);
                        let a = self.a;
                        let _ = self.subtract(a, val);

                        self.offset_pc(2);
                    }

                    Op::CMPZeroPage => {
                        let val = interconnect.read_zero_page(opcode.imm1() as usize);
                        let a = self.a;
                        let _ = self.subtract(a, val);

                        self.offset_pc(2);
                    }

                    Op::DECZeroPage => {
                        let mut val = interconnect.read_zero_page(opcode.imm1() as usize);
                        val = self.subtract(val, 1);

                        interconnect.write_zero_page(opcode.imm1() as usize, val);

                        self.offset_pc(2);
                    }

                    Op::INYImplied => {
                        self.offset_y(1);

                        self.offset_pc(1);
                    }

                    //Compare accumulator with operand
                    //(modifies carr, zero, and negative flags)
                    Op::CMPImmediate => {
                        let a = self.a;
                        let _ = self.subtract(a, opcode.imm1());

                        self.offset_pc(2);
                    }
                    
                    //Decrements the x-index
                    //(modifies zero and negative flags)
                    Op::DEXImplied => {
                        self.offset_x(0xFF);

                        self.offset_pc(1);
                    }

                    Op::CMPAbsolute => {
                        let a = self.a;
                        let val = interconnect.read_absolute(opcode.abs_addr() as usize);
                        let _ = self.subtract(a, val);

                        self.offset_pc(3);
                    }
                    
                    Op::DECAbsolute => {
                        let mut val = interconnect.read_absolute(opcode.abs_addr() as usize);
                        val = self.subtract(val, 1);

                        interconnect.write_absolute(opcode.abs_addr() as usize, val);

                        self.offset_pc(3);
                    }

                    //Branch if not equal (adds to the program counter if zero flag is not set)
                    Op::BNERelative => {
                        if self.p.zero == false {
                            print!("Branching from 0x{:04X}", self.pc);
                            self.offset_pc(opcode.imm1().cast_with_neg());
                            println!(" to 0x{:04X} (PC + 0x{:02X})", self.pc + 2, opcode.imm1().cast_with_neg());
                        }

                        self.offset_pc(2);
                    }

                    Op::DECZeroPageX => {
                        let mut val = interconnect.read_zero_paged_indexed_x(opcode.imm1() as usize, self.x as usize);
                        val = self.subtract(val, 1);

                        interconnect.write_zero_paged_indexed_x(opcode.imm1() as usize, self.x as usize, val);

                        self.offset_pc(2);
                    }

                    //Clear Decimal Mode (Sets the D flag to false)
                    Op::CLDImplied => {
                        self.p.set_decimal(false);

                        self.offset_pc(1);
                    }

                    Op::CMPAbsoluteY => {
                        let a = self.a;
                        let val = interconnect.read_absolute_indexed_y(opcode.abs_addr() as usize, self.y as usize);
                        let _ = self.subtract(a, val);

                        self.offset_pc(3);
                    }

                    Op::DECAbsoluteX => {
                        let mut val = interconnect.read_absolute_indexed_x(opcode.abs_addr() as usize, self.x as usize);
                        val = self.subtract(val, 1);

                        interconnect.write_absolute_indexed_x(opcode.abs_addr() as usize, self.x as usize, val);

                        self.offset_pc(3);
                    }

                    Op::CPXImmediate => {
                        let x = self.x;
                        let _ = self.subtract(x, opcode.imm1());

                        self.offset_pc(2);
                    }

                    Op::CPYZeroPage => {
                        let val = interconnect.read_zero_page(opcode.imm1() as usize);
                        let y = self.y;
                        let _ = self.subtract(y, val);

                        self.offset_pc(2);
                    }

                    Op::CPXZeroPage => {
                        let val = interconnect.read_zero_page(opcode.imm1() as usize);
                        let x = self.x;
                        let _ = self.subtract(x, val);

                        self.offset_pc(2);
                    }
                    
                    Op::SBCZeroPage => {
                        let mut val = interconnect.read_zero_page(opcode.imm1() as usize);
                        let a = self.a;
                        val = self.subtract_with_carry(a, val);

                        self.a = val;

                        self.offset_pc(2);
                    }

                    Op::INCZeroPage => {
                        let mut val = interconnect.read_zero_page(opcode.imm1() as usize);
                        val = self.add(val, 1);

                        interconnect.write_zero_page(opcode.imm1() as usize, val);

                        self.offset_pc(2);
                    }

                    Op::INXImplied => {
                        self.offset_x(1);

                        self.offset_pc(1);
                    }

                    Op::SBCImmediate => {
                        let a = self.a;
                        self.a = self.subtract_with_carry(a, opcode.imm1());

                        self.offset_pc(2);
                    }

                    Op::CPXAbsolute => {
                        let x = self.x;
                        let val = interconnect.read_absolute(opcode.abs_addr() as usize);
                        let _ = self.subtract(x, val);

                        self.offset_pc(3);
                    }

                    Op::INCAbsolute => {
                        let mut val = interconnect.read_absolute(opcode.abs_addr() as usize);
                        val = self.add(val, 1);

                        interconnect.write_absolute(opcode.abs_addr() as usize, val);

                        self.offset_pc(3);
                    }

                    Op::BEQRelative => {
                        if self.p.zero == true {
                            print!("Branching from 0x{:04X}", self.pc);
                            self.offset_pc(opcode.imm1().cast_with_neg());
                            println!(" to 0x{:04X} (PC + 0x{:02X})", self.pc + 2, opcode.imm1().cast_with_neg());
                        }

                        self.offset_pc(2);
                    }

                    Op::SBCAbsoluteY => {
                        let mut val = interconnect.read_absolute_indexed_y(opcode.abs_addr(), self.y as usize);
                        let a = self.a;
                        val = self.subtract_with_carry(a, val);

                        self.a = val;

                        self.offset_pc(3);
                    }

                    Op::NOPImplied => {
                        self.offset_pc(1);
                    }

                    _ => {
                        println!("{:?}", opcode);
                        println!("unimplemented opcode");
                        return false
                    }
                }
            }
            
            None => {
                println!("unknown {:?}", opcode);
                return false
            }
        }
        
        println!("{:?}", opcode);

        println!("a: 0x{:02X}\nx: 0x{:02X}\ny: 0x{:02X}\ns: 0x{:02X}\n{:?}\n\n", 
                    self.a, self.x, self.y,
                    self.s, self.p);

        true
    }
}