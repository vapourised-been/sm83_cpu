use crate::cart::CartContext;


mod cpu;
mod instruction;
mod cart;
pub type byte = u8;

fn main() {

    let path = "roms/Dr. Mario.gb".to_string();
    
    let temp = CartContext::new(path);




}

