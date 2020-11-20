pub trait Operation: Sized + std::fmt::Display {
    fn not(a: Self) -> Self;
    fn and(lhs: Self, rhs: Self) -> Self;
    fn or(lhs: Self, rhs: Self) -> Self;
    fn xor(lhs: Self, rhs: Self) -> Self;
    fn shift(a: Self, amount: isize) -> (Self, Self);

    // dfz stands for distance from zero
    fn greater_dfz(lhs: Self, rhs: Self) -> bool;

    fn half_add(lhs: Self, rhs: Self) -> (Self, Self);

    fn mul(lhs: Self, rhs: Self) -> (Self, Self);
    fn div(lhs: Self, rhs: Self) -> (Self, Self); // (quotient, remainder)

    fn sub(lhs: Self, rhs: Self, carry_in: Self) -> (Self, Self) {
        Self::add(lhs, Self::not(rhs), carry_in)
    }

    fn add(lhs: Self, rhs: Self, carry_in: Self) -> (Self, Self) {
        let (half_result, half_carry) = Self::half_add(lhs, rhs);
        let (result, carry_out) = Self::half_add(half_result, carry_in);
        (result, Self::half_add(half_carry, carry_out).0)
    }
}
