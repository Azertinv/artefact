#[cfg(test)]
mod tests {
use crate::cpu::Cpu;
use crate::word::Word;
use crate::{byte_le};
use crate::interrupt::Interrupt;

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
    let shellcode = [];

    for (i, b) in shellcode.iter().enumerate() {
        pc_space.set_byte(pc_offset+(i as isize), *b).unwrap();
    }
    println!("{}", cpu);
    cpu.run(6).unwrap();
    println!("\n{}", cpu);
    // panic!();
}

#[test]
fn test_load() {
    let mut cpu = Cpu::new();
    cpu.init_default();
    let (pc_space, pc_offset) = cpu.get_mut_space_and_offset(cpu.regs.pc).unwrap();
    let shellcode = [
        byte_le!(0,0,0,0,T,0,1,0,0), // set b, 01000000000000001T
        byte_le!(T,1,0,0,0,0,0,0,0),
        byte_le!(0,0,0,0,0,0,0,1,0),
        byte_le!(0,0,T,T,1,0,0,0,1), // load c, [b]
        byte_le!(0,0,T,T,T,0,0,1,T), // load d.b, [b]
        byte_le!(0,0,T,T,0,0,0,1,0), // load e.t, [b]
    ];
    for (i, b) in shellcode.iter().enumerate() {
        pc_space.set_byte(pc_offset+(i as isize), *b).unwrap();
    }
    println!("{}", cpu);
    cpu.run(4).unwrap();
    println!("\n{}", cpu);
    assert_eq!(i64::from(cpu.regs.b), bt_le!(T,1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1,0));
    assert_eq!(i64::from(cpu.regs.c), bt_le!(0,0,0,0,0,0,0,1,0,0,0,T,T,1,0,0,0,1));
    assert_eq!(i64::from(cpu.regs.d), bt_le!(0,0,0,0,0,0,0,1,0));
    assert_eq!(i64::from(cpu.regs.e), bt_le!(0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1,0));
}

#[test]
fn test_halt() {
    let mut cpu = Cpu::new();
    cpu.init_default();
    let (pc_space, pc_offset) = cpu.get_mut_space_and_offset(cpu.regs.pc).unwrap();
    let shellcode = [
        byte_le!(0,0,0,0,0,0,0,0,1), // addfz c, b
    ];
    for (i, b) in shellcode.iter().enumerate() {
        pc_space.set_byte(pc_offset+(i as isize), *b).unwrap();
    }
    assert_eq!(cpu.fetch_decode_execute_one(false), Err(Interrupt::Halted));
}

#[test]
fn test_addsub_fz() {
    let mut cpu = Cpu::new();
    cpu.init_default();
    cpu.regs.b = Word::from(bt_le!(0,T));
    cpu.regs.c = Word::from(bt_le!(1));
    let (pc_space, pc_offset) = cpu.get_mut_space_and_offset(cpu.regs.pc).unwrap();
    let shellcode = [
        byte_le!(0,1,T,1,0,0,0,0,1), // addfz c, b
        byte_le!(0,1,T,0,T,0,0,0,1), // subfz c, b
        byte_le!(0,1,T,1,0,0,0,0,1), // addfz c, b
    ];
    for (i, b) in shellcode.iter().enumerate() {
        pc_space.set_byte(pc_offset+(i as isize), *b).unwrap();
    }
    println!("{}", cpu);
    assert_eq!(cpu.fetch_decode_execute_one(false), Ok(()));
    assert_eq!(cpu.fetch_decode_execute_one(false), Ok(()));
    assert_eq!(i64::from(cpu.regs.c), -5);
    cpu.regs.c = Word::ZERO;
    assert_eq!(cpu.fetch_decode_execute_one(false), Err(Interrupt::AbsOpFromZero));
    println!("\n{}", cpu);
}

#[test]
fn test_cmp() {
    let mut cpu = Cpu::new();
    cpu.init_default();
    cpu.regs.b = Word::from(bt_le!(T));
    cpu.regs.c = Word::from(bt_le!(0));
    cpu.regs.d = Word::from(bt_le!(1));
    cpu.regs.e = Word::from(bt_le!(0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1));
    cpu.regs.f = Word::from(bt_le!(0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,T));
    let (pc_space, pc_offset) = cpu.get_mut_space_and_offset(cpu.regs.pc).unwrap();
    let shellcode = [
        byte_le!(0,1,T,0,0,0,0,0,0), // cmp b b
        byte_le!(0,1,T,0,0,0,1,0,0), // cmp b c
        byte_le!(0,1,T,0,0,1,T,0,0), // cmp b d
        byte_le!(0,1,T,0,0,0,1,0,1), // cmp c c
        byte_le!(0,1,T,0,0,1,T,0,1), // cmp c d
        byte_le!(0,1,T,0,0,1,T,1,T), // cmp c d
        byte_le!(0,1,T,0,0,1,1,1,0), // cmp e f
        byte_le!(0,1,T,0,0,1,0,1,1), // cmp f e
        byte_le!(0,1,T,0,0,1,0,1,0), // cmp e e
        byte_le!(0,1,T,0,0,1,1,1,1), // cmp f f
        byte_le!(T,0,0,1,0,0,0,0,0), // cmp b D1
    ];
    for (i, b) in shellcode.iter().enumerate() {
        pc_space.set_byte(pc_offset+(i as isize), *b).unwrap();
    }
    println!("{}", cpu);
    cpu.run(1).unwrap();
    assert_eq!(i64::from(cpu.regs.flags), bt_le!(0,1,T,0,0,1,T));
    cpu.run(1).unwrap();
    assert_eq!(i64::from(cpu.regs.flags), bt_le!(T,T,T,0,1,1,1));
    cpu.run(1).unwrap();
    assert_eq!(i64::from(cpu.regs.flags), bt_le!(T,T,T,0,0,1,T));
    cpu.run(1).unwrap();
    assert_eq!(i64::from(cpu.regs.flags), bt_le!(0,1,T,0,0,0,0));
    cpu.run(1).unwrap();
    assert_eq!(i64::from(cpu.regs.flags), bt_le!(T,T,T,0,0,0,0));
    cpu.run(1).unwrap();
    assert_eq!(i64::from(cpu.regs.flags), bt_le!(0,1,T,0,0,1,T));
    cpu.run(1).unwrap();
    assert_eq!(i64::from(cpu.regs.flags), bt_le!(1,1,1,1,0,1,T));
    cpu.run(1).unwrap();
    assert_eq!(i64::from(cpu.regs.flags), bt_le!(T,T,T,T,0,1,T));
    cpu.run(1).unwrap();
    assert_eq!(i64::from(cpu.regs.flags), bt_le!(0,1,T,0,0,1,T));
    cpu.run(1).unwrap();
    assert_eq!(i64::from(cpu.regs.flags), bt_le!(0,1,T,0,0,1,T));
    cpu.run(1).unwrap();
    assert_eq!(i64::from(cpu.regs.flags), bt_le!(T,T,T,0,0,1,T));
    println!("\n{}", cpu);
}

#[test]
fn test_shift() {
    let mut cpu = Cpu::new();
    cpu.init_default();
    cpu.regs.b = Word::from(bt_le!(1));
    let (pc_space, pc_offset) = cpu.get_mut_space_and_offset(cpu.regs.pc).unwrap();
    let shellcode = [
        byte_le!(1,T,0,1,0,0,0,0,0), // shift b, 1
        byte_le!(1,T,0,0,1,0,0,0,0), // shift b, 3
        byte_le!(1,T,0,T,T,0,0,0,0), // shift b, -4
        byte_le!(1,T,0,1,1,0,1,0,0), // shift b, -4
    ];
    for (i, b) in shellcode.iter().enumerate() {
        pc_space.set_byte(pc_offset+(i as isize), *b).unwrap();
    }
    println!("{}", cpu);
    cpu.run(1).unwrap();
    assert_eq!(i64::from(cpu.regs.b), 3);
    cpu.run(1).unwrap();
    assert_eq!(i64::from(cpu.regs.b), 81);
    cpu.run(1).unwrap();
    assert_eq!(i64::from(cpu.regs.b), 1);
    cpu.run(1).unwrap();
    println!("\n{}", cpu);
    assert_eq!(i64::from(cpu.regs.b), 0);
}

#[test]
fn test_inimm_insts() {
    let mut cpu = Cpu::new();
    cpu.init_default();
    cpu.regs.b = Word::from(bt_le!(0,0,1));
    let (pc_space, pc_offset) = cpu.get_mut_space_and_offset(cpu.regs.pc).unwrap();
    let shellcode = [
        byte_le!(T,T,T,1,0,0,0,0,0), // add b, 1
        byte_le!(T,T,1,1,T,0,0,0,0), // sub b, T1
        byte_le!(T,T,0,1,T,1,0,0,0), // mul b, 1T1
        byte_le!(T,1,T,1,T,1,T,0,0), // div b, T1T1
        byte_le!(T,1,1,0,1,0,0,0,0), // mod b, 10
        byte_le!(T,1,0,1,0,0,0,0,0), // addfz b, 1
        byte_le!(T,0,T,1,0,0,0,0,0), // subfz b, 1
    ];
    for (i, b) in shellcode.iter().enumerate() {
        pc_space.set_byte(pc_offset+(i as isize), *b).unwrap();
    }
    println!("{}", cpu);
    cpu.run(8).unwrap();
    println!("\n{}", cpu);
    assert_eq!(i64::from(cpu.regs.b), -1);
}

#[test]
fn test_andorxormov() {
    let mut cpu = Cpu::new();
    cpu.init_default();
    cpu.regs.b = Word::from(bt_le!(0,1,T,0,1,T,0,1,T));
    cpu.regs.c = Word::from(bt_le!(1,T,0,T,0,1,0,1,T));
    let (pc_space, pc_offset) = cpu.get_mut_space_and_offset(cpu.regs.pc).unwrap();
    let shellcode = [
        byte_le!(0,1,1,0,0,0,1,1,T), // mov d, c
        byte_le!(0,1,1,T,T,0,0,1,T), // and d, b
        byte_le!(0,1,1,T,1,0,0,1,T), // or d, b
        byte_le!(0,1,1,T,0,0,1,0,0), // xor d, b
    ];
    for (i, b) in shellcode.iter().enumerate() {
        pc_space.set_byte(pc_offset+(i as isize), *b).unwrap();
    }
    println!("{}", cpu);
    cpu.run(1).unwrap();
    assert_eq!(cpu.regs.c, cpu.regs.d);
    cpu.run(1).unwrap();
    assert_eq!(i64::from(cpu.regs.d), bt_le!(0,T,T,T,0,T,0,1,T));
    cpu.run(1).unwrap();
    assert_eq!(i64::from(cpu.regs.d), bt_le!(0,1,T,0,1,T,0,1,T));
    println!("\n{}", cpu);
    cpu.run(1).unwrap();
    println!("\n{}", cpu);
    assert_eq!(i64::from(cpu.regs.b), bt_le!(0,1,0,0,0,1,0,T,T));
}

#[test]
fn test_store() {
    let mut cpu = Cpu::new();
    cpu.init_default();
    let (pc_space, pc_offset) = cpu.get_mut_space_and_offset(cpu.regs.pc).unwrap();
    let shellcode = [
        byte_le!(0,0,0,0,T,0,1,0,0), // set b, 010000000000000000
        byte_le!(0,0,0,0,0,0,0,0,0),
        byte_le!(0,0,0,0,0,0,0,1,0),
        byte_le!(0,0,T,T,1,0,0,0,1), // load c, [b]
        byte_le!(0,0,T,1,1,0,T,0,0), // store [b], a
        byte_le!(0,0,T,T,1,0,0,0,1), // load c, [b]
        byte_le!(0,0,T,T,T,0,0,1,T), // load d.b, [b]
        byte_le!(0,0,T,T,0,0,0,1,0), // load e.t, [b]
    ];
    for (i, b) in shellcode.iter().enumerate() {
        pc_space.set_byte(pc_offset+(i as isize), *b).unwrap();
    }
    println!("{}", cpu);
    cpu.run(2).unwrap();
    println!("\n{}", cpu);
    assert_eq!(i64::from(cpu.regs.b), bt_le!(0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1,0));
    assert_eq!(i64::from(cpu.regs.c), bt_le!(0,0,0,0,T,0,1));
    cpu.regs.a = Word::from(bt_le!(1,0,0,0,0,0,0,0,0,T));
    cpu.run(4).unwrap();
    println!("\n{}", cpu);
    assert_eq!(i64::from(cpu.regs.a), bt_le!(1,0,0,0,0,0,0,0,0,T));
    assert_eq!(i64::from(cpu.regs.c), bt_le!(1,0,0,0,0,0,0,0,0,T));
    assert_eq!(i64::from(cpu.regs.d), 1);
    assert_eq!(i64::from(cpu.regs.e), bt_le!(0,0,0,0,0,0,0,0,0,1));
}

#[test]
fn test_addsubmuldivmod_regreg() {
    let mut cpu = Cpu::new();
    cpu.init_default();
    let (pc_space, pc_offset) = cpu.get_mut_space_and_offset(cpu.regs.pc).unwrap();
    let shellcode = [
        byte_le!(0,0,0,0,T,0,1,0,0), // set b, 1T000
        byte_le!(0,0,0,T,1,0,0,0,0),
        byte_le!(0,0,0,0,0,0,0,0,0),
        byte_le!(0,0,0,0,T,0,T,0,T), // set a, 10
        byte_le!(0,1,0,0,0,0,0,0,0),
        byte_le!(0,1,T,T,T,0,0,0,0), // add b, b
        byte_le!(0,1,T,T,T,0,0,0,1), // add c, b
        byte_le!(0,1,T,T,1,0,1,1,T), // sub d, c
        byte_le!(0,1,T,T,1,0,0,0,1), // sub c, b
        byte_le!(0,1,T,T,0,0,T,0,0), // mul b, a
        byte_le!(0,1,T,1,T,0,T,1,T), // div d, a
        byte_le!(0,0,0,0,T,0,T,0,0), // set b, 101
        byte_le!(1,0,1,0,0,0,0,0,0),
        byte_le!(0,1,T,1,1,0,T,0,0), // mod b, a
    ];
    for (i, b) in shellcode.iter().enumerate() {
        pc_space.set_byte(pc_offset+(i as isize), *b).unwrap();
    }
    println!("{}", cpu);
    cpu.run(6).unwrap();
    println!("\n{}", cpu);
    assert_eq!(i64::from(cpu.regs.b), 108);
    assert_eq!(i64::from(cpu.regs.c), 0);
    assert_eq!(i64::from(cpu.regs.d), -108);
    cpu.run(2).unwrap();
    println!("\n{}", cpu);
    assert_eq!(i64::from(cpu.regs.b), 108*3);
    assert_eq!(i64::from(cpu.regs.d), -108/3);
    cpu.run(2).unwrap();
    println!("\n{}", cpu);
    assert_eq!(i64::from(cpu.regs.b), 1);
}

#[test]
fn test_cjumprel() {
    let mut cpu = Cpu::new();
    cpu.init_default();
    let (pc_space, pc_offset) = cpu.get_mut_space_and_offset(cpu.regs.pc).unwrap();
    let shellcode = [
        byte_le!(0,0,0,0,1,T,T,0,0), // cjumprel if flags.diff == T
        byte_le!(0,0,0,T,0,0,0,0,0),
        byte_le!(0,0,0,0,1,T,0,0,0), // cjumprel if flags.diff == 0
        byte_le!(0,0,0,1,0,0,0,0,0),
    ];
    for (i, b) in shellcode.iter().enumerate() {
        pc_space.set_byte(pc_offset+(i as isize), *b).unwrap();
    }
    println!("{}", cpu);
    cpu.run(2).unwrap();
    println!("\n{}", cpu);
    assert_eq!(i64::from(cpu.regs.pc.bytes[0]), 29);
}

#[test]
fn test_cjumpabs() {
    let mut cpu = Cpu::new();
    cpu.init_default();
    let (pc_space, pc_offset) = cpu.get_mut_space_and_offset(cpu.regs.pc).unwrap();
    let shellcode = [
        byte_le!(0,0,0,0,1,1,T,0,0), // cjumpabs if flags.diff == T
        byte_le!(0,0,0,T,0,0,0,0,0),
        byte_le!(0,0,0,0,0,0,0,T,0),
        byte_le!(0,0,0,0,1,1,0,0,0), // cjumpabs if flags.diff == 0
        byte_le!(0,0,0,1,0,0,0,0,0),
        byte_le!(0,0,0,0,0,0,0,1,0),
    ];
    for (i, b) in shellcode.iter().enumerate() {
        pc_space.set_byte(pc_offset+(i as isize), *b).unwrap();
    }
    println!("{}", cpu);
    cpu.run(2).unwrap();
    println!("\n{}", cpu);
    assert_eq!(i64::from(cpu.regs.pc.bytes[0]), 27);
}

#[test]
fn test_callrel() {
    let mut cpu = Cpu::new();
    cpu.init_default();
    let (pc_space, pc_offset) = cpu.get_mut_space_and_offset(cpu.regs.pc).unwrap();
    let shellcode = [
        byte_le!(0,0,0,0,0,0,0,1,T), // callrel 1T0
        byte_le!(0,T,1,0,0,0,0,0,0),
        byte_le!(0,0,0,0,0,0,0,0,0),
        byte_le!(0,0,0,0,0,0,0,0,0),
        byte_le!(0,0,0,0,0,0,0,0,0),
        byte_le!(0,0,0,0,0,0,0,0,0),
        byte_le!(0,0,0,0,0,0,0,0,0),
        byte_le!(0,0,0,0,0,1,1,T,T), // pop pc // ret
    ];
    for (i, b) in shellcode.iter().enumerate() {
        pc_space.set_byte(pc_offset+(i as isize), *b).unwrap();
    }
    println!("{}", cpu);
    cpu.run(4).unwrap();
    println!("\n{}", cpu);
    assert_eq!(i64::from(cpu.regs.pc.bytes[0]), 3);
}

#[test]
fn test_callabs() {
    let mut cpu = Cpu::new();
    cpu.init_default();
    let (pc_space, pc_offset) = cpu.get_mut_space_and_offset(cpu.regs.pc).unwrap();
    let shellcode = [
        byte_le!(0,0,0,0,0,0,0,1,1), // callabs 0100000000000001T1
        byte_le!(1,T,1,0,0,0,0,0,0),
        byte_le!(0,0,0,0,0,0,0,1,0),
        byte_le!(0,0,0,0,0,0,0,0,0),
        byte_le!(0,0,0,0,0,0,0,0,0),
        byte_le!(0,0,0,0,0,0,0,0,0),
        byte_le!(0,0,0,0,0,0,0,0,0),
        byte_le!(0,0,0,0,0,0,0,0,0),
        byte_le!(0,0,0,0,0,1,1,T,T), // pop pc // ret
    ];
    for (i, b) in shellcode.iter().enumerate() {
        pc_space.set_byte(pc_offset+(i as isize), *b).unwrap();
    }
    println!("{}", cpu);
    cpu.run(4).unwrap();
    println!("\n{}", cpu);
    assert_eq!(i64::from(cpu.regs.pc.bytes[0]), 4);
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
    cpu.run(3).unwrap();
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
        byte_le!(0,0,0,0,0,1,T,0,0), // push b
        byte_le!(0,0,0,0,0,1,1,0,1), // pop c
        byte_le!(0,0,0,0,0,1,T,0,1), // push c
        byte_le!(0,0,0,0,0,1,1,1,T), // pop d
        byte_le!(0,0,0,0,0,1,T,1,T), // push d
        byte_le!(0,0,0,0,0,1,1,1,0), // pop e
        byte_le!(0,0,0,0,0,1,T,1,T), // push d
        byte_le!(0,0,0,0,0,1,T,1,T), // push d
        byte_le!(0,0,0,0,0,1,T,1,T), // push d
    ];
    for (i, b) in shellcode.iter().enumerate() {
        pc_space.set_byte(pc_offset+(i as isize), *b).unwrap();
    }
    cpu.run(10).unwrap();
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
    cpu.run(3).unwrap();
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
    cpu.run(5).unwrap();
    assert_eq!(i64::from(old_pc) + 5, i64::from(cpu.regs.pc))
}
}
