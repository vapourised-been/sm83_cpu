use crate::byte;
use crate::mmu;

struct reg_8 {
    b: byte ,
    c: byte ,
    d: byte ,
    e: byte ,
    h: byte ,
    l: byte ,
    a: byte ,
    f: byte ,

}
    // sp: word_register,
    // pc: word_register,
    
    // ie: byte, // interrupt enable
    // ir: byte, // instruction Register


struct word_register {
    low: byte,
    high: byte,

}

struct reg_pair {
    low: reg_8,
    high: reg_8,

}

enum cond {
    nz,
    z,
    nc,
    c,
}

enum size_flag {
    Imm8(byte),
    Imm16(u16),
}

// TODO opcode struct/enum


struct instruction {

    instruction_type: Option<type>,
    addr_mode: address_mode, // TODO make addr type from instruction set

    reg1: Option<reg_type>,
    reg2: Option<reg_type>,
    // instr_size: Option<size_flag>,
    // dest_addr:,
    // mnemonic: Option<OPCODE>,
    cond_type: Option<cond>,
    param: byte,

}

// TODO cpu_context


