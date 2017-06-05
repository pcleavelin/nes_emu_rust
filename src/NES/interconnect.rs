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

    pub fn insert_cart(&mut self, rom: &str) {
        self.cart = NESCart::new(rom);

        self.cart.print_header();
    }

    //Reading Memory
    
    //BIG TODO: This will probably need to be /totally/ overhauled
    //in order for ROMS with different mappers to work; so for now
    //we are just going to hardcode mapper 000
    //https://wiki.nesdev.com/w/index.php/CPU_memory_map
    pub fn read_mem(&self, addr: usize) -> u8 {
        match addr {
            0x0000...0x1FFF => {
                self.internal_ram[addr % 0x0800]
            }

            0x2000...0x3FFF => {
                0 //TODO: return ppu registers
            }

            0x4000...0x4017 => {
                0 //TODO: return apu and i/o registers
            }

            0x4018...0x401F => {
                0 //do we need this?
            }

            0x4020...0xFFFF => {
                self.cart.read(addr - 0x4010)
            }

            _ => {
                println!("shouldn't get here, as the above code is exhaustive");
                return 0;
            }
        }
    }

    pub fn read_zero_page(&self, addr: usize) -> u8 {
        self.read_mem(addr % 256)
    }

    pub fn read_absolute(&self, addr: usize) -> u8 {
        self.read_mem(addr)
    }

    pub fn read_zero_paged_indexed_x(&self, addr: usize, x: usize) -> u8 {
        self.read_mem((addr + x) % 256)
    }

    pub fn read_zero_paged_indexed_y(&self, addr: usize, y: usize) -> u8 {
        self.read_mem((addr + y) % 256)
    }

    pub fn read_absolute_indexed_x(&self, addr: usize, x: usize) -> u8 {
        self.read_mem((addr + x))
    }

    pub fn read_absolute_indexed_y(&self, addr: usize, y: usize) -> u8 {
        self.read_mem((addr + y))
    }

    pub fn read_indexed_indirect_x(&self, addr: usize, x: usize) -> u8 {
        let pointer: usize = self.read_mem((addr + x) % 256) as usize + self.read_mem((addr + x + 1) % 256) as usize * 256;
        self.read_mem(pointer)
    }

    pub fn read_indexed_indirect_y(&self, addr: usize, y: usize) -> u8 {
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
            }

            0x4000...0x4017 => {
                //TODO: write to apu and i/o registers
            }

            0x4018...0x401F => {
                //do we need this?
            }

            0x4020...0xFFFF => {
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
}