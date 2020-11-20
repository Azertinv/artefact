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
    pub fn new() -> Cpu {
        Cpu{regs: Registers::new(), mem: HashMap::new()}
    }

    pub fn init_default(&mut self) {
        self.mem.insert(0, MemorySpace::new()); // general purpose space
        self.mem.insert(1, MemorySpace::new()); // code space
        self.regs.pc = Word{bytes: [Byte::ZERO, byte_le!(0,0,0,0,0,0,0,1,0)]}; // start in code space
        self.regs.sp = Word{bytes: [Byte::TERN, Byte::ZERO]}; // start in code space
    }

    fn get_space_index(addr: Word) -> i64 {
        i64::from_trits(&addr.bytes[1].trits[9-MemorySpace::INDEX_WIDTH..9])
    }

    pub fn get_space_and_offset(&self, addr: Word) -> Result<(&MemorySpace, isize)> {
        let space_index: i64 = Cpu::get_space_index(addr);
        let space: &MemorySpace = self.mem.get(&space_index).ok_or(Interrupt::SpaceFault)?;
        let offset: i64 = i64::from(addr.bytes[0]) + 2187 * i64::from(addr.bytes[1]);
        Ok((space, offset as isize))
    }

    pub fn get_mut_space_and_offset(&mut self, addr: Word) -> Result<(&mut MemorySpace, isize)> {
        let space_index: i64 = Cpu::get_space_index(addr);
        let space: &mut MemorySpace = self.mem.get_mut(&space_index).ok_or(Interrupt::SpaceFault)?;
        let offset: i64 = i64::from(addr.bytes[0]) + 2187 * i64::from(addr.bytes[1]);
        Ok((space, offset as isize))
    }

    fn push(&mut self, value: Word) -> Result<()> {
        self.regs.sp = Word::sub(self.regs.sp, Word::from(2), Word::ZERO).0;
        let (s_space, s_offset) = self.get_mut_space_and_offset(self.regs.sp)?;
        s_space.set_word(s_offset, value)?;
        Ok(())
    }

    fn pop(&mut self) -> Result<Word> {
        let (s_space, s_offset) = self.get_mut_space_and_offset(self.regs.sp)?;
        let value = s_space.get_word(s_offset)?;
        self.regs.sp = Word::add(self.regs.sp, Word::from(2), Word::ZERO).0;
        Ok(value)
    }

    pub fn fetch_decode_execute_one(&mut self) -> Result<()> {
        let (pc_space, pc_offset) = self.get_space_and_offset(self.regs.pc)?;
        let b0: Byte = pc_space.get_byte(pc_offset)?;
        print!("{}: ", self.regs.pc);
        let saved_pc = self.regs.pc;
        let mut inst_size: usize = 1;
        match b0.trits {
            bt_le_pattern!(0,0,0,0,0,0,0,0,0) => {
                println!("nop");
            },
            bt_le_pattern!(0,0,0,0,0,0,0,1,1) => { // callabs
                inst_size += Word::BYTE_COUNT;
                let imm: Word = pc_space.get_word(pc_offset + 1)?;
                self.push(Word::add(self.regs.pc, Word::from(inst_size as i64), Word::ZERO).0)?;
                self.regs.pc = imm;
                println!("callabs {}", imm);
            },
            bt_le_pattern!(0,0,0,0,0,0,0,1,T) => { // callrel
                inst_size += 1;
                let imm: Byte = pc_space.get_byte(pc_offset + 1)?;
                self.push(Word::add(self.regs.pc, Word::from(inst_size as i64), Word::ZERO).0)?;
                self.regs.pc = Word::add(self.regs.pc, Word{bytes: [imm, Byte::ZERO]}, Word::ZERO).0;
                println!("callrel {}", imm);
            },
            bt_le_pattern!(0,0,0,0,0,1,_,_,_) => { // 1 reg param
                let reg: &[Trit] = &b0.trits[7..9];
                match b0.trits {
                    bt_le_pattern!(0,0,0,0,0,1,0,_,_) => { // not reg
                        let value = self.regs.get(reg);
                        self.regs.set_w(reg, Word::not(value));
                        println!("not {}", self.regs.to_str(reg));
                    },
                    bt_le_pattern!(0,0,0,0,0,1,T,_,_) => { // push reg
                        let value = self.regs.get(reg);
                        self.push(value)?;
                        println!("push {}", self.regs.to_str(reg));
                    },
                    bt_le_pattern!(0,0,0,0,0,1,1,_,_) => { // pop reg
                        let value = self.pop()?;
                        self.regs.set_w(reg, value);
                        println!("pop {}", self.regs.to_str(reg));
                    },
                    _ => { return Err(Interrupt::InvalidOpcode); },
                }
            }
            bt_le_pattern!(0,0,0,0,T,0,_,_,_) => { // set reg.t/b/w, imm.b/w
                let lhs_reg: &[Trit] = &b0.trits[7..9];
                let slice: Trit = b0.trits[6];
                if slice == Trit::ONE {
                    inst_size += Word::BYTE_COUNT;
                    let rhs_imm: Word = pc_space.get_word(pc_offset + 1)?;
                    self.regs.set_w(lhs_reg, rhs_imm);
                    println!("set.w {}, {}", self.regs.to_str(lhs_reg), rhs_imm);
                } else {
                    inst_size += 1;
                    let rhs_imm: Byte = pc_space.get_byte(pc_offset + 1)?;
                    if slice == Trit::TERN {
                        self.regs.set_b(lhs_reg, rhs_imm);
                        println!("set.b {}, {}", self.regs.to_str(lhs_reg), rhs_imm);
                    } else {
                        self.regs.set_t(lhs_reg, rhs_imm);
                        println!("set.t {}, {}", self.regs.to_str(lhs_reg), rhs_imm);
                    }
                }
            },
            bt_le_pattern!(0,0,0,0,1,_,_,_,_) => { // 1 inimm (1), 1 index, 1 imm.b/w
                let index: usize = (i64::from_trits(&b0.trits[7..9]) + 4) as usize;
                let value: Trit = b0.trits[6];
                match b0.trits {
                    bt_le_pattern!(0,0,0,0,1,1,_,_,_) => { // cjumpabs
                        inst_size += Word::BYTE_COUNT;
                        let imm: Word = pc_space.get_word(pc_offset + 1)?;
                        if self.regs.flags.bytes[0].trits[index] == value {
                            self.regs.pc = imm;
                        }
                        println!("cjumpabs {} {} {}", value, index, imm);
                    },
                    bt_le_pattern!(0,0,0,0,1,T,_,_,_) => { // cjumprel
                        inst_size += 1;
                        let imm: Byte = pc_space.get_byte(pc_offset + 1)?;
                        if self.regs.flags.bytes[0].trits[index] == value {
                            self.regs.pc = Word::add(self.regs.pc, Word{bytes: [imm, Byte::ZERO]}, Word::ZERO).0;
                        }
                        println!("cjumprel {} {} {}", value, index, imm);
                    },
                    _ => { return Err(Interrupt::InvalidOpcode); },
                }
            },
            bt_le_pattern!(0,0,T,_,_,_,_,_,_) => { // 2 index, 1 slice
                unimplemented!();
            },
            bt_le_pattern!(0,1,_,_,_,_,_,_,_) => { // 2 index
                let lhs_reg: &[Trit] = &b0.trits[7..9];
                let lhs_value: Word = self.regs.get(lhs_reg);
                let rhs_value: Word = self.regs.get(&b0.trits[5..7]);
                match b0.trits {
                    bt_le_pattern!(0,1,T,T,T,_,_,_,_) => { // add
                        self.regs.set_w(lhs_reg, Word::add(lhs_value, rhs_value, Word::ZERO).0)
                    },
                    bt_le_pattern!(0,1,T,T,1,_,_,_,_) => { // sub
                        self.regs.set_w(lhs_reg, Word::sub(lhs_value, rhs_value, Word::ZERO).0)
                    },
                    bt_le_pattern!(0,1,T,T,0,_,_,_,_) => { // mul
                        self.regs.set_w(lhs_reg, Word::mul(lhs_value, rhs_value).0)
                    },
                    bt_le_pattern!(0,1,T,1,T,_,_,_,_) => { // div
                        self.regs.set_w(lhs_reg, Word::div(lhs_value, rhs_value).0)
                    },
                    bt_le_pattern!(0,1,T,1,1,_,_,_,_) => { // mod
                        self.regs.set_w(lhs_reg, Word::div(lhs_value, rhs_value).1)
                    },
                    bt_le_pattern!(0,1,T,1,0,_,_,_,_) => { // add_fz
                        unimplemented!();
                    },
                    bt_le_pattern!(0,1,T,0,T,_,_,_,_) => { // sub_fz
                        unimplemented!();
                    },
                    bt_le_pattern!(0,1,T,0,1,_,_,_,_) => { // test
                        unimplemented!();
                    },
                    bt_le_pattern!(0,1,1,T,T,_,_,_,_) => { // and
                        self.regs.set_w(lhs_reg, Word::and(lhs_value, rhs_value))
                    },
                    bt_le_pattern!(0,1,1,T,1,_,_,_,_) => { // or
                        self.regs.set_w(lhs_reg, Word::or(lhs_value, rhs_value))
                    },
                    bt_le_pattern!(0,1,1,T,0,_,_,_,_) => { // xor
                        self.regs.set_w(lhs_reg, Word::xor(lhs_value, rhs_value))
                    },
                    bt_le_pattern!(0,1,1,0,0,_,_,_,_) => { // mov
                        self.regs.set_w(lhs_reg, rhs_value)
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

    pub fn run(&mut self, inst_count: usize) {
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
