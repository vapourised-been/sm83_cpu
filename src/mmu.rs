use std::array;
use std::print;
use std::str::Bytes;

use crate::Byte;
use crate::cart;
use crate::cart::CartContext;

const ROM_SIZE: usize = 0x8000; // 16KiB // ROM Bank 00 to Bank 01-NN
const VRAM_SIZE: usize = 0x2000; // 8KiB // Video RAM
const WRAM_SIZE: usize = 0x2000; // 8KiB // Work RAM
const ERAM_SIZE: usize = 0x2000; // 8KiB // Mirrors of WRAM 
const OAM_SIZE: usize = 0x00A0; // 160B  // Object Attribute Memory
const HRAM_SIZE: usize = 0x007F; // 127B // High RAM

pub struct MemUnit {
    rom: [Byte; ROM_SIZE],
    vram: [Byte; VRAM_SIZE],
    wram: [Byte; WRAM_SIZE],
    eram: [Byte; ERAM_SIZE],
    oam: [Byte; OAM_SIZE],
    hram: [Byte; HRAM_SIZE],
    interrupt_en: Byte,
}

impl MemUnit {
    pub fn new(cart: &CartContext) -> MemUnit {
        return Self {
            rom: {
                match (cart.rom_data).as_array() {
                    Some(data) => *data,
                    None => {
                        let test;
                        if cart.rom_data.len() > ROM_SIZE {
                            print!("Error with rom data - {:?}\n", cart.rom_data.len());
                            test = cart.rom_data[0..ROM_SIZE].as_array::<ROM_SIZE>().unwrap();
                            *test // data loss, !TODO Memory controller code.. 
                        } else {
                            array::repeat(0x00)
                        }
                    }
                }
            },
            vram: array::repeat(0x00),
            wram: array::repeat(0x00),
            eram: array::repeat(0x00),
            oam: array::repeat(0x00),
            hram: array::repeat(0x00),
            interrupt_en: 0,
        };
    }

    pub fn bus_read(&self, addr: u16) -> Byte {
        match addr {
            0x0000..=0x7FFF => self.rom[addr as usize],
            0x8000..=0x9FFF => self.vram[addr as usize - 0x8000],
            0xA000..=0xBFFF => self.eram[addr as usize - 0xA000],
            0xC000..=0xDFFF => self.wram[addr as usize - 0xC000],
            0xFE00..=0xFE9F => self.oam[addr as usize - 0xFE00],
            0xFF80..=0xFFFE => self.hram[addr as usize - 0xFF80],
            0xFFFF => self.interrupt_en,
            _ => 0xFF,
        }
    }

    pub fn bus_write(&mut self, addr: u16, val: Byte) {
        match addr {
            0x0000..=0x7FFF => self.rom[addr as usize] = val,
            0x8000..=0x9FFF => self.vram[addr as usize - 0x8000] = val,
            0xA000..=0xBFFF => self.eram[addr as usize - 0xA000] = val,
            0xC000..=0xDFFF => self.wram[addr as usize - 0xC000] = val,
            0xFE00..=0xFE9F => self.oam[addr as usize - 0xFE00] = val,
            0xFF80..=0xFFFE => self.hram[addr as usize - 0xFF80] = val,
            0xFFFF => self.interrupt_en = val,
            _ => {}
        }
    }
}
