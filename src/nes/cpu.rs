use std::ops::{BitOr, BitAnd};

use super::mmu::*;
use super::integer_casting::CastWithNegation;
use super::opcode::*;
use enum_primitive::FromPrimitive;

#[derive(Copy, Clone, Debug)]
pub struct CPUStatus {
    carry: bool,
    zero: bool,
    irq_disable: bool,
    decimal: bool, //not used by the 2A03
    overflow: bool,
    negative: bool,

    bit_4: bool,
    bit_5: bool,
}

impl From<u8> for CPUStatus {
    fn from(val: u8) -> CPUStatus {
        CPUStatus {
            carry: (val&0x1) > 0,
            zero: (val&0x2) > 0,
            irq_disable: (val&0x4) > 0,
            decimal: (val&0x8) > 0,
            overflow: (val&0x40) > 0,
            negative: (val&0x80) > 0,

            bit_4: (val&0x10) > 0,
            bit_5: (val&0x20) > 0,
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

            bit_4: false,
            bit_5: false,
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
    pub fn set_bit_4(&mut self, val: bool) {
        self.bit_4 = val;
    }
    pub fn set_bit_5(&mut self, val: bool) {
        self.bit_5 = val;
    }

    pub fn to_u8(&self) -> u8 {
        let mut val = self.carry as u8;
        val |= (self.zero as u8) << 1;
        val |= (self.irq_disable as u8) << 2;
        val |= (self.decimal as u8) << 3;
        val |= (self.bit_4 as u8) << 4;
        val |= (self.bit_5 as u8) << 5;
        val |= (self.overflow as u8) << 6;
        val |= (self.negative as u8) << 7;

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

        if result == 0 {
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

        self.p.set_negative(false);

        if result == 0 {
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

        if result == 0 {
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

        if result == 0 {
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

        if val == 0 {
            self.p.set_zero(true);
        } else {
            self.p.set_zero(false);
        }

        if (val & 0x80) > 0 {
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

        if (self.x & 0x80) > 0 {
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

        if (self.y & 0x80) > 0 {
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
        self.s = self.s.wrapping_add(val);
    }
    pub fn offset_pc(&mut self, val: u16) {
        self.pc = self.pc.wrapping_add(val);
    }

    pub fn add_with_carry(&mut self, lhs: u8, rhs: u8) -> u8 {
        let result = (lhs as u16).wrapping_add(rhs as u16).wrapping_add(self.p.carry as u16);
        
        if ((result as u8)&0x80) > 0 {
            self.p.set_negative(true);
        }
        else {
            self.p.set_negative(false);
        }
        if result&0xFF == 0 {
            self.p.set_zero(true);
        }
        else {
            self.p.set_zero(false);
        }

        if (lhs^((result&0xFF) as u8))&(rhs^((result&0xFF) as u8))&0x80 > 0 {
            self.p.set_overflow(true);
        } else {
            self.p.set_overflow(false);
        }

        if result > 255 {
            self.p.set_carry(true);
        } else {
            self.p.set_carry(false);
        }
        
        (result&0xFF) as u8
    }

    fn add(&mut self, lhs: u8, rhs: u8) -> u8 {
        let val = self.add_no_carry(lhs, rhs);
        
        if val < lhs {
            self.p.set_carry(true);
        } else {
            self.p.set_carry(false);
        }

        val
    }

    fn add_no_carry(&mut self, lhs: u8, rhs: u8) -> u8 {
        let val = lhs.wrapping_add(rhs);
        
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
        return self.add_with_carry(lhs, rhs^0xFF);
    }

    pub fn subtract(&mut self, lhs: u8, rhs: u8) -> u8 {
        let val = self.subtract_no_carry(lhs, rhs);
        
        if lhs >= rhs {
            self.p.set_carry(true);
        } else {
            self.p.set_carry(false);
        }

        val
    }

    pub fn subtract_no_carry(&mut self, lhs: u8, rhs: u8) -> u8 {
        let val = lhs.wrapping_sub(rhs);
        
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

    pub fn push_stack(&mut self, mmu: &mut NESMmu, val: u8) {
        mmu.write_mem(0x100 + self.s as u16, val);
        self.offset_s(0xFF);
    }

    pub fn pop_stack(&mut self, mmu: &mut NESMmu) -> u8 {
        self.offset_s(1);
        let val = mmu.read_mem(0x100 + self.s as u16);

        val
    }

    pub fn do_nmi(&mut self, mmu: &mut NESMmu) {
        let return_point = self.pc;

        let p = self.p.to_u8();
        self.push_stack(mmu, ((return_point&0xFF00) >> 8) as u8);
        self.push_stack(mmu, (return_point&0xFF) as u8);
        self.push_stack(mmu, p);

        let addr_lo = mmu.read_absolute(0xFFFA) as u16;
        let addr_hi = mmu.read_absolute(0xFFFB) as u16;

        self.pc = (addr_hi << 8) | addr_lo;
    }

    fn get_op_cycles(&self, op: &Op) -> u32 {
        match *op {
            Op::BRKImmediate   => 7,
            Op::ORAIndirectX   => 6,
            Op::ORAZeroPage    => 3,
            Op::ASLZeroPage    => 5,
            Op::PHPImplied     => 3,
            Op::ORAImmediate   => 2,
            Op::ASLAccumulator => 2,
            Op::ORAAbsolute    => 4,
            Op::ASLAbsolute    => 6,
            Op::BPLRelative    => 2,
            Op::ORAIndirectY   => 5,
            Op::ORAZeroPageX   => 4,
            Op::ASLZeroPageX   => 6,
            Op::CLCImplied     => 2,
            Op::ORAAbsoluteY   => 4,
            Op::ORAAbsoluteX   => 4,
            Op::ASLAbsoluteX   => 7,
            Op::JSRAbsolute    => 6,
            Op::ANDIndirectX   => 6,
            Op::BITZeroPage    => 3,
            Op::ANDZeroPage    => 3,
            Op::ROLZeroPage    => 5,
            Op::PLPImplied     => 4,
            Op::ANDImmediate   => 2,
            Op::ROLAccumulator => 2,
            Op::BITAbsolute    => 4,
            Op::ANDAbsolute    => 4,
            Op::ROLAbsolute    => 6,
            Op::BMIRelative    => 2,
            Op::ANDIndirectY   => 5,
            Op::ANDZeroPageX   => 4,
            Op::ROLZeroPageX   => 6,
            Op::SECImplied     => 2,
            Op::ANDAbsoluteY   => 4,
            Op::ANDAbsoluteX   => 4,
            Op::ROLAbsoluteX   => 7,
            Op::RTIImplied     => 6,
            Op::EORIndirectX   => 6,
            Op::EORZeroPage    => 3,
            Op::LSRZeroPage    => 5,
            Op::PHAImplied     => 3,
            Op::EORImmediate   => 2,
            Op::LSRAccumulator => 2,
            Op::JMPAbsolute    => 3,
            Op::EORAbsolute    => 4,
            Op::LSRAbsolute    => 6,
            Op::BVCRelative    => 2,
            Op::EORIndirectY   => 5,
            Op::EORZeroPageX   => 4,
            Op::LSRZeroPageX   => 6,
            Op::CLIImplied     => 2,
            Op::EORAbsoluteY   => 4,
            Op::EORAbsoluteX   => 4,
            Op::LSRAbsoluteX   => 7,
            Op::RTSImplied     => 6,
            Op::ADCIndirectX   => 6,
            Op::ADCZeroPage    => 3,
            Op::RORZeroPage    => 5,
            Op::PLAImplied     => 4,
            Op::ADCImmediate   => 2,
            Op::RORAccumulator => 2,
            Op::JMPAbsIndirect => 5,
            Op::ADCAbsolute    => 4,
            Op::RORAbsolute    => 6,
            Op::BVSRelative    => 2,
            Op::ADCIndirectY   => 5,
            Op::ADCZeroPageX   => 4,
            Op::RORZeroPageX   => 6,
            Op::SEIImplied     => 2,
            Op::ADCAbsoluteY   => 4,
            Op::ADCAbsoluteX   => 4,
            Op::RORAbsoluteX   => 7,
            Op::STAIndirectX   => 6,
            Op::STYZeroPage    => 3,
            Op::STAZeroPage    => 3,
            Op::STXZeroPage    => 3,
            Op::DEYImplied     => 2,
            Op::TXAImplied     => 2,
            Op::STYAbsolute    => 4,
            Op::STXAbsolute    => 4,
            Op::STAAbsolute    => 4,
            Op::BCCRelative    => 2,
            Op::STAIndirectY   => 6,
            Op::STYZeroPageX   => 4,
            Op::STAZeroPageX   => 4,
            Op::STXZeroPageY   => 4,
            Op::TYAImplied     => 2,
            Op::STAAbsoluteY   => 5,
            Op::TXSImplied     => 2,
            Op::STAAbsoluteX   => 5,
            Op::LDYImmediate   => 2,
            Op::LDAIndirectX   => 6,
            Op::LDXImmediate   => 2,
            Op::LDYZeroPage    => 3,
            Op::LDAZeroPage    => 3,
            Op::LDXZeroPage    => 3,
            Op::TAYImplied     => 2,
            Op::LDAImmediate   => 2,
            Op::TAXImplied     => 2,
            Op::LDAAbsolute    => 4,
            Op::LDYAbsolute    => 4,
            Op::LDXAbsolute    => 4,
            Op::BCSRelative    => 2,
            Op::LDAIndirectY   => 5,
            Op::LDYZeroPageX   => 4,
            Op::LDAZeroPageX   => 4,
            Op::LDXZeroPageY   => 4,
            Op::CLVImplied     => 2,
            Op::LDAAbsoluteY   => 4,
            Op::TSXImplied     => 2,
            Op::LDYAbsoluteX   => 4,
            Op::LDAAbsoluteX   => 4,
            Op::LDXAbsoluteY   => 4,
            Op::CPYImmediate   => 2,
            Op::CMPIndirectX   => 6,
            Op::CPYZeroPage    => 3,
            Op::CMPZeroPage    => 3,
            Op::DECZeroPage    => 5,
            Op::INYImplied     => 2,
            Op::CMPImmediate   => 2,
            Op::DEXImplied     => 2,
            Op::CPYAbsolute    => 4,
            Op::CMPAbsolute    => 4,
            Op::DECAbsolute    => 4,
            Op::BNERelative    => 2,
            Op::CMPIndirectY   => 5,
            Op::CMPZeroPageX   => 4,
            Op::DECZeroPageX   => 6,
            Op::CLDImplied     => 2,
            Op::CMPAbsoluteY   => 4,
            Op::CMPAbsoluteX   => 4,
            Op::DECAbsoluteX   => 7,
            Op::CPXImmediate   => 2,
            Op::SBCIndirectX   => 6,
            Op::CPXZeroPage    => 3,
            Op::SBCZeroPage    => 3,
            Op::INCZeroPage    => 5,
            Op::INXImplied     => 2,
            Op::SBCImmediate   => 2,
            Op::CPXAbsolute    => 4,
            Op::INCAbsolute    => 6,
            Op::SBCAbsolute    => 4,
            Op::BEQRelative    => 2,
            Op::SBCIndirectY   => 5,
            Op::SBCZeroPageX   => 4,
            Op::INCZeroPageX   => 6,
            Op::SEDImplied     => 2,
            Op::SBCAbsoluteY   => 2,
            Op::SBCAbsoluteX   => 4,
            Op::INCAbsoluteX   => 7,
            Op::NOPImplied     => 2,
        }
    }

    //6502 opcode info http://obelisk.me.uk/6502/reference.html
    pub fn do_instruction(&mut self, mmu: &mut NESMmu) -> (bool, u32) {
        //Read 3 bytes (1st is opcode, 2nd is first operand (if any), 3rd is second operand (if any))
        let op = mmu.read_mem(self.pc) as u32;
        let imm1 = (mmu.read_mem(self.pc.wrapping_add(1)) as u32) << 8;
        let imm2 = (mmu.read_mem(self.pc.wrapping_add(2)) as u32) << 16;

        let opcode = Opcode::new(op | imm1 | imm2);
        let mut delta_cycles = 0;

        //println!("pc: 0x{:04X}", self.pc);

        //using a nifty crate that can convert integers to enums
        //to make pattern matching nicer
        match Op::from_i32(opcode.op() as i32) {
            Some(op) => {
                //print!("{:?} ", op);
                delta_cycles = self.get_op_cycles(&op);

                match op {

                    //IMPLIED
                    Op::BRKImmediate => {
                        //if self.p.irq_disable == false {
                            self.p.set_irq_disable(true);
                            let return_point = self.pc.wrapping_add(2);

                            let p = self.p.to_u8();
                            self.push_stack(mmu, ((return_point&0xFF00) >> 8) as u8);
                            self.push_stack(mmu, (return_point&0xFF) as u8);
                            self.push_stack(mmu, p);

                            let addr_lo = mmu.read_absolute(0xFFFE) as u16;
                            let addr_hi = mmu.read_absolute(0xFFFF) as u16;

                            self.pc = (addr_hi << 8) | addr_lo;
                        //} else {
                            //self.offset_pc(1);
                        //}

                        //return false;
                    }

                    Op::ORAIndirectX => {
                        let mut val = mmu.read_indexed_indirect_x(opcode.imm1(), self.x);
                        let a = self.a;
                        val = self.or(a, val);

                        self.a = val;

                        self.offset_pc(2);
                    }

                    //ORs the accumulator with memory
                    //(modifies zero and negative flag)
                    Op::ORAZeroPage => {
                        let mut val = mmu.read_zero_page(opcode.imm1());
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
                        let mut val = mmu.read_zero_page(opcode.imm1());
                        val = self.shift_left(val);

                        mmu.write_zero_page(opcode.imm1(), val);

                        self.offset_pc(2);
                    }

                    Op::PHPImplied => {
                        let mut p = self.p.to_u8();
                        p |= 1 << 4;
                        p |= 1 << 5;
                        self.push_stack(mmu, p);

                        self.offset_pc(1);
                    }

                    Op::ASLAccumulator => {
                        let a = self.a;

                        self.a = self.shift_left(a);

                        self.offset_pc(1);
                    }

                    Op::ORAAbsolute => {
                        let val = mmu.read_absolute(opcode.abs_addr());
                        let a = self.a;

                        self.a = self.or(a, val);

                        self.offset_pc(3);
                    }

                    Op::ASLAbsolute => {
                        let mut val = mmu.read_absolute(opcode.abs_addr());

                        val = self.shift_left(val);

                        mmu.write_absolute(opcode.abs_addr(), val);

                        self.offset_pc(3);
                    }

                    //Branch if Plus (adds to the program counter if negative flag is clear)
                    Op::BPLRelative => {
                        if self.p.negative == false {
                            ////print!("Branching from 0x{:04X}", self.pc);
                            self.offset_pc(opcode.imm1().cast_with_neg());
                            ////println!(" to 0x{:04X} (PC + 0x{:02X})", self.pc + 2, opcode.imm1().cast_with_neg());
                        }

                        self.offset_pc(2);
                    }

                    Op::ORAIndirectY => {
                        let mut val = mmu.read_indexed_indirect_y(opcode.imm1(), self.y);
                        let a = self.a;
                        val = self.or(a, val);

                        self.a = val;

                        self.offset_pc(2);
                    }

                    Op::ORAZeroPageX => {
                        let mut val = mmu.read_zero_paged_indexed_x(opcode.imm1(), self.x);
                        let a = self.a;

                        let val = self.or(a, val);

                        self.a = val;

                        self.offset_pc(2);
                    }

                    Op::ASLZeroPageX => {
                        let mut val = mmu.read_zero_paged_indexed_x(opcode.imm1(), self.x);
                        val = self.shift_left(val);

                        mmu.write_zero_paged_indexed_x(opcode.imm1(), self.x, val);

                        self.offset_pc(2);
                    }

                    //Clears the Carry flag
                    Op::CLCImplied => {
                        self.p.set_carry(false);

                        self.offset_pc(1);
                    }

                    Op::ORAAbsoluteY => {
                        let mut val = mmu.read_absolute_indexed_y(opcode.abs_addr(), self.y);
                        let a = self.a;

                        val = self.or(a, val);

                        self.a = val;

                        self.offset_pc(3);
                    }

                    Op::ORAAbsoluteX => {
                        let mut val = mmu.read_absolute_indexed_x(opcode.abs_addr(), self.x);
                        let a = self.a;

                        val = self.or(a, val);

                        self.a = val;

                        self.offset_pc(3);
                    }

                    Op::ASLAbsoluteX => {
                        let mut val = mmu.read_absolute_indexed_x(opcode.abs_addr(), self.x);

                        val = self.shift_left(val);

                        mmu.write_absolute_indexed_x(opcode.abs_addr(), self.x, val);

                        self.offset_pc(3);
                    }

                    //Pushes return point onto stack (pc + 3), then sets pc to absolute address
                    Op::JSRAbsolute => {
                        //print!("Jumping from 0x{:04X} --- ", self.pc);

                        let return_point = self.pc.wrapping_add(2);
                        let addr = opcode.abs_addr() as u16;

                        self.push_stack(mmu, ((return_point&0xFF00) >> 8) as u8);
                        self.push_stack(mmu, (return_point&0xFF) as u8);
                        self.pc = addr;

                        //println!("Jumping to 0x{:04X}", self.pc);
                    }

                    Op::ANDIndirectX => {
                        let mut val = mmu.read_indexed_indirect_x(opcode.imm1(), self.x);
                        let a = self.a;

                        let val = self.and(a, val);

                        self.a = val;

                        self.offset_pc(2);
                    }
                    
                    Op::BITZeroPage => {
                        let val = mmu.read_zero_page(opcode.imm1());
                        let a = self.a;

                        let _ = self.and(a, val);
                        
                        self.p.set_overflow((val&0x40) > 0);
                        self.p.set_negative((val&0x80) > 0);

                        self.offset_pc(2);
                    }

                    Op::ANDAbsolute => {
                        let val = mmu.read_absolute(opcode.abs_addr());
                        let a = self.a;

                        self.a = self.and(a, val);

                        self.offset_pc(3);
                    }

                    Op::ANDZeroPage => {
                        let mut val = mmu.read_zero_page(opcode.imm1());
                        let a = self.a;

                        let val = self.and(a, val);

                        self.a = val;

                        self.offset_pc(2);
                    }

                    Op::ROLZeroPage => {
                        let mut val = mmu.read_zero_page(opcode.imm1());
                        val = self.rol(val);

                        mmu.write_zero_page(opcode.imm1(), val);

                        self.offset_pc(2);
                    }

                    Op::PLPImplied => {
                        let val = self.pop_stack(mmu);

                        self.p = CPUStatus::from(val);

                        self.offset_pc(1);
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
                        let val = mmu.read_absolute(opcode.abs_addr());
                        let a = self.a;

                        let _ = self.and(a, val);
                        
                        self.p.set_overflow((val&0x40) > 0);
                        self.p.set_negative((val&0x80) > 0);

                        self.offset_pc(3);
                    }

                    Op::ROLAbsolute => {
                        let mut val = mmu.read_absolute(opcode.abs_addr());
                        val = self.rol(val);

                        mmu.write_absolute(opcode.abs_addr(), val);

                        self.offset_pc(3);
                    }

                    Op::BMIRelative => {
                        if self.p.negative == true {
                            ////print!("Branching from 0x{:04X}", self.pc);
                            self.offset_pc(opcode.imm1().cast_with_neg());
                            ////println!(" to 0x{:04X} (PC + 0x{:02X})", self.pc + 2, opcode.imm1().cast_with_neg());
                        }

                        self.offset_pc(2);
                    }

                    Op::ANDIndirectY => {
                        let mut val = mmu.read_indexed_indirect_y(opcode.imm1(), self.y);
                        let a = self.a;

                        let val = self.and(a, val);

                        self.a = val;

                        self.offset_pc(2);
                    }

                    Op::ANDAbsoluteY => {
                        let mut val = mmu.read_absolute_indexed_y(opcode.abs_addr(), self.y);
                        let a = self.a;

                        let val = self.and(a, val);

                        self.a = val;

                        self.offset_pc(3);
                    }

                    Op::ANDAbsoluteX => {
                        let mut val = mmu.read_absolute_indexed_x(opcode.abs_addr(), self.x);
                        let a = self.a;

                        let val = self.and(a, val);

                        self.a = val;

                        self.offset_pc(3);
                    }

                    Op::ANDZeroPageX => {
                        let mut val = mmu.read_zero_paged_indexed_x(opcode.imm1(), self.x);
                        let a = self.a;

                        let val = self.and(a, val);

                        self.a = val;

                        self.offset_pc(2);
                    }

                    Op::ROLZeroPageX => {
                        let mut val = mmu.read_zero_paged_indexed_x(opcode.imm1(), self.x);
                        
                        val = self.rol(val);

                        mmu.write_zero_paged_indexed_x(opcode.imm1(), self.x, val);

                        self.offset_pc(2);
                    }

                    Op::SECImplied => {
                        self.p.set_carry(true);

                        self.offset_pc(1);
                    }

                    Op::ROLAbsoluteX => {
                        let mut val = mmu.read_absolute_indexed_x(opcode.abs_addr(), self.x);
                        
                        val = self.rol(val);

                        mmu.write_absolute_indexed_x(opcode.abs_addr(), self.x, val);

                        self.offset_pc(3);
                    }

                    Op::RTIImplied => {
                        let p = self.pop_stack(mmu) as u8;
                        let lo = self.pop_stack(mmu) as u16;
                        let hi = self.pop_stack(mmu) as u16;
                        let ret = (hi << 8) | lo;

                        self.p = CPUStatus::from(p);
                        self.pc = ret;
                    }

                    Op::EORIndirectX => {
                        let mut val = mmu.read_indexed_indirect_x(opcode.imm1(), self.x);
                        let a = self.a;
                        val = self.eor(a, val);

                        self.a = val;

                        self.offset_pc(2);
                    }

                    Op::LSRZeroPage => {
                        let mut val = mmu.read_zero_page(opcode.imm1());
                        val = self.shift_right(val);

                        mmu.write_zero_page(opcode.imm1(), val);

                        self.offset_pc(2);
                    }

                    Op::EORZeroPage => {
                        let mut val = mmu.read_zero_page(opcode.imm1());
                        let a = self.a;
                        val = self.eor(a, val);

                        self.a = val;

                        self.offset_pc(2);
                    }

                    Op::PHAImplied => {
                        let a = self.a;
                        self.push_stack(mmu, a);

                        self.offset_pc(1);
                    }

                    Op::EORImmediate => {
                        let a = self.a;
                        self.a = self.eor(a, opcode.imm1());

                        self.offset_pc(2);
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
                        let val = mmu.read_absolute(opcode.abs_addr());
                        let a = self.a;

                        self.a = self.eor(a, val);

                        self.offset_pc(3);
                    }

                    Op::LSRAbsolute => {
                        let mut val = mmu.read_absolute(opcode.abs_addr());

                        val = self.shift_right(val);

                        mmu.write_absolute(opcode.abs_addr(), val);

                        self.offset_pc(3);
                    }

                    Op::BVCRelative => {
                        if self.p.overflow == false {
                            ////print!("Branching from 0x{:04X}", self.pc);
                            self.offset_pc(opcode.imm1().cast_with_neg());
                            ////println!(" to 0x{:04X} (PC + 0x{:02X})", self.pc + 2, opcode.imm1().cast_with_neg());
                        }

                        self.offset_pc(2);
                    }

                    Op::EORIndirectY => {
                        let mut val = mmu.read_indexed_indirect_y(opcode.imm1(), self.y);
                        let a = self.a;
                        val = self.eor(a, val);

                        self.a = val;

                        self.offset_pc(2);
                    }

                    Op::EORZeroPageX => {
                        let mut val = mmu.read_zero_paged_indexed_x(opcode.imm1(), self.x);
                        let a = self.a;

                        let val = self.eor(a, val);

                        self.a = val;

                        self.offset_pc(2);
                    }

                    Op::LSRZeroPageX => {
                        let mut val = mmu.read_zero_paged_indexed_x(opcode.imm1(), self.x);
                        val = self.shift_right(val);

                        mmu.write_zero_paged_indexed_x(opcode.imm1(), self.x, val);

                        self.offset_pc(2);
                    }

                    Op::CLIImplied => {
                        self.p.set_irq_disable(false);

                        self.offset_pc(1);
                    }

                    Op::EORAbsoluteY => {
                        let mut val = mmu.read_absolute_indexed_y(opcode.abs_addr(), self.y);
                        let a = self.a;

                        let val = self.eor(a, val);

                        self.a = val;

                        self.offset_pc(3);
                    }

                    Op::EORAbsoluteX => {
                        let mut val = mmu.read_absolute_indexed_x(opcode.abs_addr(), self.x);
                        let a = self.a;

                        let val = self.eor(a, val);

                        self.a = val;

                        self.offset_pc(3);
                    }

                    Op::LSRAbsoluteX => {
                        let mut val = mmu.read_absolute_indexed_x(opcode.abs_addr(), self.x);

                        val = self.shift_right(val);

                        mmu.write_absolute_indexed_x(opcode.abs_addr(), self.x, val);

                        self.offset_pc(3);
                    }

                    Op::RTSImplied => {
                        //print!("Returning from 0x{:04X} sp: 0x{:04X} -- ", self.pc, self.s);

                        let lo = self.pop_stack(mmu) as u16;
                        let hi = self.pop_stack(mmu) as u16;
                        let ret = (hi << 8) | lo;

                        self.pc = ret;

                        self.offset_pc(1);

                        //////println!("S: {:04X}", self.s);
                        //println!("Returning to 0x{:04X} - sp: 0x{:04X}", self.pc, self.s);
                    }

                    Op::ADCIndirectX => {
                        let mut val = mmu.read_indexed_indirect_x(opcode.imm1(), self.x);
                        let a = self.a;
                        val = self.add_with_carry(a, val);

                        self.a = val;

                        self.offset_pc(2);
                    }

                    Op::ADCZeroPage => {
                        let mut val = mmu.read_zero_page(opcode.imm1());
                        let a = self.a;
                        val = self.add_with_carry(a, val);

                        self.a = val;

                        self.offset_pc(2);
                    }

                    Op::RORZeroPage => {
                        let mut val = mmu.read_zero_page(opcode.imm1());
                        
                        val = self.ror(val);

                        mmu.write_zero_page(opcode.imm1(), val);

                        self.offset_pc(2);
                    }

                    Op::PLAImplied => {
                        let val = self.pop_stack(mmu);

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
                        let mut val = mmu.read_absolute(opcode.abs_addr());
                        let a = self.a;
                        val = self.add_with_carry(a, val);

                        self.a = val;

                        self.offset_pc(3);
                    }

                    Op::RORAbsolute => {
                        let mut val = mmu.read_absolute(opcode.abs_addr());
                        
                        val = self.ror(val);

                        mmu.write_absolute(opcode.abs_addr(), val);

                        self.offset_pc(3);
                    }

                    Op::BVSRelative => {
                        if self.p.overflow == true {
                            ////print!("Branching from 0x{:04X}", self.pc);
                            self.offset_pc(opcode.imm1().cast_with_neg());
                            ////println!(" to 0x{:04X} (PC + 0x{:02X})", self.pc + 2, opcode.imm1().cast_with_neg());
                        }

                        self.offset_pc(2);
                    }

                    Op::ADCIndirectY => {
                        let mut val = mmu.read_indexed_indirect_y(opcode.imm1(), self.y);
                        let a = self.a;
                        val = self.add_with_carry(a, val);

                        self.a = val;

                        self.offset_pc(2);
                    }

                    Op::ADCZeroPageX => {
                        let mut val = mmu.read_zero_paged_indexed_x(opcode.imm1(), self.x);
                        let a = self.a;

                        let val = self.add_with_carry(a, val);

                        self.a = val;

                        self.offset_pc(2);
                    }

                    Op::RORZeroPageX => {
                        let mut val = mmu.read_zero_paged_indexed_x(opcode.imm1(), self.x);
                        
                        val = self.ror(val);

                        mmu.write_zero_paged_indexed_x(opcode.imm1(), self.x, val);

                        self.offset_pc(2);
                    }

                    //Set Interrupt Disable (Sets the I flag to true)
                    Op::SEIImplied => {
                        self.p.set_irq_disable(true);

                        self.offset_pc(1);
                    }

                    Op::ADCAbsoluteY => {
                        let mut val = mmu.read_absolute_indexed_y(opcode.abs_addr(), self.y);
                        let a = self.a;
                        val = self.add_with_carry(a, val);

                        self.a = val;

                        self.offset_pc(3);
                    }

                    Op::ADCAbsoluteX => {
                        let mut val = mmu.read_absolute_indexed_x(opcode.abs_addr(), self.x);
                        let a = self.a;
                        val = self.add_with_carry(a, val);

                        self.a = val;

                        self.offset_pc(3);
                    }

                    Op::RORAbsoluteX => {
                        let mut val = mmu.read_absolute_indexed_x(opcode.abs_addr(), self.x);
                        
                        val = self.ror(val);

                        mmu.write_absolute_indexed_x(opcode.abs_addr(), self.x, val);

                        self.offset_pc(3);
                    }

                    Op::RORAccumulator => {
                        let a = self.a;

                        self.a = self.ror(a);

                        self.offset_pc(1);
                    }

                    Op::JMPAbsIndirect => {
                        let addr = mmu.read_absolute(opcode.abs_addr()) as u16 | ((mmu.read_absolute(opcode.abs_addr() + 1) as u16) << 8);

                        self.pc = addr;
                    }

                    Op::STAIndirectX => {
                        mmu.write_indexed_indirect_x(opcode.imm1(), self.x, self.a);

                        self.offset_pc(2);
                    }

                    Op::STYZeroPage => {
                        mmu.write_zero_page(opcode.imm1(), self.y);

                        self.offset_pc(2);
                    }

                    Op::STAZeroPage => {
                        mmu.write_zero_page(opcode.imm1(), self.a);

                        self.offset_pc(2);
                    }

                    Op::STXZeroPage => {
                        mmu.write_zero_page(opcode.imm1(), self.x);

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
                        mmu.write_absolute(opcode.abs_addr(), self.y);

                        self.offset_pc(3);
                    }

                    Op::STXAbsolute => {
                        mmu.write_absolute(opcode.abs_addr(), self.x);

                        self.offset_pc(3);
                    }

                    //Store Accumulator (Stores a into memory with absolute addressing)
                    Op::STAAbsolute => {
                        mmu.write_absolute(opcode.abs_addr(), self.a);

                        self.offset_pc(3);
                    }

                    Op::BCCRelative => {
                        if self.p.carry == false {
                            ////print!("Branching from 0x{:04X}", self.pc);
                            self.offset_pc(opcode.imm1().cast_with_neg());
                            ////println!(" to 0x{:04X} (PC + 0x{:02X})", self.pc + 2, opcode.imm1().cast_with_neg());
                        }

                        self.offset_pc(2);
                    }

                    Op::STAIndirectY => {
                        mmu.write_indexed_indirect_y(opcode.imm1(), self.y, self.a);

                        self.offset_pc(2);
                    }

                    Op::STYZeroPageX => {
                        mmu.write_zero_paged_indexed_x(opcode.imm1(), self.x, self.y);

                        self.offset_pc(2);
                    }

                    //Store Accumulator (Stores into memory with zero paged x addressing)
                    Op::STAZeroPageX => {
                        mmu.write_zero_paged_indexed_x(opcode.imm1(), self.x, self.a);

                        self.offset_pc(2);
                    }

                    Op::STXZeroPageY => {
                        mmu.write_zero_paged_indexed_y(opcode.imm1(), self.y, self.x);

                        self.offset_pc(2);
                    }

                    Op::TYAImplied => {
                        let y = self.y;
                        self.set_a(y);

                        self.offset_pc(1);
                    }

                    Op::STAAbsoluteY => {
                        mmu.write_absolute_indexed_y(opcode.abs_addr(), self.y, self.a);

                        self.offset_pc(3);
                    }

                    //Transfers x-index into stack pointer
                    Op::TXSImplied => {
                        self.s = self.x;

                        //////println!("TXS sp: 0x{:04X}", self.s);

                        self.offset_pc(1);
                    }

                    Op::STAAbsoluteX => {
                        mmu.write_absolute_indexed_x(opcode.abs_addr(), self.x, self.a);

                        self.offset_pc(3);
                    }

                    //Loads operand into y-index (modifies zero and negatives flags)
                    Op::LDYImmediate => {
                        self.set_y(opcode.imm1());

                        self.offset_pc(2);
                    }

                    Op::LDAIndirectX => {
                        let val = mmu.read_indexed_indirect_x(opcode.imm1(), self.x);

                        self.set_a(val);

                        self.offset_pc(2);
                    }

                    //Loads operand into x-index (modifies zero and negatives flags)
                    Op::LDXImmediate => {
                        self.set_x(opcode.imm1());

                        self.offset_pc(2);
                    }

                    Op::LDYZeroPage => {
                        let val = mmu.read_zero_page(opcode.imm1());

                        self.set_y(val);

                        self.offset_pc(2);
                    }

                    Op::LDXZeroPage => {
                        let val = mmu.read_zero_page(opcode.imm1());

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
                        let val = mmu.read_absolute(opcode.abs_addr());

                        self.set_y(val);

                        self.offset_pc(3);
                    }


                    Op::LDXAbsolute => {
                        let val = mmu.read_absolute(opcode.abs_addr());

                        self.set_x(val);

                        self.offset_pc(3);
                    }

                    Op::LDAZeroPage => {
                        let val = mmu.read_zero_page(opcode.imm1());

                        self.set_a(val);

                        self.offset_pc(2);
                    }

                    //Loads operand into accumulator using absolute addressing
                    //(modifies zero and negatives flags)
                    Op::LDAAbsolute => {
                        let val = mmu.read_absolute(opcode.abs_addr());

                        self.set_a(val);

                        self.offset_pc(3);
                    }

                    //Branch if carry set (adds to the program counter if carry flag is set)
                    Op::BCSRelative => {
                        if self.p.carry == true {
                            ////print!("Branching from 0x{:04X}", self.pc);
                            self.offset_pc(opcode.imm1().cast_with_neg());
                            ////println!(" to 0x{:04X} (PC + 0x{:02X})", self.pc + 2, opcode.imm1().cast_with_neg());
                        }

                        self.offset_pc(2);
                    }

                    Op::LDAIndirectY => {
                        let val = mmu.read_indexed_indirect_y(opcode.imm1(), self.y);

                        self.set_a(val);

                        self.offset_pc(2);
                    }

                    //Loads value into y-index from zero page x memory
                    Op::LDYZeroPageX => {
                        let val = mmu.read_zero_paged_indexed_x(opcode.imm1(), self.x);

                        self.set_y(val);

                        self.offset_pc(2);
                    }

                    //Loads value into accumulator from zero page x memory
                    Op::LDAZeroPageX => {
                        let val = mmu.read_zero_paged_indexed_x(opcode.imm1(), self.x);

                        self.set_a(val);

                        self.offset_pc(2);
                    }

                    Op::LDXZeroPageY => {
                        let val = mmu.read_zero_paged_indexed_y(opcode.imm1(), self.y);

                        self.set_x(val);

                        self.offset_pc(2);
                    }

                    Op::CLVImplied => {
                        self.p.set_overflow(false);

                        self.offset_pc(1);
                    }

                    Op::LDAAbsoluteY => {
                        let val = mmu.read_absolute_indexed_y(opcode.abs_addr(), self.y);

                        self.set_a(val);

                        self.offset_pc(3);
                    }

                    Op::TSXImplied => {
                        let s = self.s;

                        self.set_x(s);

                        self.offset_pc(1);
                    }

                    Op::LDYAbsoluteX => {
                        let val = mmu.read_absolute_indexed_x(opcode.abs_addr(), self.x);
                        
                        self.set_y(val);

                        self.offset_pc(3);
                    }

                    //Loads operand into accumulator using absolute indexed addressing
                    //(modifies zero and negatives flags)
                    Op::LDAAbsoluteX => {
                        let val = mmu.read_absolute_indexed_x(opcode.abs_addr(), self.x);
                        
                        self.set_a(val);

                        self.offset_pc(3);
                    }

                    Op::LDXAbsoluteY => {
                        let val = mmu.read_absolute_indexed_y(opcode.abs_addr(), self.y);
                        
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
                        let val = mmu.read_indexed_indirect_x(opcode.imm1(), self.x);
                        let a = self.a;
                        let _ = self.subtract(a, val);

                        self.offset_pc(2);
                    }

                    Op::CMPZeroPage => {
                        let val = mmu.read_zero_page(opcode.imm1());
                        let a = self.a;
                        let _ = self.subtract(a, val);

                        self.offset_pc(2);
                    }

                    Op::DECZeroPage => {
                        let mut val = mmu.read_zero_page(opcode.imm1());
                        val = self.subtract_no_carry(val, 1);

                        mmu.write_zero_page(opcode.imm1(), val);

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

                    Op::CPYAbsolute => {
                        let y = self.y;
                        let val = mmu.read_absolute(opcode.abs_addr());
                        let _ = self.subtract(y, val);

                        self.offset_pc(3);
                    }

                    Op::CMPAbsolute => {
                        let a = self.a;
                        let val = mmu.read_absolute(opcode.abs_addr());
                        let _ = self.subtract(a, val);

                        self.offset_pc(3);
                    }
                    
                    Op::DECAbsolute => {
                        let mut val = mmu.read_absolute(opcode.abs_addr());
                        val = self.subtract_no_carry(val, 1);

                        mmu.write_absolute(opcode.abs_addr(), val);

                        self.offset_pc(3);
                    }

                    //Branch if not equal (adds to the program counter if zero flag is not set)
                    Op::BNERelative => {
                        if self.p.zero == false {
                            ////print!("Branching from 0x{:04X}", self.pc);
                            self.offset_pc(opcode.imm1().cast_with_neg());
                            ////println!(" to 0x{:04X} (PC + 0x{:02X})", self.pc + 2, opcode.imm1().cast_with_neg());
                        }

                        self.offset_pc(2);
                    }

                    Op::CMPIndirectY => {
                        let val = mmu.read_indexed_indirect_y(opcode.imm1(), self.y);
                        let a = self.a;
                        let _ = self.subtract(a, val);

                        self.offset_pc(2);
                    }

                    Op::CMPZeroPageX => {
                        let val = mmu.read_zero_paged_indexed_x(opcode.imm1(), self.x);
                        let a = self.a;
                        let _ = self.subtract(a, val);

                        self.offset_pc(2);
                    }

                    Op::DECZeroPageX => {
                        let mut val = mmu.read_zero_paged_indexed_x(opcode.imm1(), self.x);
                        val = self.subtract_no_carry(val, 1);

                        mmu.write_zero_paged_indexed_x(opcode.imm1(), self.x, val);

                        self.offset_pc(2);
                    }

                    //Clear Decimal Mode (Sets the D flag to false)
                    Op::CLDImplied => {
                        self.p.set_decimal(false);

                        self.offset_pc(1);
                    }

                    Op::CMPAbsoluteY => {
                        let a = self.a;
                        let val = mmu.read_absolute_indexed_y(opcode.abs_addr(), self.y);
                        let _ = self.subtract(a, val);

                        self.offset_pc(3);
                    }

                    Op::CMPAbsoluteX => {
                        let a = self.a;
                        let val = mmu.read_absolute_indexed_x(opcode.abs_addr(), self.x);
                        let _ = self.subtract(a, val);

                        self.offset_pc(3);
                    }

                    Op::DECAbsoluteX => {
                        let mut val = mmu.read_absolute_indexed_x(opcode.abs_addr(), self.x);
                        val = self.subtract_no_carry(val, 1);

                        mmu.write_absolute_indexed_x(opcode.abs_addr(), self.x, val);

                        self.offset_pc(3);
                    }

                    Op::CPXImmediate => {
                        let x = self.x;
                        let _ = self.subtract(x, opcode.imm1());

                        self.offset_pc(2);
                    }

                    Op::SBCIndirectX => {
                        let mut val = mmu.read_indexed_indirect_x(opcode.imm1(), self.x);
                        let a = self.a;
                        val = self.subtract_with_carry(a, val);

                        self.a = val;

                        self.offset_pc(2);
                    }

                    Op::CPYZeroPage => {
                        let val = mmu.read_zero_page(opcode.imm1());
                        let y = self.y;
                        let _ = self.subtract(y, val);

                        self.offset_pc(2);
                    }

                    Op::CPXZeroPage => {
                        let val = mmu.read_zero_page(opcode.imm1());
                        let x = self.x;
                        let _ = self.subtract(x, val);

                        self.offset_pc(2);
                    }
                    
                    Op::SBCZeroPage => {
                        let mut val = mmu.read_zero_page(opcode.imm1());
                        let a = self.a;
                        val = self.subtract_with_carry(a, val);

                        self.a = val;

                        self.offset_pc(2);
                    }

                    Op::INCZeroPage => {
                        let mut val = mmu.read_zero_page(opcode.imm1());
                        val = self.add_no_carry(val, 1);

                        mmu.write_zero_page(opcode.imm1(), val);

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
                        let val = mmu.read_absolute(opcode.abs_addr());
                        let _ = self.subtract(x, val);

                        self.offset_pc(3);
                    }

                    Op::INCAbsolute => {
                        let mut val = mmu.read_absolute(opcode.abs_addr());
                        val = self.add_no_carry(val, 1);

                        mmu.write_absolute(opcode.abs_addr(), val);

                        self.offset_pc(3);
                    }

                    Op::SBCAbsolute => {
                        let mut val = mmu.read_absolute(opcode.abs_addr());
                        let a = self.a;
                        val = self.subtract_with_carry(a, val);

                        self.a = val;

                        self.offset_pc(3);
                    }

                    Op::BEQRelative => {
                        if self.p.zero == true {
                            ////print!("Branching from 0x{:04X}", self.pc);
                            self.offset_pc(opcode.imm1().cast_with_neg());
                            ////println!(" to 0x{:04X} (PC + 0x{:02X})", self.pc + 2, opcode.imm1().cast_with_neg());
                        }

                        self.offset_pc(2);
                    }

                    Op::SBCIndirectY => {
                        let mut val = mmu.read_indexed_indirect_y(opcode.imm1(), self.y);
                        let a = self.a;
                        val = self.subtract_with_carry(a, val);

                        self.a = val;

                        self.offset_pc(2);
                    }

                    Op::SBCZeroPageX => {
                        let mut val = mmu.read_zero_paged_indexed_x(opcode.imm1(), self.x);
                        let a = self.a;

                        let val = self.subtract_with_carry(a, val);

                        self.a = val;

                        self.offset_pc(2);
                    }

                    Op::INCZeroPageX => {
                        let mut val = mmu.read_zero_paged_indexed_x(opcode.imm1(), self.x);
                        val = self.add_no_carry(val, 1);

                        mmu.write_zero_paged_indexed_x(opcode.imm1(), self.x, val);

                        self.offset_pc(2);
                    }

                    Op::SEDImplied => {
                        self.p.set_decimal(true);

                        self.offset_pc(1);
                    }

                    Op::SBCAbsoluteY => {
                        let mut val = mmu.read_absolute_indexed_y(opcode.abs_addr(), self.y);
                        let a = self.a;
                        val = self.subtract_with_carry(a, val);

                        self.a = val;

                        self.offset_pc(3);
                    }

                    Op::SBCAbsoluteX => {
                        let mut val = mmu.read_absolute_indexed_x(opcode.abs_addr(), self.x);
                        let a = self.a;
                        val = self.subtract_with_carry(a, val);

                        self.a = val;

                        self.offset_pc(3);
                    }

                    Op::INCAbsoluteX => {
                        let mut val = mmu.read_absolute_indexed_x(opcode.abs_addr(), self.x);
                        val = self.add_no_carry(val, 1);

                        mmu.write_absolute_indexed_x(opcode.abs_addr(), self.x, val);

                        self.offset_pc(3);
                    }

                    Op::NOPImplied => {
                        self.offset_pc(1);
                    }

                    _ => {
                        //println!("{:?}", opcode);
                        //println!("unimplemented opcode");
                        return (false,0)
                    }
                }
            }
            
            None => {
                //println!("unknown {:?}", opcode);
                return (false,0)
            }
        }
        
        //println!("{:?}", opcode);

        //println!("a: 0x{:02X}\nx: 0x{:02X}\ny: 0x{:02X}\ns: 0x{:02X}\n{:?}\n\n", self.a, self.x, self.y, self.s, self.p);

        return (true,delta_cycles);
    }
}