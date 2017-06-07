#[macro_use] extern crate enum_primitive;
extern crate minifb;

use std::env;

mod nes;
use nes::NES;

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
