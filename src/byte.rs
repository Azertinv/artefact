use std::fmt;
use super::trit::Trit;
use super::operation::Operation;

#[macro_export]
macro_rules! byte_le {
    ( 0 ) => { crate::trit::Trit::ZERO };
    ( 1 ) => { crate::trit::Trit::ONE };
    ( T ) => { crate::trit::Trit::TERN };
    ( $t0:tt,$t1:tt,$t2:tt,$t3:tt,$t4:tt,$t5:tt,$t6:tt,$t7:tt,$t8:tt ) => {
        crate::byte::Byte{trits: [
            byte_le!($t0),
            byte_le!($t1),
            byte_le!($t2),
            byte_le!($t3),
            byte_le!($t4),
            byte_le!($t5),
            byte_le!($t6),
            byte_le!($t7),
            byte_le!($t8),
        ] }
    };
    ($i:expr) => {$i};
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Byte {
    pub trits: [Trit; Byte::WIDTH],
}

impl Byte {
    pub const ZERO: Byte = byte_le!(0,0,0,0,0,0,0,0,0);
    pub const ONE: Byte = byte_le!(1,0,0,0,0,0,0,0,0);
    pub const TERN: Byte = byte_le!(T,0,0,0,0,0,0,0,0);

    pub const WIDTH: usize = 9;
    pub const IWIDTH: isize = 9;

    pub const MAX: i64 = 9841;
    pub const MIN: i64 = -Byte::MAX;

    pub fn from_trits(slice: &[Trit]) -> Byte {
        assert_eq!(slice.len(), Byte::WIDTH);
        let mut result = Byte::ZERO;
        result.trits.copy_from_slice(&slice[..Byte::WIDTH]);
        result
    }

    pub fn from_vec(vec: Vec<Trit>) -> Byte {
        Byte::from_trits(&vec[..Byte::WIDTH])
    }

    pub fn sign(&self) -> Trit {
        for i in (0..Byte::WIDTH).rev() {
            if self.trits[i] != Trit::ZERO {
                return self.trits[i];
            }
        }
        return Trit::ZERO;
    }

    pub fn highest_mst(&self) -> usize {
        for i in (0..Byte::WIDTH).rev() {
            if self.trits[i] != Trit::ZERO {
                return i;
            }
        }
        return 0;
    }
}

impl Operation for Byte {
    fn not(a: Byte) -> Byte {
        let mut result = Byte::ZERO;
        for i in 0..Byte::WIDTH {
            result.trits[i] = Trit::not(a.trits[i]);
        }
        result
    }
    fn and(lhs: Byte, rhs: Byte) -> Byte {
        let mut result = Byte::ZERO;
        for i in 0..Byte::WIDTH {
            result.trits[i] = Trit::and(lhs.trits[i], rhs.trits[i]);
        }
        result
    }
    fn or(lhs: Byte, rhs: Byte) -> Byte {
        let mut result = Byte::ZERO;
        for i in 0..Byte::WIDTH {
            result.trits[i] = Trit::or(lhs.trits[i], rhs.trits[i]);
        }
        result
    }
    fn xor(lhs: Byte, rhs: Byte) -> Byte {
        let mut result = Byte::ZERO;
        for i in 0..Byte::WIDTH {
            result.trits[i] = Trit::xor(lhs.trits[i], rhs.trits[i]);
        }
        result
    }

    fn greater_dfz(lhs: Byte, rhs: Byte) -> bool {
        i64::from(lhs).abs() > i64::from(rhs).abs()
    }

    fn half_add(lhs: Byte, rhs: Byte) -> (Byte, Byte) {
        let mut result = Byte::ZERO;
        let mut carry = Trit::ZERO;
        for i in 0..Byte::WIDTH {
            let addition = Trit::add(lhs.trits[i], rhs.trits[i], carry);
            carry = addition.1;
            result.trits[i] = addition.0;
        }
        (result, byte_le!(carry,0,0,0,0,0,0,0,0))
    }

    fn mul(lhs: Byte, rhs: Byte) -> (Byte, Byte) {
        let mut result = Byte::ZERO;
        let mut carry = Byte::ZERO;
        for i in 0..Byte::WIDTH {
            let shift = Byte::shift(lhs, i as isize);
            let results = match rhs.trits[i] {
                // should carry from result to carry into 2nd operation
                Trit::TERN => {
                    let half_mul = Byte::sub(result, shift.0, Byte::ZERO);
                    (half_mul.0, Byte::sub(carry, shift.1, half_mul.1).0)
                },
                Trit::ZERO => (result, carry),
                Trit::ONE  => {
                    let half_mul = Byte::add(result, shift.0, Byte::ZERO);
                    (half_mul.0, Byte::add(carry, shift.1, half_mul.1).0)
                },
                _ => panic!(),
            };
            result = results.0;
            carry = results.1;
        }
        (result, carry)
    }

    fn div(lhs: Byte, rhs: Byte) -> (Byte, Byte) {
        assert_ne!(rhs, Byte::ZERO);
        // println!("{} / {}", i64::from(lhs), i64::from(rhs));
        let mut quotient = Byte::ZERO;
        let mut remainder = lhs;
        for i in (0..Byte::WIDTH).rev() {
            // println!("quotient: {}, remainder: {}", quotient, remainder);
            for _ in 0..2 {
                let dividend = Byte::shift(remainder, -(i as isize)).0;
                let high = Byte::add(dividend, rhs, Byte::ZERO).0;
                let mid = dividend;
                let low = Byte::sub(dividend, rhs, Byte::ZERO).0;
                // println!("high: {}, mid: {}, low: {}", high, mid, low);

                let mut intermediate = high;
                if Byte::greater_dfz(intermediate, mid) {
                    intermediate = mid;
                }
                if Byte::greater_dfz(intermediate, low) {
                    intermediate = low;
                }
                // println!("intermediate: {}", intermediate);

                if intermediate == high {
                    remainder = Byte::add(remainder, Byte::shift(rhs, i as isize).0, Byte::ZERO).0;
                    quotient = Byte::add(quotient, Byte::shift(Byte::TERN, i as isize).0, Byte::ZERO).0;
                } else if intermediate == low {
                    remainder = Byte::sub(remainder, Byte::shift(rhs, i as isize).0, Byte::ZERO).0;
                    quotient = Byte::sub(quotient, Byte::shift(Byte::TERN, i as isize).0, Byte::ZERO).0;
                } else {
                    break;
                }
            }
        }
        // println!("quotient pre result: {}", quotient);
        // println!("remainder pre result: {}\n", remainder);
        if Byte::greater_dfz(Byte::add(remainder, remainder, Byte::ZERO).0, rhs) {
            if remainder.sign() != rhs.sign() {
                quotient = Byte::add(quotient, Byte::TERN, Byte::ZERO).0;
                remainder = Byte::add(remainder, rhs, Byte::ZERO).0;
            } else {
                quotient = Byte::sub(quotient, Byte::TERN, Byte::ZERO).0;
                remainder = Byte::sub(remainder, rhs, Byte::ZERO).0;
            }
        }
        if lhs.sign() == rhs.sign() && !Byte::greater_dfz(rhs, Byte::add(remainder, remainder, Byte::ZERO).0) {
            if remainder.sign() != rhs.sign() {
                quotient = Byte::add(quotient, Byte::TERN, Byte::ZERO).0;
                remainder = Byte::add(remainder, rhs, Byte::ZERO).0;
            } else {
                quotient = Byte::sub(quotient, Byte::TERN, Byte::ZERO).0;
                remainder = Byte::sub(remainder, rhs, Byte::ZERO).0;
            }
        }
        // println!("quotient result: {}", quotient);
        // println!("remainder result: {}\n", remainder);
        (quotient, remainder)
    }

    fn shift(a: Byte, amount: isize) -> (Byte, Byte) {
        if amount.abs() > Byte::IWIDTH*2 {
            return (Byte::ZERO, Byte::ZERO);
        }
        let mut trits = [Trit::ZERO; Byte::WIDTH*5];
        trits[Byte::WIDTH*2..Byte::WIDTH*3].copy_from_slice(&a.trits);
        let index = (Byte::IWIDTH*2 - amount) as usize;
        let result = Byte::from_trits(&trits[index..index+Byte::WIDTH]);
        let carry = if amount >= 0 {
            Byte::from_trits(&trits[index+Byte::WIDTH..index+Byte::WIDTH*2])
        } else {
            Byte::from_trits(&trits[index-Byte::WIDTH..index])
        };
        (result, carry)
    }
}

impl fmt::Display for Byte {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mst = self.highest_mst();
        for i in (0..=mst).rev() {
            write!(f, "{}", self.trits[i])?;
        }
        Ok(())
    }
}

impl From<i64> for Byte {
    fn from(mut item: i64) -> Self {
        assert!(item <= Byte::MAX);
        assert!(item >= Byte::MIN);
        let mut trits = [Trit::ZERO; Byte::WIDTH];
        for i in 0..Byte::WIDTH {
            let mut trit = item % 3;
            if trit < 0 {
                trit += 3;
            }
            if trit == 2 {
                trit = -1i64;
            }
            trits[i] = match trit {
                -1 => Trit::TERN,
                0 => Trit::ZERO,
                1 => Trit::ONE,
                _ => Trit::ZERO,
            };
            item -= trit;
            item /= 3;
        }
        Byte{trits: trits}
    }
}

impl From<Byte> for i64 {
    fn from(item: Byte) -> Self {
        let mut result: Self = 0;
        for i in 0..Byte::WIDTH {
            let value: Self = item.trits[i].into();
            result += value * Self::pow(3, i as u32)
        }
        result
    }
}

#[test]
fn test_max() {
    assert_eq!(Byte::MAX, 3i64.pow(Byte::WIDTH as u32) / 2i64);
}

#[test]
fn test_size() {
    assert_eq!(std::mem::size_of::<Byte>(), 9);
}

#[test]
fn test_add() {
    for i in (Byte::MIN..=Byte::MAX).step_by(10) {
        for j in (Byte::MIN..=Byte::MAX).step_by(10) {
            if i+j < Byte::MIN { continue; }
            if i+j > Byte::MAX { continue; }
            assert_eq!(i+j, i64::from(Byte::add(Byte::from(i), Byte::from(j), Byte::ZERO).0));
        }
    }
}

#[test]
fn test_sub() {
    for i in (Byte::MIN..=Byte::MAX).step_by(10) {
        for j in (Byte::MIN..=Byte::MAX).step_by(10) {
            if i-j < Byte::MIN { continue; }
            if i-j > Byte::MAX { continue; }
            assert_eq!(i-j, i64::from(Byte::sub(Byte::from(i), Byte::from(j), Byte::ZERO).0));
        }
    }
}

#[test]
fn test_mul() {
    for i in (Byte::MIN..=Byte::MAX).step_by(10) {
        for j in (Byte::MIN..=Byte::MAX).step_by(10) {
            let result = Byte::mul(Byte::from(i), Byte::from(j));
            assert_eq!(i * j, i64::from(result.0) + (Byte::MAX*2+1)*i64::from(result.1));
        }
    }
}

#[test]
fn test_greater_dfz() {
    for i in (Byte::MIN..=Byte::MAX).step_by(10) {
        for j in (Byte::MIN..=Byte::MAX).step_by(10) {
            let result = Byte::greater_dfz(Byte::from(i), Byte::from(j));
            assert_eq!(result, i.abs() > j.abs());
        }
    }
}

#[test]
fn test_conversion() {
    for i in Byte::MIN..=Byte::MAX {
        assert_eq!(i, i64::from(Byte::from(i)));
    }
}

#[test]
fn test_add_overflow() {
    let min = Byte::from(Byte::MIN);
    let max = Byte::from(Byte::MAX);
    assert_eq!(max, Byte::add(min, Byte::from(-1i64), Byte::ZERO).0);
    assert_eq!(min, Byte::add(max, Byte::from(1i64), Byte::ZERO).0);
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
    for i in Byte::MIN..=Byte::MAX {
        for j in 0..Byte::IWIDTH {
            let (result, carry) = Byte::shift(Byte::from(i), j);
            assert_eq!(i64::from(result) + i64::from(carry)*3i64.pow(Byte::WIDTH as u32), i * 3i64.pow(j as u32));
        }
        for j in -Byte::IWIDTH..0 {
            let (result, _) = Byte::shift(Byte::from(i), j);
            let divisor = 3i64.pow(j.abs() as u32);
            assert_eq!(i64::from(result), round_div(i, divisor));
        }
    }
}

#[test]
fn test_div() {
    for i in (Byte::MIN..=Byte::MAX).step_by(10) {
        for j in (Byte::MIN..=Byte::MAX).step_by(10) {
            if j == 0 { continue; }
            let result = Byte::div(Byte::from(i), Byte::from(j)).0;
            assert_eq!(i64::from(result), round_div(i, j));
        }
    }
}
