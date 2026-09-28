use crate::Byte;
use crate::cart;
use crate::cpu;
use crate::cpu::{CpuContext, WordRegister};
use crate::instruction;
use crate::instruction::{
    opcode_instruction, 
    AddressMode, Register};
use crate::cpu_process;
// use crate::instruction::Condition;
// use crate::instruction::Instruction;
use crate::mmu::MemUnit;

pub struct Emulator{
    pub cpu: CpuContext,
    pub memory: MemUnit,
    
}

impl Emulator {
    
    pub fn new(cpu_ctx: CpuContext, mmu_unit: MemUnit) -> Emulator{
        Self{
            cpu: cpu_ctx,
            memory: mmu_unit,
        }

    }
    fn fetch_instruction(&mut self){
        self.cpu.current_opcode = self.memory.bus_read(self.cpu.regs.pc);
        self.cpu.regs.pc += 1;
        self.cpu.current_instr = match opcode_instruction(self.cpu.current_opcode){
            Some(instr) => instr.clone(),
            None => instruction::Instruction::default(), //create void instr
        };
    }   

    fn fetch_data(&mut self){
        self.cpu.mem_dest = 0;
        self.cpu.dest_mem_flag = false;

        match self.cpu.current_instr.addr_mode {
            AddressMode::Imp => return,
            AddressMode::RToD8 => {
                self.cpu.fetch = (self.memory.bus_read(self.cpu.regs.pc)).into();
                emu_cycles(1);
                self.cpu.regs.pc += 1;
            },
            AddressMode::D16 => {
                let low = self.memory.bus_read(self.cpu.regs.pc);
                emu_cycles(1);
                let high= self.memory.bus_read(self.cpu.regs.pc + 1);
                emu_cycles(1);
                // self.cpu.fetch = low as u16;
                self.cpu.fetch = u16::from_le_bytes([low, high]);
                self.cpu.regs.pc += 2;
            },
            AddressMode::R => {
                
                self.cpu.fetch = match (self.cpu.regs.cpu_regs_read(self.cpu.current_instr.reg1.as_ref())).into() {
                    Some(WordRegister::low(word_reg)) => word_reg as u16,
                    Some(WordRegister::high(word_reg)) => word_reg,
                    None => 0x00,
                };
            },
            

            _ => {
                print!("unimplemented Address Mode: {:?}\n", self.cpu.current_instr.addr_mode)
        },
        }
    }

}

pub fn cpu_step(emu: &mut Emulator) -> bool{

    // let mut cpu_ctx = CpuContext::new(cpu_mmu, cartContext);
    if !emu.cpu.halted {
        let pc = emu.cpu.regs.pc;   
        emu.fetch_instruction();
        emu.fetch_data();
        // print!("Instruction Executed: {:#4x}    PC: {:#4x}\n", emu.current_opcode, pc);
        
        print!("{:#4X}: {:?} ({:#02X} {:#2X} {:#2X}) A: {:#2X} B: {:#2X}, C: {:#2X}\n", 
        pc, emu.cpu.current_instr.inst_type,
        emu.cpu.current_opcode, emu.memory.bus_read(pc + 1), emu.memory.bus_read(pc + 2),
        emu.cpu.regs.a, emu.cpu.regs.b, emu.cpu.regs.c);
        cpu::execute(emu);
        return false;

    }
    return true;
}



pub fn emu_cycles(cycles: u32){
    // TODO Sync PPU and CPU
    let _ = cycles;
    // unimplemented!()
}