use core::panic;
use std::default;
use std::ops::BitAnd;
use std::print;
// use std::str::Bytes;
use std::unimplemented;

use crate::Byte;
use crate::cart;
use crate::cpu;
use crate::emu;
use crate::instruction;
use crate::cpu_process;
use crate::instruction::{Condition,
    Instruction,
    Register};
use crate::mmu::MemUnit;

// use crate::mmu::bus_read;


pub type PROC = CpuContext;

pub struct CpuReg {
    pub b: Byte,
    pub c: Byte,
    pub d: Byte,
    pub e: Byte,
    pub h: Byte,
    pub l: Byte,
    pub a: Byte,
    pub f: Byte,
    pub sp: u16,
    pub pc: u16,            //pc: WordRegister,
    pub af: u16,
    pub bc: u16,
    pub de: u16,
    pub hl: u16,

}

impl CpuReg {

    pub fn check_cond(cpu_ctx: &cpu::CpuContext) -> bool{
        let flag_c: bool = cpu_ctx.regs.f.bitand(0b00001000) > 0;
        let flag_z: bool = cpu_ctx.regs.f.bitand(0b01000000) > 0;

        match cpu_ctx.current_instr.cond {
            Some(Condition::C) => return flag_c,
            Some(Condition::Z) => return flag_z,
            Some(Condition::NC) => return !flag_c,
            Some(Condition::NZ) => return !flag_z,
            None => return true,
        }
        
      }

    pub fn cpu_regs_read(&self, reg: Option<&instruction::Register>) -> WordRegister{
        match reg {
            Some(Register::A) => return WordRegister::low(self.a) ,
            Some(Register::F) => return WordRegister::low(self.f) ,
            Some(Register::B) => return WordRegister::low(self.b) ,
            Some(Register::C) => return WordRegister::low(self.c) ,
            Some(Register::D) => return WordRegister::low(self.d) ,
            Some(Register::E) => return WordRegister::low(self.e) ,
            Some(Register::H) => return WordRegister::low(self.h) ,
            Some(Register::L) => return WordRegister::low(self.l) ,
            
            Some(Register::AF) => return WordRegister::high((self.a as u16).swap_bytes(),),
            Some(Register::BC) => return WordRegister::high((self.b as u16).swap_bytes(),),
            Some(Register::DE) => return WordRegister::high((self.d as u16).swap_bytes(),),
            Some(Register::HL) => return WordRegister::high((self.h as u16).swap_bytes(),),
            
            Some(Register::PC) => {
                return WordRegister::high(self.pc)
            },
            Some(Register::SP) => return WordRegister::high(self.sp),
            None => WordRegister::default(),
        }
    }

    pub fn cpu_set_regs(&mut self, reg: Option<&instruction::Register>, val: u16) {
        match reg {
            Some(Register::A) => self.a = val as u8 ,
            Some(Register::F) => self.f = val as u8 ,
            Some(Register::B) => self.b = val as u8 ,
            Some(Register::C) => self.c = val as u8 ,
            Some(Register::D) => self.d = val as u8 ,
            Some(Register::E) => self.e = val as u8 ,
            Some(Register::H) => self.h = val as u8 ,
            Some(Register::L) => self.l = val as u8 ,
            
            Some(Register::AF) => { 
                self.a = (val >> 8) as u8; 
                self.f = val as u8},
            Some(Register::BC) => {
                self.b = (val >> 8) as u8; 
                self.c = val as u8},
            Some(Register::DE) => {
                self.d = (val >> 8) as u8; 
                self.e = val as u8},
            Some(Register::HL) => {
                self.h = (val >> 8) as u8; 
                self.l = val as u8},
            
            Some(Register::PC) => {
                self.pc = val;
            },
            Some(Register::SP) => self.sp = val,
            None => return,
        }
    }

    pub fn cpu_set_flags(cpu_ctx: &mut cpu::CpuContext, z: u8, n: u8, h: u8, c: u8){
        if z != 0 {
            cpu_ctx.regs.f = 0b01000000;
        }
        if n != 0 {
            cpu_ctx.regs.f = 0b00100000;
        }
        if h != 0 {
            cpu_ctx.regs.f = 0b00010000;
        }
        if c != 0 {
            cpu_ctx.regs.f = 0b00001000;
        }            
      }
}

    // ie: byte, // interrupt enable
    // ir: byte, // instruction Register


    // impl a conversion from wordReg to u16 and vice vearsa
// #[derive(Add)]
    pub enum WordRegister {
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
    pub regs: CpuReg,
    pub fetch: u16,
    pub mem_dest: u16,
    pub dest_mem_flag: bool,
    pub current_opcode: Byte,
    pub current_instr: instruction::Instruction,
    pub halted: bool, // status can be joined to one no? 
    pub stepping: bool,
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
                        pc: (0x100),
                        af: (0x0000),
                        bc: (0x0000),
                        de: (0x0000),
                        hl: (0x0000),}, // make CpuReg::default/init
                         
            fetch: 0x00,
            mem_dest: 0x000,
            dest_mem_flag: false,
            current_opcode: 0x00,
            current_instr: Instruction::default(),
            halted: true,
            stepping: false,

        }
    }


    
   

    pub fn begin(&mut self) {
        self.halted = false;
    }

}

 pub fn execute(emulator: &mut emu::Emulator) {
        let op = emulator.cpu.current_instr.inst_type.clone();
        instruction::Operation::instr_get_process(op, emulator);
        
    }

