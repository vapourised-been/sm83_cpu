use std::{print, unimplemented};

use crate::cart::CartContext;
mod cpu;
mod instruction;
mod cart;
mod mmu;
pub type Byte = u8;

fn main() {

    let path = "roms/Tetris.gb".to_string();
    
    let temp_cart = CartContext::new(path);
    
    let temp_mmu = mmu::MemUnit::new(&temp_cart);

    let mut cpu_ctx = cpu::CpuContext::new();
    cpu_ctx.begin();

    for i in 0..10{
        let _ = cpu::cpu_step(&mut cpu_ctx, &temp_mmu, &temp_cart);   
        

    }

}

#[cfg(test)]
mod tests {
    use std::{assert_eq, format, println, vec};
    use crate::cart;
    use crate::cart::CartContext;
    
    #[test]
    fn test_headers() {

        let roms = vec!["Donkey Kong Land.gb",
        "Dr. Mario.gb",
        "Game Boy Camera.gb",
        "Kirby's Dream Land 2.gb",
        "Mega Man V.gb",
        "Pokemon - Blue.gb",
        "Super Mario Land.gb",
        "Tetris.gb",
        "Wario Land.gb"
        ];
        let rom_names = vec!["DONKEYKONGLAND95", 
            "DR.MARIO",
        "GAMEBOYCAMERA",
        "KIRBY2",
        "MEGAMAN5",
        "POKEMON BLUE",
        "SUPER MARIOLAND",
        "TETRIS",
        "SUPERMARIOLAND3"];


            let test_cart = cart::CartContext::new(format!("roms/{}",roms[0]).to_string());
            assert_eq!(String::from_utf8(test_cart.header.title.to_vec()).unwrap().trim_end_matches(char::from(0)), rom_names[0]);
            println!("TEST OUTPUT -    {}: OK\n", rom_names[0]);
            let test_cart = cart::CartContext::new(format!("roms/{}",roms[1]).to_string());
            assert_eq!(String::from_utf8(test_cart.header.title.to_vec()).unwrap().trim_end_matches(char::from(0)), rom_names[1]);
            println!("TEST OUTPUT -    {}: OK\n", rom_names[1]);
            let test_cart = cart::CartContext::new(format!("roms/{}",roms[2]).to_string());
            assert_eq!(String::from_utf8(test_cart.header.title.to_vec()).unwrap().trim_end_matches(char::from(0)), rom_names[2]);
            println!("TEST OUTPUT -    {}: OK\n", rom_names[2]);
            let test_cart = cart::CartContext::new(format!("roms/{}",roms[3]).to_string());
            assert_eq!(String::from_utf8(test_cart.header.title.to_vec()).unwrap().trim_end_matches(char::from(0)), rom_names[3]);
            println!("TEST OUTPUT -    {}: OK\n", rom_names[3]);
            let test_cart = cart::CartContext::new(format!("roms/{}",roms[4]).to_string());
            assert_eq!(String::from_utf8(test_cart.header.title.to_vec()).unwrap().trim_end_matches(char::from(0)), rom_names[4]);
            println!("TEST OUTPUT -    {}: OK\n", rom_names[4]);
            let test_cart = cart::CartContext::new(format!("roms/{}",roms[5]).to_string());
            assert_eq!(String::from_utf8(test_cart.header.title.to_vec()).unwrap().trim_end_matches(char::from(0)), rom_names[5]);
            println!("TEST OUTPUT -    {}: OK\n", rom_names[5]);
            let test_cart = cart::CartContext::new(format!("roms/{}",roms[6]).to_string());
            assert_eq!(String::from_utf8(test_cart.header.title.to_vec()).unwrap().trim_end_matches(char::from(0)), rom_names[6]);
            println!("TEST OUTPUT -    {}: OK\n", rom_names[6]);
            let test_cart = cart::CartContext::new(format!("roms/{}",roms[7]).to_string());
            assert_eq!(String::from_utf8(test_cart.header.title.to_vec()).unwrap().trim_end_matches(char::from(0)), rom_names[7]);
            println!("TEST OUTPUT -    {}: OK\n", rom_names[7]);
            let test_cart = cart::CartContext::new(format!("roms/{}",roms[8]).to_string());
            assert_eq!(String::from_utf8(test_cart.header.title.to_vec()).unwrap().trim_end_matches(char::from(0)), rom_names[8]);
            println!("TEST OUTPUT -    {}: OK\n", rom_names[8]);
            // let test_cart = cart::CartContext::new(format!("roms/{}",roms[9]).to_string());
            // assert_eq!(String::from_utf8(test_cart.header.title.to_vec()).unwrap().trim_end_matches(char::from(0)), rom_names[9]);
        
        //assert_eq!(String::from_utf8(test_cart.header.title.to_vec()).unwrap().trim_end_matches(char::from(0)), "DRMARIO");
    }
}
