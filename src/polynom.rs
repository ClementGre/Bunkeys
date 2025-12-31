use std::fmt::{Display, Formatter};
use rand::rngs::OsRng;
use rand::TryRngCore;

pub struct Polynom {
    pub coefficients: Vec<u64>,
}

impl Polynom {
    pub fn new(coefficients: Vec<u64>) -> Self {
        Polynom { coefficients }
    }

    /// Creates a random polynom of degree deg
    /// The coefficients are random non-zero u32 numbers modulo 997
    pub fn new_random_of_degree_with_constant_term(deg: usize, constant_term: u64) -> Self {
        let mut rng = OsRng::default();

        let mut coefficients = Vec::with_capacity(deg + 1);
        coefficients.push(constant_term);
        for _ in 1..=deg {
            let mut coef: u32 = 0;
            while coef == 0 {
                coef = rng.try_next_u32().unwrap() % 997;
            }
            coefficients.push(coef as u64);
        }
        Polynom { coefficients }
    }

    /// Returns the first n points of the polynom: f(1), f(2), ..., f(n)
    pub fn get_firsts_n_points(&self, n: usize) -> Vec<(u64, u64)> {
        let mut points = Vec::with_capacity(n);
        for x in 1..=n as u64 {
            let y = self.calculate_y(x);
            points.push((x, y));
        }
        points
    }

    pub fn calculate_y(&self, x: u64) -> u64 {
        let mut y = 0;
        for (i, &coef) in self.coefficients.iter().enumerate() {
            y += coef * x.pow(i as u32);
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
