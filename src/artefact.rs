use crate::byte::Byte;
use crate::word::Word;
use crate::cpu::Cpu;
use crate::byte_le;

use gdnative::prelude::*;

#[derive(NativeClass)]
#[inherit(Node)]
pub struct Artefact {
    cpu: Cpu,
}

type GodotTrits = Int32Array;
type GodotBytes = Int32Array;

fn word_to_godot_trits(value: Word) -> GodotTrits {
    let mut result = GodotTrits::new();
    for i in 0..Word::WIDTH {
        result.push(value.bytes[i/Byte::WIDTH].trits[i%Byte::WIDTH].val.into());
    }
    result
}

#[methods]
impl Artefact {
    fn new(_owner: &Node) -> Self {
        let mut cpu: Cpu = Cpu::new();
        cpu.init_default();
        let (pc_space, pc_offset) = cpu.get_mut_space_and_offset(cpu.regs.pc).unwrap();
        let shellcode = [
            byte_le!(T,T,T,1,0,0,0,0,0), // add b, 1
            byte_le!(T,T,1,1,0,0,0,T,T), // sub pc, 1
        ];
        for (i, b) in shellcode.iter().enumerate() {
            pc_space.set_byte(pc_offset+(i as isize), *b).unwrap();
        }
        Artefact{cpu}
    }

    #[export]
    fn _ready(&self, _owner: &Node) {
        godot_print!("Artefact Initialized");
    }

    #[export]
    fn run(&mut self, _owner: &Node, i: usize) {
        self.cpu.run(i);
    }

    #[export]
    fn get_mem_chunk(&self, _owner: &Node, addr_value: i64, size: i64) -> GodotBytes {
        let mut result = GodotBytes::new();
        let addr: Word = Word::from(addr_value);
        if let Ok((space, offset)) = self.cpu.get_space_and_offset(addr) {
            for i in 0..size {
                if let Ok(byte) = space.get_byte(offset + i as isize) {
                    result.push(i64::from(byte) as i32);
                } else {
                    break;
                }
            }
        }
        result
    }

    #[export]
    fn get_reg_trits(&self, _owner: &Node, index: i64) -> GodotTrits {
        let value: Word = match index {
            0 => { self.cpu.regs.pc },
            1 => { self.cpu.regs.sp },
            2 => { self.cpu.regs.flags },
            3 => { self.cpu.regs.a },
            4 => { self.cpu.regs.b },
            5 => { self.cpu.regs.c },
            6 => { self.cpu.regs.d },
            7 => { self.cpu.regs.e },
            8 => { self.cpu.regs.f },
            _ => {
                godot_error!("Bad register index in get_reg_trits");
                Word::ZERO
            },
        };
        word_to_godot_trits(value)
    }

    #[export]
    fn get_reg_value(&self, _owner: &Node, index: i64) -> i64 {
        let value: Word = match index {
            0 => { self.cpu.regs.pc },
            1 => { self.cpu.regs.sp },
            2 => { self.cpu.regs.flags },
            3 => { self.cpu.regs.a },
            4 => { self.cpu.regs.b },
            5 => { self.cpu.regs.c },
            6 => { self.cpu.regs.d },
            7 => { self.cpu.regs.e },
            8 => { self.cpu.regs.f },
            _ => {
                godot_error!("Bad register index in get_reg_value");
                Word::ZERO
            },
        };
        i64::from(value)
    }

    #[export]
    fn set_reg_trits(&mut self, _owner: &Node, index: i64, trits: GodotTrits) {
        let value: &mut Word = match index {
            0 => { &mut self.cpu.regs.pc },
            1 => { &mut self.cpu.regs.sp },
            2 => { &mut self.cpu.regs.flags },
            3 => { &mut self.cpu.regs.a },
            4 => { &mut self.cpu.regs.b },
            5 => { &mut self.cpu.regs.c },
            6 => { &mut self.cpu.regs.d },
            7 => { &mut self.cpu.regs.e },
            8 => { &mut self.cpu.regs.f },
            _ => {
                godot_error!("Bad register index in set_reg_trits");
                return ;
            },
        };
        for i in 0..(trits.len() as usize) {
            (*value).bytes[i/Byte::WIDTH].trits[i%Byte::WIDTH].val = trits.get(i as i32) as i8;
        }
    }
}
