use crate::{Byte, instruction::{self, Register::*}};


#[derive(Clone)]
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

#[derive(Clone)]
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

#[derive(Clone)]
pub enum Condition {
    NZ,
    Z,
    NC,
    C,
}

#[derive(Clone)]
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

pub static INSTRUCTIONS: [Option<Instruction>; 0x92] = [
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

    Some(Instruction {inst_type: Operation::XOR,
         addr_mode: AddressMode::R,
          reg1: Option::Some(Register::A),
          reg2: Option::None,
          cond: Option::None,
          param: Option::None}), // 0xAF
          
    Some(Instruction {inst_type: Operation::JP,
         addr_mode: AddressMode::D16,
          reg1: Option::None,
          reg2: Option::None,
          cond: Option::None,
          param: Option::None}), // 0xC3
];


