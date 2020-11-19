use crate::cpu::Cpu;
use crate::{byte_le};

macro_rules! bt_le {
    ( 0 ) => { 0 };
    ( 1 ) => { 1 };
    ( T ) => { -1 };
    ( $trit:tt, $($rest:tt),+ ) => { bt_le!($trit) + 3 * bt_le!($($rest),+) };
}

#[test]
fn test_init() {
    let mut cpu = Cpu::new();
    cpu.init_default();
    let (pc_space, pc_offset) = cpu.get_mut_space_and_offset(cpu.regs.pc).unwrap();
    let shellcode = [
        byte_le!(0,0,0,0,T,0,1,0,0),
        byte_le!(0,0,T,0,1,T,1,0,1),
        byte_le!(0,0,T,0,1,T,1,0,1),
        byte_le!(0,0,0,0,0,0,0,0,0),
        byte_le!(0,0,0,0,T,0,T,0,0),
        byte_le!(0,0,T,T,T,0,0,0,0),
        byte_le!(0,0,0,0,0,1,0,0,0),
        byte_le!(0,0,0,0,0,1,T,0,0),
        byte_le!(0,0,0,0,0,1,1,0,1),
    ];
    for (i, b) in shellcode.iter().enumerate() {
        pc_space.set_byte(pc_offset+(i as isize), *b).unwrap();
    }
    println!("{}", cpu);
    cpu.run(6);
    println!("\n{}", cpu);
    panic!();
}

#[test]
fn test_set() {
    let mut cpu = Cpu::new();
    cpu.init_default();
    let (pc_space, pc_offset) = cpu.get_mut_space_and_offset(cpu.regs.pc).unwrap();
    let shellcode = [
        byte_le!(0,0,0,0,T,0,1,0,0), // set reg.w
        byte_le!(0,0,0,0,T,1,1,1,1),
        byte_le!(0,0,0,0,1,T,T,T,T),
        byte_le!(0,0,0,0,T,0,T,0,1), // set reg.b
        byte_le!(0,0,0,0,1,1,T,1,1),
        byte_le!(0,0,0,0,T,0,0,0,1), // set reg.t
        byte_le!(0,0,0,0,T,T,1,T,T),
    ];
    for (i, b) in shellcode.iter().enumerate() {
        pc_space.set_byte(pc_offset+(i as isize), *b).unwrap();
    }
    cpu.run(3);
    println!("{}", cpu);
    assert_eq!(i64::from(cpu.regs.b), bt_le!(0,0,0,0,T,1,1,1,1,0,0,0,0,1,T,T,T,T));
    assert_eq!(i64::from(cpu.regs.c), bt_le!(0,0,0,0,1,1,T,1,1,0,0,0,0,T,T,1,T,T));
}

#[test]
fn test_push_pop() {
    let mut cpu = Cpu::new();
    cpu.init_default();
    let (pc_space, pc_offset) = cpu.get_mut_space_and_offset(cpu.regs.pc).unwrap();
    let shellcode = [
        byte_le!(0,0,0,0,T,0,1,0,0),
        byte_le!(0,0,T,0,1,T,1,0,1),
        byte_le!(0,0,T,0,1,T,1,0,1),
        byte_le!(0,0,0,0,0,1,T,0,0),
        byte_le!(0,0,0,0,0,1,1,0,1),
        byte_le!(0,0,0,0,0,1,T,0,1),
        byte_le!(0,0,0,0,0,1,1,1,T),
        byte_le!(0,0,0,0,0,1,T,1,T),
        byte_le!(0,0,0,0,0,1,1,1,0),
        byte_le!(0,0,0,0,0,1,T,1,T),
        byte_le!(0,0,0,0,0,1,T,1,T),
        byte_le!(0,0,0,0,0,1,T,1,T),
    ];
    for (i, b) in shellcode.iter().enumerate() {
        pc_space.set_byte(pc_offset+(i as isize), *b).unwrap();
    }
    cpu.run(10);
    assert_eq!(cpu.regs.b, cpu.regs.c);
    assert_eq!(cpu.regs.c, cpu.regs.d);
    assert_eq!(cpu.regs.d, cpu.regs.e);
    assert_eq!(i64::from(cpu.regs.sp), -7);
}

#[test]
fn test_not() {
    let mut cpu = Cpu::new();
    cpu.init_default();
    let (pc_space, pc_offset) = cpu.get_mut_space_and_offset(cpu.regs.pc).unwrap();
    let shellcode = [
        byte_le!(0,0,0,0,T,0,T,0,0), // set b.b
        byte_le!(0,0,T,0,1,T,1,0,1),
        byte_le!(0,0,0,0,T,0,T,0,1), // set c.b
        byte_le!(0,0,T,0,1,T,1,0,1),
        byte_le!(0,0,0,0,0,1,0,0,1), // not c
    ];
    for (i, b) in shellcode.iter().enumerate() {
        pc_space.set_byte(pc_offset+(i as isize), *b).unwrap();
    }
    cpu.run(3);
    assert_eq!(i64::from(cpu.regs.b), -i64::from(cpu.regs.c))
}

#[test]
fn test_nop() {
    let mut cpu = Cpu::new();
    cpu.init_default();
    let (pc_space, pc_offset) = cpu.get_mut_space_and_offset(cpu.regs.pc).unwrap();
    let shellcode = [
        byte_le!(0,0,0,0,0,0,0,0,0),
        byte_le!(0,0,0,0,0,0,0,0,0),
        byte_le!(0,0,0,0,0,0,0,0,0),
        byte_le!(0,0,0,0,0,0,0,0,0),
        byte_le!(0,0,0,0,0,0,0,0,0),
    ];
    for (i, b) in shellcode.iter().enumerate() {
        pc_space.set_byte(pc_offset+(i as isize), *b).unwrap();
    }
    let old_pc = cpu.regs.pc;
    cpu.run(5);
    assert_eq!(i64::from(old_pc) + 5, i64::from(cpu.regs.pc))
}
