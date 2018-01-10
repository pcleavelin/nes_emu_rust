use minifb::{WindowOptions, Window, Key, Scale};

use super::interconnect::Interconnect;
use super::cart::*;

pub const WIDTH: usize = 256 + 256;// 341;
pub const HEIGHT: usize = 240;// + 240;

pub struct NESPpu {
    ctrl: u8,
    mask: u8,
    status: u8,
    oam_addr: u8,
    oam_data: u8,
    scroll_x: u8,
    scroll_y: u8,
    addr: u16,
    data: u8,
    oam_dma: u8,
    oam: [u8; 0xFF],

    scroll_write: bool,
    addr_write: bool,

    nametable1: [u8; 0x400],
    nametable2: [u8; 0x400],

    bg_palette0: [u8;3],
    bg_palette1: [u8;3],
    bg_palette2: [u8;3],
    bg_palette3: [u8;3],

    palette: [u32;0x40],

    vram: Vec<u32>,

    cycles: i32,
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
            data: 0,
            //odd_frame: false,
            oam_dma: 0,
            oam: [0; 0xFF],

            scroll_write: false,
            addr_write: false,

            nametable1: [0; 0x400],
            nametable2: [0; 0x400],

            bg_palette0: [0u8;3],
            bg_palette1: [0u8;3],
            bg_palette2: [0u8;3],
            bg_palette3: [0u8;3],

            palette: [0x6D6D6D,0x002491,0x0000DA,0x6D48DA,0x91006D,0xB6006D,0xB62400,0x914800,0x6D4800,0x244800,0x006D24,0x009100,0x004848,0x000000,0x000000,0x000000,
                      0xB6B6B6,0x006DDA,0x0048FF,0x9100FF,0xB600FF,0xFF0091,0xFF0000,0xDA6D00,0x916D00,0x249100,0x009100,0x00B66D,0x009191,0x000000,0x000000,0x000000,
                      0xFFFFFF,0x6DB6FF,0x9191FF,0xDA6DFF,0xFF00FF,0xFF6DFF,0xFF9100,0xFFB600,0xDADA00,0x6DDA00,0x00FF00,0x48FFDA,0x00FFFF,0x000000,0x000000,0x000000,
                      0xFFFFFF,0xB6DAFF,0xDAB6FF,0xFFB6FF,0xFF91FF,0xFFB6B6,0xFFDA91,0xFFFF48,0xFFFF6D,0xB6FF48,0x91FF6D,0x48FFDA,0x91DAFF,0x000000,0x000000,0x000000],

            vram: vec![0u32; WIDTH*HEIGHT],

            cycles: 0,
        }
    }

    pub fn ctrl(&self) -> u8 {
        self.ctrl
    }

    pub fn cycles(&self) -> i32 {
        self.cycles
    }

    //Info on what address maps to what
    //https://wiki.nesdev.com/w/index.php/PPU_registers
    pub fn read_ppu(&mut self, addr: usize) -> u8 {
        match addr {
            //Write-Only
            0 => {
                //I'm currently unclear as to which value
                //should be returned here, since both $2005 and $2006
                //are labeled as the latch. So for now we'll just
                //return $2005 (ppu scroll)

                self.scroll_x
            }
            
            //Write-Only
            1 => {
                self.scroll_x
            }
            
            //Read-Only
            2 => {
                //This one is interesting, since only the top 3 bits
                //actually contain the status register
                
                if self.cycles == -1 {
                    let status = self.status&0xE0 | self.scroll_x&0x1F;
                    self.status &= 0x7F;

                    return status;
                }
                
                self.status&0xE0 | self.scroll_x&0x1F
            }
            
            //Write-Only
            3 => {
                self.scroll_x
            }
            
            //Read-Write
            4 => {
                self.oam[self.oam_addr as usize]
            }
            
            //Write-Only
            5 => {
                self.scroll_x
            }
            
            //Write-Only
            6 => {
                self.scroll_x
            }
            
            //Read-Write
            7 => {
                self.data
            }

            //If the interconnect is programmed properly
            //this should never be reached
            _ => {
                println!("invalid addr given to ppu structure, is interconnect wrong?");

                //Why not
                self.scroll_x
            }
        }
    }

    //Info on what address maps to what
    //https://wiki.nesdev.com/w/index.php/PPU_registers
    pub fn write_ppu(&mut self, addr: usize, val: u8){
        match addr {
            //Write-Only
            0 => {
                //I'm currently unclear as to which value
                //should be returned here, since both $2005 and $2006
                //are labeled as the latch. So for now we'll just
                //return $2005 (ppu scroll)

                self.ctrl = val;
            }
            
            //Write-Only
            1 => {
                self.mask = val;
            }
            
            //Read-Only
            2 => {
                
            }
            
            //Write-Only
            3 => {
                self.oam_addr = val;
            }
            
            //Read-Write
            4 => {
                self.oam[self.oam_addr as usize] = val;
                self.oam_addr = self.oam_addr.wrapping_add(1);
            }
            
            //Write-Only
            5 => {
                if !self.scroll_write {
                    self.scroll_x = val;
                } else {
                    self.scroll_y = val%240;
                }

                self.scroll_write = !self.scroll_write;
            }
            
            //Write-Only
            6 => {
                if !self.addr_write {
                    self.addr = (val as u16) << 8;
                } else {
                    self.addr |= val as u16;
                    //panic!("addr: 0x{:04X}", self.addr);
                }

                self.addr_write = !self.addr_write;
            }
            
            //Read-Write
            7 => {
                match self.addr {
                    0x0000...0x1FFF => {

                    }

                    0x2000...0x23FF => {
                        self.nametable1[(self.addr - 0x2000) as usize] = val;
                    }

                    0x2400...0x27FF => {
                        self.nametable2[(self.addr - 0x2400) as usize] = val;
                    }

                    0x2800...0x2BFF => {
                        self.nametable1[(self.addr - 0x2800) as usize] = val;
                    }

                    0x2C00...0x2FFF => {
                        self.nametable2[(self.addr - 0x2C00) as usize] = val;
                    }

                    0x3F00 => {

                    }

                    0x3F01...0x3F03 => {
                        self.bg_palette0[(self.addr - 0x3F01) as usize] = val;
                    }
                    0x3F05...0x3F07 => {
                        self.bg_palette1[(self.addr - 0x3F05) as usize] = val;
                    }
                    0x3F09...0x3F0B => {
                        self.bg_palette2[(self.addr - 0x3F09) as usize] = val;
                    }
                    0x3F0D...0x3F0F => {
                        self.bg_palette3[(self.addr - 0x3F0D) as usize] = val;
                    }

                    _ => {

                    }
                }

                if self.ctrl&0x4 > 0 {
                    if self.addr/32 >= 511 {
                        self.addr = self.addr - (32*511)+1;
                    } else {
                        self.addr = self.addr.wrapping_add(32);
                    }
                } else {
                    self.addr = self.addr.wrapping_add(1);
                }
            }

            //If the interconnect is programmed properly
            //this should never be reached
            _ => {
                println!("invalid addr given to ppu structure, is interconnect wrong?");
            }
        }
    }

    pub fn do_cycle(&mut self, pt0: &[u8], pt1: &[u8], nt0: &[u8], nt1: &[u8], ram: &[u8;0x10000], delta_ram: &mut [bool;0x10000], _cart: &NESCart, window: &mut Window) {

        if self.mask&0x8 >= 0 {
            for y in 0..HEIGHT {
                let mut x = 0;
                loop {
                    if x >= WIDTH/2 {
                        break;
                    }

                    //self.vram[x + (y*WIDTH)] = (((x ^ y) & 0xff) * 1) as u32;

                    let nt_x = ((x as u8).wrapping_sub(self.scroll_x) as usize)%(WIDTH/2);
                    let nt_y = ((y as u8).wrapping_sub(self.scroll_y) as usize)%(HEIGHT);

                    let nt = if self.ctrl&0x3 == 0 || self.ctrl&0x3 == 2 {
                        //nt0
                        &self.nametable1
                    } else {
                        //nt1
                        &self.nametable2
                    };
                    let pt = if self.ctrl&0x10 == 0 {
                        pt0
                    } else {
                        pt1
                    };


                    let attrib = nt[(nt_x/32) + (nt_y/32)*8 + 0x3C0];
                    let palette = match (nt_x/16,nt_y/16) {
                        (0,0) => {
                            attrib&0x3
                        }
                        (1,0) => {
                            (attrib&0xC) >> 2
                        }
                        (0,1) => {
                            (attrib&0x30) >> 4
                        }
                        (1,1) => {
                            (attrib&0xC0) >> 6
                        }

                        _ => {
                            attrib&0x3
                        }
                    };
                    //let topright_palette = (attrib&0xC) >> 2;
                    //let bottomleft_palette = (attrib&0x30) >> 4;
                    //let bottomright_palette = (attrib&0xC0) >> 6;

                    let bg_pal = match palette {
                        0 => {
                            self.bg_palette0
                        }
                        1 => {
                            self.bg_palette1
                        }
                        2 => {
                            self.bg_palette2
                        }
                        3 => {
                            self.bg_palette3
                        }
                        _ => {
                            self.bg_palette0
                        }
                    };

                    let pattern_addr = nt[nt_x/8 + ((nt_y/8)*32)] as usize;
                    
                    let sliver1 = pt[pattern_addr*16 + (nt_y%8)];// scroll_x;
                    let sliver2 = pt[pattern_addr*16 + (nt_y%8) + 8];// >> scroll_x;

                    for i in 0..8 {
                        let color1 = (sliver1 >> (7-i)) & 0x1;
                        let color2 = (sliver2 >> (7-i)) & 0x1;

                        if color1 > 0 && color2 > 0 {
                            self.vram[x+i + (y*WIDTH)] = self.palette[bg_pal[2] as usize];
                        }
                        else if color1 > 0 {
                            self.vram[x+i + (y*WIDTH)] = self.palette[bg_pal[0] as usize];
                        }
                        else if color2 > 0 {
                            self.vram[x+i + (y*WIDTH)] = self.palette[bg_pal[1] as usize];
                        }
                        else {
                            self.vram[x+i + (y*WIDTH)] = 0;
                        }
                    }

                    x += 8;
                }
            }
        }

        if self.cycles%16 == 0 {
            for y in 0..HEIGHT/4 {
                for x in 0..WIDTH/2 {
                    let val = (ram[(x) + (y*WIDTH/2)] as u32) << 8;
                    if delta_ram[x + (y*WIDTH/2)] || (self.vram[x+256 + (y*WIDTH)]&0xFF00 != val) {
                        self.vram[x+256 + (y*WIDTH)] |= 0xFF;
                        delta_ram[x + (y*WIDTH/2)] = false;
                    }

                    self.vram[x+256 + (y*WIDTH)] &= 0x00FF;
                    self.vram[x+256 + (y*WIDTH)] |= val;

                    if self.vram[x+256 + (y*WIDTH)]&0xFF >= 9 {
                        self.vram[x+256 + (y*WIDTH)] -= 9;
                    }
                }
            }
        }

          

        self.cycles += 1;

        match self.cycles {
            -1 => {
                self.status &= 0x7F;
            }

            241...339 => {
                self.status |= 0x80;
            }

            340 => {
                self.cycles = -2;
            }

            _ => {

            }
        }

        if self.cycles == 241 {
            window.update_with_buffer(&self.vram);
        } else {
            window.update();
        }
    }
}