mod cpu;
mod ppu;
mod apu;
mod io;
mod cart;
mod interconnect;
mod opcode;
mod integer_casting;

use self::cpu::*;
use self::interconnect::*;
use minifb::{WindowOptions, Window, Key, Scale};

pub struct NES {
    cpu: NESCpu,
    interconnect: Interconnect,
    window: Window,
}

impl NES {
    pub fn new() -> NES {
        NES {
            cpu: NESCpu::new(),
            interconnect: Interconnect::new(),
            window: Window::new("NES Emulator", ppu::WIDTH, ppu::HEIGHT, WindowOptions {
                borderless: false,
                title: true,
                resize: false,
                scale: Scale::X2,
            }).expect("Failed to create window"),
        }
    }

    //Power up info obtained from NESDev wiki
    //https://wiki.nesdev.com/w/index.php/CPU_power_up_state 
    pub fn hard_restart(&mut self) {
        self.cpu.set_p_u8(0x34);
        self.cpu.set_s(0xFD);

        self.cpu.set_a(0);
        self.cpu.set_x(0);
        self.cpu.set_y(0);
        //self.cpu.set_pc(0);

        self.interconnect.write_mem(0x4015, 0); //frame irq enabled
        self.interconnect.write_mem(0x4017, 0); //all channels disabled

        for i in 0..0xF {
            self.interconnect.write_mem(0x4000 + i, 0);
        }
    }

    pub fn soft_restart(&mut self) {
        self.cpu.offset_s(0xFD); //decrement by 3

        let p = self.cpu.p();
        self.cpu.set_p(p | 0x04);

        self.interconnect.write_mem(0x4015, 0);
    }

    pub fn insert_cart(&mut self, rom: &str) {
        self.interconnect.insert_cart(rom);
    }

    pub fn run(&mut self) {
        let addr_lo = self.interconnect.read_absolute(0xFFFC) as u16;
        let addr_hi = self.interconnect.read_absolute(0xFFFD) as u16;

        let restart_vector = ((addr_hi << 8) | addr_lo)+0;
        println!("Jumping to restart vector {:04X}", restart_vector);

        self.cpu.set_pc(restart_vector);

        while self.window.is_open() && !self.window.is_key_down(Key::Escape) {
            if self.cpu.do_instruction(&mut self.interconnect) == false {
                break;
                self.cpu.offset_pc(1);
            }

            self.interconnect.update(&mut self.window);

            if self.interconnect.ppu().cycles() == 241 && self.interconnect.ppu().ctrl()&0x80 > 0 {
                self.cpu.do_nmi(&mut self.interconnect);
            }
        }
    }
}