#[macro_use] extern crate enum_primitive;
extern crate minifb;
extern crate libusb;

use std::env;

mod nes;
use nes::NES;
use nes::controller_scanner::*;

fn main() {
    let mut nes = NES::new();

    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        let rom_name = &args[1];

        nes.insert_cart(rom_name);
    }

    nes.hard_restart();
    nes.run();

}