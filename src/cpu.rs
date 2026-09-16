use core::panic;
use std::default;
use std::print;
use std::str::Bytes;
use std::unimplemented;

use crate::Byte;
use crate::cart;
use crate::cpu;
use crate::instruction;
use crate::instruction::Instruction;
use crate::mmu::MemUnit;

// use crate::mmu::bus_read;
use crate::instruction::opcode_instruction;
use crate::instruction::AddressMode;
use crate::instruction::Register;

pub struct CpuReg {
    b: Byte,
    c: Byte,
    d: Byte,
    e: Byte,
    h: Byte,
    l: Byte,
    a: Byte,
    f: Byte,
    sp: u16,
    pc: u16,            //pc: WordRegister,
}

impl CpuReg {

    pub fn cpu_regs_read(&self, reg: Option<&instruction::Register>) -> Option<WordRegister>{
        match reg {
            Some(Register::A) => return Some(WordRegister::low(self.a) ),
            Some(Register::F) => return Some(WordRegister::low(self.f) ),
            Some(Register::B) => return Some(WordRegister::low(self.b) ),
            Some(Register::C) => return Some(WordRegister::low(self.c) ),
            Some(Register::D) => return Some(WordRegister::low(self.d) ),
            Some(Register::E) => return Some(WordRegister::low(self.e) ),
            Some(Register::H) => return Some(WordRegister::low(self.h) ),
            Some(Register::L) => return Some(WordRegister::low(self.l) ),
            
            Some(Register::AF) => return Some(WordRegister::high((self.a as u16).swap_bytes(),)),
            Some(Register::BC) => return Some(WordRegister::high((self.b as u16).swap_bytes(),)),
            Some(Register::DE) => return Some(WordRegister::high((self.d as u16).swap_bytes(),)),
            Some(Register::HL) => return Some(WordRegister::high((self.h as u16).swap_bytes(),)),
            
            Some(Register::PC) => {
                return Some(WordRegister::high(self.pc))
            },
            Some(Register::SP) => return Some(WordRegister::high(self.sp)),
            None => Some(WordRegister::default()),
        }
    }
}

    // ie: byte, // interrupt enable
    // ir: byte, // instruction Register


    // impl a conversion from wordReg to u16 and vice vearsa
enum WordRegister {
    low(Byte),
    high(u16),
}


impl WordRegister {
    fn default() -> WordRegister{

        return WordRegister::low(0x00)

    }
}

enum size_flag {
    Imm8(Byte),
    Imm16(u16),
}


// TODO cpu_context


pub struct CpuContext{
    regs: CpuReg,
    fetch: u16,
    mem_dest: u16,
    dest_mem_flag: bool,
    current_opcode: Byte,
    current_instr: instruction::Instruction,
    halted: bool, // status can be joined to one no? 
    stepping: bool,
}

impl CpuContext {

    // pub fn new(cpu_mmu: &MemUnit, cartContext: cart::CartContext) -> CpuContext{
    pub fn new() -> CpuContext{
        
        return Self{
            regs: CpuReg { b: (0x00),
                 c: (0x00),
                  d: (0x00),
                   e: (0x00),
                    h: (0x00),
                     l: (0x00),
                      a: (0x01),
                       f: (0x00),
                        sp: (0x0000),
                         pc: (0x100) }, // make CpuReg::default/init
            fetch: 0x00,
            mem_dest: 0x000,
            dest_mem_flag: false,
            current_opcode: 0x00,
            current_instr: Instruction::default(),
            halted: true,
            stepping: false,

        }
    }

    fn fetch_instruction(&mut self, cpu_mmu: &MemUnit){
        self.current_opcode = cpu_mmu.bus_read(self.regs.pc);
        self.regs.pc += 1;
        self.current_instr = match opcode_instruction(self.current_opcode){
            Some(instr) => instr.clone(),
            None => instruction::Instruction::default(), //create void instr
        };
    }   

    fn fetch_data(&mut self, cpu_mmu: &MemUnit){
        self.mem_dest = 0;
        self.dest_mem_flag = false;

        match self.current_instr.addr_mode {
            AddressMode::Imp => return,
            AddressMode::RToD8 => {
                self.fetch = (cpu_mmu.bus_read(self.regs.pc)).into();
                emu_cycles(1);
                self.regs.pc += 1;
            },
            AddressMode::D16 => {
                let low = cpu_mmu.bus_read(self.regs.pc);
                emu_cycles(1);
                let high= cpu_mmu.bus_read(self.regs.pc + 1);
                emu_cycles(1);
                // self.fetch = low as u16;
                self.fetch = u16::from_le_bytes([low, high]);
                self.regs.pc += 2;
            },
            AddressMode::R => {
                
                self.fetch = match (self.regs.cpu_regs_read(self.current_instr.reg1.as_ref())).into() {
                    Some(WordRegister::low(word_reg)) => word_reg as u16,
                    Some(WordRegister::high(word_reg)) => word_reg,
                    None => 0x00,
                };
            },
            

            _ => panic!("Err - fetch_data"),
        }
    }


    
    
    fn execute(&self) {
    
        print!("\n\nInstruction Executed: {:#4x}    PC: {:#4x}\n", self.current_opcode, self.regs.pc);
        //unimplemented!("end of testing");
    
    }

    pub fn begin(&mut self) {
        self.halted = false;
    }

}

pub fn cpu_step(cpu_ctx: &mut CpuContext, cpu_mmu: &MemUnit, cartContext: &cart::CartContext) -> bool{

    // let mut cpu_ctx = CpuContext::new(cpu_mmu, cartContext);
    if !cpu_ctx.halted {
        cpu_ctx.fetch_instruction(cpu_mmu);
        cpu_ctx.fetch_data(&cpu_mmu);
        cpu_ctx.execute();
        return false;

    }
    return true;
}
fn emu_cycles(cycles: u32){
    // TODO Sync PPU and CPU
    let _ = cycles;
    // unimplemented!()
}
