use std::fmt;
use super::trit::Trit;
use super::byte::Byte;
use super::operation::Operation;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Word {
    pub bytes: [Byte; Word::BYTE_COUNT],
}

impl Word {
    pub const BYTE_COUNT: usize = 3;
    pub const IBYTE_COUNT: isize = 3;
    pub const WIDTH: usize = Word::BYTE_COUNT * Byte::WIDTH; // 3 * 7 trits
    pub const IWIDTH: isize = Word::IBYTE_COUNT * Byte::IWIDTH;

    pub const MIN: i64 = -5230176601;
    pub const MAX: i64 = 5230176601;

    pub const ZERO: Word = Word{bytes: [Byte::ZERO, Byte::ZERO, Byte::ZERO]};
    pub const ONE: Word = Word{bytes: [Byte::ONE, Byte::ZERO, Byte::ZERO]};
    pub const TERN: Word = Word{bytes: [Byte::TERN, Byte::ZERO, Byte::ZERO]};

    pub fn from_trits(slice: &[Trit]) -> Word {
        assert_eq!(slice.len(), Word::WIDTH);
        let mut result = Word::ZERO;
        for i in 0..Word::BYTE_COUNT {
            result.bytes[i].trits.copy_from_slice(&slice[Byte::WIDTH*i..Byte::WIDTH*(i+1)]);
        }
        result
    }

    pub fn sign(&self) -> Trit {
        for i in (0..Word::BYTE_COUNT).rev() {
            let sign = self.bytes[i].sign();
            if sign != Trit::ZERO {
                return sign;
            }
        }
        return Trit::ZERO;
    }

    pub fn highest_mst(&self) -> usize {
        for i in (0..Word::BYTE_COUNT).rev() {
            for j in (0..Byte::WIDTH).rev() {
                if self.bytes[i].trits[j] != Trit::ZERO {
                    return i*Byte::WIDTH+j;
                }
            }
        }
        return 0;
    }
}

impl Operation for Word {
    fn not(a: Word) -> Word {
        let mut result = Word::ZERO;
        for i in 0..Word::BYTE_COUNT {
            result.bytes[i] = Byte::not(a.bytes[i]);
        }
        result
    }
    fn and(lhs: Word, rhs: Word) -> Word {
        let mut result = Word::ZERO;
        for i in 0..Word::BYTE_COUNT {
            result.bytes[i] = Byte::and(lhs.bytes[i], rhs.bytes[i]);
        }
        result
    }
    fn or(lhs: Word, rhs: Word) -> Word {
        let mut result = Word::ZERO;
        for i in 0..Word::BYTE_COUNT {
            result.bytes[i] = Byte::or(lhs.bytes[i], rhs.bytes[i]);
        }
        result
    }

    fn greater_dfz(lhs: Word, rhs: Word) -> bool {
        i64::from(lhs).abs() > i64::from(rhs).abs()
    }

    fn half_add(lhs: Word, rhs: Word) -> (Word, Word) {
        let mut result = Word::ZERO;
        let mut carry = Byte::ZERO;
        for i in 0..Word::BYTE_COUNT {
            let addition = Byte::add(lhs.bytes[i], rhs.bytes[i], carry);
            carry = addition.1;
            result.bytes[i] = addition.0;
        }
        (result, Word{bytes: [carry, Byte::ZERO, Byte::ZERO]})
    }

    fn mul(lhs: Word, rhs: Word) -> (Word, Word) {
        unimplemented!();
    // fn mul(lhs: Byte, rhs: Byte) -> (Byte, Byte) {
    //     let mut result = Byte::ZERO;
    //     let mut carry = Byte::ZERO;
    //     for i in 0..Word::BYTE_COUNT {
    //         let shift = Byte::shift(lhs, i as isize);
    //         let results = match rhs.trits[i] {
    //             // should carry from result to carry into 2nd operation
    //             Trit::TERN => {
    //                 let half_mul = Byte::sub(result, shift.0, Byte::ZERO);
    //                 (half_mul.0, Byte::sub(carry, shift.1, half_mul.1).0)
    //             },
    //             Trit::ZERO => (result, carry),
    //             Trit::ONE  => {
    //                 let half_mul = Byte::add(result, shift.0, Byte::ZERO);
    //                 (half_mul.0, Byte::add(carry, shift.1, half_mul.1).0)
    //             },
    //         };
    //         result = results.0;
    //         carry = results.1;
    //     }
    //     (result, carry)
    }

    fn div(lhs: Word, rhs: Word) -> (Word, Word) {
        unimplemented!();
    // fn div(lhs: Byte, rhs: Byte) -> (Byte, Byte) {
    //     assert_ne!(rhs, Byte::ZERO);
    //     // println!("{} / {}", i64::from(lhs), i64::from(rhs));
    //     let mut quotient = Byte::ZERO;
    //     let mut remainder = lhs;
    //     for i in (0..Word::BYTE_COUNT).rev() {
    //         // println!("quotient: {}, remainder: {}", quotient, remainder);
    //         for _ in 0..2 {
    //             let dividend = Byte::shift(remainder, -(i as isize)).0;
    //             let high = Byte::add(dividend, rhs, Byte::ZERO).0;
    //             let mid = dividend;
    //             let low = Byte::sub(dividend, rhs, Byte::ZERO).0;
    //             // println!("high: {}, mid: {}, low: {}", high, mid, low);

    //             let mut intermediate = high;
    //             if Byte::greater_dfz(intermediate, mid) {
    //                 intermediate = mid;
    //             }
    //             if Byte::greater_dfz(intermediate, low) {
    //                 intermediate = low;
    //             }
    //             // println!("intermediate: {}", intermediate);

    //             if intermediate == high {
    //                 remainder = Byte::add(remainder, Byte::shift(rhs, i as isize).0, Byte::ZERO).0;
    //                 quotient = Byte::add(quotient, Byte::shift(Byte::TERN, i as isize).0, Byte::ZERO).0;
    //             } else if intermediate == low {
    //                 remainder = Byte::sub(remainder, Byte::shift(rhs, i as isize).0, Byte::ZERO).0;
    //                 quotient = Byte::sub(quotient, Byte::shift(Byte::TERN, i as isize).0, Byte::ZERO).0;
    //             } else {
    //                 break;
    //             }
    //         }
    //     }
    //     // println!("quotient pre result: {}", quotient);
    //     // println!("remainder pre result: {}\n", remainder);
    //     if Byte::greater_dfz(Byte::add(remainder, remainder, Byte::ZERO).0, rhs) {
    //         if remainder.sign() != rhs.sign() {
    //             quotient = Byte::add(quotient, Byte::TERN, Byte::ZERO).0;
    //             remainder = Byte::add(remainder, rhs, Byte::ZERO).0;
    //         } else {
    //             quotient = Byte::sub(quotient, Byte::TERN, Byte::ZERO).0;
    //             remainder = Byte::sub(remainder, rhs, Byte::ZERO).0;
    //         }
    //     }
    //     if lhs.sign() == rhs.sign() && !Byte::greater_dfz(rhs, Byte::add(remainder, remainder, Byte::ZERO).0) {
    //         if remainder.sign() != rhs.sign() {
    //             quotient = Byte::add(quotient, Byte::TERN, Byte::ZERO).0;
    //             remainder = Byte::add(remainder, rhs, Byte::ZERO).0;
    //         } else {
    //             quotient = Byte::sub(quotient, Byte::TERN, Byte::ZERO).0;
    //             remainder = Byte::sub(remainder, rhs, Byte::ZERO).0;
    //         }
    //     }
    //     // println!("quotient result: {}", quotient);
    //     // println!("remainder result: {}\n", remainder);
    //     (quotient, remainder)
    }

    fn shift(a: Word, amount: isize) -> (Word, Word) {
        if amount.abs() > Word::IWIDTH*2 {
            return (Word::ZERO, Word::ZERO);
        }
        let mut trits = [Trit::ZERO; Word::WIDTH*5];
        for i in 0..Word::BYTE_COUNT {
            trits[Word::WIDTH*2+Byte::WIDTH*i..Word::WIDTH*2+Byte::WIDTH*(i+1)].copy_from_slice(&a.bytes[i].trits);
        }
        let index = (Word::IWIDTH*2 - amount) as usize;
        let result = Word::from_trits(&trits[index..index+Word::WIDTH]);
        let carry = if amount >= 0 {
            Word::from_trits(&trits[index+Word::WIDTH..index+Word::WIDTH*2])
        } else {
            Word::from_trits(&trits[index-Word::WIDTH..index])
        };
        (result, carry)
    }
}

impl fmt::Display for Word{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        assert_eq!(Word::BYTE_COUNT, 3);
        let mst = self.highest_mst();
        for i in (0..=mst).rev() {
            write!(f, "{}", self.bytes[i / Byte::WIDTH].trits[i % Byte::WIDTH])?;
        }
        Ok(())
    }
}

impl From<i64> for Word {
    fn from(mut item: i64) -> Self {
        assert!(item <= Word::MAX);
        assert!(item >= Word::MIN);
        let mut bytes = [Byte::ZERO; Word::BYTE_COUNT];
        for i in 0..Word::BYTE_COUNT {
            for j in 0..Byte::WIDTH {
                let mut trit = item % 3;
                if trit < 0 {
                    trit += 3;
                }
                if trit == 2 {
                    trit = -1i64;
                }
                bytes[i].trits[j] = match trit {
                    -1 => Trit::TERN,
                    0 => Trit::ZERO,
                    1 => Trit::ONE,
                    _ => Trit::ZERO,
                };
                item -= trit;
                item /= 3;
            }
        }
        Word{bytes: bytes}
    }
}

impl From<Word> for i64 {
    fn from(item: Word) -> Self {
        let mut result: Self = 0;
        for i in 0..Word::BYTE_COUNT {
            let value: Self = item.bytes[i].into();
            result += value * Self::pow(3, (i*Byte::WIDTH) as u32);
        }
        result
    }
}

#[test]
fn test_size() {
    assert_eq!(std::mem::size_of::<Word>(), 21);
}

#[test]
fn test_conversion() {
    for i in (Word::MIN..=Word::MAX).step_by(1000) {
        assert_eq!(i, i64::from(Word::from(i)));
    }
}

#[test]
fn test_add() {
    for i in (Word::MIN..=Word::MAX).step_by(10000000) {
        for j in (Word::MIN..=Word::MAX).step_by(10000000) {
            if i+j < Word::MIN { continue; }
            if i+j > Word::MAX { continue; }
            assert_eq!(i+j, i64::from(Word::add(Word::from(i), Word::from(j), Word::ZERO).0));
        }
    }
}

#[test]
fn test_sub() {
    for i in (Word::MIN..=Word::MAX).step_by(10000000) {
        for j in (Word::MIN..=Word::MAX).step_by(10000000) {
            if i-j < Word::MIN { continue; }
            if i-j > Word::MAX { continue; }
            assert_eq!(i-j, i64::from(Word::sub(Word::from(i), Word::from(j), Word::ZERO).0));
        }
    }
}

#[test]
fn test_greater_dfz() {
    for i in (Word::MIN..=Word::MAX).step_by(10000000) {
        for j in (Word::MIN..=Word::MAX).step_by(10000000) {
            let result = Word::greater_dfz(Word::from(i), Word::from(j));
            assert_eq!(result, i.abs() > j.abs());
        }
    }
}

#[test]
fn test_add_overflow() {
    let min = Word::from(Word::MIN);
    let max = Word::from(Word::MAX);
    assert_eq!(max, Word::add(min, Word::from(-1i64), Word::ZERO).0);
    assert_eq!(min, Word::add(max, Word::from(1i64), Word::ZERO).0);
}

fn round_div(lhs: i64, rhs: i64) -> i64 {
    if (lhs > 0) != (rhs > 0) {
        (lhs - rhs / 2) / rhs
    } else {
        (lhs + rhs / 2) / rhs
    }
}

#[test]
fn test_shift() {
    for i in (Word::MIN..=Word::MAX).step_by(10000) {
        for j in 0..Word::IWIDTH {
            let (result, carry) = Word::shift(Word::from(i), j);
            assert_eq!(i64::from(result) + i64::from(carry)*3i64.pow(7*3), i * 3i64.pow((j) as u32));
        }
        for j in -Word::IWIDTH..0 {
            let (result, _) = Word::shift(Word::from(i), j);
            let divisor = 3i64.pow(j.abs() as u32);
            assert_eq!(i64::from(result), round_div(i, divisor));
        }
    }
}

/*
#[test]
fn test_mul() {
    for i in Byte::MIN..=Byte::MAX {
        for j in Byte::MIN..=Byte::MAX {
            let result = Byte::mul(Byte::from(i), Byte::from(j));
            assert_eq!(i * j, i64::from(result.0) + 2187*i64::from(result.1));
        }
    }
}

#[test]
fn test_div() {
    for i in Byte::MIN..=Byte::MAX {
        for j in Byte::MIN..=Byte::MAX {
            if j == 0 { continue; }
            let result = Byte::div(Byte::from(i), Byte::from(j)).0;
            let test = f64::from(i as i32) / f64::from(j as i32);
            assert_eq!(i64::from(result), test.round() as i64);
        }
    }
}
*/
