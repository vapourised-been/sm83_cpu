use crate::{Byte, cpu::{self, CpuContext}, emu::{self, emu_cycles}, instruction::{self, AddressMode::R, Register::*}, mmu::MemUnit};
use std::ops::BitAnd;
use crate::cpu::WordRegister;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub enum AddressMode {
    Imp,
    RToD16,
    D16ToR,
    MrToR,
    R,
    RToD8,
    A16ToR,
    RToA16,
    RToR,
    RToMr,
    HliToR, // < use case on fraudwatch
    HldToR, // < use case on fraudwatch
    RToHli, // < use case on fraudwatch
    RToHld, // < use case on fraudwatch
    A8ToR,
    RToA8, 
    RToSpr,
    D16,
    D8,
    MrToD8,
    Mr,
}

#[derive(Clone, Debug,)]
pub enum Operation{
    NONE,
    NOP,
    LD,
    INC,
    DEC,
    RLCA,
    ADD,
    RRCA,
    STOP,
    JR,
    RRA,
    DAA,
    CPL,
    SCF,
    CCF,
    HALT,
    ADC,
    SUB,
    SBC,
    AND,
    XOR,
    OR,
    CP,
    RET,
    POP,
    JP,
    CALL,
    PUSH,
    RST,
    PREFIX, // <--
    RETI,
    LDH,
    JPHL,
    DI,
    EI, // prefix CB instructions 
    RLC,
    RRC,
    RLA,
    RR,
    SLA,
    SRA,
    SWAP,
    SRL,
    BIT,
    RES,
    SET,
}

impl Operation {

      
           

      fn is_16bit(reg: &Register) -> bool{
            return reg >= &Register::SP;
      }
      
      fn process_jp(cpu_ctx: &mut cpu::CpuContext) {
           
            if cpu::CpuReg::check_cond(&cpu_ctx) {
                  cpu_ctx.regs.pc = cpu_ctx.fetch;
                  emu::emu_cycles(1);
            }
            
      }

      

      fn process_inc(cpu_ctx: &mut cpu::CpuContext, mem: &mut MemUnit){
            
            let mut val: u16 = 0;            
            match cpu_ctx.regs.cpu_regs_read(cpu_ctx.current_instr.reg1.as_ref()) {
                  WordRegister::low(byte_reg) => {
                        // INC r8 should be the only valid instruction here, so we simulate only 1 cycle
                        val = (byte_reg + 1u8) as u16;
                        cpu_ctx.regs.cpu_set_regs(cpu_ctx.current_instr.reg1.as_ref(), val);
                        emu_cycles(1);
                        cpu::CpuReg::cpu_set_flags(cpu_ctx, if val == 0 {1} else {0}, 0, ((val & 0x0F) == 0) as u8, 0);
                        },

                  WordRegister::high(byte_reg) => {
                        // INC [HL] should be the onl valid instruction here, so we simulate 3 cycles
                        if cpu_ctx.current_instr.addr_mode == AddressMode::Mr {
                              match cpu_ctx.regs.cpu_regs_read(Some(&Register::HL)){
                                    WordRegister::low(_) => return, // cant return as low
                                    WordRegister::high(addr) => {
                                          let val_u8 = mem.bus_read(addr) + 1u8;
                                          mem.bus_write(addr, val_u8);
                                          emu_cycles(3);
                                          cpu::CpuReg::cpu_set_flags(cpu_ctx, if val == 0 {1} else {0}, 0, ((val & 0x0F) == 0) as u8, 0);
                                    },
                              }
                        } else {
                              // INC r16 & INC SP should be the instructions captured here, so we simulate 2 cycles
                              val = byte_reg + 1;
                              cpu_ctx.regs.cpu_set_regs(cpu_ctx.current_instr.reg1.as_ref(), val);
                              emu_cycles(2);
                        }
                  },
            }
      }

      fn process_dec(cpu_ctx: &mut cpu::CpuContext, mem: &mut MemUnit){
            
            let mut val: u16 = 0;            
            match cpu_ctx.regs.cpu_regs_read(cpu_ctx.current_instr.reg1.as_ref()) {
                  WordRegister::low(byte_reg) => {
                        // INC r8 should be the only valid instruction here, so we simulate only 1 cycle
                        val = (byte_reg - 1u8) as u16;
                        cpu_ctx.regs.cpu_set_regs(cpu_ctx.current_instr.reg1.as_ref(), val);
                        emu_cycles(1);
                        cpu::CpuReg::cpu_set_flags(cpu_ctx, if val == 0 {1} else {0}, 0, ((val & 0x0F) == 0) as u8, 0);
                        },

                  WordRegister::high(byte_reg) => {
                        // INC [HL] should be the onl valid instruction here, so we simulate 3 cycles
                        if cpu_ctx.current_instr.addr_mode == AddressMode::Mr {
                              match cpu_ctx.regs.cpu_regs_read(Some(&Register::HL)){
                                    WordRegister::low(_) => return, // cant return as low
                                    WordRegister::high(addr) => {
                                          let val_u8 = mem.bus_read(addr) - 1u8;
                                          mem.bus_write(addr, val_u8);
                                          emu_cycles(3);
                                          cpu::CpuReg::cpu_set_flags(cpu_ctx, if val == 0 {1} else {0}, 0, ((val & 0x0F) == 0) as u8, 0);
                                    },
                              }
                        } else {
                              // INC r16 & INC SP should be the instructions captured here, so we simulate 2 cycles
                              val = byte_reg - 1;
                              cpu_ctx.regs.cpu_set_regs(cpu_ctx.current_instr.reg1.as_ref(), val);
                              emu_cycles(2);
                        }
                  },
            }
      }


      fn process_xor(cpu_ctx: &mut cpu::CpuContext){
            cpu_ctx.regs.a = cpu_ctx.regs.a ^ (cpu_ctx.fetch as u8);
            cpu::CpuReg::cpu_set_flags(cpu_ctx, if cpu_ctx.regs.a == 0 {1} else {0}, 0, 0, 0);
      }

      pub fn instr_get_process(op: Operation, ctx: &mut emu::Emulator){

            match op {
                  Operation::JP => Operation::process_jp(&mut ctx.cpu),
                  Operation::XOR => Operation::process_xor(&mut ctx.cpu),
                  Operation::INC => Operation::process_inc(&mut ctx.cpu, &mut ctx.memory),
                  Operation::DEC => Operation::process_dec(&mut ctx.cpu, &mut ctx.memory),

                  _ => return,
            }
            
      }



}

#[derive(Clone)]
pub enum Condition {
    NZ,
    Z,
    NC,
    C,
}

#[derive(Clone, PartialEq, PartialOrd)]
pub enum Register {
    B,
    C,
    D,
    E,
    H,
    L,
    A,
    F,
    SP,
    PC,
    AF,
    BC,
    DE,
    HL,
}

#[derive(Clone)]
pub struct Instruction {

    pub inst_type: Operation,
    pub addr_mode: AddressMode, 
    pub reg1: Option<Register>,
    pub reg2: Option<Register>,
    pub cond: Option<Condition>,
    pub param: Option<u16>,

}

pub fn opcode_instruction(opcode: Byte) -> Option<&'static Instruction>{

    if opcode as usize > INSTRUCTIONS.to_vec().capacity() {
      return INSTRUCTIONS[0 as usize].as_ref();
    } else {
      return INSTRUCTIONS[opcode as usize].as_ref();

    }
}

impl Default for Instruction {
    fn default() -> Self {
        Self {inst_type: Operation::NOP,
         addr_mode: AddressMode::Imp,
          reg1: Option::None,
          reg2: Option::None,
          cond: Option::None,
          param: Option::None}
    }
}

pub static INSTRUCTIONS: [Option<Instruction>; 0x100] = [
    Some(Instruction {inst_type: Operation::NOP,
          addr_mode: AddressMode::Imp,
          reg1: Option::None,
          reg2: Option::None,
          cond: Option::None,
          param: Option::None}), // 0x00
    Some(Instruction {inst_type: Operation::LD,
          addr_mode: AddressMode::RToD16,
          reg1: Option::Some(Register::BC),
          reg2: Option::None,
          cond: Option::None,
          param: Option::None}), // 0x01
    Some(Instruction {inst_type: Operation::LD,
          addr_mode: AddressMode::MrToR,
          reg1: Option::Some(BC),
          reg2: Option::Some(A),
          cond: Option::None,
          param: Option::None}), // 0x02
    Some(Instruction {inst_type: Operation::INC,
          addr_mode: AddressMode::R,
          reg1: Option::Some(BC),
          reg2: Option::None,
          cond: Option::None,
          param: Option::None}), // 0x03
    Some(Instruction {inst_type: Operation::INC,
          addr_mode: AddressMode::R,
          reg1: Option::Some(B),
          reg2: Option::None,
          cond: Option::None,
          param: Option::None}), // 0x04
    Some(Instruction {inst_type: Operation::DEC,
          addr_mode: AddressMode::R,
          reg1: Option::Some(Register::B),
          reg2: Option::None,
          cond: Option::None,
          param: Option::None}), // 0x05
    Some(Instruction {inst_type: Operation::LD,
          addr_mode: AddressMode::RToD8,
          reg1: Option::Some(Register::B),
          reg2: Option::None,
          cond: Option::None,
          param: Option::None}), // 0x06
    Some(Instruction {inst_type: Operation::RLCA,
         addr_mode: AddressMode::Imp,
          reg1: Option::None,
          reg2: Option::None,
          cond: Option::None,
          param: Option::None}), // 0x07
    Some(Instruction {inst_type: Operation::LD,
         addr_mode: AddressMode::A16ToR,
          reg1: Option::None,
          reg2: Option::Some(Register::SP),
          cond: Option::None,
          param: Option::None}), // 0x08
    Some(Instruction {inst_type: Operation::ADD,
          addr_mode: AddressMode::RToR,
          reg1: Option::Some(Register::HL),
          reg2: Option::Some(BC),
          cond: Option::None,
          param: Option::None}), // 0x09 
    Some(Instruction {inst_type: Operation::LD,
          addr_mode: AddressMode::RToMr,
          reg1: Option::Some(Register::A),
          reg2: Option::Some(BC),
          cond: Option::None,
          param: Option::None}), // 0x0A         
    Some(Instruction {inst_type: Operation::DEC,
          addr_mode: AddressMode::R,
          reg1: Option::Some(Register::BC),
          reg2: Option::None,
          cond: Option::None,
          param: Option::None}), // 0x0B
    Some(Instruction {inst_type: Operation::INC,
          addr_mode: AddressMode::R,
          reg1: Option::Some(Register::C),
          reg2: Option::None,
          cond: Option::None,
          param: Option::None}), // 0x0C
    Some(Instruction {inst_type: Operation::DEC,
          addr_mode: AddressMode::R,
          reg1: Option::Some(Register::C),
          reg2: Option::None,
          cond: Option::None,
          param: Option::None}), // 0x0D
    Some(Instruction {inst_type: Operation::LD,
         addr_mode: AddressMode::RToD8,
          reg1: Option::Some(Register::C),
          reg2: Option::None,
          cond: Option::None,
          param: Option::None}), // 0x0E
    Some(Instruction {inst_type: Operation::RRCA,
          addr_mode: AddressMode::Imp,
          reg1: Option::None,
          reg2: Option::None,
          cond: Option::None,
          param: Option::None}), // 0x0F

    Some(Instruction {inst_type: Operation::STOP,
          addr_mode: AddressMode::D8,
          reg1: Option::None,
          reg2: Option::None,
          cond: Option::None,
          param: Option::None}), // 0x10
    Some(Instruction {inst_type: Operation::LD,
          addr_mode: AddressMode::RToD16,
          reg1: Option::Some(Register::DE),
          reg2: Option::None,
          cond: Option::None,
          param: Option::None}), // 0x11
    Some(Instruction {inst_type: Operation::LD,
          addr_mode: AddressMode::MrToR,
          reg1: Option::Some(Register::DE),
          reg2: Option::Some(A),
          cond: Option::None,
          param: Option::None}), // 0x12
    Some(Instruction {inst_type: Operation::INC,
          addr_mode: AddressMode::R,
          reg1: Option::Some(Register::DE),
          reg2: Option::None,
          cond: Option::None,
          param: Option::None}), // 0x13
    Some(Instruction {inst_type: Operation::INC,
          addr_mode: AddressMode::R,
          reg1: Option::Some(Register::D),
          reg2: Option::None,
          cond: Option::None,
          param: Option::None}), // 0x14
    Some(Instruction {inst_type: Operation::DEC,
          addr_mode: AddressMode::R,
          reg1: Option::Some(Register::D),
          reg2: Option::None,
          cond: Option::None,
          param: Option::None}), // 0x15
    Some(Instruction {inst_type: Operation::LD,
          addr_mode: AddressMode::RToD8,
          reg1: Option::Some(Register::D),
          reg2: Option::None,
          cond: Option::None,
          param: Option::None}), // 0x16
    Some(Instruction {inst_type: Operation::RLA,
          addr_mode: AddressMode::Imp,
          reg1: Option::None,
          reg2: Option::None,
          cond: Option::None,
          param: Option::None}), // 0x17
    Some(Instruction {inst_type: Operation::JR,
          addr_mode: AddressMode::D8,
          reg1: Option::None,
          reg2: Option::None,
          cond: Option::None,
          param: Option::None}), // 0x18
    Some(Instruction {inst_type: Operation::ADD,
          addr_mode: AddressMode::RToR,
          reg1: Option::Some(Register::HL),
          reg2: Option::Some(DE),
          cond: Option::None,
          param: Option::None}), // 0x19
    Some(Instruction {inst_type: Operation::LD,
          addr_mode: AddressMode::RToMr,
          reg1: Option::Some(Register::A),
          reg2: Option::Some(DE),
          cond: Option::None,
          param: Option::None}), // 0x1A
    Some(Instruction {inst_type: Operation::DEC,
          addr_mode: AddressMode::R,
          reg1: Option::Some(Register::DE),
          reg2: Option::None,
          cond: Option::None,
          param: Option::None}), // 0x1B
    Some(Instruction {inst_type: Operation::INC,
          addr_mode: AddressMode::R,
          reg1: Option::Some(Register::E),
          reg2: Option::None,
          cond: Option::None,
          param: Option::None}), // 0x1C
    Some(Instruction {inst_type: Operation::DEC,
          addr_mode: AddressMode::R,
          reg1: Option::Some(Register::E),
          reg2: Option::None,
          cond: Option::None,
          param: Option::None}), // 0x1D
    Some(Instruction {inst_type: Operation::LD,
          addr_mode: AddressMode::RToD8,
          reg1: Option::Some(Register::E),
          reg2: Option::None,
          cond: Option::None,
          param: Option::None}), // 0x1E
    Some(Instruction {inst_type: Operation::RRA,
          addr_mode: AddressMode::Imp,
          reg1: Option::None,
          reg2: Option::None,
          cond: Option::None,
          param: Option::None}), // 0x1F

    Some(Instruction {inst_type: Operation::JR,
          addr_mode: AddressMode::D8,
          reg1: Option::None,
          reg2: Option::None,
          cond: Option::Some(Condition::NZ),
          param: Option::None}), // 0x20
    Some(Instruction {inst_type: Operation::LD,
          addr_mode: AddressMode::RToD16,
          reg1: Option::Some(Register::HL),
          reg2: Option::None,
          cond: Option::None,
          param: Option::None}), // 0x21
    Some(Instruction {inst_type: Operation::LD,
          addr_mode: AddressMode::HliToR,
          reg1: Option::Some(Register::HL),
          reg2: Option::Some(A),
          cond: Option::None,
          param: Option::None}), // 0x22
    Some(Instruction {inst_type: Operation::INC,
          addr_mode: AddressMode::R,
          reg1: Option::Some(Register::HL),
          reg2: Option::None,
          cond: Option::None,
          param: Option::None}), // 0x23
    Some(Instruction {inst_type: Operation::INC,
          addr_mode: AddressMode::R,
          reg1: Option::Some(Register::H),
          reg2: Option::None,
          cond: Option::None,
          param: Option::None}), // 0x24
    Some(Instruction {inst_type: Operation::DEC,
          addr_mode: AddressMode::R,
          reg1: Option::Some(Register::H),
          reg2: Option::None,
          cond: Option::None,
          param: Option::None}), // 0x25
    Some(Instruction {inst_type: Operation::LD,
          addr_mode: AddressMode::RToD8,
          reg1: Option::Some(Register::H),
          reg2: Option::None,
          cond: Option::None,
          param: Option::None}), // 0x26
    Some(Instruction {inst_type: Operation::DEC,
          addr_mode: AddressMode::R,
          reg1: Option::Some(Register::BC),
          reg2: Option::None,
          cond: Option::None,
          param: Option::None}), // 0x27
    Some(Instruction {inst_type: Operation::JR,
          addr_mode: AddressMode::D8,
          reg1: Option::None,
          reg2: Option::None,
          cond: Option::None,
          param: Option::None}), // 0x28
    Some(Instruction {inst_type: Operation::ADD,
          addr_mode: AddressMode::RToR,
          reg1: Option::Some(Register::HL),
          reg2: Option::Some(HL),
          cond: Option::None,
          param: Option::None}), // 0x29
    Some(Instruction {inst_type: Operation::LD,
          addr_mode: AddressMode::RToHli,
          reg1: Option::Some(Register::A),
          reg2: Option::Some(HL),
          cond: Option::None,
          param: Option::None}), // 0x2A
    Some(Instruction {inst_type: Operation::DEC,
          addr_mode: AddressMode::R,
          reg1: Option::Some(Register::HL),
          reg2: Option::None,
          cond: Option::None,
          param: Option::None}), // 0x2B
    Some(Instruction {inst_type: Operation::INC,
          addr_mode: AddressMode::R,
          reg1: Option::Some(Register::L),
          reg2: Option::None,
          cond: Option::None,
          param: Option::None}), // 0x2C
    Some(Instruction {inst_type: Operation::DEC,
          addr_mode: AddressMode::R,
          reg1: Option::Some(Register::L),
          reg2: Option::None,
          cond: Option::None,
          param: Option::None}), // 0x2D
    Some(Instruction {inst_type: Operation::LD,
          addr_mode: AddressMode::RToD8,
          reg1: Option::Some(Register::E),
          reg2: Option::None,
          cond: Option::None,
          param: Option::None}), // 0x2E
    Some(Instruction {inst_type: Operation::RRA,
          addr_mode: AddressMode::Imp,
          reg1: Option::None,
          reg2: Option::None,
          cond: Option::None,
          param: Option::None}), // 0x2F
    
    Some(Instruction {inst_type: Operation::JR,
          addr_mode: AddressMode::Imp,
          reg1: Option::None,
          reg2: Option::None,
          cond: Option::Some(Condition::NZ),
          param: Option::None}), // 0x30
    Some(Instruction {inst_type: Operation::LD,
          addr_mode: AddressMode::RToD16,
          reg1: Option::Some(Register::SP),
          reg2: Option::None,
          cond: Option::None,
          param: Option::None}), // 0x31
    Some(Instruction {inst_type: Operation::LD,
          addr_mode: AddressMode::HldToR,
          reg1: Option::Some(Register::HL),
          reg2: Option::Some(A),
          cond: Option::None,
          param: Option::None}), // 0x32
    Some(Instruction {inst_type: Operation::INC,
          addr_mode: AddressMode::R,
          reg1: Option::Some(Register::SP),
          reg2: Option::None,
          cond: Option::None,
          param: Option::None}), // 0x33
    Some(Instruction {inst_type: Operation::INC,
          addr_mode: AddressMode::Mr,
          reg1: Option::Some(Register::HL),
          reg2: Option::None,
          cond: Option::None,
          param: Option::None}), // 0x34
    Some(Instruction {inst_type: Operation::DEC,
          addr_mode: AddressMode::Mr,
          reg1: Option::Some(Register::HL),
          reg2: Option::None,
          cond: Option::None,
          param: Option::None}), // 0x35
    Some(Instruction {inst_type: Operation::LD,
          addr_mode: AddressMode::MrToD8,
          reg1: Option::Some(Register::HL),
          reg2: Option::None,
          cond: Option::None,
          param: Option::None}), // 0x36
    Some(Instruction {inst_type: Operation::SCF,
          addr_mode: AddressMode::Imp,
          reg1: Option::None,
          reg2: Option::None,
          cond: Option::None,
          param: Option::None}), // 0x37
    Some(Instruction {inst_type: Operation::JR,
          addr_mode: AddressMode::D8,
          reg1: Option::Some(Register::C),
          reg2: Option::None,
          cond: Option::None,
          param: Option::None}), // 0x38
    Some(Instruction {inst_type: Operation::ADD,
          addr_mode: AddressMode::RToR,
          reg1: Option::Some(Register::HL),
          reg2: Option::Some(SP),
          cond: Option::None,
          param: Option::None}), // 0x39
    Some(Instruction {inst_type: Operation::LD,
          addr_mode: AddressMode::RToHld,
          reg1: Option::Some(Register::A),
          reg2: Option::Some(HL),
          cond: Option::None,
          param: Option::None}), // 0x3A
    Some(Instruction {inst_type: Operation::DEC,
          addr_mode: AddressMode::R,
          reg1: Option::Some(Register::SP),
          reg2: Option::None,
          cond: Option::None,
          param: Option::None}), // 0x3B
    Some(Instruction {inst_type: Operation::INC,
          addr_mode: AddressMode::R,
          reg1: Option::Some(Register::A),
          reg2: Option::None,
          cond: Option::None,
          param: Option::None}), // 0x3C
    Some(Instruction {inst_type: Operation::DEC,
          addr_mode: AddressMode::R,
          reg1: Option::Some(Register::A),
          reg2: Option::None,
          cond: Option::None,
          param: Option::None}), // 0x3D
    Some(Instruction {inst_type: Operation::LD,
          addr_mode: AddressMode::RToD8,
          reg1: Option::Some(Register::A),
          reg2: Option::None,
          cond: Option::None,
          param: Option::None}), // 0x3E
    Some(Instruction {inst_type: Operation::CCF,
          addr_mode: AddressMode::Imp,
          reg1: Option::None,
          reg2: Option::None,
          cond: Option::None,
          param: Option::None}), // 0x3F

    Some(Instruction {inst_type: Operation::LD,
          addr_mode: AddressMode::RToR,
          reg1: Option::Some(B),
          reg2: Option::Some(Register::B),
          cond: Option::None,
          param: Option::None}), // 0x40
    Some(Instruction {inst_type: Operation::LD,
          addr_mode: AddressMode::RToR,
          reg1: Option::Some(B),
          reg2: Option::Some(Register::C),
          cond: Option::None,
          param: Option::None}), // 0x41
    Some(Instruction {inst_type: Operation::LD,
          addr_mode: AddressMode::RToR,
          reg1: Option::Some(B),
          reg2: Option::Some(D),
          cond: Option::None,
          param: Option::None}), // 0x42
    Some(Instruction {inst_type: Operation::LD,
          addr_mode: AddressMode::RToR,
          reg1: Option::Some(B),
          reg2: Option::Some(E),
          cond: Option::None,
          param: Option::None}), // 0x43
    Some(Instruction {inst_type: Operation::LD,
          addr_mode: AddressMode::RToR,
          reg1: Option::Some(B),
          reg2: Option::Some(H),
          cond: Option::None,
          param: Option::None}), // 0x44
    Some(Instruction {inst_type: Operation::LD,
          addr_mode: AddressMode::RToR,
          reg1: Option::Some(B),
          reg2: Option::Some(L),
          cond: Option::None,
          param: Option::None}), // 0x45
    Some(Instruction {inst_type: Operation::LD,
          addr_mode: AddressMode::RToMr,
          reg1: Option::Some(B),
          reg2: Option::Some(Register::HL),
          cond: Option::None,
          param: Option::None}), // 0x46
    Some(Instruction {inst_type: Operation::LD,
          addr_mode: AddressMode::RToR,
          reg1: Option::Some(B),
          reg2: Option::Some(A),
          cond: Option::None,
          param: Option::None}), // 0x47
    Some(Instruction {inst_type: Operation::LD,
          addr_mode: AddressMode::RToR,
          reg1: Option::Some(C),
          reg2: Option::Some(B),
          cond: Option::None,
          param: Option::None}), // 0x48
    Some(Instruction {inst_type: Operation::LD,
          addr_mode: AddressMode::RToR,
          reg1: Option::Some(C),
          reg2: Option::Some(C),
          cond: Option::None,
          param: Option::None}), // 0x49
    Some(Instruction {inst_type: Operation::LD,
          addr_mode: AddressMode::RToR,
          reg1: Option::Some(C),
          reg2: Option::Some(D),
          cond: Option::None,
          param: Option::None}), // 0x4A
    Some(Instruction {inst_type: Operation::LD,
          addr_mode: AddressMode::RToR,
          reg1: Option::Some(C),
          reg2: Option::Some(D),
          cond: Option::None,
          param: Option::None}), // 0x4B
    Some(Instruction {inst_type: Operation::LD,
          addr_mode: AddressMode::RToR,
          reg1: Option::Some(C),
          reg2: Option::Some(E),
          cond: Option::None,
          param: Option::None}), // 0x4C
    Some(Instruction {inst_type: Operation::LD,
          addr_mode: AddressMode::RToR,
          reg1: Option::Some(C),
          reg2: Option::Some(H),
          cond: Option::None,
          param: Option::None}), // 0x4D
    Some(Instruction {inst_type: Operation::LD,
          addr_mode: AddressMode::RToR,
          reg1: Option::Some(C),
          reg2: Option::Some(L),
          cond: Option::None,
          param: Option::None}), // 0x4E
    Some(Instruction {inst_type: Operation::LD,
          addr_mode: AddressMode::RToMr,
          reg1: Option::Some(C),
          reg2: Option::Some(HL),
          cond: Option::None,
          param: Option::None}), // 0x4F
          
    Some(Instruction {inst_type: Operation::LD,
          addr_mode: AddressMode::RToR,
          reg1: Option::Some(Register::D),
          reg2: Option::Some(Register::B),
          cond: Option::None,
          param: Option::None}), // 0x50
    Some(Instruction {inst_type: Operation::LD,
          addr_mode: AddressMode::RToR,
          reg1: Option::Some(Register::D),
          reg2: Option::Some(Register::C),
          cond: Option::None,
          param: Option::None}), // 0x51
    Some(Instruction {inst_type: Operation::LD,
          addr_mode: AddressMode::RToR,
          reg1: Option::Some(Register::D),
          reg2: Option::Some(Register::D),
          cond: Option::None,
          param: Option::None}), // 0x52
    Some(Instruction {inst_type: Operation::LD,
          addr_mode: AddressMode::RToR,
          reg1: Option::Some(Register::D),
          reg2: Option::Some(Register::E),
          cond: Option::None,
          param: Option::None}), // 0x53
    Some(Instruction {inst_type: Operation::LD,
          addr_mode: AddressMode::RToR,
          reg1: Option::Some(Register::D),
          reg2: Option::Some(Register::H),
          cond: Option::None,
          param: Option::None}), // 0x54
    Some(Instruction {inst_type: Operation::LD,
          addr_mode: AddressMode::RToR,
          reg1: Option::Some(Register::D),
          reg2: Option::Some(Register::L),
          cond: Option::None,
          param: Option::None}), // 0x55
    Some(Instruction {inst_type: Operation::LD,
          addr_mode: AddressMode::RToMr,
          reg1: Option::Some(Register::D),
          reg2: Option::Some(Register::HL),
          cond: Option::None,
          param: Option::None}), // 0x56
    Some(Instruction {inst_type: Operation::LD,
          addr_mode: AddressMode::RToR,
          reg1: Option::Some(Register::D),
          reg2: Option::Some(Register::A),
          cond: Option::None,
          param: Option::None}), // 0x57
    Some(Instruction {inst_type: Operation::LD,
          addr_mode: AddressMode::RToR,
          reg1: Option::Some(Register::E),
          reg2: Option::Some(Register::B),
          cond: Option::None,
          param: Option::None}), // 0x58
    Some(Instruction {inst_type: Operation::LD,
          addr_mode: AddressMode::RToR,
          reg1: Option::Some(Register::E),
          reg2: Option::Some(Register::C),
          cond: Option::None,
          param: Option::None}), // 0x59
    Some(Instruction {inst_type: Operation::LD,
          addr_mode: AddressMode::RToR,
          reg1: Option::Some(Register::E),
          reg2: Option::Some(Register::D),
          cond: Option::None,
          param: Option::None}), // 0x5A
    Some(Instruction {inst_type: Operation::LD,
          addr_mode: AddressMode::RToR,
          reg1: Option::Some(Register::E),
          reg2: Option::Some(Register::E),
          cond: Option::None,
          param: Option::None}), // 0x5B
    Some(Instruction {inst_type: Operation::LD,
          addr_mode: AddressMode::RToR,
          reg1: Option::Some(Register::E),
          reg2: Option::Some(Register::H),
          cond: Option::None,
          param: Option::None}), // 0x5C
    Some(Instruction {inst_type: Operation::LD,
          addr_mode: AddressMode::RToR,
          reg1: Option::Some(Register::E),
          reg2: Option::Some(Register::L),
          cond: Option::None,
          param: Option::None}), // 0x5D
    Some(Instruction {inst_type: Operation::LD,
          addr_mode: AddressMode::RToMr,
          reg1: Option::Some(Register::E),
          reg2: Option::Some(Register::HL),
          cond: Option::None,
          param: Option::None}), // 0x5E
    Some(Instruction {inst_type: Operation::LD,
          addr_mode: AddressMode::RToR,
          reg1: Option::Some(Register::E),
          reg2: Option::Some(Register::A),
          cond: Option::None,
          param: Option::None}), // 0x5F
     
    Some(Instruction {inst_type: Operation::LD,
          addr_mode: AddressMode::RToR,
          reg1: Option::Some(Register::H),
          reg2: Option::Some(Register::B),
          cond: Option::None,
          param: Option::None}), // 0x60
    Some(Instruction {inst_type: Operation::LD,
          addr_mode: AddressMode::RToR,
          reg1: Option::Some(Register::H),
          reg2: Option::Some(Register::C),
          cond: Option::None,
          param: Option::None}), // 0x61
    Some(Instruction {inst_type: Operation::LD,
          addr_mode: AddressMode::RToR,
          reg1: Option::Some(Register::H),
          reg2: Option::Some(Register::D),
          cond: Option::None,
          param: Option::None}), // 0x62
    Some(Instruction {inst_type: Operation::LD,
          addr_mode: AddressMode::RToR,
          reg1: Option::Some(Register::H),
          reg2: Option::Some(Register::E),
          cond: Option::None,
          param: Option::None}), // 0x63
    Some(Instruction {inst_type: Operation::LD,
          addr_mode: AddressMode::RToR,
          reg1: Option::Some(Register::H),
          reg2: Option::Some(Register::H),
          cond: Option::None,
          param: Option::None}), // 0x64
    Some(Instruction {inst_type: Operation::LD,
          addr_mode: AddressMode::RToR,
          reg1: Option::Some(Register::H),
          reg2: Option::Some(Register::L),
          cond: Option::None,
          param: Option::None}), // 0x65
    Some(Instruction {inst_type: Operation::LD,
          addr_mode: AddressMode::RToMr,
          reg1: Option::Some(Register::H),
          reg2: Option::Some(Register::HL),
          cond: Option::None,
          param: Option::None}), // 0x66
    Some(Instruction {inst_type: Operation::LD,
          addr_mode: AddressMode::RToR,
          reg1: Option::Some(Register::H),
          reg2: Option::Some(Register::A),
          cond: Option::None,
          param: Option::None}), // 0x67
    Some(Instruction {inst_type: Operation::LD,
          addr_mode: AddressMode::RToR,
          reg1: Option::Some(Register::L),
          reg2: Option::Some(Register::B),
          cond: Option::None,
          param: Option::None}), // 0x68
    Some(Instruction {inst_type: Operation::LD,
          addr_mode: AddressMode::RToR,
          reg1: Option::Some(Register::L),
          reg2: Option::Some(Register::C),
          cond: Option::None,
          param: Option::None}), // 0x69
    Some(Instruction {inst_type: Operation::LD,
          addr_mode: AddressMode::RToR,
          reg1: Option::Some(Register::L),
          reg2: Option::Some(Register::D),
          cond: Option::None,
          param: Option::None}), // 0x6A
    Some(Instruction {inst_type: Operation::LD,
          addr_mode: AddressMode::RToR,
          reg1: Option::Some(Register::L),
          reg2: Option::Some(Register::E),
          cond: Option::None,
          param: Option::None}), // 0x6B
    Some(Instruction {inst_type: Operation::LD,
          addr_mode: AddressMode::RToR,
          reg1: Option::Some(Register::L),
          reg2: Option::Some(Register::H),
          cond: Option::None,
          param: Option::None}), // 0x6C
    Some(Instruction {inst_type: Operation::LD,
          addr_mode: AddressMode::RToR,
          reg1: Option::Some(Register::L),
          reg2: Option::Some(Register::L),
          cond: Option::None,
          param: Option::None}), // 0x6D
    Some(Instruction {inst_type: Operation::LD,
          addr_mode: AddressMode::RToMr,
          reg1: Option::Some(Register::L),
          reg2: Option::Some(Register::HL),
          cond: Option::None,
          param: Option::None}), // 0x6E
    Some(Instruction {inst_type: Operation::LD,
          addr_mode: AddressMode::RToR,
          reg1: Option::Some(Register::L),
          reg2: Option::Some(Register::A),
          cond: Option::None,
          param: Option::None}), // 0x6F

    Some(Instruction {inst_type: Operation::LD,
          addr_mode: AddressMode::MrToR,
          reg1: Option::Some(Register::HL),
          reg2: Option::Some(Register::B),
          cond: Option::None,
          param: Option::None}), // 0x70
    Some(Instruction {inst_type: Operation::LD,
          addr_mode: AddressMode::MrToR,
          reg1: Option::Some(Register::HL),
          reg2: Option::Some(Register::C),
          cond: Option::None,
          param: Option::None}), // 0x71
    Some(Instruction {inst_type: Operation::LD,
          addr_mode: AddressMode::MrToR,
          reg1: Option::Some(Register::HL),
          reg2: Option::Some(Register::HL),
          cond: Option::None,
          param: Option::None}), // 0x72
    Some(Instruction {inst_type: Operation::LD,
          addr_mode: AddressMode::MrToR,
          reg1: Option::Some(Register::HL),
          reg2: Option::Some(Register::E),
          cond: Option::None,
          param: Option::None}), // 0x73
    Some(Instruction {inst_type: Operation::LD,
          addr_mode: AddressMode::MrToR,
          reg1: Option::Some(Register::HL),
          reg2: Option::Some(Register::H),
          cond: Option::None,
          param: Option::None}), // 0x74
    Some(Instruction {inst_type: Operation::LD,
          addr_mode: AddressMode::MrToR,
          reg1: Option::Some(Register::HL),
          reg2: Option::Some(Register::L),
          cond: Option::None,
          param: Option::None}), // 0x75
    Some(Instruction {inst_type: Operation::HALT,
          addr_mode: AddressMode::Imp,
          reg1: Option::None,
          reg2: Option::None,
          cond: Option::None,
          param: Option::None}), // 0x76
    Some(Instruction {inst_type: Operation::LD,
          addr_mode: AddressMode::MrToR,
          reg1: Option::Some(Register::HL),
          reg2: Option::Some(Register::A),
          cond: Option::None,
          param: Option::None}), // 0x77
    Some(Instruction {inst_type: Operation::LD,
          addr_mode: AddressMode::RToR,
          reg1: Option::Some(Register::A),
          reg2: Option::Some(Register::B),
          cond: Option::None,
          param: Option::None}), // 0x78
    Some(Instruction {inst_type: Operation::LD,
          addr_mode: AddressMode::RToR,
          reg1: Option::Some(Register::A),
          reg2: Option::Some(Register::C),
          cond: Option::None,
          param: Option::None}), // 0x79
    Some(Instruction {inst_type: Operation::LD,
          addr_mode: AddressMode::RToR,
          reg1: Option::Some(Register::A),
          reg2: Option::Some(Register::D),
          cond: Option::None,
          param: Option::None}), // 0x7A
    Some(Instruction {inst_type: Operation::LD,
          addr_mode: AddressMode::RToR,
          reg1: Option::Some(Register::A),
          reg2: Option::Some(Register::E),
          cond: Option::None,
          param: Option::None}), // 0x7B
    Some(Instruction {inst_type: Operation::LD,
          addr_mode: AddressMode::RToR,
          reg1: Option::Some(Register::A),
          reg2: Option::Some(Register::H),
          cond: Option::None,
          param: Option::None}), // 0x7C
    Some(Instruction {inst_type: Operation::LD,
          addr_mode: AddressMode::RToR,
          reg1: Option::Some(Register::A),
          reg2: Option::Some(Register::L),
          cond: Option::None,
          param: Option::None}), // 0x7D
    Some(Instruction {inst_type: Operation::LD,
          addr_mode: AddressMode::RToMr,
          reg1: Option::Some(Register::A),
          reg2: Option::Some(Register::HL),
          cond: Option::None,
          param: Option::None}), // 0x7E
    Some(Instruction {inst_type: Operation::LD,
          addr_mode: AddressMode::RToR,
          reg1: Option::Some(Register::A),
          reg2: Option::Some(Register::A),
          cond: Option::None,
          param: Option::None}), // 0x7F

    Some(Instruction {inst_type: Operation::ADD,
          addr_mode: AddressMode::RToR,
          reg1: Option::Some(Register::A),
          reg2: Option::Some(Register::B),
          cond: Option::None,
          param: Option::None}), // 0x80
    Some(Instruction {inst_type: Operation::ADD,
          addr_mode: AddressMode::RToR,
          reg1: Option::Some(Register::A),
          reg2: Option::Some(Register::C),
          cond: Option::None,
          param: Option::None}), // 0x81
    Some(Instruction {inst_type: Operation::ADD,
          addr_mode: AddressMode::RToR,
          reg1: Option::Some(Register::A),
          reg2: Option::Some(Register::D),
          cond: Option::None,
          param: Option::None}), // 0x82
    Some(Instruction {inst_type: Operation::ADD,
          addr_mode: AddressMode::RToR,
          reg1: Option::Some(Register::A),
          reg2: Option::Some(Register::E),
          cond: Option::None,
          param: Option::None}), // 0x83
    Some(Instruction {inst_type: Operation::ADD,
          addr_mode: AddressMode::RToR,
          reg1: Option::Some(Register::A),
          reg2: Option::Some(Register::H),
          cond: Option::None,
          param: Option::None}), // 0x84
    Some(Instruction {inst_type: Operation::ADD,
          addr_mode: AddressMode::RToR,
          reg1: Option::Some(Register::A),
          reg2: Option::Some(Register::L),
          cond: Option::None,
          param: Option::None}), // 0x85
    Some(Instruction {inst_type: Operation::ADD,
          addr_mode: AddressMode::RToMr,
          reg1: Option::Some(Register::A),
          reg2: Option::Some(Register::HL),
          cond: Option::None,
          param: Option::None}), // 0x86
    Some(Instruction {inst_type: Operation::ADD,
          addr_mode: AddressMode::RToR,
          reg1: Option::Some(Register::A),
          reg2: Option::Some(Register::A),
          cond: Option::None,
          param: Option::None}), // 0x87
    Some(Instruction {inst_type: Operation::ADC,
          addr_mode: AddressMode::RToR,
          reg1: Option::Some(Register::A),
          reg2: Option::Some(Register::B),
          cond: Option::None,
          param: Option::None}), // 0x88
    Some(Instruction {inst_type: Operation::ADC,
          addr_mode: AddressMode::RToR,
          reg1: Option::Some(Register::A),
          reg2: Option::Some(Register::C),
          cond: Option::None,
          param: Option::None}), // 0x89
    Some(Instruction {inst_type: Operation::ADC,
          addr_mode: AddressMode::RToR,
          reg1: Option::Some(Register::A),
          reg2: Option::Some(Register::D),
          cond: Option::None,
          param: Option::None}), // 0x8A
    Some(Instruction {inst_type: Operation::ADC,
          addr_mode: AddressMode::RToR,
          reg1: Option::Some(Register::A),
          reg2: Option::Some(Register::E),
          cond: Option::None,
          param: Option::None}), // 0x8B
    Some(Instruction {inst_type: Operation::ADC,
          addr_mode: AddressMode::RToR,
          reg1: Option::Some(Register::A),
          reg2: Option::Some(Register::H),
          cond: Option::None,
          param: Option::None}), // 0x8C
    Some(Instruction {inst_type: Operation::ADC,
          addr_mode: AddressMode::RToR,
          reg1: Option::Some(Register::A),
          reg2: Option::Some(Register::L),
          cond: Option::None,
          param: Option::None}), // 0x8D
    Some(Instruction {inst_type: Operation::ADC,
          addr_mode: AddressMode::RToMr,
          reg1: Option::Some(Register::A),
          reg2: Option::Some(Register::HL),
          cond: Option::None,
          param: Option::None}), // 0x8E
    Some(Instruction {inst_type: Operation::ADC,
          addr_mode: AddressMode::RToR,
          reg1: Option::Some(Register::A),
          reg2: Option::Some(Register::A),
          cond: Option::None,
          param: Option::None}), // 0x8F

    Some(Instruction {inst_type: Operation::SUB,
          addr_mode: AddressMode::RToR,
          reg1: Option::Some(Register::A),
          reg2: Option::Some(Register::B),
          cond: Option::None,
          param: Option::None}), // 0x90
    Some(Instruction {inst_type: Operation::SUB,
          addr_mode: AddressMode::RToR,
          reg1: Option::Some(Register::A),
          reg2: Option::Some(Register::C),
          cond: Option::None,
          param: Option::None}), // 0x91
    Some(Instruction {inst_type: Operation::SUB,
          addr_mode: AddressMode::RToR,
          reg1: Option::Some(Register::A),
          reg2: Option::Some(Register::D),
          cond: Option::None,
          param: Option::None}), // 0x92
    Some(Instruction {inst_type: Operation::SUB,
          addr_mode: AddressMode::RToR,
          reg1: Option::Some(Register::A),
          reg2: Option::Some(Register::E),
          cond: Option::None,
          param: Option::None}), // 0x93
    Some(Instruction {inst_type: Operation::SUB,
          addr_mode: AddressMode::RToR,
          reg1: Option::Some(Register::A),
          reg2: Option::Some(Register::H),
          cond: Option::None,
          param: Option::None}), // 0x94
    Some(Instruction {inst_type: Operation::SUB,
          addr_mode: AddressMode::RToR,
          reg1: Option::Some(Register::A),
          reg2: Option::Some(Register::L),
          cond: Option::None,
          param: Option::None}), // 0x95
    Some(Instruction {inst_type: Operation::SUB,
          addr_mode: AddressMode::RToMr,
          reg1: Option::Some(Register::A),
          reg2: Option::Some(Register::HL),
          cond: Option::None,
          param: Option::None}), // 0x96
    Some(Instruction {inst_type: Operation::SUB,
          addr_mode: AddressMode::RToR,
          reg1: Option::Some(Register::A),
          reg2: Option::Some(Register::A),
          cond: Option::None,
          param: Option::None}), // 0x97
    Some(Instruction {inst_type: Operation::SBC,
          addr_mode: AddressMode::RToR,
          reg1: Option::Some(Register::A),
          reg2: Option::Some(Register::B),
          cond: Option::None,
          param: Option::None}), // 0x98
    Some(Instruction {inst_type: Operation::SBC,
          addr_mode: AddressMode::RToR,
          reg1: Option::Some(Register::A),
          reg2: Option::Some(Register::C),
          cond: Option::None,
          param: Option::None}), // 0x99
    Some(Instruction {inst_type: Operation::SBC,
          addr_mode: AddressMode::RToR,
          reg1: Option::Some(Register::A),
          reg2: Option::Some(Register::D),
          cond: Option::None,
          param: Option::None}), // 0x9A
    Some(Instruction {inst_type: Operation::SBC,
          addr_mode: AddressMode::RToR,
          reg1: Option::Some(Register::A),
          reg2: Option::Some(Register::E),
          cond: Option::None,
          param: Option::None}), // 0x9B
    Some(Instruction {inst_type: Operation::SBC,
          addr_mode: AddressMode::RToR,
          reg1: Option::Some(Register::A),
          reg2: Option::Some(Register::H),
          cond: Option::None,
          param: Option::None}), // 0x9C
    Some(Instruction {inst_type: Operation::SBC,
          addr_mode: AddressMode::RToR,
          reg1: Option::Some(Register::A),
          reg2: Option::Some(Register::L),
          cond: Option::None,
          param: Option::None}), // 0x9D
    Some(Instruction {inst_type: Operation::SBC,
          addr_mode: AddressMode::RToMr,
          reg1: Option::Some(Register::A),
          reg2: Option::Some(Register::HL),
          cond: Option::None,
          param: Option::None}), // 0x9E
    Some(Instruction {inst_type: Operation::SBC,
          addr_mode: AddressMode::RToR,
          reg1: Option::Some(Register::A),
          reg2: Option::Some(Register::A),
          cond: Option::None,
          param: Option::None}), // 0x9F

    Some(Instruction {inst_type: Operation::AND,
          addr_mode: AddressMode::RToR,
          reg1: Option::Some(Register::A),
          reg2: Option::Some(Register::B),
          cond: Option::None,
          param: Option::None}), // 0xA0
    Some(Instruction {inst_type: Operation::AND,
          addr_mode: AddressMode::RToR,
          reg1: Option::Some(Register::A),
          reg2: Option::Some(Register::C),
          cond: Option::None,
          param: Option::None}), // 0xA1
    Some(Instruction {inst_type: Operation::AND,
          addr_mode: AddressMode::RToR,
          reg1: Option::Some(Register::A),
          reg2: Option::Some(Register::D),
          cond: Option::None,
          param: Option::None}), // 0xA2
    Some(Instruction {inst_type: Operation::AND,
          addr_mode: AddressMode::RToR,
          reg1: Option::Some(Register::A),
          reg2: Option::Some(Register::E),
          cond: Option::None,
          param: Option::None}), // 0xA3
    Some(Instruction {inst_type: Operation::AND,
          addr_mode: AddressMode::RToR,
          reg1: Option::Some(Register::A),
          reg2: Option::Some(Register::H),
          cond: Option::None,
          param: Option::None}), // 0xA4
    Some(Instruction {inst_type: Operation::AND,
          addr_mode: AddressMode::RToR,
          reg1: Option::Some(Register::A),
          reg2: Option::Some(Register::L),
          cond: Option::None,
          param: Option::None}), // 0xA5
    Some(Instruction {inst_type: Operation::AND,
          addr_mode: AddressMode::RToMr,
          reg1: Option::Some(Register::A),
          reg2: Option::Some(Register::HL),
          cond: Option::None,
          param: Option::None}), // 0xA6
    Some(Instruction {inst_type: Operation::AND,
          addr_mode: AddressMode::RToR,
          reg1: Option::Some(Register::A),
          reg2: Option::Some(Register::A),
          cond: Option::None,
          param: Option::None}), // 0xA7
    Some(Instruction {inst_type: Operation::XOR,
          addr_mode: AddressMode::RToR,
          reg1: Option::Some(Register::A),
          reg2: Option::Some(Register::B),
          cond: Option::None,
          param: Option::None}), // 0xA8
    Some(Instruction {inst_type: Operation::XOR,
          addr_mode: AddressMode::RToR,
          reg1: Option::Some(Register::A),
          reg2: Option::Some(Register::C),
          cond: Option::None,
          param: Option::None}), // 0xA9
    Some(Instruction {inst_type: Operation::XOR,
          addr_mode: AddressMode::RToR,
          reg1: Option::Some(Register::A),
          reg2: Option::Some(Register::D),
          cond: Option::None,
          param: Option::None}), // 0xAA
    Some(Instruction {inst_type: Operation::XOR,
          addr_mode: AddressMode::RToR,
          reg1: Option::Some(Register::A),
          reg2: Option::Some(Register::E),
          cond: Option::None,
          param: Option::None}), // 0xAB
    Some(Instruction {inst_type: Operation::XOR,
          addr_mode: AddressMode::RToR,
          reg1: Option::Some(Register::A),
          reg2: Option::Some(Register::H),
          cond: Option::None,
          param: Option::None}), // 0xAC
    Some(Instruction {inst_type: Operation::XOR,
          addr_mode: AddressMode::RToR,
          reg1: Option::Some(Register::A),
          reg2: Option::Some(Register::L),
          cond: Option::None,
          param: Option::None}), // 0xAD
    Some(Instruction {inst_type: Operation::XOR,
          addr_mode: AddressMode::RToMr,
          reg1: Option::Some(Register::A),
          reg2: Option::Some(Register::HL),
          cond: Option::None,
          param: Option::None}), // 0xAE
    Some(Instruction {inst_type: Operation::XOR,
          addr_mode: AddressMode::RToR,
          reg1: Option::Some(Register::A),
          reg2: Option::Some(Register::A),
          cond: Option::None,
          param: Option::None}), // 0xAF
          
    Some(Instruction {inst_type: Operation::OR,
          addr_mode: AddressMode::RToR,
          reg1: Option::Some(Register::A),
          reg2: Option::Some(Register::B),
          cond: Option::None,
          param: Option::None}), // 0xB0
    Some(Instruction {inst_type: Operation::OR,
          addr_mode: AddressMode::RToR,
          reg1: Option::Some(Register::A),
          reg2: Option::Some(Register::C),
          cond: Option::None,
          param: Option::None}), // 0xB1
    Some(Instruction {inst_type: Operation::OR,
          addr_mode: AddressMode::RToR,
          reg1: Option::Some(Register::A),
          reg2: Option::Some(Register::D),
          cond: Option::None,
          param: Option::None}), // 0xB2
    Some(Instruction {inst_type: Operation::OR,
          addr_mode: AddressMode::RToR,
          reg1: Option::Some(Register::A),
          reg2: Option::Some(Register::E),
          cond: Option::None,
          param: Option::None}), // 0xB3
    Some(Instruction {inst_type: Operation::OR,
          addr_mode: AddressMode::RToR,
          reg1: Option::Some(Register::A),
          reg2: Option::Some(Register::H),
          cond: Option::None,
          param: Option::None}), // 0xB4
    Some(Instruction {inst_type: Operation::OR,
          addr_mode: AddressMode::RToR,
          reg1: Option::Some(Register::A),
          reg2: Option::Some(Register::L),
          cond: Option::None,
          param: Option::None}), // 0xB5
    Some(Instruction {inst_type: Operation::OR,
          addr_mode: AddressMode::RToMr,
          reg1: Option::Some(Register::A),
          reg2: Option::Some(Register::HL),
          cond: Option::None,
          param: Option::None}), // 0xB6
    Some(Instruction {inst_type: Operation::OR,
          addr_mode: AddressMode::RToR,
          reg1: Option::Some(Register::A),
          reg2: Option::Some(Register::A),
          cond: Option::None,
          param: Option::None}), // 0xB7
    Some(Instruction {inst_type: Operation::CP,
          addr_mode: AddressMode::RToR,
          reg1: Option::Some(Register::A),
          reg2: Option::Some(Register::B),
          cond: Option::None,
          param: Option::None}), // 0xB8
    Some(Instruction {inst_type: Operation::CP,
          addr_mode: AddressMode::RToR,
          reg1: Option::Some(Register::A),
          reg2: Option::Some(Register::C),
          cond: Option::None,
          param: Option::None}), // 0xB9
    Some(Instruction {inst_type: Operation::CP,
          addr_mode: AddressMode::RToR,
          reg1: Option::Some(Register::A),
          reg2: Option::Some(Register::D),
          cond: Option::None,
          param: Option::None}), // 0xBA
    Some(Instruction {inst_type: Operation::CP,
          addr_mode: AddressMode::RToR,
          reg1: Option::Some(Register::A),
          reg2: Option::Some(Register::E),
          cond: Option::None,
          param: Option::None}), // 0xBB
    Some(Instruction {inst_type: Operation::CP,
          addr_mode: AddressMode::RToR,
          reg1: Option::Some(Register::A),
          reg2: Option::Some(Register::H),
          cond: Option::None,
          param: Option::None}), // 0xBC
    Some(Instruction {inst_type: Operation::CP,
          addr_mode: AddressMode::RToR,
          reg1: Option::Some(Register::A),
          reg2: Option::Some(Register::L),
          cond: Option::None,
          param: Option::None}), // 0xBD
    Some(Instruction {inst_type: Operation::CP,
          addr_mode: AddressMode::RToMr,
          reg1: Option::Some(Register::A),
          reg2: Option::Some(Register::HL),
          cond: Option::None,
          param: Option::None}), // 0xBE
    Some(Instruction {inst_type: Operation::CP,
          addr_mode: AddressMode::RToR,
          reg1: Option::Some(Register::A),
          reg2: Option::Some(Register::A),
          cond: Option::None,
          param: Option::None}), // 0xBF
          

    Some(Instruction {inst_type: Operation::RET,
          addr_mode: AddressMode::Imp,
          reg1: Option::None,
          reg2: Option::None,
          cond: Option::Some(Condition::NZ),
          param: Option::None}), // 0xC0
    Some(Instruction {inst_type: Operation::POP,
          addr_mode: AddressMode::R,
          reg1: Option::Some(Register::BC),
          reg2: Option::None,
          cond: Option::None,
          param: Option::None}), // 0xC1
    Some(Instruction {inst_type: Operation::JP,
          addr_mode: AddressMode::D16,
          reg1: Option::None,
          reg2: Option::None,
          cond: Option::Some(Condition::NZ),
          param: Option::None}), // 0xC2
    Some(Instruction {inst_type: Operation::JP,
          addr_mode: AddressMode::D16,
          reg1: Option::None,
          reg2: Option::None,
          cond: Option::None,
          param: Option::None}), // 0xC3
    Some(Instruction {inst_type: Operation::CALL,
          addr_mode: AddressMode::D16,
          reg1: Option::None,
          reg2: Option::None,
          cond: Option::Some(Condition::NZ),
          param: Option::None}), // 0xC4
    Some(Instruction {inst_type: Operation::PUSH,
          addr_mode: AddressMode::R,
          reg1: Option::Some(Register::BC),
          reg2: Option::None,
          cond: Option::None,
          param: Option::None}), // 0xC5
    Some(Instruction {inst_type: Operation::ADD,
          addr_mode: AddressMode::RToD8,
          reg1: Option::Some(Register::A),
          reg2: Option::None,
          cond: Option::None,
          param: Option::None}), // 0xC6
    Some(Instruction {inst_type: Operation::RST,
          addr_mode: AddressMode::Imp,
          reg1: Option::None,
          reg2: Option::None,
          cond: Option::None,
          param: Option::Some(0x00)}), // 0xC7
    Some(Instruction {inst_type: Operation::RET,
          addr_mode: AddressMode::Imp,
          reg1: Option::None,
          reg2: Option::None,
          cond: Option::Some(Condition::Z),
          param: Option::None}), // 0xC8
    Some(Instruction {inst_type: Operation::RET,
          addr_mode: AddressMode::Imp,
          reg1: Option::None,
          reg2: Option::None,
          cond: Option::None,
          param: Option::None}), // 0xC9
    Some(Instruction {inst_type: Operation::JP,
          addr_mode: AddressMode::D8,
          reg1: Option::None,
          reg2: Option::None,
          cond: Option::Some(Condition::Z),
          param: Option::None}), // 0xCA
    Some(Instruction {inst_type: Operation::PREFIX,
          addr_mode: AddressMode::D8,
          reg1: Option::None,
          reg2: Option::None,
          cond: Option::None,
          param: Option::None}), // 0xCB
    Some(Instruction {inst_type: Operation::CALL,
          addr_mode: AddressMode::D16,
          reg1: Option::None,
          reg2: Option::None,
          cond: Option::Some(Condition::Z),
          param: Option::None}), // 0xCC
    Some(Instruction {inst_type: Operation::CALL,
          addr_mode: AddressMode::D16,
          reg1: Option::None,
          reg2: Option::None,
          cond: Option::None,
          param: Option::None}), // 0xCD
    Some(Instruction {inst_type: Operation::ADC,
          addr_mode: AddressMode::RToD8,
          reg1: Option::Some(Register::A),
          reg2: Option::None,
          cond: Option::None,
          param: Option::None}), // 0xCE
    Some(Instruction {inst_type: Operation::RST,
          addr_mode: AddressMode::Imp,
          reg1: Option::None,
          reg2: Option::None,
          cond: Option::None,
          param: Option::Some(0x08)}), // 0xCF
          
    Some(Instruction {inst_type: Operation::RET,
          addr_mode: AddressMode::Imp,
          reg1: Option::None,
          reg2: Option::None,
          cond: Option::Some(Condition::NC),
          param: Option::None}), // 0xD0
    Some(Instruction {inst_type: Operation::POP,
          addr_mode: AddressMode::R,
          reg1: Option::Some(Register::DE),
          reg2: Option::None,
          cond: Option::None,
          param: Option::None}), // 0xD1
    Some(Instruction {inst_type: Operation::JP,
          addr_mode: AddressMode::D16,
          reg1: Option::None,
          reg2: Option::None,
          cond: Option::Some(Condition::NC),
          param: Option::None}), // 0xD2
    Some(Instruction {inst_type: Operation::NOP,
          addr_mode: AddressMode::Imp,
          reg1: Option::None,
          reg2: Option::None,
          cond: Option::None,
          param: Option::None}), // 0xD3
    Some(Instruction {inst_type: Operation::CALL,
          addr_mode: AddressMode::D16,
          reg1: Option::None,
          reg2: Option::None,
          cond: Option::Some(Condition::NC),
          param: Option::None}), // 0xD4
    Some(Instruction {inst_type: Operation::PUSH,
          addr_mode: AddressMode::R,
          reg1: Option::Some(Register::DE),
          reg2: Option::None,
          cond: Option::None,
          param: Option::None}), // 0xD5
    Some(Instruction {inst_type: Operation::SUB,
          addr_mode: AddressMode::RToD8,
          reg1: Option::Some(Register::A),
          reg2: Option::None,
          cond: Option::None,
          param: Option::None}), // 0xD6
    Some(Instruction {inst_type: Operation::RST,
          addr_mode: AddressMode::Imp,
          reg1: Option::None,
          reg2: Option::None,
          cond: Option::None,
          param: Option::Some(0x10)}), // 0xD7
    Some(Instruction {inst_type: Operation::RET,
          addr_mode: AddressMode::Imp,
          reg1: Option::Some(Register::C),
          reg2: Option::None,
          cond: Option::None,
          param: Option::None}), // 0xD8
    Some(Instruction {inst_type: Operation::RETI,
          addr_mode: AddressMode::Imp,
          reg1: Option::None,
          reg2: Option::None,
          cond: Option::None,
          param: Option::None}), // 0xD9
    Some(Instruction {inst_type: Operation::JP,
          addr_mode: AddressMode::RToD8,
          reg1: Option::Some(Register::C),
          reg2: Option::None,
          cond: Option::None,
          param: Option::None}), // 0xDA
    Some(Instruction {inst_type: Operation::NOP,
          addr_mode: AddressMode::Imp,
          reg1: Option::None,
          reg2: Option::None,
          cond: Option::None,
          param: Option::None}), // 0xDB
    Some(Instruction {inst_type: Operation::CALL,
          addr_mode: AddressMode::RToD16,
          reg1: Option::Some(Register::C),
          reg2: Option::None,
          cond: Option::Some(Condition::Z),
          param: Option::None}), // 0xDC
    Some(Instruction {inst_type: Operation::NOP,
          addr_mode: AddressMode::Imp,
          reg1: Option::None,
          reg2: Option::None,
          cond: Option::None,
          param: Option::None}), // 0xDD
    Some(Instruction {inst_type: Operation::SBC,
          addr_mode: AddressMode::RToD8,
          reg1: Option::Some(Register::A),
          reg2: Option::None,
          cond: Option::None,
          param: Option::None}), // 0xDE
    Some(Instruction {inst_type: Operation::RST,
          addr_mode: AddressMode::Imp,
          reg1: Option::None,
          reg2: Option::None,
          cond: Option::None,
          param: Option::Some(0x18)}), // 0xDF
          
    Some(Instruction {inst_type: Operation::LDH,
          addr_mode: AddressMode::A8ToR,
          reg1: Option::None,
          reg2: Option::Some(Register::A),
          cond: Option::None,
          param: Option::None}), // 0xE0
    Some(Instruction {inst_type: Operation::POP,
          addr_mode: AddressMode::R,
          reg1: Option::Some(Register::HL),
          reg2: Option::None,
          cond: Option::None,
          param: Option::None}), // 0xE1
    Some(Instruction {inst_type: Operation::LDH,
          addr_mode: AddressMode::MrToR,
          reg1: Option::Some(Register::C),
          reg2: Option::Some(Register::A),
          cond: Option::Some(Condition::NC),
          param: Option::None}), // 0xE2
    Some(Instruction {inst_type: Operation::NOP,
          addr_mode: AddressMode::Imp,
          reg1: Option::None,
          reg2: Option::None,
          cond: Option::None,
          param: Option::None}), // 0xE3
    Some(Instruction {inst_type: Operation::NOP,
          addr_mode: AddressMode::Imp,
          reg1: Option::None,
          reg2: Option::None,
          cond: Option::None,
          param: Option::None}), // 0xE4
    Some(Instruction {inst_type: Operation::PUSH,
          addr_mode: AddressMode::R,
          reg1: Option::Some(Register::HL),
          reg2: Option::None,
          cond: Option::None,
          param: Option::None}), // 0xE5
    Some(Instruction {inst_type: Operation::AND,
          addr_mode: AddressMode::RToD8,
          reg1: Option::Some(Register::A),
          reg2: Option::None,
          cond: Option::None,
          param: Option::None}), // 0xE6
    Some(Instruction {inst_type: Operation::RST,
          addr_mode: AddressMode::Imp,
          reg1: Option::None,
          reg2: Option::None,
          cond: Option::None,
          param: Option::Some(0x20)}), // 0xE7
    Some(Instruction {inst_type: Operation::ADD,
          addr_mode: AddressMode::RToD8,
          reg1: Option::Some(Register::SP),
          reg2: Option::None,
          cond: Option::None,
          param: Option::None}), // 0xE8
    Some(Instruction {inst_type: Operation::JP,
          addr_mode: AddressMode::R,
          reg1: Option::Some(Register::HL),
          reg2: Option::None,
          cond: Option::None,
          param: Option::None}), // 0xE9
    Some(Instruction {inst_type: Operation::LD,
          addr_mode: AddressMode::A16ToR,
          reg1: Option::None,
          reg2: Option::Some(Register::A),
          cond: Option::None,
          param: Option::None}), // 0xEA
    Some(Instruction {inst_type: Operation::NOP,
          addr_mode: AddressMode::Imp,
          reg1: Option::None,
          reg2: Option::None,
          cond: Option::None,
          param: Option::None}), // 0xEB
    Some(Instruction {inst_type: Operation::NOP,
          addr_mode: AddressMode::Imp,
          reg1: Option::None,
          reg2: Option::None,
          cond: Option::None,
          param: Option::None}), // 0xEC
    Some(Instruction {inst_type: Operation::NOP,
          addr_mode: AddressMode::Imp,
          reg1: Option::None,
          reg2: Option::None,
          cond: Option::None,
          param: Option::None}), // 0xED
    Some(Instruction {inst_type: Operation::XOR,
          addr_mode: AddressMode::RToD8,
          reg1: Option::Some(Register::A),
          reg2: Option::None,
          cond: Option::None,
          param: Option::None}), // 0xEE
    Some(Instruction {inst_type: Operation::RST,
          addr_mode: AddressMode::Imp,
          reg1: Option::None,
          reg2: Option::None,
          cond: Option::None,
          param: Option::Some(0x28)}), // 0xEF
          
    Some(Instruction {inst_type: Operation::LDH,
          addr_mode: AddressMode::RToA8,
          reg1: Option::Some(Register::A),
          reg2: Option::None,
          cond: Option::None,
          param: Option::None}), // 0xF0
    Some(Instruction {inst_type: Operation::POP,
          addr_mode: AddressMode::R,
          reg1: Option::Some(Register::AF),
          reg2: Option::None,
          cond: Option::None,
          param: Option::None}), // 0xF1
    Some(Instruction {inst_type: Operation::LDH,
          addr_mode: AddressMode::RToMr,
          reg1: Option::Some(Register::A),
          reg2: Option::Some(Register::C),
          cond: Option::Some(Condition::NC),
          param: Option::None}), // 0xF2
    Some(Instruction {inst_type: Operation::DI,
          addr_mode: AddressMode::Imp,
          reg1: Option::None,
          reg2: Option::None,
          cond: Option::None,
          param: Option::None}), // 0xF3
    Some(Instruction {inst_type: Operation::NOP,
          addr_mode: AddressMode::Imp,
          reg1: Option::None,
          reg2: Option::None,
          cond: Option::None,
          param: Option::None}), // 0xF4
    Some(Instruction {inst_type: Operation::PUSH,
          addr_mode: AddressMode::R,
          reg1: Option::Some(Register::AF),
          reg2: Option::None,
          cond: Option::None,
          param: Option::None}), // 0xF5
    Some(Instruction {inst_type: Operation::OR,
          addr_mode: AddressMode::RToD8,
          reg1: Option::Some(Register::A),
          reg2: Option::None,
          cond: Option::None,
          param: Option::None}), // 0xF6
    Some(Instruction {inst_type: Operation::RST,
          addr_mode: AddressMode::Imp,
          reg1: Option::None,
          reg2: Option::None,
          cond: Option::None,
          param: Option::Some(0x30)}), // 0xF7
    Some(Instruction {inst_type: Operation::LD,
          addr_mode: AddressMode::RToSpr, // HLToSpr?
          reg1: Option::Some(Register::HL),
          reg2: Option::Some(Register::SP),
          cond: Option::None,
          param: Option::None}), // 0xF8
    Some(Instruction {inst_type: Operation::LD,
          addr_mode: AddressMode::RToR,
          reg1: Option::Some(Register::SP),
          reg2: Option::Some(Register::HL),
          cond: Option::None,
          param: Option::None}), // 0xF9
    Some(Instruction {inst_type: Operation::LD,
          addr_mode: AddressMode::RToA16,
          reg1: Option::Some(Register::A),
          reg2: Option::None,
          cond: Option::None,
          param: Option::None}), // 0xFA
    Some(Instruction {inst_type: Operation::EI,
          addr_mode: AddressMode::Imp,
          reg1: Option::None,
          reg2: Option::None,
          cond: Option::None,
          param: Option::None}), // 0xFB
    Some(Instruction {inst_type: Operation::NOP,
          addr_mode: AddressMode::Imp,
          reg1: Option::None,
          reg2: Option::None,
          cond: Option::None,
          param: Option::None}), // 0xFC
    Some(Instruction {inst_type: Operation::NOP,
          addr_mode: AddressMode::Imp,
          reg1: Option::None,
          reg2: Option::None,
          cond: Option::None,
          param: Option::None}), // 0xFD
    Some(Instruction {inst_type: Operation::CP,
          addr_mode: AddressMode::RToD8,
          reg1: Option::Some(Register::A),
          reg2: Option::None,
          cond: Option::None,
          param: Option::None}), // 0xFE
    Some(Instruction {inst_type: Operation::RST,
          addr_mode: AddressMode::Imp,
          reg1: Option::None,
          reg2: Option::None,
          cond: Option::None,
          param: Option::Some(0x38)}), // 0xFF
          

    
];
