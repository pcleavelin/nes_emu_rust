mod cpu;
mod ppu;
mod apu;
mod io;
mod cart;
mod interconnect;

use self::cpu::*;
use self::interconnect::*;

pub struct NES {
    cpu: NESCpu,
    interconnect: Interconnect,
}

impl NES {
    pub fn new() -> NES {
        NES {
            cpu: NESCpu::new(),
            interconnect: Interconnect::new(),
        }
    }
}