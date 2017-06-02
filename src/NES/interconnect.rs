use super::ppu::*;
use super::apu::*;
use super::io::*;
use super::cart::*;

/*
This struct is really only used to pass the RAM and
the registers of other components to the cpu.
*/

pub struct Interconnect {
    internal_ram: [u8;0x0100],

    ppu: NESPpu,
    apu: NESApu, //Can't wait for this (never done audio before)
    io: NESIo,

    cart: NESCart,
}

impl Interconnect {
    pub fn new() -> Interconnect {
        Interconnect {
            internal_ram: [0u8;0x0100],
            
            ppu: NESPpu::new(),
            apu: NESApu::new(),
            io: NESIo::new(),

            cart: NESCart::none(),
        }
    }
}