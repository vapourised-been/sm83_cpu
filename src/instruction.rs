use crate::{Byte, instruction::Register::*};


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

    // if matches!(INSTRUCTIONS[(opcode as usize)].inst_type, Operation::NONE) {
    //     return Option::None;
    // } 
    return INSTRUCTIONS[opcode as usize].as_ref();
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

pub static INSTRUCTIONS: [Option<Instruction>; 0x42] = [
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
    Some(Instruction {inst_type: Operation::DEC,
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
    Some(Instruction {inst_type: Operation::DEC,
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


