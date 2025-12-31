use std::ops::AddAssign;
use rand::rand_core::OsRng;
use rand::TryRngCore;
use crate::fraction::Fraction;
use bigint::{M256, U256};
mod polynom;
mod fraction;

// 128-bit prime modulus (2¹²⁷ - 1)
const PRIME_256: U256 = U256([0x7FFFFFFFFFFFFFFF, 0, 0, 0]);

fn main() {

    let secret: u64 = OsRng::default().try_next_u32().unwrap() as u64;
    let points_number: u64 = 6;
    let threshold: usize = 3;

    // A polynom of degree threshold - 1 can be determined by threshold points.
    let polynom = polynom::Polynom::new_random_of_degree_with_constant_term(threshold - 1, secret);
    let points = polynom.get_firsts_n_points(points_number as usize);

    println!("Polynom: {}", polynom);
    println!("Points: {}", points.iter().map(|(x, y)| format!("({}, {})", x, y)).collect::<Vec<String>>().join(", "));

    // Generate back the secret
    let ans = get_polynom_constant_value(&points);

    println!("Reconstituted secret: {}", ans);
    println!("Secret: {}", polynom.coefficients[0]);

    // Testing that all combinations of threshold points are enough to find the secret
    use itertools::Itertools;
    for combo in points.iter().combinations(threshold) {
        let combo_points: Vec<(u64, u64)> = combo.into_iter().map(|(x, y)| (*x, *y)).collect();
        let secret_reconstituted = get_polynom_constant_value(&combo_points);
        println!("Secret recostituted from points {} is {}", combo_points.iter().map(|(x, y)| format!("({}, {})", x, y)).collect::<Vec<String>>().join(", "), secret_reconstituted);
        assert_eq!(secret_reconstituted as u64, secret);
    }
}

pub fn get_polynom_constant_value(points: &[(u64, u64)]) -> i64 {
    let mut ans: Fraction<i64> = Fraction::new(0, 1);
    // Loop to iterate through the given points
    for (x, y) in points.iter() {
        let mut frac = Fraction::new(*y as i64, 1);

        for (xl, yl) in points.iter() {
            // Computing the lagrange terms
            if xl != x {
                frac = frac * Fraction::new(-(*xl as i64), *x as i64 - *xl as i64)
            }
        }
        ans += frac;
    }
    ans.numerator / ans.denominator
}
