use std::ops::{BitOr, BitAnd};

use super::interconnect::Interconnect;

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
    p: CPUStatus, //cpu status
}

impl NESCpu {
    pub fn new() -> NESCpu {
        NESCpu {
            a: 0,
            x: 0,
            y: 0,
            
            s: 0,
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
    }
    pub fn set_x(&mut self, val: u8) {
        self.x = val;
    }
    pub fn set_y(&mut self, val: u8) {
        self.y = val;
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

    pub fn do_instruction(&mut self, interconnect: &mut Interconnect) {
        //insert funness here
    }
}