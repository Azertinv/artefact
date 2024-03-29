use crate::byte::Byte;
use crate::word::Word;
use crate::cpu::Cpu;
use crate::byte_le;
use crate::program::Program;
use crate::memory::MemorySpace;

use gdnative::prelude::*;

#[derive(NativeClass)]
#[inherit(Node)]
pub struct Artefact {
    cpu: Cpu,
    program: Option<Program>,
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
    fn new(owner: &Node) -> Self {
        let mut result = Artefact{cpu: Cpu::new(), program: None};
        result.reset(owner);
        result
    }

    #[export]
    fn _ready(&self, _owner: &Node) {
        godot_print!("Artefact Initialized");
    }

    #[export]
    fn load_program_from_json(&mut self, _owner: &Node, json: String) {
        let program_result = Program::load_from_json(json);
        match program_result {
            Err(err) => {
                godot_print!("Program load failed {:?}", err);
                self.program = None;
            },
            Ok(program) => {
                godot_print!("Program loaded successfully");
                self.program = Some(program);
                self.reset(_owner);
            }
        }
    }

    #[export]
    fn reset(&mut self, _owner: &Node) {
        if let Some(ref program) = self.program {
            self.cpu.mem = [None, None, None, None, None, None, None, None, None];
            self.cpu.regs = program.regs;
            self.cpu.load_data_chunks(&program.data_chunks);
        } else {
            self.cpu.init_default();
            let (pc_space, pc_offset) = self.cpu.get_mut_space_and_offset(self.cpu.regs.pc).unwrap();
            let shellcode = [
                byte_le!(T,T,T,1,0,0,0,0,0), // add b, 1
                byte_le!(T,T,1,1,0,0,0,T,T), // sub pc, 1
            ];
            for (i, b) in shellcode.iter().enumerate() {
                pc_space.set_byte(pc_offset+(i as isize), *b).unwrap();
            }
        }
    }

    #[export]
    fn run(&mut self, _owner: &Node, i: usize) -> isize {
        if let Err(interrupt) = self.cpu.run(i) {
            return interrupt as isize;
        }
        return 0;
    }

    #[export]
    fn add_breakpoint(&mut self, _owner: &Node, addr_value: i64) {
        self.cpu.breakpoints.insert(Word::from(addr_value));
    }

    #[export]
    fn del_breakpoint(&mut self, _owner: &Node, addr_value: i64) {
        self.cpu.breakpoints.remove(&Word::from(addr_value));
    }

    #[export]
    fn get_breakpoints(&mut self, _owner: &Node) -> Int32Array {
        let mut result = Int32Array::new();
        for a in &self.cpu.breakpoints {
            result.push(i64::from(*a) as i32);
        }
        result
    }

    #[export]
    fn get_mem_perm(&self, _owner: &Node, addr_value: i64, size: i64) -> Int32Array {
        let mut result = Int32Array::new();
        if let Some(ref program) = self.program {
            for _ in 0..size {
                result.push(0b000000000);
            }
            let memspace: usize = ((addr_value as isize + MemorySpace::MAX_ADDR) / MemorySpace::SIZE as isize + 4) as usize;
            let addr: isize = addr_value as isize - ((memspace-4) * MemorySpace::SIZE) as isize;
            for pc in &program.perm_chunks {
                if memspace != pc.memspace {
                    continue;
                }
                if addr + size as isize <= pc.addr
                    || pc.addr + pc.permissions.len() as isize <= addr {
                    continue;
                }
                let (mut i, mut pc_i) = if pc.addr < addr {
                    (0, addr - pc.addr)
                } else {
                    (pc.addr - addr, 0)
                };
                while i < size as isize && pc_i < pc.permissions.len() as isize {
                    result.set(i as i32, pc.permissions.get(pc_i as i32));
                    i += 1;
                    pc_i += 1;
                }
            }
        } else {
            for _ in 0..size {
                result.push(0b111111111);
            }
        };
        result
    }

    #[export]
    fn get_reg_perm(&self, _owner: &Node) -> Int32Array {
        let mut result = Int32Array::new();
        result.push(0b111111111111111111); // PC
        result.push(0b111111111111111111); // SP
        result.push(0b111111111111111111); // FLAGS
        result.push(0b111111111111111111); // A
        result.push(0b111111111111111111); // B
        result.push(0b111111111111111111); // C
        result.push(0b111111111111111111); // D
        result.push(0b111111111111111111); // E
        result.push(0b111111111111111111); // F
        result
    }

    #[export]
    fn mem_read(&self, _owner: &Node, addr_value: i64, size: i64) -> GodotBytes {
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
    fn mem_write(&mut self, _owner: &Node, addr_value: i64, data: GodotBytes) {
        let addr: Word = Word::from(addr_value);
        if let Ok((space, offset)) = self.cpu.get_mut_space_and_offset(addr) {
            for i in 0..data.len() {
                if space.set_byte(offset + i as isize, Byte::from(data.get(i) as i64)).is_err() {
                    break;
                }
            }
        }
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
