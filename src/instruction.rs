use crate::{byte};

// #[derive(Clone)]
pub enum AddressMode {
    Imp,
    RToN16,
    N16ToR,
    MrToR,
    R,
    RToN8,
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
    N16,
    N8,
    MrToN8,
    Mr,
}

// #[derive(Clone)]
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
    RAA,
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
    RL,
    RR,
    SLA,
    SRA,
    SWAP,
    SRL,
    BIT,
    RES,
    SET,
}

// #[derive(Clone)]
enum Condition {
    NZ,
    Z,
    NC,
    C,
}

// #[derive(Clone)]
enum Register {
    None,
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

// #[derive(Clone)]
pub struct Instruction {

    inst_type: Operation,
    addr_mode: AddressMode, 
    reg1: Option<Register>,
    reg2: Option<Register>,
    cond: Option<Condition>,
    param: Option<u16>,

}

pub fn opcode_instruction(opcode: byte) -> Option<&'static Instruction>{

    // if matches!(INSTRUCTIONS[(opcode as usize)].inst_type, Operation::NONE) {
    //     return Option::None;
    // } 
    return INSTRUCTIONS[opcode as usize].as_ref();
}


pub static INSTRUCTIONS: [Option<Instruction>; 0x5] = [
    Some(Instruction {inst_type: Operation::NOP,
         addr_mode: AddressMode::Imp,
          reg1: Option::None,
          reg2: Option::None,
          cond: Option::None,
          param: Option::None}), // 0x00

    Some(Instruction {inst_type: Operation::DEC,
         addr_mode: AddressMode::R,
          reg1: Option::Some(Register::B),
          reg2: Option::None,
          cond: Option::None,
          param: Option::None}), // 0x05
          
    Some(Instruction {inst_type: Operation::LD,
         addr_mode: AddressMode::RToN8,
          reg1: Option::Some(Register::C),
          reg2: Option::None,
          cond: Option::None,
          param: Option::None}), // 0x0E
          
    Some(Instruction {inst_type: Operation::XOR,
         addr_mode: AddressMode::R,
          reg1: Option::Some(Register::A),
          reg2: Option::None,
          cond: Option::None,
          param: Option::None}), // 0xAF
          
    Some(Instruction {inst_type: Operation::JP,
         addr_mode: AddressMode::N16,
          reg1: Option::None,
          reg2: Option::None,
          cond: Option::None,
          param: Option::None}), // 0xC3
];


