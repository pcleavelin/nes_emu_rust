use minifb::{WindowOptions, Window, Key, Scale};
use super::interconnect::Interconnect;

pub const WIDTH: usize = 256;// 341;
pub const HEIGHT: usize = 240;

pub struct NESPpu {
    ctrl: u8,
    mask: u8,
    status: u8,
    oam_addr: u8,
    oam_data: u8,
    scroll: u8,
    addr: u8,
    data: u8,
    oam_dma: u8,

    oam: [u8; 0xFF],

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
            status: 0b0010_0000,
            oam_addr: 0,
            oam_data: 0,
            scroll: 0,
            addr: 0,
            data: 0,
            //odd_frame: false,
            oam_dma: 0,

            oam: [0; 0xFF],

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

                self.scroll
            }
            
            //Write-Only
            1 => {
                self.scroll
            }
            
            //Read-Only
            2 => {
                //This one is interesting, since only the top 3 bits
                //actually contain the status register
                
                if self.cycles == -1 {
                    self.status &= 0x7F;
                }
                
                self.status&0xE0 | self.scroll&0x1F
            }
            
            //Write-Only
            3 => {
                self.scroll
            }
            
            //Read-Write
            4 => {
                self.oam_data
            }
            
            //Write-Only
            5 => {
                self.scroll
            }
            
            //Write-Only
            6 => {
                self.scroll
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
                self.scroll
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
                
            }
            
            //Write-Only
            6 => {
                
            }
            
            //Read-Write
            7 => {
                
            }

            //If the interconnect is programmed properly
            //this should never be reached
            _ => {
                println!("invalid addr given to ppu structure, is interconnect wrong?");
            }
        }
    }

    pub fn do_cycle(&mut self, pt0: &[u8], pt1: &[u8], nt0: &[u8], nt1: &[u8], window: &mut Window) {
        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                //self.vram[x + (y*WIDTH)] = (((x ^ y) & 0xff) * 1) as u32;

                let nt = if self.ctrl&0x3 == 0 || self.ctrl&0x3 == 1 {
                    nt0
                } else {
                    nt1
                };
                let pt = if self.ctrl&0x8 == 1 {
                    pt0
                } else {
                    pt1
                };

                let pattern_addr = nt[x/32 + ((y/30)*32)] as usize;
                
                let sliver1 = pt[pattern_addr*16 + (y%8)];
                let sliver2 = pt[pattern_addr*16 + (y%8) + 8];

                let color1 = (sliver1 >> (7-(x%8))) & 0x1;
                let color2 = (sliver2 >> (7-(x%8))) & 0x1;

                if color1 > 0 && color2 > 0 {
                    self.vram[x + (y*WIDTH)] = 0xFF0000;
                }
                else if color1 > 0 {
                    self.vram[x + (y*WIDTH)] = 0x00FF00;
                }
                else if color2 > 0 {
                    self.vram[x + (y*WIDTH)] = 0x0000FF;
                }
                else {
                    self.vram[x + (y*WIDTH)] = 0;
                }

                //self.vram[x + (y*WIDTH)] = ((colorr as u32) << 16) 
                //    | ((colorg as u32) << 16)
                //    | (colorb as u32);
            }
        }
        
        self.cycles += 1;

        match self.cycles {
            -1 => {
                self.status &= 0x7F;
            }

            241 => {
                self.status |= 0x80;
            }

            340 => {
                self.cycles = -1;
            }

            _ => {

            }
        }

        window.update_with_buffer(&self.vram);
    }
}