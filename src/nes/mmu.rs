use super::ppu::NESPpu;

pub struct NESMmu<'a> {
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

    ppu: Option<&'a mut NESPpu>,

    /** Cartridge Memory */
    cart_rom: Vec<u8>,

    /** CPU Memory */
    cpu_ram: [u8;0x800],
}

impl<'a> NESMmu<'a> {
    pub fn new() -> NESMmu<'a> {
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

            /** Cartridge Memory */
            cart_rom: Vec::new(),

            /** CPU Memory */
            cpu_ram: [0u8;0x800],
        }
    }

    pub fn fill_rom(&mut self, rom_data: Vec<u8>) {
        self.cart_rom = rom_data;
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
        self.read_mem((addr + x as u16))
    }

    pub fn read_absolute_indexed_y(&mut self, addr: u16, y: u8) -> u8 {
        self.read_mem((addr + y as u16))
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
        let addr = addr as usize;
        match addr {
            
            //CPU
            0x0000...0x1FFF => {
                self.cpu_ram[addr % 0x800]
            }

            //PPU
            0x2000...0x3FFF => {
                let ppu_addr = addr % 8;

                return self.read_ppu(ppu_addr);
            }

            //IO
            0x4016...0x4017 => {
                //self.io.read(fixed_addr)
                0
            }

            0x6000...0x7FFF => {
                //self.cart.read_ram(fixed_addr - 0x6000)
                0
            }

            0x8000...0xBFFF => {
                self.cart_rom[(addr - 0x8000) + 0x10]
            }

            0xC000...0xFFFF => {
                self.cart_rom[(addr - 0xC000) + 0x4000 + 0x10]
            }


            _ => {
                0
            }
        }
    }

    pub fn write_mem(&mut self, addr: u16, val: u8) {
        
    }

    pub fn read_ppu(&mut self, addr: usize) -> u8 {
        match addr {
            //Write-Only
            0 => {
                match self.ppu {
                    Some(ref mut ppu) => {
                        return ppu.latch;
                    }
                    _ => return 0,
                }
            }
            //Write-Only
            1 => {
                match self.ppu {
                    Some(ref mut ppu) => {
                        return ppu.latch;
                    }
                    _ => return 0,
                }
            }
            //Read-Only
            2 => {
                match self.ppu {
                    Some(ref mut ppu) => {
                        return ppu.status&0xE0 | ppu.latch&0x1F;
                    }
                    _ => return 0,
                }
            }
            //Write-Only
            3 => {
                match self.ppu {
                    Some(ref mut ppu) => {
                        return ppu.latch;
                    }
                    _ => return 0,
                }
            }
            //Read/Write
            4 => {
                match self.ppu {
                    Some(ref mut ppu) => {
                        return ppu.oam[ppu.oam_addr as usize];
                    }
                    _ => return 0,
                }
            }
            //Write-Only
            5 => {
                match self.ppu {
                    Some(ref mut ppu) => {
                        return ppu.latch;
                    }
                    _ => return 0,
                }
            }
            //Write-Only
            6 => {
                match self.ppu {
                    Some(ref mut ppu) => {
                        return ppu.latch;
                    }
                    _ => return 0,
                }
            }
            7 => {
                let addr = match self.ppu {
                    Some(ref mut ppu) => {
                        let addr = ppu.addr;
                        if ppu.ctrl&0x4 > 0 {
                            if ppu.addr/32 >= 511 {
                                ppu.addr = ppu.addr - (32*511)+1;
                            } else {
                                ppu.addr = ppu.addr.wrapping_add(32);
                            }
                        } else {
                            ppu.addr = ppu.addr.wrapping_add(1);
                        }

                        addr
                    }
                    _ => 0
                };
                return self.read_ppu_data(addr);
            }
            _ => {}
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
}