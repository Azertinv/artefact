use crate::memory::MemorySpace;
use crate::trit::{Trit, FromTrits};
use crate::byte::Byte;
use crate::word::Word;
use crate::operation::Operation;
use crate::register::Registers;
use crate::interrupt::{Interrupt, Result};

use crate::{byte_le};

use std::collections::HashMap;

macro_rules! bt_le_pattern {
    ( _ ) => { _ };
    ( 0 ) => { Trit::ZERO };
    ( 1 ) => { Trit::ONE };
    ( T ) => { Trit::TERN };
    ( $($trit:tt),+ ) => { [$(bt_le_pattern!($trit),)*] };
    ( $i:ident ) => { $i };
}

pub struct Cpu {
    pub regs: Registers,
    pub mem: HashMap<i64, MemorySpace>,
}

impl Cpu {
    fn new() -> Cpu {
        Cpu{regs: Registers::new(), mem: HashMap::new()}
    }

    fn init_default(&mut self) {
        self.mem.insert(0, MemorySpace::new()); // general purpose space
        self.mem.insert(1, MemorySpace::new()); // code space
        self.regs.pc = Word{bytes: [Byte::ZERO, byte_le!(0,0,0,0,0,0,0,1,0)]}; // start in code space
        self.regs.sp = Word{bytes: [Byte::TERN, Byte::ZERO]}; // start in code space
    }

    fn get_space_index(addr: Word) -> i64 {
        i64::from_trits(&addr.bytes[1].trits[9-MemorySpace::INDEX_WIDTH..9])
    }

    fn get_space_and_offset(&self, addr: Word) -> Result<(&MemorySpace, isize)> {
        let space_index: i64 = Cpu::get_space_index(addr);
        let space: &MemorySpace = self.mem.get(&space_index).ok_or(Interrupt::SpaceFault)?;
        let offset: i64 = i64::from(addr.bytes[0]) + 2187 * i64::from(addr.bytes[1]);
        Ok((space, offset as isize))
    }

    fn get_mut_space_and_offset(&mut self, addr: Word) -> Result<(&mut MemorySpace, isize)> {
        let space_index: i64 = Cpu::get_space_index(addr);
        let space: &mut MemorySpace = self.mem.get_mut(&space_index).ok_or(Interrupt::SpaceFault)?;
        let offset: i64 = i64::from(addr.bytes[0]) + 2187 * i64::from(addr.bytes[1]);
        Ok((space, offset as isize))
    }

    fn fetch_decode_execute_one(&mut self) -> Result<()> {
        let (pc_space, pc_offset) = self.get_space_and_offset(self.regs.pc)?;
        let b0: Byte = pc_space.get_byte(pc_offset)?;
        print!("{}: ", self.regs.pc);
        let saved_pc = self.regs.pc;
        let mut inst_size: usize = 1;
        match b0.trits {
            bt_le_pattern!(0,0,0,0,0,0,0,0,_) => { // no param
                match b0.trits {
                    bt_le_pattern!(_,_,_,_,_,_,_,_,0) => {
                        println!("nop");
                    },
                    _ => { return Err(Interrupt::InvalidOpcode); },
                }
            },
            bt_le_pattern!(0,0,0,0,0,1,_,_,_) => { // 1 reg param
                let reg: &[Trit] = &b0.trits[7..9];
                match b0.trits {
                    bt_le_pattern!(_,_,_,_,_,_,0,_,_) => { // not reg
                        let value = self.regs.get(reg);
                        self.regs.set_w(reg, Word::not(value));
                        println!("not {}", self.regs.to_str(reg));
                    },
                    bt_le_pattern!(_,_,_,_,_,_,T,_,_) => { // push reg
                        let value = self.regs.get(reg);
                        self.regs.sp = Word::add(self.regs.sp, Word::from(-3i64), Word::ZERO).0;
                        let (s_space, s_offset) = self.get_mut_space_and_offset(self.regs.sp)?;
                        s_space.set_word(s_offset, value)?;
                        println!("push {}", self.regs.to_str(reg));
                    },
                    bt_le_pattern!(_,_,_,_,_,_,1,_,_) => { // pop reg
                        let (s_space, s_offset) = self.get_mut_space_and_offset(self.regs.sp)?;
                        let value = s_space.get_word(s_offset)?;
                        self.regs.set_w(reg, value);
                        self.regs.sp = Word::add(self.regs.sp, Word::from(3i64), Word::ZERO).0;
                        println!("pop {}", self.regs.to_str(reg));
                    },
                    _ => { return Err(Interrupt::InvalidOpcode); },
                }
            }
            bt_le_pattern!(0,0,0,0,0,T,_,_,_) => { // 1 imm.w/b param, 1 reg param
                let lhs_reg: &[Trit] = &b0.trits[7..9];
                match b0.trits {
                    bt_le_pattern!(_,_,_,_,_,_,1,_,_) => { // set.w reg, imm.w
                        inst_size += Word::BYTE_COUNT;
                        let rhs_imm: Word = pc_space.get_word(pc_offset + 1)?;
                        self.regs.set_w(lhs_reg, rhs_imm);
                        println!("set.w {}, {}", self.regs.to_str(lhs_reg), rhs_imm);
                    },
                    bt_le_pattern!(_,_,_,_,_,_,T,_,_) => { // set.b reg, imm.b
                        inst_size += 1;
                        let rhs_imm: Byte = pc_space.get_byte(pc_offset + 1)?;
                        self.regs.set_b(lhs_reg, rhs_imm);
                        println!("set.b {}, {}", self.regs.to_str(lhs_reg), rhs_imm);
                    },
                    _ => { return Err(Interrupt::InvalidOpcode); },
                }
            },
            _ => { return Err(Interrupt::InvalidOpcode); },
        }
        if saved_pc == self.regs.pc {
            self.regs.pc = Word::add(self.regs.pc, Word::from(inst_size as i64), Word::ZERO).0;
        }
        Ok(())
    }

    fn run(&mut self, inst_count: usize) {
        for _ in 0..inst_count {
            if let Err(interrupt) = self.fetch_decode_execute_one() {
                println!("unhandled interrupt: {:?}", interrupt);
                println!("{}", self);
            }
        }
    }
}

impl std::fmt::Display for Cpu {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}\n", self.regs)
    }
}

#[test]
fn test_init() {
    let mut cpu = Cpu::new();
    cpu.init_default();
    let (pc_space, pc_offset) = cpu.get_mut_space_and_offset(cpu.regs.pc).unwrap();
    pc_space.set_byte(pc_offset+0, byte_le!(0,0,0,0,0,T,1,0,0));
    pc_space.set_byte(pc_offset+1, byte_le!(0,0,T,0,1,T,1,0,1));
    pc_space.set_byte(pc_offset+2, byte_le!(0,0,T,0,1,T,1,0,1));
    pc_space.set_byte(pc_offset+3, byte_le!(0,0,0,0,0,0,0,0,0));
    pc_space.set_byte(pc_offset+4, byte_le!(0,0,0,0,0,T,T,0,0));
    pc_space.set_byte(pc_offset+5, byte_le!(0,0,T,T,T,0,0,0,0));
    pc_space.set_byte(pc_offset+6, byte_le!(0,0,0,0,0,1,0,0,0));
    pc_space.set_byte(pc_offset+7, byte_le!(0,0,0,0,0,1,T,0,0));
    pc_space.set_byte(pc_offset+8, byte_le!(0,0,0,0,0,1,1,0,1));
    cpu.run(6);
    println!("{}", cpu);
    panic!();
}
