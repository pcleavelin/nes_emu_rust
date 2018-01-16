use minifb::{Window, Key, Scale};

use super::mmu::NESMmu;

pub const WIDTH: usize = 256;// + 256;// 341;
pub const HEIGHT: usize = 240;// + 240;

pub struct NESPpu {
    pub ctrl: u8,
    pub mask: u8,
    pub status: u8,
    pub oam_addr: u8,
    pub oam_data: u8,
    pub scroll_x: u8,
    pub scroll_y: u8,
    pub addr: u16,
    pub oam: [u8; 0x100],

    pub latch: u8,

    pub scroll_write: bool,
    pub addr_write: bool,
    vertical_mirror: bool,

    palette: [u32;0x40],

    vram: Vec<u32>,

    cycles: u32,
    in_progress_cycles: u32,
    current_scanline: u32,
    nt_latch: u8,
    attrib_latch: u8,
    tile_low_latch: u8,
    tile_high_latch: u8,

    scanline_sprite_y: [u8;8],
    scanline_sprite_tile: [u8;8],
    scanline_sprite_attrib: [u8;8],
    scanline_sprite_x: [u8;8],
}

impl NESPpu {

    //Power-up values for ppu
    //https://wiki.nesdev.com/w/index.php/PPU_power_up_state
    pub fn new() -> NESPpu {
        NESPpu {
            ctrl: 0,
            mask: 0,
            //status: 0b1010_0000,
            status: 0b1010_0000,
            oam_addr: 0,
            oam_data: 0,
            scroll_x: 0,
            scroll_y: 0,
            addr: 0,
            oam: [25; 0x100],

            latch: 0,

            scroll_write: false,
            addr_write: false,
            vertical_mirror: false,

            palette: [0x6D6D6D,0x002491,0x0000DA,0x6D48DA,0x91006D,0xB6006D,0xB62400,0x914800,0x6D4800,0x244800,0x006D24,0x009100,0x004848,0x000000,0x000000,0x000000,
                      0xB6B6B6,0x006DDA,0x0048FF,0x9100FF,0xB600FF,0xFF0091,0xFF0000,0xDA6D00,0x916D00,0x249100,0x009100,0x00B66D,0x009191,0x000000,0x000000,0x000000,
                      0xFFFFFF,0x6DB6FF,0x9191FF,0xDA6DFF,0xFF00FF,0xFF6DFF,0xFF9100,0xFFB600,0xDADA00,0x6DDA00,0x00FF00,0x48FFDA,0x00FFFF,0x000000,0x000000,0x000000,
                      0xFFFFFF,0xB6DAFF,0xDAB6FF,0xFFB6FF,0xFF91FF,0xFFB6B6,0xFFDA91,0xFFFF48,0xFFFF6D,0xB6FF48,0x91FF6D,0x48FFDA,0x91DAFF,0x000000,0x000000,0x000000],

            vram: vec![0u32; WIDTH*HEIGHT],

            cycles: 0,
            in_progress_cycles: 0,
            current_scanline: 0,

            nt_latch: 0,
            attrib_latch: 0,
            tile_low_latch: 0,
            tile_high_latch: 0,

            scanline_sprite_y: [0u8;8],
            scanline_sprite_tile: [0u8;8],
            scanline_sprite_attrib: [0u8;8],
            scanline_sprite_x: [0u8;8],
        }
    }

    pub fn set_mirroring(&mut self, vertical_mirror: bool) {
        self.vertical_mirror = vertical_mirror;
    }

    pub fn dma(&mut self, data: &[u8]) {
        //self.oam.copy_from_slice(data);
        for i in 0..0x100 {
            self.oam[(self.oam_addr.wrapping_add(i as u8)) as usize] = data[i];
        }
    }

    pub fn blit_bg(&mut self, bg0: u8, bg1: u8, bg2: u8) {
        if self.mask&0x8 > 0 {
            for i in 0..8 {
                let color = (self.tile_low_latch >> (7-i)) & 0x1 | ((self.tile_high_latch >> (7-i)) & 0x1) << 1;

                match color {
                    0 => {
                        self.vram[(self.cycles-1) as usize + i + (self.current_scanline as usize * WIDTH)] = 1;
                    }
                    1 => {
                        self.vram[(self.cycles-1) as usize + i + (self.current_scanline as usize * WIDTH)] = self.palette[bg0 as usize];
                    }
                    2 => {
                        self.vram[(self.cycles-1) as usize + i + (self.current_scanline as usize * WIDTH)] = self.palette[bg1 as usize];
                    }
                    3 => {
                        self.vram[(self.cycles-1) as usize + i + (self.current_scanline as usize * WIDTH)] = self.palette[bg2 as usize];
                    }

                    _ => {}
                }
            }
        }
    }

    pub fn blit_sprite(&mut self, sp0: u8, sp1: u8, sp2: u8, x: u8, flip_horizontal: bool) {
        if self.mask&0x10 > 0 {
            for i in 0..8 {
                if (x as usize)+i >= WIDTH {
                    break;
                }

                let color = if !flip_horizontal {
                    (self.tile_low_latch >> (7-i)) & 0x1 | ((self.tile_high_latch >> (7-i)) & 0x1) << 1
                } else {
                    ((self.tile_low_latch >> i) & 0x1) | (((self.tile_high_latch >> i) & 0x1)) << 1
                };

                if self.cycles == 257 {
                    if self.vram[x as usize + i + (self.current_scanline as usize * WIDTH)] != 1 && color > 0 {
                        self.status |= 0x40;
                        println!("Sprite 0 hit");
                    }
                }

                match color {
                    0 => {
                        //self.vram[x as usize + i + (self.current_scanline as usize * WIDTH)] = 0;
                    }
                    1 => {
                        self.vram[x as usize + i + (self.current_scanline as usize * WIDTH)] = self.palette[sp0 as usize];
                    }
                    2 => {
                        self.vram[x as usize + i + (self.current_scanline as usize * WIDTH)] = self.palette[sp1 as usize];
                    }
                    3 => {
                        self.vram[x as usize + i + (self.current_scanline as usize * WIDTH)] = self.palette[sp2 as usize];
                    }

                    _ => {}
                }
            }
        }
    }

    pub fn update_window(&self, window: &mut Window) {
        let _ = window.update_with_buffer(&self.vram);
    }

    pub fn evaluate_sprites(&mut self) {
        for i in 0..8 {
            self.scanline_sprite_y[i] = 0xFF;
            self.scanline_sprite_tile[i] = 0xFF;
            self.scanline_sprite_attrib[i] = 0xFF;
            self.scanline_sprite_x[i] = 0xFF;
        }

        let mut found_sprites = 0;
        for i in 0..64 {
            let y = self.oam[i*4] as u8;
            let tile = self.oam[i*4 + 1] as u8;
            let attrib = self.oam[i*4 + 2];
            let x = self.oam[i*4 + 3] as u8;

            if self.current_scanline >= y as u32 && self.current_scanline <= y as u32 + 8 {
                self.scanline_sprite_y[found_sprites] = y as u8;
                self.scanline_sprite_tile[found_sprites] = tile;
                self.scanline_sprite_attrib[found_sprites] = attrib;
                self.scanline_sprite_x[found_sprites] = x;

                found_sprites += 1;
            }

            if found_sprites >= 8 {
                break;
            }
        }
    }

    //https://wiki.nesdev.com/w/index.php/PPU_rendering
    pub fn do_cycle(&mut self, cycles: u32, mmu: &mut NESMmu, window: &mut Window) -> bool {
        //self.cycles += cycles;

        //TODO: do cycle exact ppu rendering
        let nt_offset = match self.ctrl&0x3 {
            0 => {
                0x2000
            }

            1 => {
                if self.vertical_mirror {
                    0x2400
                } else {
                    0
                }
            }

            2 => {
                if self.vertical_mirror {
                    0x2000
                } else {
                    0x2800
                }
            }

            3 => {
                if self.vertical_mirror {
                    0x2400
                } else {
                    0x2C00
                }
            }

            _ => {
                0x2000
            }
        };
        let pt_offset: u16 = if self.ctrl&0x10 == 0 {
            0
        } else {
            0x1000
        };

        let sprite_pt_offset: u16 = if self.ctrl&0x8 == 0 {
            0
        } else {
            0x1000
        };

        for _ in 0..cycles*3 {

            if self.current_scanline < 240 {
                match self.cycles {
                    0 => {
                        self.cycles += 1;
                        self.status &= 0x30;
                    }

                    1...256 => {
                        self.in_progress_cycles += 1;
                        match (self.in_progress_cycles-1)%8 {
                            //Nametable byte
                            0...1 => {
                                self.nt_latch = mmu.read_ppu_data(nt_offset + ((self.cycles - 1)/8) as u16 + ((self.current_scanline/8) as u16)*32);
                                //println!("latch 0x{:02X}", self.nt_latch);
                            }
                            //Attribute table byte
                            2...3 => {
                                self.attrib_latch = mmu.read_ppu_data(nt_offset + ((self.cycles - 1)/32) as u16 + ((self.current_scanline/32) as u16)*8 + 0x3C0);
                            }
                            //Tile bitmap low
                            4...5 => {
                                self.tile_low_latch = mmu.read_ppu_data(pt_offset + (self.nt_latch as u16 * 16) + ((self.current_scanline%8) as u16));
                            }
                            //Tile bitmap high (+8 bytes from tile bitmap low)
                            6...7 => {
                                self.tile_high_latch = mmu.read_ppu_data(pt_offset + (self.nt_latch as u32 * 16) as u16 + ((self.current_scanline%8) as u16 + 8));

                                let palette = match ((self.cycles - 1)/16,self.current_scanline/16) {
                                    (0,0) => {
                                        self.attrib_latch&0x3
                                    }
                                    (1,0) => {
                                        (self.attrib_latch >> 2)&3
                                    }
                                    (0,1) => {
                                        (self.attrib_latch >> 4)&3
                                    }
                                    (1,1) => {
                                        (self.attrib_latch >> 6)&3
                                    }

                                    _ => {
                                        self.attrib_latch&0x3
                                    }
                                };

                                let (bg0,bg1,bg2) = match palette {
                                    0 => {
                                        (mmu.read_ppu_data(0x3F01), mmu.read_ppu_data(0x3F02), mmu.read_ppu_data(0x3F03))
                                    }
                                    1 => {
                                        (mmu.read_ppu_data(0x3F05), mmu.read_ppu_data(0x3F06), mmu.read_ppu_data(0x3F07))
                                    }
                                    2 => {
                                        (mmu.read_ppu_data(0x3F09), mmu.read_ppu_data(0x3F0A), mmu.read_ppu_data(0x3F0B))
                                    }
                                    3 => {
                                        (mmu.read_ppu_data(0x3F0D), mmu.read_ppu_data(0x3F0E), mmu.read_ppu_data(0x3F0F))
                                    }
                                    _ => {
                                        (mmu.read_ppu_data(0x3F01), mmu.read_ppu_data(0x3F02), mmu.read_ppu_data(0x3F03))
                                    }
                                };

                                self.blit_bg(bg0, bg1, bg2);

                                self.in_progress_cycles = 0;
                                self.cycles += 8;

                                //println!("Done 4 byte fetch, Cycle: {}", self.cycles);
                            }
                            _ => {}
                        }
                    }

                    257...320 => {
                        if self.cycles == 257 {
                            self.evaluate_sprites();
                        }

                        self.in_progress_cycles += 1;
                        match (self.in_progress_cycles-1)%8 {
                            //Nametable byte
                            0...1 => {
                                self.nt_latch = mmu.read_ppu_data(nt_offset + ((self.cycles - 1)/8) as u16 + ((self.current_scanline/8) as u16)*32);
                                //println!("latch 0x{:02X}", self.nt_latch);
                            }
                            //Attribute table byte
                            2...3 => {
                                self.nt_latch = mmu.read_ppu_data(nt_offset + ((self.cycles - 1)/8) as u16 + ((self.current_scanline/8) as u16)*32);
                            }
                            //Tile bitmap low
                            4...5 => {
                                let tile = self.scanline_sprite_tile[(self.cycles as usize - 257)/8] as u16;
                                let mut y = self.scanline_sprite_y[(self.cycles as usize - 257)/8] as i32 + 1;

                                
                                let attrib = self.scanline_sprite_attrib[(self.cycles as usize - 257)/8];

                                y = if self.current_scanline as i32 >= y {
                                    self.current_scanline as i32 - y
                                } else {
                                    y - self.current_scanline as i32
                                };
                                if attrib&0x80 > 0 {
                                    //y = 7 - (y%8);
                                } else {
                                    y %= 8;
                                }

                                self.tile_low_latch = mmu.read_ppu_data(sprite_pt_offset + tile*16 + y as u16 % 8);
                            }
                            //Tile bitmap high (+8 bytes from tile bitmap low)
                            6...7 => {
                                let tile = self.scanline_sprite_tile[(self.cycles as usize - 257)/8] as u16;
                                let x = self.scanline_sprite_x[(self.cycles as usize - 257)/8];
                                let mut y = self.scanline_sprite_y[(self.cycles as usize - 257)/8] as i32 + 1;
                                let attrib = self.scanline_sprite_attrib[(self.cycles as usize - 257)/8];

                                y = if self.current_scanline as i32 >= y {
                                    self.current_scanline as i32 - y
                                } else {
                                    y - self.current_scanline as i32
                                };
                                if attrib&0x80 > 0 {
                                    //y = 7 - (y%8);
                                } else {
                                    y %= 8;
                                }

                                self.tile_high_latch = mmu.read_ppu_data(sprite_pt_offset + tile*16 + y as u16 + 8);

                                let (sp0,sp1,sp2) = match attrib&0x3 {
                                    0 => {
                                        (mmu.read_ppu_data(0x3F11), mmu.read_ppu_data(0x3F12), mmu.read_ppu_data(0x3F13))
                                    }
                                    1 => {
                                        (mmu.read_ppu_data(0x3F15), mmu.read_ppu_data(0x3F16), mmu.read_ppu_data(0x3F17))
                                    }
                                    2 => {
                                        (mmu.read_ppu_data(0x3F19), mmu.read_ppu_data(0x3F1A), mmu.read_ppu_data(0x3F1B))
                                    }
                                    3 => {
                                        (mmu.read_ppu_data(0x3F1D), mmu.read_ppu_data(0x3F1E), mmu.read_ppu_data(0x3F1F))
                                    }
                                    _ => {
                                        (mmu.read_ppu_data(0x3F11), mmu.read_ppu_data(0x3F12), mmu.read_ppu_data(0x3F13))
                                    }
                                };

                                self.blit_sprite(sp0, sp1, sp2, x, (attrib&0x40)>0);

                                self.in_progress_cycles = 0;
                                self.cycles += 8;
                            }
                            _ => {}
                        }
                    }

                    321...336 => {
                        self.cycles += 1;
                        //println!("Wasting, Cycle: {}", self.cycles);
                    }

                    337...339 => {
                        self.cycles += 1;
                        //println!("Wasting, Cycle: {}", self.cycles);
                    }

                    340 => {
                        self.cycles = 0;
                        self.current_scanline += 1;
                        //println!("Finished Scanline {}, Cycle: {}", self.current_scanline, self.cycles);
                    }

                    _ => {
                        self.cycles += 1;
                    }
                }
            } else {
                self.cycles += 1;

                if self.cycles == 340 {
                    self.cycles = 0;
                    self.current_scanline += 1;
                }

                //Trigger NMI
                if self.current_scanline == 241 && self.cycles == 0 {
                    //println!("NMI {}", self.cycles);
                    self.status |= 0x80;
                    self.oam_addr = 0;
                    return true;
                }

                if self.current_scanline == 260 {
                    self.current_scanline = 0;
                }
            }
        }

        

        return false;        
    }
}