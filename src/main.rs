use std::{print, unimplemented};

use crate::cart::CartContext;
mod cpu;
mod instruction;
mod cart;
mod mmu;
pub type Byte = u8;

fn main() {

    let path = "roms/Dr. Mario.gb".to_string();
    
    let temp_cart = CartContext::new(path);
    
    let temp_mmu = mmu::MemUnit::new(&temp_cart);

    let mut cpu_ctx = cpu::CpuContext::new();
    cpu_ctx.begin();

    for i in 0..10{
        let _ = cpu::cpu_step(&mut cpu_ctx, &temp_mmu, &temp_cart);   
        

    }

}

