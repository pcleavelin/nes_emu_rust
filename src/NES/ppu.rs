use minifb::{WindowOptions, Window, Key, Scale};

pub const WIDTH: usize = 341;
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

    vram: Vec<u32>,
}

impl NESPpu {

    //Power-up values for ppu
    //https://wiki.nesdev.com/w/index.php/PPU_power_up_state
    pub fn new() -> NESPpu {
        NESPpu {
            ctrl: 0,
            mask: 0,
            status: 0b1010_0000,
            oam_addr: 0,
            oam_data: 0,
            scroll: 0,
            addr: 0,
            data: 0,
            //odd_frame: false,
            oam_dma: 0,

            vram: vec![0u32; WIDTH*HEIGHT],
        }
    }

    //Info on what address maps to what
    //https://wiki.nesdev.com/w/index.php/PPU_registers
    pub fn read_ppu(&self, addr: usize) -> u8 {
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

    pub fn do_cycle(&mut self, window: &mut Window) {
        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                self.vram[x + (y*WIDTH)] = (((x ^ y) & 0xff) * 1) as u32;
            }
        }
        
        window.update_with_buffer(&self.vram);
    }
}