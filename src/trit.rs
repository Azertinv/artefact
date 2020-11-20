use std::fmt;
use super::operation::Operation;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Trit {
    val: i8,
}

impl Trit {
    pub const ZERO: Trit = Trit{val: 0i8};
    pub const ONE: Trit = Trit{val: 1i8};
    pub const TERN: Trit = Trit{val: -1i8};
    pub const MIN_VALUE: i8 = -1;
    pub const MAX_VALUE: i8 = 1;
}

impl Operation for Trit {
    fn not(a: Trit) -> Trit {
        assert!(a.val >= Trit::MIN_VALUE && a.val <= Trit::MAX_VALUE);
        Trit{val: -a.val}
    }

    fn and(lhs: Trit, rhs: Trit) -> Trit {
        if lhs == Trit::TERN || rhs == Trit::TERN {
            Trit::TERN
        } else if lhs == Trit::ZERO || rhs == Trit::ZERO {
            Trit::ZERO
        } else {
            Trit::ONE
        }
    }

    fn or(lhs: Trit, rhs: Trit) -> Trit {
        if lhs == Trit::ONE || rhs == Trit::ONE {
            Trit::ONE
        } else if lhs == Trit::ZERO || rhs == Trit::ZERO {
            Trit::ZERO
        } else {
            Trit::TERN
        }
    }

    fn xor(lhs: Trit, rhs: Trit) -> Trit {
        if lhs != Trit::ZERO && rhs != Trit::ZERO {
            if lhs != rhs {
                Trit::ONE
            } else {
                Trit::TERN
            }
        } else {
            Trit::ZERO
        }
    }

    fn shift(a: Trit, amount: isize) -> (Trit, Trit) {
        match amount.abs() {
            0 => (a, Trit::ZERO),
            1 => (Trit::ZERO, a),
            _ => (Trit::ZERO, Trit::ZERO),
        }
    }

    fn greater_dfz(lhs: Trit, rhs: Trit) -> bool {
        if lhs == Trit::ZERO {
            false
        } else if rhs == Trit::ZERO {
            true
        } else {
            false
        }
    }

    fn half_add(lhs: Trit, rhs: Trit) -> (Trit, Trit) {
        match lhs.val + rhs.val {
            -2i8 => (Trit::ONE, Trit::TERN),
            -1i8 => (Trit::TERN, Trit::ZERO),
            0i8 => (Trit::ZERO, Trit::ZERO),
            1i8 => (Trit::ONE, Trit::ZERO),
            2i8 => (Trit::TERN, Trit::ONE),
            _ => panic!(),
        }
    }

    fn mul(lhs: Trit, rhs: Trit) -> (Trit, Trit) {
        if lhs == Trit::ZERO || rhs == Trit::ZERO {
            (Trit::ZERO, Trit::ZERO)
        } else if lhs == Trit::TERN {
            (Trit::not(rhs), Trit::ZERO)
        } else if rhs == Trit::TERN {
            (Trit::not(lhs), Trit::ZERO)
        } else {
            (Trit::ONE, Trit::ZERO)
        }
    }

    fn div(lhs: Trit, rhs: Trit) -> (Trit, Trit) {
        assert_ne!(rhs, Trit::ZERO);
        if lhs == Trit::ZERO {
            (Trit::ZERO, Trit::ZERO)
        } else if lhs == rhs{
            (Trit::ONE, Trit::ZERO)
        } else {
            (Trit::TERN, Trit::ZERO)
        }
    }
}

impl From<Trit> for i64 {
    fn from(item: Trit) -> Self {
        item.val as i64
    }
}

pub trait FromTrits {
    fn from_trits(trits: &[Trit]) -> Self;
}

impl FromTrits for i64 {
    fn from_trits(item: &[Trit]) -> Self {
        if item.len() == 0 {
            0i64
        } else {
            i64::from(item[0]) + 3 * i64::from_trits(&item[1..])
        }
    }
}

impl fmt::Display for Trit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", match self.val {
            0 => "0",
            1 => "1",
            -1 => "T",
            _ => panic!(),
        })
    }
}

#[test]
fn test_add() {
    for lhs in [Trit::ZERO, Trit::ONE, Trit::TERN].iter() {
        for rhs in [Trit::ZERO, Trit::ONE, Trit::TERN].iter() {
            for carry_in in [Trit::ZERO, Trit::ONE, Trit::TERN].iter() {
                let (result, carry_out) = Trit::add(*lhs, *rhs, *carry_in);
                assert_eq!(i64::from(result) + i64::from(carry_out)*3,
                        i64::from(*lhs) + i64::from(*rhs) + i64::from(*carry_in));
            }
        }
    }
}

#[test]
fn test_size() {
    assert_eq!(std::mem::size_of::<Trit>(), 1);
}
