use bip39::Mnemonic;
use lazy_static::lazy_static;
use num_bigint::{BigUint, RandBigInt};
use num_traits::FromPrimitive;
use rand::rngs::OsRng;
use crate::polynom;

lazy_static! {
    // 128-bit prime modulus (2¹²⁷ - 1)
    pub static ref MODULUS_128: BigUint = {
        let prime_128_str: &str = "170141183460469231731687303715884105727"; // 2¹²⁷ - 1
        let modulus_128_parsed = BigUint::parse_bytes(prime_128_str.as_bytes(), 10).unwrap();
        let modulus_128_calculated = BigUint::from_u64(2).unwrap().pow(127) - BigUint::from_u64(1).unwrap();
        assert_eq!(modulus_128_parsed, modulus_128_calculated);
        modulus_128_parsed
    };
}

pub fn shamir_test() {
    let mut rng = OsRng::default();
    let secret = rng.gen_biguint(127);
    let secret_mnemonic = Mnemonic::from_entropy(&secret.to_bytes_be()).unwrap().to_string();
    let points_number: u64 = 6;
    let threshold: usize = 3;

    println!("Prime: {}", *MODULUS_128);
    println!("Secret: {}", secret);
    println!("Secret mnemonic: {}", secret_mnemonic);

    // A polynom of degree threshold - 1 can be determined by threshold points.
    let polynom = polynom::Polynom::new_random_of_degree_with_constant_term(threshold - 1, secret.clone());
    let points = polynom.get_firsts_n_points(points_number as usize);

    println!("Polynom: {}", polynom);
    println!(
        "Points: {}",
        points
            .iter()
            .map(|(x, y)| format!("({}, {})", x, y))
            .collect::<Vec<String>>()
            .join(", ")
    );

    // Generate back the secret
    let ans = get_polynom_constant_value(&points);
    println!("Reconstituted secret: {}", ans);
    println!("Secret: {}", polynom.coefficients[0]);


    // Testing that all combinations of threshold points are enough to find the secret
    use itertools::Itertools;
    for combo in points.iter().combinations(threshold) {
        let combo_points: Vec<(BigUint, BigUint)> = combo.into_iter().map(|(x, y)| (x.clone(), y.clone())).collect();
        let secret_reconstituted = get_polynom_constant_value(&combo_points);
        //println!("Secret recostituted from points {} is {}", combo_points.iter().map(|(x, y)| format!("({}, {})", x, y)).collect::<Vec<String>>().join(", "), secret_reconstituted);
        assert_eq!(secret_reconstituted, secret);
        let secret_mnemonic_reconstituted = Mnemonic::from_entropy(&secret_reconstituted.to_bytes_be()).unwrap().to_string();
        assert_eq!(secret_mnemonic, secret_mnemonic_reconstituted);
    }
}

// Reconstructs the constant term of a polynomial from shares using modular arithmetic.
pub fn get_polynom_constant_value(points: &[(BigUint, BigUint)]) -> BigUint {
    let mut secret = BigUint::ZERO;
    for (i, (xi, yi)) in points.iter().enumerate() {
        let mut numerator = BigUint::from_usize(1).unwrap();
        let mut denominator = BigUint::from_usize(1).unwrap();

        // Compute Lagrange coefficients: ∏ (xj / (xj - xi)) for j ≠ i
        for (j, (xj, _)) in points.iter().enumerate() {
            if i != j {
                // Numerator: ∏ (xj)
                numerator = (numerator * xj) % &*MODULUS_128;

                // Denominator: ∏ (xj - xi)
                let diff = (xj + (&*MODULUS_128 - xi)) % &*MODULUS_128;
                denominator = (denominator * diff) % &*MODULUS_128;
            }
        }

        // Lagrange coefficient: yi * (numerator / denominator) mod modulus
        let denominator_inv = &denominator.modinv(&MODULUS_128).unwrap();
        let lagrange_coef = (yi * numerator * denominator_inv) % &*MODULUS_128;

        // Accumulate the secret: secret += lagrange_coef
        secret = (secret + lagrange_coef) % &*MODULUS_128;
    }
    secret
}
