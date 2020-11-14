use crate::memory::MemorySpace;
use crate::register::Registers;
// use crate::interrupt::Interrupt;

use std::collections::HashMap;

pub struct Cpu {
    regs: Registers,
    mem: HashMap<i64, MemorySpace>,
}


impl Cpu {
    fn new() -> Cpu {
        Cpu{regs: Registers::new(), mem: HashMap::new()}
    }

    fn init(&mut self) {
        self.mem.insert(0, MemorySpace::new()); // code space
        self.mem.insert(1, MemorySpace::new()); // general purpose space
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
    cpu.init();

    println!("{}", cpu);
    panic!();
}

