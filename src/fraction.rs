use std::ops::{Add, AddAssign, DivAssign, Mul, Rem};
use num_traits::Zero;

pub struct Fraction<T>
where
    T: Add<Output = T> + Mul<Output = T> + DivAssign + Rem<Output = T> + PartialEq + Copy + Zero,
{
    pub numerator: T,
    pub denominator: T,
}

impl<T> Fraction<T>
where
    T: Add<Output = T> + Mul<Output = T> + DivAssign + Rem<Output = T> + PartialEq + Copy + Zero,
{
    pub fn new(numerator: T, denominator: T) -> Self {
        Fraction {
            numerator,
            denominator,
        }
    }

    pub fn reduce(&mut self) {
        let gcd = Self::gcd(self.numerator, self.denominator);
        self.numerator /= gcd;
        self.denominator /= gcd;
    }

    fn gcd(mut a: T, mut b: T) -> T {
        while b.is_zero() {
            let r = a % b;
            a = b;
            b = r;
        }
        a
    }
}

impl<T> Add for Fraction<T>
where
    T: Add<Output = T> + Mul<Output = T> + DivAssign + Rem<Output = T> + PartialEq + Copy + Zero,
{
    type Output = Self;
    fn add(self, other: Self) -> Self::Output {
        Fraction::new(
            self.numerator * other.denominator + other.numerator * self.denominator,
            self.denominator * other.denominator,
        )
    }
}

impl<T> Mul for Fraction<T>
where
    T: Add<Output = T> + Mul<Output = T> + DivAssign + Rem<Output = T> + PartialEq + Copy + Zero,
{
    type Output = Self;
    fn mul(self, other: Self) -> Self::Output {
        Fraction::new(
            self.numerator * other.numerator,
            self.denominator * other.denominator,
        )
    }
}

impl AddAssign for Fraction<i64> {
    fn add_assign(&mut self, other: Self) {
        self.numerator = self.numerator * other.denominator + other.numerator * self.denominator;
        self.denominator = self.denominator * other.denominator;
    }
}
