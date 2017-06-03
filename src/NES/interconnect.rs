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

    pub fn read_zero_page(&self, addr: usize) -> u8 {
        self.internal_ram[addr % 256]
    }

    pub fn read_absolute(&self, addr: usize) -> u8 {
        self.internal_ram[addr]
    }

    pub fn read_zero_paged_indexed_x(&self, addr: usize, x: usize) -> u8 {
        self.internal_ram[(addr + x) % 256]
    }

    pub fn read_zero_paged_indexed_y(&self, addr: usize, y: usize) -> u8 {
        self.internal_ram[(addr + y) % 256]
    }

    pub fn read_absolute_indexed_x(&self, addr: usize, x: usize) -> u8 {
        self.internal_ram[(addr + x)]
    }

    pub fn read_absolute_indexed_y(&self, addr: usize, y: usize) -> u8 {
        self.internal_ram[(addr + y)]
    }

    pub fn read_indexed_indirect_x(&self, addr: usize, x: usize) -> u8 {
        let pointer = self.internal_ram[(addr + x) % 256] + self.internal_ram[(addr + x + 1) % 256] * 256;
        self.internal_ram[pointer as usize]
    }

    pub fn read_indexed_indirect_y(&self, addr: usize, y: usize) -> u8 {
        let pointer = self.internal_ram[(addr + y) % 256] + self.internal_ram[(addr + y + 1) % 256] * 256;
        self.internal_ram[pointer as usize]
    }

    
    pub fn write_zero_page(&mut self, addr: usize, val: u8) {
        self.internal_ram[addr % 256] = val;
    }

    pub fn write_absolute(&mut self, addr: usize, val: u8) {
        self.internal_ram[addr] = val;
    }

    pub fn write_zero_paged_indexed_x(&mut self, addr: usize, x: usize, val: u8) {
        self.internal_ram[(addr + x) % 256] = val;
    }

    pub fn write_zero_paged_indexed_y(&mut self, addr: usize, y: usize, val: u8) {
        self.internal_ram[(addr + y) % 256] = val;
    }

    pub fn write_absolute_indexed_x(&mut self, addr: usize, x: usize, val: u8) {
        self.internal_ram[(addr + x)] = val;
    }

    pub fn write_absolute_indexed_y(&mut self, addr: usize, y: usize, val: u8) {
        self.internal_ram[(addr + y)] = val;
    }

    pub fn write_indexed_indirect_x(&mut self, addr: usize, x: usize, val: u8) {
        let pointer = self.internal_ram[(addr + x) % 256] + self.internal_ram[(addr + x + 1) % 256] * 256;
        self.internal_ram[pointer as usize] = val;
    }

    pub fn write_indexed_indirect_y(&mut self, addr: usize, y: usize, val: u8) {
        let pointer = self.internal_ram[(addr + y) % 256] + self.internal_ram[(addr + y + 1) % 256] * 256;
        self.internal_ram[pointer as usize] = val;
    }
}