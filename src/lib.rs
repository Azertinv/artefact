pub mod trit;
pub mod byte;
pub mod word;
pub mod operation;
pub mod calculator;
pub mod artefact;
pub mod cpu;
pub mod cpu_test;
pub mod memory;
pub mod interrupt;
pub mod register;
pub mod program;

pub use trit::Trit;
pub use byte::Byte;
pub use word::Word;
pub use operation::Operation;
pub use cpu::Cpu;

pub use calculator::Calculator;
pub use artefact::Artefact;

use gdnative::prelude::*;

fn init(handle: InitHandle) {
    handle.add_class::<Artefact>();
    handle.add_class::<Calculator>();
}

godot_init!(init);
