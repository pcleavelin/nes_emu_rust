use minifb::{WindowOptions, Window, Key, Scale};

use super::ppu::*;
use super::apu::*;
use super::io::*;
use super::cart::*;

/*
This struct is really only used to pass the RAM and
the registers of other components to the cpu.
*/

pub struct Interconnect {
    internal_ram: [u8;0x0800],

    ppu: NESPpu,
    apu: NESApu, //Can't wait for this (never done audio before)
    io: NESIo,

    cart: NESCart,
}

impl Interconnect {
    pub fn new() -> Interconnect {
        Interconnect {
            internal_ram: [0u8;0x0800],
            
            ppu: NESPpu::new(),
            apu: NESApu::new(),
            io: NESIo::new(),

            cart: NESCart::none(),
        }
    }

    pub fn insert_cart(&mut self, rom: &str) {
        self.cart = NESCart::new(rom);

        self.cart.print_header();
    }

    pub fn ram(&self) -> [u8;0x0800] {
        self.internal_ram
    }

    pub fn get_pattern_table(&mut self, num: usize) -> &[u8] {
        match num {
            0...3 => {
                self.cart.get_pattern_table(num)
            }

            _ => {
                &[0;0x1000]
            }
        }
    }

    pub fn get_name_table(&mut self, num: usize) -> &[u8] {
        match num {
            0...3 => {
                &self.internal_ram[(num*0x400)..((num+1)*0x400)]
            }

            _ => {
                &[0;0x400]
            }
        }
    }

    pub fn update(&mut self, window: &mut Window) {
        let mut pt0 = self.cart.get_pattern_table(0);
        let mut pt1 = self.cart.get_pattern_table(1);

        let mut nt0 = &self.internal_ram[(0*0x400)..((0+1)*0x400)];
        let mut nt1 = &self.internal_ram[(1*0x400)..((1+1)*0x400)];

        self.ppu.do_cycle(pt0, pt1, nt0, nt1, window);
    }

    //Reading Memory
    
    //BIG TODO: This will probably need to be /totally/ overhauled
    //in order for ROMS with different mappers to work; so for now
    //we are just going to hardcode mapper 000
    //https://wiki.nesdev.com/w/index.php/CPU_memory_map
    pub fn read_mem(&mut self, addr: usize) -> u8 {
        match addr {
            0x0000...0x1FFF => {
                self.internal_ram[addr % 0x0800]
            }

            0x2000...0x3FFF => {
                self.ppu.read_ppu((addr - 0x2000) % 8)
            }

            0x4000...0x4017 => {
                0 //TODO: return apu and i/o registers
            }

            0x4018...0x401F => {
                0 //do we need this?
            }

            0x4020...0x5FFF => {
                0 //self.cart.read(addr - 0x4010)
            }

            0x6000...0x7FFF => {
                self.cart.read_ram(addr - 0x6000)
            }

            0x8000...0xBFFF => {
                self.cart.read(addr - (0x8000 - 0x10))
            }

            0xC000...0xFFFF => {
                self.cart.read(addr - (0xC000 - 0x10))
            }

            _ => {
                println!("shouldn't get here, as the above code is exhaustive");
                return 0;
            }
        }
    }

    pub fn read_zero_page(&mut self, addr: usize) -> u8 {
        self.read_mem(addr % 256)
    }

    pub fn read_absolute(&mut self, addr: usize) -> u8 {
        self.read_mem(addr)
    }

    pub fn read_zero_paged_indexed_x(&mut self, addr: usize, x: usize) -> u8 {
        self.read_mem((addr + x) % 256)
    }

    pub fn read_zero_paged_indexed_y(&mut self, addr: usize, y: usize) -> u8 {
        self.read_mem((addr + y) % 256)
    }

    pub fn read_absolute_indexed_x(&mut self, addr: usize, x: usize) -> u8 {
        self.read_mem((addr + x))
    }

    pub fn read_absolute_indexed_y(&mut self, addr: usize, y: usize) -> u8 {
        self.read_mem((addr + y))
    }

    pub fn read_indexed_indirect_x(&mut self, addr: usize, x: usize) -> u8 {
        let pointer: usize = self.read_mem((addr + x) % 256) as usize + self.read_mem((addr + x + 1) % 256) as usize * 256;
        self.read_mem(pointer)
    }

    pub fn read_indexed_indirect_y(&mut self, addr: usize, y: usize) -> u8 {
        let pointer: usize = self.read_mem((addr + y) % 256) as usize + self.read_mem((addr + y + 1) % 256) as usize * 256;
        self.read_mem(pointer)
    }

    //Writing Memory

    //BIG TODO: This will probably need to be /totally/ overhauled
    //in order for ROMS with different mappers to work; so for now
    //we are just going to hardcode mapper 000
    pub fn write_mem(&mut self, addr: usize, val: u8) {
        match addr {
            0x0000...0x1FFF => {
                self.internal_ram[addr % 0x0800] = val;
            }

            0x2000...0x3FFF => {
                //TODO: write to ppu registers
                //println!("Unimplemented Write to PPU registers!");
                self.ppu().write_ppu((addr - 0x2000) % 8, val);
            }

            0x4000...0x4017 => {
                //TODO: write to apu and i/o registers
                println!("Unimplemented Write to I/O registers!");
            }

            0x4018...0x401F => {
                //do we need this?
            }

            0x4020...0x5FFF => {
                
            }

            0x6000...0x7FFF => {
                self.cart.write_ram(addr - 0x6000, val)
            }

            0x8000...0xFFFF => {
                println!("tried to write to cart rom!");
            }

            _ => {
                println!("shouldn't get here, as the above code is exhaustive");
            }
        };
    }

    pub fn write_zero_page(&mut self, addr: usize, val: u8) {
        self.write_mem(addr % 256, val);
    }

    pub fn write_absolute(&mut self, addr: usize, val: u8) {
        self.write_mem(addr, val);
    }

    pub fn write_zero_paged_indexed_x(&mut self, addr: usize, x: usize, val: u8) {
        self.write_mem((addr + x) % 256, val);
    }

    pub fn write_zero_paged_indexed_y(&mut self, addr: usize, y: usize, val: u8) {
        self.write_mem((addr + y) % 256, val);
    }

    pub fn write_absolute_indexed_x(&mut self, addr: usize, x: usize, val: u8) {
        self.write_mem((addr + x), val);
    }

    pub fn write_absolute_indexed_y(&mut self, addr: usize, y: usize, val: u8) {
        self.write_mem((addr + y), val);
    }

    pub fn write_indexed_indirect_x(&mut self, addr: usize, x: usize, val: u8) {
        let pointer: usize = self.read_mem((addr + x) % 256) as usize + self.read_mem((addr + x + 1) % 256) as usize * 256;
        self.write_mem(pointer, val);
    }

    pub fn write_indexed_indirect_y(&mut self, addr: usize, y: usize, val: u8) {
        let pointer: usize = self.read_mem((addr + y) % 256) as usize + self.read_mem((addr + y + 1) % 256) as usize * 256;
        self.write_mem(pointer, val);
    }

    pub fn ppu(&mut self) -> &mut NESPpu {
        &mut self.ppu
    }
}