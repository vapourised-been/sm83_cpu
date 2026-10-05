use std::ops::Add;

use crate::cpu::{self, PROC, WordRegister};
use crate::cpu_process;
use crate::emu::{self, emu_cycles};
use crate::instruction::{self, AddressMode, Instruction, Operation, Register};
use crate::mmu::{self, MemUnit};

// pub fn instr_get_process(cpu_ctx: &PROC, instr_type: instruction::Operation) -> instruction::Operation{

//     return instr_type;
// }

fn process_jp(cpu_ctx: &mut cpu::CpuContext) {
    if cpu::CpuReg::check_cond(&cpu_ctx) {
        cpu_ctx.regs.pc = cpu_ctx.fetch;
        emu::emu_cycles(4);
    } else if !cpu::CpuReg::check_cond(&cpu_ctx) {
        emu_cycles(3);
    }
}

fn process_inc(cpu_ctx: &mut cpu::CpuContext, mem: &mut MemUnit) {
    let mut val: u16 = 0;
    match cpu_ctx
        .regs
        .cpu_regs_read(cpu_ctx.current_instr.reg1.as_ref())
    {
        WordRegister::low(byte_reg) => {
            // INC r8 should be the only valid instruction here, so we simulate only 1 cycle
            val = (byte_reg + 1u8) as u16;
            cpu_ctx
                .regs
                .cpu_set_regs(cpu_ctx.current_instr.reg1.as_ref(), val);
            emu_cycles(1);
            cpu::CpuReg::cpu_set_flags(
                cpu_ctx,
                if val == 0 { 1 } else { 0 },
                0,
                ((val & 0x0F) == 0) as u8,
                0,
            );
        }

        WordRegister::high(byte_reg) => {
            // INC [HL] should be the onl valid instruction here, so we simulate 3 cycles
            if cpu_ctx.current_instr.addr_mode == AddressMode::Mr {
                match cpu_ctx.regs.cpu_regs_read(Some(&Register::HL)) {
                    WordRegister::low(_) => return, // cant return as low
                    WordRegister::high(addr) => {
                        let val_u8 = mem.bus_read(addr) + 1u8;
                        mem.bus_write(addr, val_u8);
                        emu_cycles(3);
                        cpu::CpuReg::cpu_set_flags(
                            cpu_ctx,
                            if val == 0 { 1 } else { 0 },
                            0,
                            ((val & 0x0F) == 0) as u8,
                            0,
                        );
                    }
                }
            } else {
                // INC r16 & INC SP should be the instructions captured here, so we simulate 2 cycles
                val = byte_reg + 1;
                cpu_ctx
                    .regs
                    .cpu_set_regs(cpu_ctx.current_instr.reg1.as_ref(), val);
                emu_cycles(2);
            }
        }
    }
}

fn process_dec(cpu_ctx: &mut cpu::CpuContext, mem: &mut MemUnit) {
    let mut val: u16 = 0;
    match cpu_ctx
        .regs
        .cpu_regs_read(cpu_ctx.current_instr.reg1.as_ref())
    {
        WordRegister::low(byte_reg) => {
            // INC r8 should be the only valid instruction here, so we simulate only 1 cycle
            val = (byte_reg - 1u8) as u16;
            cpu_ctx
                .regs
                .cpu_set_regs(cpu_ctx.current_instr.reg1.as_ref(), val);
            emu_cycles(1);
            cpu::CpuReg::cpu_set_flags(
                cpu_ctx,
                (val == 0) as u8,
                1,
                ((val & 0x0F) == 0x0F) as u8,
                0,
            );
        }

        WordRegister::high(byte_reg) => {
            // INC [HL] should be the onl valid instruction here, so we simulate 3 cycles
            if cpu_ctx.current_instr.addr_mode == AddressMode::Mr {
                match cpu_ctx.regs.cpu_regs_read(Some(&Register::HL)) {
                    WordRegister::low(_) => return, // cant return as low
                    WordRegister::high(addr) => {
                        let val_u8 = mem.bus_read(addr) - 1u8;
                        mem.bus_write(addr, val_u8);
                        emu_cycles(3);
                        cpu::CpuReg::cpu_set_flags(
                            cpu_ctx,
                            (val == 0) as u8,
                            1,
                            ((val & 0x0F) == 0x0F) as u8,
                            0,
                        );
                    }
                }
            } else {
                // INC r16 & INC SP should be the instructions captured here, so we simulate 2 cycles
                val = byte_reg - 1;
                cpu_ctx
                    .regs
                    .cpu_set_regs(cpu_ctx.current_instr.reg1.as_ref(), val);
                emu_cycles(2);
            }
        }
    }
}

fn ptr_address(ctx: &cpu::CpuContext, reg: Option<&Register>) -> u16 {
    match ctx.regs.cpu_regs_read(reg) {
        WordRegister::low(addr) => return addr as u16,
        WordRegister::high(addr) => {
            return addr;
        }
    }
}
fn process_xor(cpu_ctx: &mut cpu::CpuContext) {
    cpu_ctx.regs.a = cpu_ctx.regs.a ^ (cpu_ctx.fetch as u8);
    cpu::CpuReg::cpu_set_flags(cpu_ctx, if cpu_ctx.regs.a == 0 { 1 } else { 0 }, 0, 0, 0);
}

fn process_di(cpu_ctx: &mut cpu::CpuContext) {
    cpu_ctx.in_ime = false;
    // emu_cycles(1);
}

fn process_ei(cpu_ctx: &mut cpu::CpuContext) {
    // emu_cycles(1);
    cpu_ctx.in_ime = true;
}

fn process_ld(cpu_ctx: &mut cpu::CpuContext, mem: &mut MemUnit) {
    if cpu_ctx.dest_mem_flag {
        match cpu_ctx
            .regs
            .cpu_regs_read(cpu_ctx.current_instr.reg2.as_ref())
        {
            WordRegister::low(val) => {
                if cpu_ctx.current_instr.addr_mode == AddressMode::Mr {
                    let address = ptr_address(cpu_ctx, cpu_ctx.current_instr.reg1.as_ref());
                    mem.bus_write(address, val);
                    emu_cycles(1); // need to find out cycles for this 
                } else {
                    cpu_ctx
                        .regs
                        .cpu_set_regs(cpu_ctx.current_instr.reg1.as_ref(), val as u16);
                    emu_cycles(1); // need to find out cycles for this 
                }
            }
            WordRegister::high(val) => {
                if cpu_ctx.current_instr.addr_mode == AddressMode::Mr {


                } else {
                    

                }

            },
            _ => return,
        }
    }
}

fn process_ldh(cpu_ctx: &mut cpu::CpuContext, mem: &mut MemUnit) {
    if cpu_ctx.current_instr.addr_mode == AddressMode::Mr {
        // LDH C..
        let address = ptr_address(cpu_ctx, Some(&Register::C)) + 0xFF00;
        cpu_ctx
            .regs
            .cpu_set_regs(cpu_ctx.current_instr.reg1.as_ref(), address);
        // currently reading directly from regs.a
        // val should be fetch here
    } else {
        // LDH A ..

        mem.bus_write(cpu_ctx.mem_dest, cpu_ctx.fetch as u8);
        // cpu_ctx.dest_mem_flag = false;
    }
    emu_cycles(1);

    // match cpu_ctx
    //     .regs
    //     .cpu_regs_read(cpu_ctx.current_instr.reg2.as_ref())
    // {
    //     WordRegister::low(reg) => {}
    //     _ => return,
    // }
}

fn is_16bit(reg: &Register) -> bool {
    return reg >= &Register::SP;
}

pub fn instr_get_process(op: Operation, ctx: &mut emu::Emulator) {
    match op {
        Operation::NOP => return,
        Operation::JP => cpu_process::process_jp(&mut ctx.cpu),
        Operation::XOR => cpu_process::process_xor(&mut ctx.cpu),
        Operation::INC => cpu_process::process_inc(&mut ctx.cpu, &mut ctx.memory),
        Operation::DEC => cpu_process::process_dec(&mut ctx.cpu, &mut ctx.memory),
        Operation::DI => cpu_process::process_di(&mut ctx.cpu),
        Operation::EI => cpu_process::process_ei(&mut ctx.cpu),
        Operation::LDH => cpu_process::process_ldh(&mut ctx.cpu, &mut ctx.memory),
        Operation::LD => cpu_process::process_ld(&mut ctx.cpu, &mut ctx.memory),
        _ => return,
    }
}
