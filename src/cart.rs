use core::panic;
use std::io::{Error, SeekFrom, prelude::*};
use std::{io, result};
use std::{print, todo, u16};
use std::fs::File;
use std::convert::TryInto;
use byteorder::{BigEndian, LittleEndian, ReadBytesExt};
use crate::byte;


const B_9H: u16 = u16::from_be_bytes(*b"9H");
const B_A4: u16 = u16::from_be_bytes(*b"A4");
const B_BL: u16 = u16::from_be_bytes(*b"BL");
const B_DK: u16 = u16::from_be_bytes(*b"DK");
        
#[derive(Clone)]
struct RomHeader{
    
    entry_point: [byte; 4],
    logo: [byte; 0x30],
    title: [byte; 16],
    license_flag: u16, // Changed to String from u16 bc it needs to be displayed
                        // but idk if its gonna be important later.
    sgb_flag: byte,
    cart_type: byte,
    rom_size: byte,
    ram_size: byte,
    dest_code: byte,
    old_license: byte,
    version: byte,
    checksum: byte,
    glo_checksum: u16,    
}

pub struct CartContext {
    filename: String,
    rom_size: byte,
    rom_data: Vec<byte>,
    header: RomHeader,
}

impl CartContext {
    // TODO
    // fn new(filename, rom_size,...) -> Self {

    // }

    pub fn new(cart:String) -> CartContext {
        let ctx: CartContext;
        let mut byte_buffer = [0;1];
        let initfilename = cart.clone();
        let mut initrom_data: Vec<byte> = vec![];
        let mut game_file = match File::open(cart.clone()){
            Err(why) => panic!("couldnt open {}: {}", cart, why),
            Ok(game_file) => game_file,
        };

        print!("Game File Opened: {}", cart);
        let size = match game_file.seek(SeekFrom::End(0)) {
            Err(why) => panic!("EOF error: {}", why),
            Ok(size) => size,
        };
        
        initrom_data.resize(size as usize,0 );
        let _ = game_file.rewind();
        let _ = game_file.read_exact(&mut initrom_data);
        
        // self.filename = self.rom_data[0x134..0x0143].try_into().unwrap();

        let initheader = match RomHeader::from_be_bytes(&initrom_data){
            Err(why) => panic!("Failed to fill romheader: {}", why),
            Ok(header) => header,
        };

        Self {
            filename: initfilename,
            rom_size: initrom_data.capacity() as u8,
            rom_data: initrom_data,
            header: initheader,
        }
        // return false // TODO
    }

    
}


impl RomHeader {

    fn from_be_bytes(data: &Vec<byte>) -> io::Result<RomHeader>{
       
        let header: RomHeader = RomHeader { 
            entry_point: (data[0x0100..0x0104].try_into().unwrap()), 
            logo: (data[0x0104..0x0134].try_into().unwrap()), 
            title: (data[0x0134..0x0144].try_into().unwrap()), 
            license_flag: (
                match RomHeader::license_lookup(u16::from_be_bytes(data[0x0144..0x0146].try_into().unwrap())) {
                    Ok(_) => u16::from_be_bytes(data[0x0144..0x0146].try_into().unwrap()),
                    Err(why) => panic!("Error in license lookup: {}", why),
                }
            ), 
            sgb_flag: (data[0x0146]), 
            cart_type: (data[0x0147]), 
            rom_size: (data[0x0148]),
            ram_size: (data[0x0149]), 
            dest_code: (data[0x014A]), 
            old_license: (
                data[0x014B]), 
            version: (
                data[0x014C]), 
            checksum: (
                match RomHeader::header_checksum(data[0x014D], data) {
                    Ok(checksum ) => checksum,
                    Err(why) => panic!("Error: {}", why),
                }
            ), 
            glo_checksum: (u16::from_be_bytes(data[0x014E..0x0150].try_into().unwrap())) 
        };
        let title = header.title;
        let lic = RomHeader::license_lookup(header.license_flag).unwrap();
        print!("\ntitle       : {:?}\n", String::from_utf8(title.to_vec()).unwrap().trim_end_matches(char::from(0)));
        let _ = RomHeader::cart_type(header.cart_type);
        let _ = RomHeader::rom_size(header.rom_size);
        // let _ = RomHeader::ram_size(header.ram_size);
        // Change cart and romsize to match lic lookup
        print!("Ram Size    : {:X} ({})\n", header.ram_size, RomHeader::ram_size(header.ram_size));
        print!("LIC Code    : {:X} ({})\n", header.license_flag, lic);
        print!("Checksum    : {:X}\n", header.checksum);
        
        return Ok(header);
    }
    fn license_lookup(license_flag: u16) -> io::Result<String>{
        // TODO

        let output:String;
        match license_flag {
            00  => output = "Nintendo Research & Development 1".to_string(),
            08 => output = "Capcom".to_string(),
            13 => output = "EA (Electronic Arts)".to_string(),
            18 => output = "Hudson Soft".to_string(),
            19 => output = "B-AI".to_string(),
            20 => output = "KSS".to_string(),
            22 => output = "Planning Office WADA".to_string(),
            24 => output = "PCM Complete".to_string(),
            25 => output = "San-X".to_string(),
            28 => output = "Kemco".to_string(),
            29 => output = "SETA Corporation".to_string(),
            30 => output = "Viacom".to_string(),
            31 => output = "Nintendo".to_string(),
            32 => output = "Bandai".to_string(),
            33 => output = "Ocean Software/Acclaim Entertainment".to_string(),
            34 => output = "Konami".to_string(),
            35 => output = "HectorSoft".to_string(),
            37 => output = "Taito".to_string(),
            38 => output = "Hudson Soft".to_string(),
            39 => output = "Banpresto".to_string(),
            41 => output = "Ubi Soft1".to_string(),
            42 => output = "Atlus".to_string(),
            44 => output = "Malibu Interactive".to_string(),
            46 => output = "Angel".to_string(),
            47 => output = "Bullet-Proof Software2".to_string(),
            49 => output = "Irem".to_string(),
            50 => output = "Absolute".to_string(),
            51 => output = "Acclaim Entertainment".to_string(),
            52 => output = "Activision".to_string(),
            53 => output = "Sammy USA Corporation".to_string(),
            54 => output = "Konami".to_string(),
            55 => output = "Hi Tech Expressions".to_string(),
            56 => output = "LJN".to_string(),
            57 => output = "Matchbox".to_string(),
            58 => output = "Mattel".to_string(),
            59 => output = "Milton Bradley Company".to_string(),
            60 => output = "Titus Interactive".to_string(),
            61 => output = "Virgin Games Ltd.3".to_string(),
            64 => output = "Lucasfilm Games4".to_string(),
            67 => output = "Ocean Software".to_string(),
            69 => output = "EA (Electronic Arts)".to_string(),
            70 => output = "Infogrames5".to_string(),
            71 => output = "Interplay Entertainment".to_string(),
            72 => output = "Broderbund".to_string(),
            73 => output = "Sculptured Software6".to_string(),
            75 => output = "The Sales Curve Limited7".to_string(),
            78 => output = "THQ".to_string(),
            79 => output = "Accolade8".to_string(),
            80 => output = "Misawa Entertainment".to_string(),
            83 => output = "LOZC G.".to_string(),
            86 => output = "Tokuma Shoten".to_string(),
            87 => output = "Tsukuda Original".to_string(),
            91 => output = "Chunsoft Co.9".to_string(),
            92 => output = "Video System".to_string(),
            93 => output = "Ocean Software/Acclaim Entertainment".to_string(),
            95 => output = "Varie".to_string(),
            96 => output = "Yonezawa10/S’Pal".to_string(),
            97 => output = "Kaneko".to_string(),
            99 => output = "Pack-In-Video".to_string(),
            B_9H => output = "Bottom Up".to_string(),
            B_A4 => output = "Ko4nami (Yu-Gi-Oh!)".to_string(),
            B_BL => output = "MTO".to_string(),
            B_DK => output = "Kodansha".to_string(),
            _ => output = "error_license".to_string(),
        }

        // let result = output.into_bytes();
        return Ok(output);
        
    }

    fn cart_type (cart_type: byte) -> String {

        let output: String;
        
        match cart_type {
            0x00 => output = "ROM ONLY".to_string(),
            0x01 => output = "MBC1".to_string(),
            0x02 => output = "MBC1+RAM".to_string(),
            0x03 => output = "MBC1+RAM+BATTERY".to_string(),
            0x05 => output = "MBC2".to_string(),
            0x06 => output = "MBC2+BATTERY".to_string(),
            0x08 => output = "ROM+RAM 11".to_string(),
            0x09 => output = "ROM+RAM+BATTERY 11".to_string(),
            0x0B => output = "MMM01".to_string(),
            0x0C => output = "MMM01+RAM".to_string(),
            0x0D => output = "MMM01+RAM+BATTERY".to_string(),
            0x0F => output = "MBC3+TIMER+BATTERY".to_string(),
            0x10 => output = "MBC3+TIMER+RAM+BATTERY 12".to_string(),
            0x11 => output = "MBC3".to_string(),
            0x12 => output = "MBC3+RAM 12".to_string(),
            0x13 => output = "MBC3+RAM+BATTERY 12".to_string(),
            0x19 => output = "MBC5".to_string(),
            0x1A => output = "MBC5+RAM".to_string(),
            0x1B => output = "MBC5+RAM+BATTERY".to_string(),
            0x1C => output = "MBC5+RUMBLE".to_string(),
            0x1D => output = "MBC5+RUMBLE+RAM".to_string(),
            0x1E => output = "MBC5+RUMBLE+RAM+BATTERY".to_string(),
            0x20 => output = "MBC6".to_string(),
            0x22 => output = "MBC7+SENSOR+RUMBLE+RAM+BATTERY".to_string(),
            0xFC => output = "POCKET CAMERA".to_string(),
            0xFD => output = "BANDAI TAMA5".to_string(),
            0xFE => output = "HuC3".to_string(),
            0xFF => output = "HuC1+RAM+BATTERY".to_string(),
            
            _ => output = "error_cartType".to_string(),
        }

        print!("Cart Type   : {:X} ({})\n", cart_type, output);
        return output;

    }
    fn rom_size (rom_size: byte) -> String {
 
        let output: String;
        // let mut rom_banks = 0;
        match rom_size {
            0x00 => output = "32 KiB".to_string(),
            0x01 => output = "64 KiB".to_string(),
            0x02 => output = "128 KiB".to_string(),
            0x03 => output = "256 KiB".to_string(),
            0x04 => output = "512 KiB".to_string(),
            0x05 => output = "1 MiB".to_string(),
            0x06 => output = "2 MiB".to_string(),
            0x07 => output = "4 MiB".to_string(),
            0x08 => output = "8 MiB".to_string(),
            0x52 => output = "1.1 MiB".to_string(),
            0x53 => output = "1.2 MiB".to_string(),
            0x54 => output = "1.5 MiB".to_string(),
            _ => output = "error_romsize".to_string(),
        }

        print!("Rom Size    : {}\n", output);
        return output;
    }
    fn ram_size (rom_size: byte) -> String {
 
        let output: String;
        // let mut rom_banks = 0;
        match rom_size {
            0x00 => output = "0".to_string(),
            0x01 => output = "Unused".to_string(),
            0x02 => output = "8 KiB".to_string(),
            0x03 => output = "32 KiB".to_string(),
            0x04 => output = "128 KiB".to_string(),
            0x05 => output = "64 KiB".to_string(),
            _ => output = "error_ramsize".to_string(),
        }

        return output;
    }


    fn header_checksum (check: u8, rom_data:&Vec<u8>) -> io::Result<byte> {
        let mut checksum: i16 = 0;

        for address in 0x0134..0x014D {
            checksum = checksum - (rom_data[address] as i16) - (1 as i16);
        }

        if check == checksum as u8 {
            return Ok(check)

        } else {
            return Err(Error::new(io::ErrorKind::InvalidData, "Checksum"))
 
        }
    }
    

}