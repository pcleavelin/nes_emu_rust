mod cpu;
mod mmu;
mod ppu;
mod cart;

use self::cpu::*;
use self::mmu::*;
use self::ppu::*;
use self::cart::*;

pub struct NES {
    cpu: NESCpu,
    mmu: Mmu,
    ppu: NESPpu,

    cart: NESCart,
}

impl NES {
    pub fn new() -> NES {
        NES {
            cpu: NESCpu::new(),
            mmu: Mmu::new(),
            ppu: NESPpu::new(),

            cart: NESCart::none(),
        }
    }
}