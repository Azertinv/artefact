use crate::memory::MemorySpace;
use crate::trit::{Trit, FromTrits};
use crate::byte::Byte;
use crate::word::Word;
use crate::operation::Operation;
use crate::register::Registers;
use crate::interrupt::{Interrupt, Result};

use crate::byte_le;

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
    pub mem: [Option<MemorySpace>; 9],
}

impl Cpu {
    pub fn new() -> Cpu {
        Cpu{regs: Registers::new(), mem: [None, None, None, None, None, None, None, None, None]}
    }

    pub fn init_default(&mut self) {
        self.mem[4] = Some(MemorySpace::new()); // general purpose space
        self.mem[5] = Some(MemorySpace::new()); // code space
        self.regs.pc = Word{bytes: [Byte::ZERO, byte_le!(0,0,0,0,0,0,0,1,0)]}; // start in code space
        self.regs.sp = Word{bytes: [Byte::TERN, Byte::ZERO]}; // start in code space
    }

    fn get_space_index(addr: Word) -> usize {
        (i64::from_trits(&addr.bytes[1].trits[9-MemorySpace::INDEX_WIDTH..9]) + 4) as usize
    }

    pub fn get_space_and_offset(&self, addr: Word) -> Result<(&MemorySpace, isize)> {
        let space_index: usize = Cpu::get_space_index(addr);
        let space: &MemorySpace = self.mem[space_index].as_ref().ok_or(Interrupt::SpaceFault)?;
        let offset: i64 = i64::from(addr.bytes[0])
            + 19683 * i64::from_trits(&addr.bytes[1].trits[0..9-MemorySpace::INDEX_WIDTH]);
        Ok((space, offset as isize))
    }

    pub fn get_mut_space_and_offset(&mut self, addr: Word) -> Result<(&mut MemorySpace, isize)> {
        let space_index: usize = Cpu::get_space_index(addr);
        let space: &mut MemorySpace = self.mem[space_index].as_mut().ok_or(Interrupt::SpaceFault)?;
        let offset: i64 = i64::from(addr.bytes[0])
            + 19683 * i64::from_trits(&addr.bytes[1].trits[0..9-MemorySpace::INDEX_WIDTH]);
        Ok((space, offset as isize))
    }

    fn push(&mut self, value: Word) -> Result<()> {
        self.regs.sp = Word::sub(self.regs.sp, Word::TWO, Word::ZERO).0;
        let (s_space, s_offset) = self.get_mut_space_and_offset(self.regs.sp)?;
        s_space.set_word(s_offset, value)?;
        Ok(())
    }

    fn pop(&mut self) -> Result<Word> {
        let (s_space, s_offset) = self.get_mut_space_and_offset(self.regs.sp)?;
        let value = s_space.get_word(s_offset)?;
        self.regs.sp = Word::add(self.regs.sp, Word::TWO, Word::ZERO).0;
        Ok(value)
    }

    pub fn test(&mut self, lhs: Word, rhs: Word) -> Word {
        let mut result: Word = Word::ZERO;
        let tmp: (Word, Word) = Word::sub(lhs, rhs, Word::ZERO);
        let mut diff: Trit = tmp.1.sign();
        if diff == Trit::ZERO {
            diff = tmp.0.sign();
        }
        result.bytes[0].trits[0] = diff;
        result.bytes[0].trits[1] = if diff == Trit::ZERO { Trit::ONE } else { diff };
        result.bytes[0].trits[2] = if diff == Trit::ZERO { Trit::TERN } else { diff };
        result.bytes[0].trits[3] = tmp.1.sign();
        if lhs.sign() != Trit::ZERO {
            if rhs.sign() == Trit::ZERO {
                result.bytes[0].trits[4] = Trit::ONE;
                result.bytes[0].trits[5] = Trit::ONE;
                result.bytes[0].trits[6] = Trit::ONE;
            } else {
                let tmp_fz: (Word, Word) = if lhs.sign() == rhs.sign() {
                    Word::sub(lhs, rhs, Word::ZERO)
                } else {
                    Word::add(lhs, rhs, Word::ZERO)
                };
                let mut diff_fz: Trit = tmp_fz.1.sign();
                if diff_fz == Trit::ZERO {
                    diff_fz = tmp_fz.0.sign();
                }
                result.bytes[0].trits[4] = diff_fz;
                result.bytes[0].trits[5] = if diff_fz == Trit::ZERO { Trit::ONE } else { diff_fz };
                result.bytes[0].trits[6] = if diff_fz == Trit::ZERO { Trit::TERN } else { diff_fz };
            }
        }
        result
    }

    pub fn fetch_decode_execute_one(&mut self) -> Result<()> {
        let (pc_space, pc_offset) = self.get_space_and_offset(self.regs.pc)?;
        let b0: Byte = pc_space.get_byte(pc_offset)?;
        //print!("{}: ", self.regs.pc);
        let saved_pc = self.regs.pc;
        let mut inst_size: usize = 1;
        match b0.trits {
            bt_le_pattern!(0,0,0,0,0,0,0,0,0) => {
                // println!("nop");
            },
            bt_le_pattern!(0,0,0,0,0,0,0,1,1) => { // callabs
                inst_size += Word::BYTE_COUNT;
                let imm: Word = pc_space.get_word(pc_offset + 1)?;
                self.push(Word::add(self.regs.pc, Word::from(inst_size as i64), Word::ZERO).0)?;
                self.regs.pc = imm;
                // println!("callabs {}", imm);
            },
            bt_le_pattern!(0,0,0,0,0,0,0,1,T) => { // callrel
                inst_size += 1;
                let imm: Byte = pc_space.get_byte(pc_offset + 1)?;
                self.push(Word::add(self.regs.pc, Word::from(inst_size as i64), Word::ZERO).0)?;
                self.regs.pc = Word::add(self.regs.pc, Word{bytes: [imm, Byte::ZERO]}, Word::ZERO).0;
                // println!("callrel {}", imm);
            },
            bt_le_pattern!(0,0,0,0,0,1,_,_,_) => { // 1 reg param
                let reg: &[Trit] = &b0.trits[7..9];
                match b0.trits[6] {
                    bt_le_pattern!(0) => { // not reg
                        let value = self.regs.get(reg);
                        self.regs.set_w(reg, Word::not(value));
                        // println!("not {}", self.regs.to_str(reg));
                    },
                    bt_le_pattern!(T) => { // push reg
                        let value = self.regs.get(reg);
                        self.push(value)?;
                        // println!("push {}", self.regs.to_str(reg));
                    },
                    bt_le_pattern!(1) => { // pop reg
                        let value = self.pop()?;
                        self.regs.set_w(reg, value);
                        // println!("pop {}", self.regs.to_str(reg));
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
                    // println!("set.w {}, {}", self.regs.to_str(lhs_reg), rhs_imm);
                } else {
                    inst_size += 1;
                    let rhs_imm: Byte = pc_space.get_byte(pc_offset + 1)?;
                    if slice == Trit::TERN {
                        self.regs.set_b(lhs_reg, rhs_imm);
                        // println!("set.b {}, {}", self.regs.to_str(lhs_reg), rhs_imm);
                    } else {
                        self.regs.set_t(lhs_reg, rhs_imm);
                        // println!("set.t {}, {}", self.regs.to_str(lhs_reg), rhs_imm);
                    }
                }
            },
            bt_le_pattern!(0,0,0,0,1,_,_,_,_) => { // 1 inimm (1), 1 index, 1 imm.b/w
                let index: usize = (i64::from_trits(&b0.trits[7..9]) + 4) as usize;
                let value: Trit = b0.trits[6];
                match b0.trits[5] {
                    bt_le_pattern!(1) => { // cjumpabs
                        inst_size += Word::BYTE_COUNT;
                        let imm: Word = pc_space.get_word(pc_offset + 1)?;
                        if self.regs.flags.bytes[0].trits[index] == value {
                            self.regs.pc = imm;
                        }
                        // println!("cjumpabs {} {} {}", value, index, imm);
                    },
                    bt_le_pattern!(T) => { // cjumprel
                        inst_size += 1;
                        let imm: Byte = pc_space.get_byte(pc_offset + 1)?;
                        if self.regs.flags.bytes[0].trits[index] == value {
                            self.regs.pc = Word::add(self.regs.pc, Word{bytes: [imm, Byte::ZERO]}, Word::ZERO).0;
                        }
                        // println!("cjumprel {} {} {}", value, index, imm);
                    },
                    _ => { return Err(Interrupt::InvalidOpcode); },
                }
            },
            bt_le_pattern!(0,0,T,_,_,_,_,_,_) => { // 2 index, 1 slice
                let lhs_reg: &[Trit] = &b0.trits[7..9];
                let rhs_reg: &[Trit] = &b0.trits[5..7];
                let slice: Trit = b0.trits[4];
                match b0.trits[3] {
                    bt_le_pattern!(T) => { // load
                        let (space, offset) = self.get_space_and_offset(self.regs.get(rhs_reg))?;
                        if slice == Trit::ONE {
                            let value: Word = space.get_word(offset)?;
                            self.regs.set_w(lhs_reg, value);
                            // println!("load {}, [{}]", self.regs.to_str(lhs_reg), self.regs.to_str(rhs_reg));
                        } else {
                            let value: Byte = space.get_byte(offset)?;
                            if slice == Trit::TERN {
                                self.regs.set_b(lhs_reg, value);
                                // println!("load {}.b, [{}]", self.regs.to_str(lhs_reg), self.regs.to_str(rhs_reg));
                            } else {
                                self.regs.set_t(lhs_reg, value);
                                // println!("load {}.t, [{}]", self.regs.to_str(lhs_reg), self.regs.to_str(rhs_reg));
                            }
                        }
                    },
                    bt_le_pattern!(1) => { // store
                        if slice == Trit::ONE {
                            let value: Word = self.regs.get(rhs_reg);
                            let (space, offset) = self.get_mut_space_and_offset(self.regs.get(lhs_reg))?;
                            space.set_word(offset, value)?;
                            // println!("store [{}], {}", self.regs.to_str(lhs_reg), self.regs.to_str(rhs_reg));
                        } else {
                            if slice == Trit::TERN {
                                let value: Byte = self.regs.get(rhs_reg).bytes[0];
                                let (space, offset) = self.get_mut_space_and_offset(self.regs.get(lhs_reg))?;
                                space.set_byte(offset, value)?;
                                // println!("store [{}], {}.b", self.regs.to_str(lhs_reg), self.regs.to_str(rhs_reg));
                            } else {
                                let value: Byte = self.regs.get(rhs_reg).bytes[1];
                                let (space, offset) = self.get_mut_space_and_offset(self.regs.get(lhs_reg))?;
                                space.set_byte(offset, value)?;
                                // println!("store [{}], {}.t", self.regs.to_str(lhs_reg), self.regs.to_str(rhs_reg));
                            }
                        }
                    },
                    _ => { return Err(Interrupt::InvalidOpcode); },
                }
            },
            bt_le_pattern!(0,1,_,_,_,_,_,_,_) => { // 2 index
                let lhs_reg: &[Trit] = &b0.trits[7..9];
                let lhs_value: Word = self.regs.get(lhs_reg);
                let rhs_reg: &[Trit] = &b0.trits[5..7];
                let rhs_value: Word = self.regs.get(rhs_reg);
                match b0.trits[2..5] {
                    bt_le_pattern!(T,T,T) => { // add
                        self.regs.set_w(lhs_reg, Word::add(lhs_value, rhs_value, Word::ZERO).0);
                        // println!("add {} {}", self.regs.to_str(lhs_reg), self.regs.to_str(rhs_reg));
                    },
                    bt_le_pattern!(T,T,1) => { // sub
                        self.regs.set_w(lhs_reg, Word::sub(lhs_value, rhs_value, Word::ZERO).0);
                        // println!("sub {} {}", self.regs.to_str(lhs_reg), self.regs.to_str(rhs_reg));
                    },
                    bt_le_pattern!(T,T,0) => { // mul
                        self.regs.set_w(lhs_reg, Word::mul(lhs_value, rhs_value).0);
                        // println!("mul {} {}", self.regs.to_str(lhs_reg), self.regs.to_str(rhs_reg));
                    },
                    bt_le_pattern!(T,1,T) => { // div
                        self.regs.set_w(lhs_reg, Word::div(lhs_value, rhs_value).0);
                        // println!("div {} {}", self.regs.to_str(lhs_reg), self.regs.to_str(rhs_reg));
                    },
                    bt_le_pattern!(T,1,1) => { // mod
                        self.regs.set_w(lhs_reg, Word::div(lhs_value, rhs_value).1);
                        // println!("mod {} {}", self.regs.to_str(lhs_reg), self.regs.to_str(rhs_reg));
                    },
                    bt_le_pattern!(T,1,0) => { // add_fz
                        let new_value: Word = match lhs_value.sign() {
                            Trit::ZERO => { return Err(Interrupt::AbsOpFromZero); }
                            Trit::ONE => { Word::add(lhs_value, rhs_value, Word::ZERO).0 },
                            Trit::TERN => { Word::sub(lhs_value, rhs_value, Word::ZERO).0 },
                            _ => { return Err(Interrupt::BadCode); }
                        };
                        self.regs.set_w(lhs_reg, new_value);
                        // println!("addfz {} {}", self.regs.to_str(lhs_reg), self.regs.to_str(rhs_reg));
                    },
                    bt_le_pattern!(T,0,T) => { // sub_fz
                        let new_value: Word = match lhs_value.sign() {
                            Trit::ZERO => { return Err(Interrupt::AbsOpFromZero); }
                            Trit::ONE => { Word::sub(lhs_value, rhs_value, Word::ZERO).0 },
                            Trit::TERN => { Word::add(lhs_value, rhs_value, Word::ZERO).0 },
                            _ => { return Err(Interrupt::BadCode); }
                        };
                        self.regs.set_w(lhs_reg, new_value);
                        // println!("subfz {} {}", self.regs.to_str(lhs_reg), self.regs.to_str(rhs_reg));
                    },
                    bt_le_pattern!(T,0,0) => { // test
                        self.regs.flags = self.test(lhs_value, rhs_value);
                        // println!("test {} {}", self.regs.to_str(lhs_reg), self.regs.to_str(rhs_reg));
                    },
                    bt_le_pattern!(1,T,T) => { // and
                        self.regs.set_w(lhs_reg, Word::and(lhs_value, rhs_value));
                        // println!("and {} {}", self.regs.to_str(lhs_reg), self.regs.to_str(rhs_reg));
                    },
                    bt_le_pattern!(1,T,1) => { // or
                        self.regs.set_w(lhs_reg, Word::or(lhs_value, rhs_value));
                        // println!("or {} {}", self.regs.to_str(lhs_reg), self.regs.to_str(rhs_reg));
                    },
                    bt_le_pattern!(1,T,0) => { // xor
                        self.regs.set_w(lhs_reg, Word::xor(lhs_value, rhs_value));
                        // println!("xor {} {}", self.regs.to_str(lhs_reg), self.regs.to_str(rhs_reg));
                    },
                    bt_le_pattern!(1,0,0) => { // mov
                        self.regs.set_w(lhs_reg, rhs_value);
                        // println!("mov {} {}", self.regs.to_str(lhs_reg), self.regs.to_str(rhs_reg));
                    },
                    _ => { return Err(Interrupt::InvalidOpcode); },
                }
            },
            _ => { // 1 index, 1 inimm (4)
                let lhs_reg: &[Trit] = &b0.trits[7..9];
                let lhs_value: Word = self.regs.get(lhs_reg);
                let rhs_value: Word = Word::from_trits(&b0.trits[3..7]);
                match b0.trits[0..3] {
                    bt_le_pattern!(T,T,T) => { // add
                        self.regs.set_w(lhs_reg, Word::add(lhs_value, rhs_value, Word::ZERO).0);
                        // println!("add {} {}", self.regs.to_str(lhs_reg), rhs_value);
                    },
                    bt_le_pattern!(T,T,1) => { // sub
                        self.regs.set_w(lhs_reg, Word::sub(lhs_value, rhs_value, Word::ZERO).0);
                        // println!("sub {} {}", self.regs.to_str(lhs_reg), rhs_value);
                    },
                    bt_le_pattern!(T,T,0) => { // mul
                        self.regs.set_w(lhs_reg, Word::mul(lhs_value, rhs_value).0);
                        // println!("mul {} {}", self.regs.to_str(lhs_reg), rhs_value);
                    },
                    bt_le_pattern!(T,1,T) => { // div
                        self.regs.set_w(lhs_reg, Word::div(lhs_value, rhs_value).0);
                        // println!("div {} {}", self.regs.to_str(lhs_reg), rhs_value);
                    },
                    bt_le_pattern!(T,1,1) => { // mod
                        self.regs.set_w(lhs_reg, Word::div(lhs_value, rhs_value).1);
                        // println!("mod {} {}", self.regs.to_str(lhs_reg), rhs_value);
                    },
                    bt_le_pattern!(T,1,0) => { // add_fz
                        let new_value: Word = match lhs_value.sign() {
                            Trit::ZERO => { return Err(Interrupt::AbsOpFromZero); }
                            Trit::ONE => { Word::add(lhs_value, rhs_value, Word::ZERO).0 },
                            Trit::TERN => { Word::sub(lhs_value, rhs_value, Word::ZERO).0 },
                            _ => { return Err(Interrupt::BadCode); }
                        };
                        self.regs.set_w(lhs_reg, new_value);
                        // println!("addfz {} {}", self.regs.to_str(lhs_reg), rhs_value);
                    },
                    bt_le_pattern!(T,0,T) => { // sub_fz
                        let new_value: Word = match lhs_value.sign() {
                            Trit::ZERO => { return Err(Interrupt::AbsOpFromZero); }
                            Trit::ONE => { Word::sub(lhs_value, rhs_value, Word::ZERO).0 },
                            Trit::TERN => { Word::add(lhs_value, rhs_value, Word::ZERO).0 },
                            _ => { return Err(Interrupt::BadCode); }
                        };
                        self.regs.set_w(lhs_reg, new_value);
                        // println!("subfz {} {}", self.regs.to_str(lhs_reg), rhs_value);
                    },
                    bt_le_pattern!(T,0,0) => { // test
                        self.regs.flags = self.test(lhs_value, rhs_value);
                        // println!("test {} {}", self.regs.to_str(lhs_reg), rhs_value);
                    },
                    bt_le_pattern!(1,T,0) => { // shift
                        self.regs.set_w(lhs_reg, Word::shift(lhs_value, i64::from(rhs_value) as isize).0);
                        // println!("shift {} {}", self.regs.to_str(lhs_reg), rhs_value);
                    },
                    _ => { return Err(Interrupt::InvalidOpcode); },
                }
            },
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
