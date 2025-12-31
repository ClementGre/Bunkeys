use std::fmt::{Display, Formatter};
use num_bigint::{BigUint, RandBigInt};
use num_traits::FromPrimitive;
use rand::rngs::OsRng;
use crate::MODULUS_128;

pub struct Polynom {
    pub coefficients: Vec<BigUint>,
}

impl Polynom {
    pub fn new(coefficients: Vec<BigUint>) -> Self {
        Polynom { coefficients }
    }

    /// Creates a random polynom of degree deg
    /// The coefficients are random non-zero BigUint numbers modulo MODULUS_128
    pub fn new_random_of_degree_with_constant_term(deg: usize, constant_term: BigUint) -> Self {
        let mut rng = OsRng::default();

        let mut coefficients = Vec::with_capacity(deg + 1);
        coefficients.push(constant_term);
        for _ in 1..=deg {
            let mut coef = BigUint::ZERO;
            while coef == BigUint::ZERO {
                coef = rng.gen_biguint(128) % MODULUS_128.clone();
            }
            coefficients.push(coef);
        }
        Polynom { coefficients }
    }

    /// Returns the first n points of the polynom: f(1), f(2), ..., f(n)
    pub fn get_firsts_n_points(&self, n: usize) -> Vec<(BigUint, BigUint)> {
        let mut points = Vec::with_capacity(n);
        for x_us in 1..=n {
            let x= BigUint::from_usize(x_us).unwrap();
            let y = self.calculate_y(x.clone());
            points.push((x, y));
        }
        points
    }

    pub fn calculate_y(&self, x: BigUint) -> BigUint {
        let mut y = BigUint::ZERO;
        let mut x_pow = BigUint::from_usize(1).unwrap(); // x^0 = 1
        for coef in &self.coefficients {
            y = (y + coef * &x_pow) % MODULUS_128.clone();
            x_pow = (x_pow * &x) % MODULUS_128.clone(); // x^i = x^(i-1) * x
        }
        y
    }
}

impl Display for Polynom {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let str = self.coefficients.iter().enumerate().map(|(i, coef)| {
            if (i == 0) {
                coef.to_string()
            }else {
                format!("{}x^{}", coef, i)
            }
        }).collect::<Vec<String>>().join(" + ");
        write!(f, "{} ", str)?;
        Ok(())
    }
}
