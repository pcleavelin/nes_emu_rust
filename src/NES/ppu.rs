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
        }
    }

    //Info on what address maps to what
    //https://wiki.nesdev.com/w/index.php/PPU_registers
    pub fn read_ppu(&self, addr: usize) -> u8 {
        match addr {
            //Write-Only
            0x2000 => {
                //I'm currently unclear as to which value
                //should be returned here, since both $2005 and $2006
                //are labeled as the latch. So for now we'll just
                //return $2005 (ppu scroll)

                self.scroll
            }
            
            //Write-Only
            0x2001 => {
                self.scroll
            }
            
            //Read-Only
            0x2002 => {
                //This one is interesting, since only the top 3 bits
                //actually contain the status register

                self.status&0xE0 | self.scroll&0x1F
            }
            
            //Write-Only
            0x2003 => {
                self.scroll
            }
            
            //Read-Write
            0x2004 => {
                self.oam_data
            }
            
            //Write-Only
            0x2005 => {
                self.scroll
            }
            
            //Write-Only
            0x2006 => {
                self.scroll
            }
            
            //Read-Write
            0x2007 => {
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
}