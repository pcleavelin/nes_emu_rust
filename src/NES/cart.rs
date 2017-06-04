use std::fs::File;
use std::io::Read;
use std::vec::Vec;

pub struct NESCart {
    data: Vec<u8>
}

impl NESCart {
    pub fn new(cartridge: &str) -> NESCart {
        let mut rom_file = match File::open(cartridge) {
            Ok(file) => file,
            Err(why) => panic!("failed to open rom: {}", why),
        };

        let mut data: Vec<u8> = Vec::new();

        match rom_file.read_to_end(&mut data) {
            Ok(size) => println!("read rom {} bytes", size),
            Err(why) => panic!("error reading rom: {}", why),
        }

        NESCart {
            data: data
        }
    }

    pub fn none() -> NESCart{
        NESCart {
            data: Vec::new()
        }
    }

    pub fn read(&self, addr: usize) -> u8 {
        if addr > self.data.len() {
            println!("error reading from cart: reached end of rom!");
            return 0;
        }
        self.data[addr]
    }

    //Header info obtained from NESDev wiki
    //https://wiki.nesdev.com/w/index.php/INES 
    pub fn print_header(&self) {
        println!("ROM Size:");
        println!("Size of PRG ROM * 16KB: 0x{:0X}", self.data[4] as u16 * 16000);
        println!("Size of CHR ROM * 8KB:  0x{:0X}", self.data[5] as u16 * 8000);

        println!("\nFlags:");
        print!("Mirroring: ");
        if (self.data[6] & 0x1) > 0 {
            println!("Vertical (Horizontal Arrangment)");
        } else {
            println!("Horizontal (Vertical Arrangment)");
        }

        if (self.data[6] & 0x2) > 0 {
            println!("Has persistent memory ($6000 - $7FFF)");
        } else {
            println!("No persistent memory");
        }

        if (self.data[6] & 0x4) > 0 {
            println!("Has trainer ($7000 - $71FF)");
        } else {
            println!("No trainer");
        }

        if (self.data[6] & 0x8) > 0 {
            println!("Ignore mirror bit (bit 0), provide four-screen vram");
        }

        println!("\nMapper: {:03}", ((self.data[6] & 0b1111) >> 4) & (self.data[6] & 0b1111));
    }
}