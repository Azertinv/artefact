use std::fmt;
use super::trit::Trit;
use super::operation::Operation;

pub const BYTE_WIDTH: usize = 7;
pub const IBYTE_WIDTH: isize = 7;

pub const MIN_BALANCED_VALUE: i64 = -1093;
pub const MAX_BALANCED_VALUE: i64 = 1093;


#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Byte {
    pub trits: [Trit; BYTE_WIDTH],
}

impl Byte {
    pub fn zero() -> Byte {
        Byte{trits: [Trit::Zero; BYTE_WIDTH]}
    }

    pub fn one() -> Byte {
        let mut result = Byte{trits: [Trit::Zero; BYTE_WIDTH]};
        result.trits[0] = Trit::One;
        result
    }

    pub fn tern() -> Byte {
        let mut result = Byte{trits: [Trit::Zero; BYTE_WIDTH]};
        result.trits[0] = Trit::Tern;
        result
    }

    pub fn from_slice(slice: &[Trit]) -> Byte {
        let mut result = Byte::zero();
        result.trits.copy_from_slice(&slice[..BYTE_WIDTH]);
        result
    }

    pub fn from_vec(vec: Vec<Trit>) -> Byte {
        Byte::from_slice(&vec[..BYTE_WIDTH])
    }

    pub fn sign(&self) -> Trit {
        for i in (0..BYTE_WIDTH).rev() {
            if self.trits[i] != Trit::Zero {
                return self.trits[i];
            }
        }
        return Trit::Zero;
    }

    pub fn highest_mst(&self) -> usize {
        for i in (0..BYTE_WIDTH).rev() {
            if self.trits[i] != Trit::Zero {
                return i;
            }
        }
        return 0;
    }
}

impl Operation for Byte {
    fn not(a: Byte) -> Byte {
        let mut result = Byte::zero();
        for i in 0..BYTE_WIDTH {
            result.trits[i] = Trit::not(a.trits[i]);
        }
        result
    }
    fn and(lhs: Byte, rhs: Byte) -> Byte {
        let mut result = Byte::zero();
        for i in 0..BYTE_WIDTH {
            result.trits[i] = Trit::and(lhs.trits[i], rhs.trits[i]);
        }
        result
    }
    fn or(lhs: Byte, rhs: Byte) -> Byte {
        let mut result = Byte::zero();
        for i in 0..BYTE_WIDTH {
            result.trits[i] = Trit::or(lhs.trits[i], rhs.trits[i]);
        }
        result
    }

    fn greater_dfz(lhs: Byte, rhs: Byte) -> bool {
        i64::from(lhs).abs() > i64::from(rhs).abs()
    }

    fn half_add(lhs: Byte, rhs: Byte) -> (Byte, Byte) {
        let mut result = Byte::zero();
        let mut carry = Trit::Zero;
        for i in 0..BYTE_WIDTH {
            let addition = Trit::add(lhs.trits[i], rhs.trits[i], carry);
            carry = addition.1;
            result.trits[i] = addition.0;
        }
        (result, Byte{trits: [carry, Trit::Zero, Trit::Zero, Trit::Zero, Trit::Zero, Trit::Zero, Trit::Zero]})
    }

    fn mul(lhs: Byte, rhs: Byte) -> (Byte, Byte) {
        let mut result = Byte::zero();
        let mut carry = Byte::zero();
        for i in 0..BYTE_WIDTH {
            let shift = Byte::shift(lhs, i as isize);
            let results = match rhs.trits[i] {
                // should carry from result to carry into 2nd operation
                Trit::Tern => {
                    let half_mul = Byte::sub(result, shift.0, Byte::zero());
                    (half_mul.0, Byte::sub(carry, shift.1, half_mul.1).0)
                },
                Trit::Zero => (result, carry),
                Trit::One  => {
                    let half_mul = Byte::add(result, shift.0, Byte::zero());
                    (half_mul.0, Byte::add(carry, shift.1, half_mul.1).0)
                },
            };
            result = results.0;
            carry = results.1;
        }
        (result, carry)
    }

    fn div(lhs: Byte, rhs: Byte) -> (Byte, Byte) {
        assert_ne!(rhs, Byte::zero());
        // println!("{} / {}", i64::from(lhs), i64::from(rhs));
        let mut quotient = Byte::zero();
        let mut remainder = lhs;
        for i in (0..BYTE_WIDTH).rev() {
            // println!("quotient: {}, remainder: {}", quotient, remainder);
            for _ in 0..2 {
                let dividend = Byte::shift(remainder, -(i as isize)).0;
                let high = Byte::add(dividend, rhs, Byte::zero()).0;
                let mid = dividend;
                let low = Byte::sub(dividend, rhs, Byte::zero()).0;
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
                    remainder = Byte::add(remainder, Byte::shift(rhs, i as isize).0, Byte::zero()).0;
                    quotient = Byte::add(quotient, Byte::shift(Byte::tern(), i as isize).0, Byte::zero()).0;
                } else if intermediate == low {
                    remainder = Byte::sub(remainder, Byte::shift(rhs, i as isize).0, Byte::zero()).0;
                    quotient = Byte::sub(quotient, Byte::shift(Byte::tern(), i as isize).0, Byte::zero()).0;
                } else {
                    break;
                }
            }
        }
        // println!("quotient pre result: {}", quotient);
        // println!("remainder pre result: {}\n", remainder);
        if Byte::greater_dfz(Byte::add(remainder, remainder, Byte::zero()).0, rhs) {
            if remainder.sign() != rhs.sign() {
                quotient = Byte::add(quotient, Byte::tern(), Byte::zero()).0;
                remainder = Byte::add(remainder, rhs, Byte::zero()).0;
            } else {
                quotient = Byte::sub(quotient, Byte::tern(), Byte::zero()).0;
                remainder = Byte::sub(remainder, rhs, Byte::zero()).0;
            }
        }
        if lhs.sign() == rhs.sign() && !Byte::greater_dfz(rhs, Byte::add(remainder, remainder, Byte::zero()).0) {
            if remainder.sign() != rhs.sign() {
                quotient = Byte::add(quotient, Byte::tern(), Byte::zero()).0;
                remainder = Byte::add(remainder, rhs, Byte::zero()).0;
            } else {
                quotient = Byte::sub(quotient, Byte::tern(), Byte::zero()).0;
                remainder = Byte::sub(remainder, rhs, Byte::zero()).0;
            }
        }
        // println!("quotient result: {}", quotient);
        // println!("remainder result: {}\n", remainder);
        (quotient, remainder)
    }

    fn shift(a: Byte, amount: isize) -> (Byte, Byte) {
        if amount.abs() > IBYTE_WIDTH*2 {
            return (Byte::zero(), Byte::zero());
        }
        let mut trits = [Trit::Zero; BYTE_WIDTH*5];
        trits[BYTE_WIDTH*2..BYTE_WIDTH*3].copy_from_slice(&a.trits);
        let index = (IBYTE_WIDTH*2 - amount) as usize;
        let result = Byte::from_slice(&trits[index..index+BYTE_WIDTH]);
        let carry = if amount >= 0 {
            Byte::from_slice(&trits[index+BYTE_WIDTH..index+BYTE_WIDTH*2])
        } else {
            Byte::from_slice(&trits[index-BYTE_WIDTH..index])
        };
        (result, carry)
    }
}

impl fmt::Display for Byte {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        assert_eq!(BYTE_WIDTH, 7);
        let mst = self.highest_mst();
        for i in (0..=mst).rev() {
            write!(f, "{}", self.trits[i])?;
        }
        Ok(())
    }
}

impl From<i64> for Byte {
    fn from(mut item: i64) -> Self {
        assert!(item <= MAX_BALANCED_VALUE);
        assert!(item >= MIN_BALANCED_VALUE);
        let mut trits = [Trit::Zero; BYTE_WIDTH];
        for i in 0..BYTE_WIDTH {
            let mut trit = item % 3;
            if trit < 0 {
                trit += 3;
            }
            if trit == 2 {
                trit = -1i64;
            }
            trits[i] = match trit {
                -1 => Trit::Tern,
                0 => Trit::Zero,
                1 => Trit::One,
                _ => Trit::Zero,
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
        for i in 0..BYTE_WIDTH {
            let value: Self = item.trits[i].into();
            result += value * Self::pow(3, i as u32)
        }
        result
    }
}

#[test]
fn test_add() {
    for i in MIN_BALANCED_VALUE..=MAX_BALANCED_VALUE {
        for j in MIN_BALANCED_VALUE..=MAX_BALANCED_VALUE {
            if i+j < MIN_BALANCED_VALUE { continue; }
            if i+j > MAX_BALANCED_VALUE { continue; }
            assert_eq!(i+j, i64::from(Byte::add(Byte::from(i), Byte::from(j), Byte::zero()).0));
        }
    }
}

#[test]
fn test_sub() {
    for i in MIN_BALANCED_VALUE..=MAX_BALANCED_VALUE {
        for j in MIN_BALANCED_VALUE..=MAX_BALANCED_VALUE {
            if i-j < MIN_BALANCED_VALUE { continue; }
            if i-j > MAX_BALANCED_VALUE { continue; }
            assert_eq!(i-j, i64::from(Byte::sub(Byte::from(i), Byte::from(j), Byte::zero()).0));
        }
    }
}

#[test]
fn test_mul() {
    for i in MIN_BALANCED_VALUE..=MAX_BALANCED_VALUE {
        for j in MIN_BALANCED_VALUE..=MAX_BALANCED_VALUE {
            let result = Byte::mul(Byte::from(i), Byte::from(j));
            assert_eq!(i * j, i64::from(result.0) + 2187*i64::from(result.1));
        }
    }
}

#[test]
fn test_greater_dfz() {
    for i in MIN_BALANCED_VALUE..=MAX_BALANCED_VALUE {
        for j in MIN_BALANCED_VALUE..=MAX_BALANCED_VALUE {
            let result = Byte::greater_dfz(Byte::from(i), Byte::from(j));
            assert_eq!(result, i.abs() > j.abs());
        }
    }
}

#[test]
fn test_conversion() {
    for i in MIN_BALANCED_VALUE..=MAX_BALANCED_VALUE {
        assert_eq!(i, i64::from(Byte::from(i)));
    }
}

#[test]
fn test_add_overflow() {
    let min = Byte::from(MIN_BALANCED_VALUE);
    let max = Byte::from(MAX_BALANCED_VALUE);
    assert_eq!(max, Byte::add(min, Byte::from(-1i64), Byte::zero()).0);
    assert_eq!(min, Byte::add(max, Byte::from(1i64), Byte::zero()).0);
}

#[test]
fn test_shift() {
    for i in MIN_BALANCED_VALUE..=MAX_BALANCED_VALUE {
        for j in 0..IBYTE_WIDTH {
            let (result, carry) = Byte::shift(Byte::from(i), j);
            assert_eq!(i64::from(result) + i64::from(carry)*3i64.pow(7), i * 3i64.pow(j as u32));
        }
        for j in -IBYTE_WIDTH..0 {
            let (result, _) = Byte::shift(Byte::from(i), j);
            assert_eq!(i64::from(result), (f64::from(i as i32) / f64::from(3i32.pow(j.abs() as u32))).round() as i64);
        }
    }
}

#[test]
fn test_div() {
    for i in MIN_BALANCED_VALUE..=MAX_BALANCED_VALUE {
        for j in MIN_BALANCED_VALUE..=MAX_BALANCED_VALUE {
            if j == 0 { continue; }
            let result = Byte::div(Byte::from(i), Byte::from(j)).0;
            let test = f64::from(i as i32) / f64::from(j as i32);
            assert_eq!(i64::from(result), test.round() as i64);
        }
    }
}
