
mod cpu;
pub type byte = u8;

const ROM_SIZE:usize = 0x8000; // 16KiB // ROM Bank 00 to Bank 01-NN
const VRAM_SIZE:usize = 0x2000; // 8KiB // Video RAM
const ERAM_SIZE:usize = 0x2000; // 8KiB // Mirrors of WRAM 
const WRAM_SIZE:usize = 0x2000; // 8KiB // Work RAM
const OAM_SIZE:usize = 0x00A0; // 160B  // Object Attribute Memory
const HRAM_SIZE:usize = 0x007F; // 127B // High RAM

pub struct mmu {
    
    rom: [byte; ROM_SIZE],
    vram: [byte; VRAM_SIZE],
    eram: [byte; ERAM_SIZE],
    wram: [byte; WRAM_SIZE],
    oam: [byte; OAM_SIZE],
    hram: [byte; HRAM_SIZE],
    interrupt_en: byte, 
}


impl mmu {
    pub fn read(&self, addr: u16) -> byte {
        match addr {
            0x0000..=0x7FFF => self.rom[addr as usize],
            0x8000..=0x9FFF => self.vram[addr as usize - 0x8000],
            0xA000..=0xBFFF => self.eram[addr as usize - 0xA000],
            0xC000..=0xDFFF => self.wram[addr as usize - 0xC000],
            0xFE00..=0xFE9F => self.oam[addr as usize - 0xFE00],
            0xFF80..=0xFFFE => self.hram[addr as usize - 0xFF80],
            0xFFFF          => self.interrupt_en,
            _               => 0xFF,
        }

    }

    pub fn write(&mut self, addr: u16, val: byte) {
        match addr {
            0x0000..=0x7FFF => self.rom[addr as usize] = val,
            0x8000..=0x9FFF => self.vram[addr as usize - 0x8000] = val,
            0xA000..=0xBFFF => self.eram[addr as usize - 0xA000] = val,
            0xC000..=0xDFFF => self.wram[addr as usize - 0xC000] = val,
            0xFE00..=0xFE9F => self.oam[addr as usize - 0xFE00] = val,
            0xFF80..=0xFFFE => self.hram[addr as usize - 0xFF80] = val,
            0xFFFF          => self.interrupt_en = val,
            _               => {},
        }

    }

}
fn main() {






}

