use std::fmt;
use super::operation::Operation;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Trit {
    Zero,
    One,
    Tern,
}

impl Operation for Trit {
    fn not(a: Trit) -> Trit {
        match a {
            Trit::Tern => Trit::One,
            Trit::Zero => Trit::Zero,
            Trit::One => Trit::Tern,
        }
    }

    fn and(lhs: Trit, rhs: Trit) -> Trit {
        if lhs == Trit::Tern || rhs == Trit::Tern {
            Trit::Tern
        } else if lhs == Trit::Zero || rhs == Trit::Zero {
            Trit::Zero
        } else {
            Trit::One
        }
    }

    fn or(lhs: Trit, rhs: Trit) -> Trit {
        if lhs == Trit::One || rhs == Trit::One {
            Trit::One
        } else if lhs == Trit::Zero || rhs == Trit::Zero {
            Trit::Zero
        } else {
            Trit::Tern
        }
    }

    fn shift(a: Trit, amount: isize) -> (Trit, Trit) {
        match amount.abs() {
            0 => (a, Trit::Zero),
            1 => (Trit::Zero, a),
            _ => (Trit::Zero, Trit::Zero),
        }
    }

    fn greater_dfz(lhs: Trit, rhs: Trit) -> bool {
        if lhs == Trit::Zero {
            false
        } else if rhs == Trit::Zero {
            true
        } else {
            false
        }
    }

    fn half_add(lhs: Trit, rhs: Trit) -> (Trit, Trit) {
        match (lhs, rhs) {
            (Trit::Zero, Trit::Zero)|(Trit::One, Trit::Tern)|(Trit::Tern, Trit::One) => (Trit::Zero, Trit::Zero),
            (Trit::Tern, Trit::Zero)|(Trit::Zero, Trit::Tern) => (Trit::Tern, Trit::Zero),
            (Trit::One, Trit::Zero)|(Trit::Zero, Trit::One) => (Trit::One, Trit::Zero),
            (Trit::Tern, Trit::Tern) => (Trit::One, Trit::Tern),
            (Trit::One, Trit::One) => (Trit::Tern, Trit::One),
        }
    }

    fn mul(lhs: Trit, rhs: Trit) -> (Trit, Trit) {
        if lhs == Trit::Zero || rhs == Trit::Zero {
            (Trit::Zero, Trit::Zero)
        } else if lhs == Trit::Tern {
            (Trit::not(rhs), Trit::Zero)
        } else if rhs == Trit::Tern {
            (Trit::not(lhs), Trit::Zero)
        } else {
            (Trit::One, Trit::Zero)
        }
    }

    fn div(lhs: Trit, rhs: Trit) -> (Trit, Trit) {
        assert_ne!(rhs, Trit::Zero);
        if lhs == Trit::Zero {
            (Trit::Zero, Trit::Zero)
        } else if lhs == rhs{
            (Trit::One, Trit::Zero)
        } else {
            (Trit::Tern, Trit::Zero)
        }
    }
}

impl From<Trit> for i64 {
    fn from(item: Trit) -> Self {
        match item {
            Trit::Zero => 0i64,
            Trit::One => 1i64,
            Trit::Tern => -1i64,
        }
    }
}

impl fmt::Display for Trit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", match self {
            Trit::Zero => "0",
            Trit::One => "1",
            Trit::Tern => "T",
        })
    }
}

#[test]
fn test_add() {
    for lhs in [Trit::Zero, Trit::One, Trit::Tern].iter() {
        for rhs in [Trit::Zero, Trit::One, Trit::Tern].iter() {
            for carry_in in [Trit::Zero, Trit::One, Trit::Tern].iter() {
                let (result, carry_out) = Trit::add(*lhs, *rhs, *carry_in);
                assert_eq!(i64::from(result) + i64::from(carry_out)*3,
                        i64::from(*lhs) + i64::from(*rhs) + i64::from(*carry_in));
            }
        }
    }
}
