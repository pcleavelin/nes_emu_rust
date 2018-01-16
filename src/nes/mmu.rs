use super::ppu::NESPpu;
use super::io::NESIo;

pub struct NESMmu {
    /** PPU Memory */
    ppu_nametable1: [u8; 0x400],
    ppu_nametable2: [u8; 0x400],
    ppu_nametable3: [u8; 0x400],
    ppu_nametable4: [u8; 0x400],

    ppu_patterntable1: [u8; 0x1000],
    ppu_patterntable2: [u8; 0x1000],

    ppu_bg_palette0: [u8;3],
    ppu_bg_palette1: [u8;3],
    ppu_bg_palette2: [u8;3],
    ppu_bg_palette3: [u8;3],

    ppu_sprite_palette0: [u8;3],
    ppu_sprite_palette1: [u8;3],
    ppu_sprite_palette2: [u8;3],
    ppu_sprite_palette3: [u8;3],

    /** Cartridge Memory */
    cart_rom: Vec<u8>,

    /** CPU Memory */
    cpu_ram: [u8;0x800],

    pub ppu: Option<*mut NESPpu>,
    pub io: Option<*mut NESIo>,
}

impl NESMmu {
    pub fn new() -> NESMmu {
        NESMmu {
            /** PPU Memory */
            ppu_nametable1: [0u8; 0x400],
            ppu_nametable2: [0u8; 0x400],
            ppu_nametable3: [0u8; 0x400],
            ppu_nametable4: [0u8; 0x400],

            ppu_patterntable1: [0u8; 0x1000],
            ppu_patterntable2: [0u8; 0x1000],

            ppu_bg_palette0: [0u8;3],
            ppu_bg_palette1: [0u8;3],
            ppu_bg_palette2: [0u8;3],
            ppu_bg_palette3: [0u8;3],

            ppu_sprite_palette0: [0u8;3],
            ppu_sprite_palette1: [0u8;3],
            ppu_sprite_palette2: [0u8;3],
            ppu_sprite_palette3: [0u8;3],

            ppu: None,
            io: None,

            /** Cartridge Memory */
            cart_rom: Vec::new(),

            /** CPU Memory */
            cpu_ram: [0u8;0x800],
        }
    }

    pub fn fill_rom(&mut self, rom_data: Vec<u8>) {
        self.cart_rom = rom_data;
        println!("Filled rom with 0x{:08X} bytes", self.cart_rom.len());

        for i in 0..0x1000 {
            self.ppu_patterntable1[i] = self.cart_rom[self.cart_rom[4] as usize * 0x4000 + 0x10 + i];
            self.ppu_patterntable2[i] = self.cart_rom[self.cart_rom[4] as usize * 0x4000 + 0x1000 + 0x10 + i];
        }

        unsafe {
            match self.ppu {
                Some(ppu) => (*ppu).set_mirroring((self.cart_rom[6] & 0x1) > 0),
                None => {}
            }
        }
    }

    pub fn read_zero_page(&mut self, addr: u8) -> u8 {
        self.read_mem(addr as u16)
    }

    pub fn read_absolute(&mut self, addr: u16) -> u8 {
        self.read_mem(addr)
    }

    pub fn read_zero_paged_indexed_x(&mut self, addr: u8, x: u8) -> u8 {
        self.read_mem(addr.wrapping_add(x) as u16)
    }

    pub fn read_zero_paged_indexed_y(&mut self, addr: u8, y: u8) -> u8 {
        self.read_mem(addr.wrapping_add(y) as u16)
    }

    pub fn read_absolute_indexed_x(&mut self, addr: u16, x: u8) -> u8 {
        self.read_mem(addr.wrapping_add(x as u16))
    }

    pub fn read_absolute_indexed_y(&mut self, addr: u16, y: u8) -> u8 {
        self.read_mem(addr.wrapping_add(y as u16))
    }

    pub fn read_indexed_indirect_x(&mut self, addr: u8, x: u8) -> u8 {
        let pointer: u16 = self.read_mem(addr.wrapping_add(x) as u16) as u16 + ((self.read_mem(addr.wrapping_add(x).wrapping_add(1) as u16) as u16) << 8);
        self.read_mem(pointer)
    }

    pub fn read_indexed_indirect_y(&mut self, addr: u8, y: u8) -> u8 {
        let pointer: u16 = self.read_mem(addr as u16) as u16 + ((self.read_mem(addr.wrapping_add(1) as u16) as u16) << 8);
        self.read_mem(pointer.wrapping_add(y as u16))
    }

    pub fn write_zero_page(&mut self, addr: u8, val: u8) {
        self.write_mem(addr as u16, val);
    }

    pub fn write_absolute(&mut self, addr: u16, val: u8) {
        self.write_mem(addr, val);
    }

    pub fn write_zero_paged_indexed_x(&mut self, addr: u8, x: u8, val: u8) {
        self.write_mem(addr.wrapping_add(x) as u16, val);
    }

    pub fn write_zero_paged_indexed_y(&mut self, addr: u8, y: u8, val: u8) {
        self.write_mem(addr.wrapping_add(y) as u16, val);
    }

    pub fn write_absolute_indexed_x(&mut self, addr: u16, x: u8, val: u8) {
        self.write_mem((addr + x as u16), val);
    }

    pub fn write_absolute_indexed_y(&mut self, addr: u16, y: u8, val: u8) {
        self.write_mem((addr + y as u16), val);
    }

    pub fn write_indexed_indirect_x(&mut self, addr: u8, x: u8, val: u8) {
        let pointer: u16 = self.read_mem(addr.wrapping_add(x) as u16) as u16 + ((self.read_mem(addr.wrapping_add(x).wrapping_add(1) as u16) as u16) << 8);
        self.write_mem(pointer, val);
    }

    pub fn write_indexed_indirect_y(&mut self, addr: u8, y: u8, val: u8) {
        let pointer: u16 = self.read_mem(addr as u16) as u16 + ((self.read_mem(addr.wrapping_add(1) as u16) as u16) << 8);
        self.write_mem(pointer.wrapping_add(y as u16), val);
    }
    
    //BIG TODO: This will probably need to be /totally/ overhauled
    //in order for ROMS with different mappers to work; so for now
    //we are just going to hardcode mapper 000
    //https://wiki.nesdev.com/w/index.php/CPU_memory_map
    pub fn read_mem(&mut self, addr: u16) -> u8 {
        match addr {
            
            //CPU
            0x0000...0x1FFF => {
                self.cpu_ram[addr as usize % 0x800]
            }

            //PPU
            0x2000...0x3FFF => {
                let ppu_addr = (addr % 8) as usize;

                return self.read_ppu(ppu_addr);
            }

            //IO
            0x4016...0x4017 => {
                unsafe {
                    match self.io {
                        Some(io) => (*io).read(addr as usize),
                        None => 0
                    }
                }
            }

            0x6000...0x7FFF => {
                //self.cart.read_ram(fixed_addr - 0x6000)
                0
            }

            0x8000...0xBFFF => {
                if ((addr-0x8000)+0x10) as usize >= self.cart_rom.len() {
                    println!("error reading from cart: reached end of rom!");
                    return 0;
                }

                self.cart_rom[((addr - 0x8000) + 0x10) as usize]
            }

            0xC000...0xFFFF => {
                let offset = if self.cart_rom[4] > 1 {
                    0x4000
                } else {
                    0
                };

                if ((addr - 0xC000) + offset + 0x10) as usize >= self.cart_rom.len() {
                    println!("error reading from cart: reached end of rom!");
                    return 0;
                }
                self.cart_rom[((addr - 0xC000) + offset + 0x10) as usize]
            }


            _ => {
                0
            }
        }
    }

    pub fn write_mem(&mut self, addr: u16, val: u8) {
        match addr {
            0x0000...0x1FFF => {
                self.cpu_ram[addr as usize % 0x0800] = val;
            }

            0x2000...0x3FFF => {
                self.write_ppu((addr as usize - 0x2000) % 8, val);
            }

            0x4000...0x4017 => {
                if addr == 0x4014 {

                    match self.ppu {
                        Some(ppu) => {
                            unsafe {
                                (*ppu).dma(&self.cpu_ram[((val as usize) << 8)..(0xFF + ((val as usize) << 8) + 1)]);
                            }
                        }
                        None => {}
                    }
                }

                if addr == 0x4016 {
                    unsafe {
                        match self.io {
                            Some(io) => (*io).write(addr as usize, val),
                            None => {}
                        }
                    }
                }
            }

            0x4018...0x401F => {
                //do we need this?
            }

            0x4020...0x5FFF => {
            }

            0x6000...0x7FFF => {
            }

            0x8000...0xFFFF => {
            }

            _ => {
                println!("shouldn't get here, as the above code is exhaustive");
            }
        };
    }

    pub fn read_ppu(&mut self, addr: usize) -> u8 {
        unsafe {
            match addr {
                //Write-Only
                0 => {
                    match self.ppu {
                        Some(ppu) => {
                            return (*ppu).latch;
                        }
                        _ => return 0,
                    }
                }
                //Write-Only
                1 => {
                    match self.ppu {
                        Some(ppu) => {
                            return (*ppu).latch;
                        }
                        _ => return 0,
                    }
                }
                //Read-Only
                2 => {
                    match self.ppu {
                        Some( ppu) => {
                            return (*ppu).status&0xE0 | (*ppu).latch&0x1F;
                        }
                        None => {
                            return 0
                        }
                    }
                }
                //Write-Only
                3 => {
                    match self.ppu {
                        Some(ppu) => {
                            return (*ppu).latch;
                        }
                        _ => return 0,
                    }
                }
                //Read/Write
                4 => {
                    match self.ppu {
                        Some(ppu) => {
                            return (*ppu).oam[(*ppu).oam_addr as usize];
                        }
                        _ => return 0,
                    }
                }
                //Write-Only
                5 => {
                    match self.ppu {
                        Some(ppu) => {
                            return (*ppu).latch;
                        }
                        _ => return 0,
                    }
                }
                //Write-Only
                6 => {
                    match self.ppu {
                        Some(ppu) => {
                            return (*ppu).latch;
                        }
                        _ => return 0,
                    }
                }
                //Read/Write
                7 => {
                    let ppu_addr = match self.ppu {
                        Some(ppu) => {
                            let addr = (*ppu).addr;
                            if (*ppu).ctrl&0x4 > 0 {
                                if (*ppu).addr/32 >= 511 {
                                    (*ppu).addr = (*ppu).addr - (32*511)+1;
                                } else {
                                    (*ppu).addr = (*ppu).addr.wrapping_add(32);
                                }
                            } else {
                                (*ppu).addr = (*ppu).addr.wrapping_add(1);
                            }

                            addr
                        }
                        _ => 0
                    };
                    return self.read_ppu_data(ppu_addr);
                }
                _ => {}
            }
        }

        0
    }

    pub fn read_ppu_data(&mut self, addr: u16) -> u8 {
        let val: u8 = match addr {
            0x0000...0x0FFF => {
                self.ppu_patterntable1[addr as usize]
            }
            0x1000...0x1FFF => {
                self.ppu_patterntable2[(addr - 0x1000) as usize]
            }

            0x2000...0x23FF => {
                self.ppu_nametable1[(addr - 0x2000) as usize]
            }

            0x2400...0x27FF => {
                self.ppu_nametable2[(addr - 0x2400) as usize]
            }

            0x2800...0x2BFF => {
                self.ppu_nametable3[(addr - 0x2800) as usize]
            }

            0x2C00...0x2FFF => {
                self.ppu_nametable4[(addr - 0x2C00) as usize]
            }

            0x3F01...0x3F03 => {
                self.ppu_bg_palette0[(addr - 0x3F01) as usize]
            }
            0x3F05...0x3F07 => {
                self.ppu_bg_palette1[(addr - 0x3F05) as usize]
            }
            0x3F09...0x3F0B => {
                self.ppu_bg_palette2[(addr - 0x3F09) as usize]
            }
            0x3F0D...0x3F0F => {
                self.ppu_bg_palette3[(addr - 0x3F0D) as usize]
            }

            0x3F11...0x3F13 => {
                self.ppu_sprite_palette0[(addr - 0x3F11) as usize]
            }
            0x3F15...0x3F17 => {
                self.ppu_sprite_palette1[(addr - 0x3F15) as usize]
            }
            0x3F19...0x3F1B => {
                self.ppu_sprite_palette2[(addr - 0x3F19) as usize]
            }
            0x3F1D...0x3F1F => {
                self.ppu_sprite_palette3[(addr - 0x3F1D) as usize]
            }

            _ => {
                0
            }
        };

        return val;
    }

    pub fn write_ppu(&mut self, addr: usize, val: u8) -> u8 {
        unsafe {
            match addr {
                //Write-Only
                0 => {
                    match self.ppu {
                        Some(ppu) => {
                            (*ppu).ctrl = val;
                            (*ppu).latch = val;
                        }
                        _ => return 0,
                    }
                }
                //Write-Only
                1 => {
                    match self.ppu {
                        Some(ppu) => {
                            (*ppu).mask = val;
                            (*ppu).latch = val;
                        }
                        _ => return 0,
                    }
                }
                //Read-Only
                2 => {
                }
                //Write-Only
                3 => {
                    match self.ppu {
                        Some(ppu) => {
                            (*ppu).oam_addr = val;
                            (*ppu).latch = val;
                        }
                        _ => return 0,
                    }
                }
                //Read/Write
                4 => {
                    match self.ppu {
                        Some(ppu) => {
                            (*ppu).oam[(*ppu).oam_addr as usize] = val;
                            (*ppu).oam_addr = (*ppu).oam_addr.wrapping_add(1);
                            (*ppu).latch = val;
                        }
                        _ => return 0,
                    }
                }
                //Write-Only
                5 => {
                    match self.ppu {
                        Some(ppu) => {
                            if !(*ppu).scroll_write {
                                (*ppu).scroll_x = val;
                            } else {
                                (*ppu).scroll_y = val%240;
                            }

                            (*ppu).scroll_write = !(*ppu).scroll_write;

                            (*ppu).latch = val;
                        }
                        _ => return 0,
                    }
                }
                //Write-Only
                6 => {
                    match self.ppu {
                        Some(ppu) => {
                            if !(*ppu).addr_write {
                                (*ppu).addr = (val as u16) << 8;

                                (*ppu).addr %= 0x4000;
                            } else {
                                (*ppu).addr = ((*ppu).addr&0xFF00) | val as u16;
                                
                            }

                            (*ppu).addr_write = !(*ppu).addr_write;

                            (*ppu).latch = val;
                        }
                        _ => return 0,
                    }
                }
                //Read/Write
                7 => {
                    let ppu_addr = match self.ppu {
                        Some(ppu) => {
                            let addr = (*ppu).addr;
                            if (*ppu).ctrl&0x4 > 0 {
                                if (*ppu).addr/32 >= 511 {
                                    (*ppu).addr = (*ppu).addr - (32*511)+1;
                                } else {
                                    (*ppu).addr = (*ppu).addr.wrapping_add(32);
                                }
                            } else {
                                (*ppu).addr = (*ppu).addr.wrapping_add(1);
                            }

                            addr
                        }
                        _ => 0
                    };
                    if ppu_addr >= 0x3F01 && ppu_addr <= 0x3F0f {
                        //panic!("asd");
                    }
                    self.write_ppu_data(ppu_addr, val);
                }
                _ => {}
            }
        }

        0
    }

    pub fn write_ppu_data(&mut self, addr: u16, val: u8) {
        match addr {
            0x0000...0x0FFF => {
                self.ppu_patterntable1[addr as usize] = val;
            }
            0x1000...0x1FFF => {
                self.ppu_patterntable2[(addr - 0x1000) as usize] = val;
            }

            0x2000...0x23FF => {
                self.ppu_nametable1[(addr - 0x2000) as usize] = val;
            }

            0x2400...0x27FF => {
                self.ppu_nametable2[(addr - 0x2400) as usize] = val;
            }

            0x2800...0x2BFF => {
                self.ppu_nametable3[(addr - 0x2800) as usize] = val;
            }

            0x2C00...0x2FFF => {
                self.ppu_nametable4[(addr - 0x2C00) as usize] = val;
            }

            0x3F01...0x3F03 => {
                self.ppu_bg_palette0[(addr - 0x3F01) as usize] = val;
            }
            0x3F05...0x3F07 => {
                self.ppu_bg_palette1[(addr - 0x3F05) as usize] = val;
            }
            0x3F09...0x3F0B => {
                self.ppu_bg_palette2[(addr - 0x3F09) as usize] = val;
            }
            0x3F0D...0x3F0F => {
                self.ppu_bg_palette3[(addr - 0x3F0D) as usize] = val;
            }

            0x3F11...0x3F13 => {
                self.ppu_sprite_palette0[(addr - 0x3F11) as usize] = val;
            }
            0x3F15...0x3F17 => {
                self.ppu_sprite_palette1[(addr - 0x3F15) as usize] = val;
            }
            0x3F19...0x3F1B => {
                self.ppu_sprite_palette2[(addr - 0x3F19) as usize] = val;
            }
            0x3F1D...0x3F1F => {
                self.ppu_sprite_palette3[(addr - 0x3F1D) as usize] = val;
            }

            _ => {}
        }
    }
}