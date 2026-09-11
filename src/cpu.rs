use crate::byte;
use crate::instruction;
use crate::mmu;
// use crate::mmu::bus_read;
use crate::instruction::opcode_instruction;
use crate::instruction::AddressMode;
use crate::instruction::Instruction;

struct CpuReg {
    b: byte ,
    c: byte ,
    d: byte ,
    e: byte ,
    h: byte ,
    l: byte ,
    a: byte ,
    f: byte ,
    sp: WordRegister,
    pc: WordRegister,
}

    // ie: byte, // interrupt enable
    // ir: byte, // instruction Register


struct WordRegister {
    low: byte,
    high: byte,

}



enum size_flag {
    Imm8(byte),
    Imm16(u16),
}


// TODO cpu_context


struct CpuContext{
    regs: CpuReg,
    fetch: u16,
    mem_dest: u16,
    current_opcode: byte,
    current_instr: instruction::Instruction,
    halted: bool, // status can be joined to one no? 
    stepping: bool,
}

impl CpuContext {
    fn fetch_instruction(&self){
        self.current_opcode = mmu::bus_read(&self, addr)

    }


}
