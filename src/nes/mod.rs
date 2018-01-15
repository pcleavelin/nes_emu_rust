mod cpu;
mod ppu;
mod apu;
pub mod controller_scanner;
mod io;
mod cart;
mod mmu;
mod opcode;
mod integer_casting;

use self::cpu::*;
use self::ppu::*;
use self::io::*;
use self::apu::*;
use self::mmu::*;
use self::controller_scanner::*;
use self::io::NESIoButton;

use std::time::{Duration, Instant};
use std::fs::File;
use std::io::Read;
use std::vec::Vec;
use minifb::{WindowOptions, Window, Key, Scale};

pub struct NES<'a> {
    cpu: NESCpu,
    mmu: NESMmu<'a>,
    ppu: NESPpu,
    apu: NESApu,
    io: NESIo,
    window: Window,

    last_frame_instant: Instant,

    current_cycle: u32,
    elapsed_cycles: u32,
}

impl<'a> NES<'a> {
    pub fn new() -> NES<'a> {
        let ppu = NESPpu::new();
        NES {
            cpu: NESCpu::new(),
            ppu: ppu,
            mmu: NESMmu::new(),
            apu: NESApu::new(),
            io: NESIo::new(),
            
            window: Window::new("NES Emulator", ppu::WIDTH, ppu::HEIGHT, WindowOptions {
                borderless: false,
                title: true,
                resize: false,
                scale: Scale::X4,
            }).expect("Failed to create window"),
            last_frame_instant: Instant::now(),

            current_cycle: 0,
            elapsed_cycles: 0,
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

        self.mmu.write_mem(0x4015, 0); //frame irq enabled
        self.mmu.write_mem(0x4017, 0); //all channels disabled

        for i in 0..0xF {
            self.mmu.write_mem(0x4000 + i, 0);
        }

        let addr_lo = self.mmu.read_absolute(0xFFFC) as u16;
        let addr_hi = self.mmu.read_absolute(0xFFFD) as u16;

        let restart_vector = ((addr_hi << 8) | addr_lo)+0;
        println!("Jumping to restart vector {:04X}", restart_vector);

        self.cpu.set_pc(restart_vector);
    }

    pub fn soft_restart(&mut self) {
        self.cpu.offset_s(0xFD); //decrement by 3

        let p = self.cpu.p();
        self.cpu.set_p(p | 0x04);

        self.mmu.write_mem(0x4015, 0);
    }

    pub fn insert_cart(&mut self, rom: &str) -> bool{
        let mut rom_file = match File::open(rom) {
            Ok(file) => file,
            Err(why) => {
                println!("failed to open rom: {}", why);
                return false;
            }
        };

        let mut data: Vec<u8> = Vec::new();

        match rom_file.read_to_end(&mut data) {
            Ok(size) => {
                println!("read rom {:04X} bytes", size);
                self.mmu.fill_rom(data);
                return true;
            }
            Err(why) => {
                println!("error reading rom: {}", why);
                return false;
            }
        }
    }

    pub fn run(&mut self) {

        let mut scanner = ControllerScanner::new();
        let mut adapter = scanner.find_adapter(0x11c0, 0x5500).unwrap().unwrap();
        let mut listener = adapter.listen().unwrap();

        while self.window.is_open() && !self.window.is_key_down(Key::Escape) {
            
            let (success, delta_cycles) = self.cpu.do_instruction(&mut self.mmu);
            if !success {
                break;
            }

            if self.ppu.do_cycle(delta_cycles) {
                self.cpu.do_nmi(&mut self.mmu);
            }
            
            if self.last_frame_instant.elapsed() >= Duration::from_millis(1) {
                match listener.read() {
                    Ok(Some(controller)) => {
                        self.io.set_controller_button(NESIoButton::A, controller.a);
                        self.io.set_controller_button(NESIoButton::B, controller.b);
                        self.io.set_controller_button(NESIoButton::Select, controller.select);
                        self.io.set_controller_button(NESIoButton::Start, controller.start);
                        self.io.set_controller_button(NESIoButton::Up, controller.up);
                        self.io.set_controller_button(NESIoButton::Down, controller.down);
                        self.io.set_controller_button(NESIoButton::Left, controller.left);
                        self.io.set_controller_button(NESIoButton::Right, controller.right);
                    }
                    _ => {}
                }

                self.last_frame_instant = Instant::now();
                self.window.update();
            }
        }
    }
}