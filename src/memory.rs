use crate::byte::Byte;
use crate::word::Word;
use crate::interrupt::{Interrupt, Result};

pub struct MemorySpace {
    memory: Box<[Byte; MemorySpace::SIZE]>,
}

macro_rules! box_array {
    ($val:expr ; $len:expr) => {{
        // Use a generic function so that the pointer cast remains type-safe
        fn vec_to_boxed_array<T>(vec: Vec<T>) -> Box<[T; $len]> {
            let boxed_slice = vec.into_boxed_slice();
            let ptr = ::std::boxed::Box::into_raw(boxed_slice) as *mut [T; $len];
            unsafe { Box::from_raw(ptr) }
        }
        vec_to_boxed_array(vec![$val; $len])
    }};
}

impl MemorySpace {
    const SIZE: usize = 4782969; //Word::BYTE_COUNT^(Byte::WIDTH*2)
    const MAX_ADDR: isize = Self::SIZE as isize/2;
    const MIN_ADDR: isize = -(Self::SIZE as isize)/2;

    pub fn new() -> MemorySpace {
        MemorySpace{memory: box_array![Byte::ZERO; MemorySpace::SIZE]}
    }

    fn to_offset(addr: isize) -> Result<usize> {
        if addr < Self::MIN_ADDR || addr > Self::MAX_ADDR {
            Err(Interrupt::BadCode)
        } else {
            Ok((addr + Self::MAX_ADDR) as usize)
        }
    }

    pub fn get_byte(&self, addr: isize) -> Result<Byte> {
        let index = Self::to_offset(addr)?;
        Ok(self.memory[index])
    }

    pub fn set_byte(&mut self, addr: isize, value: Byte) -> Result<()> {
        let index = Self::to_offset(addr)?;
        self.memory[index] = value;
        Ok(())
    }

    pub fn get_word(&self, addr: isize) -> Result<Word> {
        let index = Self::to_offset(addr)?;
        if addr + Word::BYTE_COUNT as isize > Self::MAX_ADDR {
            Err(Interrupt::MemoryFault)
        } else {
            Ok(Word::from_bytes(&self.memory[index..index+Word::BYTE_COUNT]))
        }
    }

    pub fn set_word(&mut self, addr: isize, value: Word) -> Result<()> {
        let index = Self::to_offset(addr)?;
        if addr + Word::BYTE_COUNT as isize > Self::MAX_ADDR {
            Err(Interrupt::MemoryFault)
        } else {
            self.memory[index..index+Word::BYTE_COUNT].copy_from_slice(&value.bytes);
            Ok(())
        }
    }
}

#[test]
fn test_offset() {
    assert_eq!(MemorySpace::to_offset(MemorySpace::MIN_ADDR), Ok(0));
    assert_eq!(MemorySpace::to_offset(MemorySpace::MAX_ADDR), Ok(MemorySpace::SIZE - 1));
    assert_eq!(MemorySpace::to_offset(MemorySpace::MIN_ADDR-1), Err(Interrupt::BadCode));
    assert_eq!(MemorySpace::to_offset(MemorySpace::MAX_ADDR+1), Err(Interrupt::BadCode));
}

#[test]
fn test_getset_byte() {
    let mut memspace = MemorySpace::new();
    assert_eq!(memspace.get_byte(MemorySpace::MIN_ADDR-1), Err(Interrupt::BadCode));
    assert_eq!(memspace.set_byte(MemorySpace::MIN_ADDR-1, Byte::ONE), Err(Interrupt::BadCode));
    assert_eq!(memspace.get_byte(MemorySpace::MAX_ADDR+1), Err(Interrupt::BadCode));
    assert_eq!(memspace.set_byte(MemorySpace::MAX_ADDR+1, Byte::ONE), Err(Interrupt::BadCode));

    assert_eq!(memspace.get_byte(0), Ok(Byte::ZERO));
    memspace.memory[MemorySpace::MAX_ADDR as usize] = Byte::ONE;
    assert_eq!(memspace.get_byte(0), Ok(Byte::ONE));
    memspace.set_byte(MemorySpace::MAX_ADDR, Byte::TERN).unwrap();
    assert_eq!(memspace.get_byte(MemorySpace::MAX_ADDR), Ok(Byte::TERN));
    memspace.set_byte(MemorySpace::MIN_ADDR, Byte::ONE).unwrap();
    assert_eq!(memspace.get_byte(MemorySpace::MIN_ADDR), Ok(Byte::ONE));
}

#[test]
fn test_getset_word() {
    let mut memspace = MemorySpace::new();
    assert_eq!(memspace.get_word(MemorySpace::MIN_ADDR-1), Err(Interrupt::BadCode));
    assert_eq!(memspace.get_word(MemorySpace::MAX_ADDR+1), Err(Interrupt::BadCode));
    assert_eq!(memspace.get_word(MemorySpace::MAX_ADDR-2), Err(Interrupt::MemoryFault));
    assert_eq!(memspace.get_word(MemorySpace::MAX_ADDR-1), Err(Interrupt::MemoryFault));
    assert_eq!(memspace.get_word(MemorySpace::MAX_ADDR), Err(Interrupt::MemoryFault));
    assert_eq!(memspace.set_word(MemorySpace::MIN_ADDR-1, Word::ONE), Err(Interrupt::BadCode));
    assert_eq!(memspace.set_word(MemorySpace::MAX_ADDR+1, Word::ONE), Err(Interrupt::BadCode));

    assert_eq!(memspace.get_word(0), Ok(Word::ZERO));
    memspace.memory[MemorySpace::MAX_ADDR as usize] = Byte::ONE;
    assert_eq!(memspace.get_word(0), Ok(Word::ONE));
    memspace.set_word(0, Word::TERN).unwrap();
    assert_eq!(memspace.get_word(0), Ok(Word::TERN));
    memspace.set_word(10, Word::TERN).unwrap();
    assert_eq!(memspace.get_word(10), Ok(Word::TERN));
}
