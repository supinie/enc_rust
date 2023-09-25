#[cfg(test)]
mod sample_tests {
    use crate::{params::*, poly::*};
    use std::{ops::Range, collections::HashMap};
    use rand::{Rng, SeedableRng};
    use rand::rngs::StdRng;

    const EPSILON: f64 = 0.1;

    fn generate_random_seed() -> [u8; 32] {
        let mut rng = StdRng::from_entropy();
        let mut seed = [0u8; 32];
        rng.fill(&mut seed);
        seed
    }

    fn generate_random_nonce() -> u8 {
        let mut rng =StdRng::from_entropy();
        rng.gen::<u8>()
    }

    #[test]
    fn derive_noise_2_range_test() {
        let range: Range<i16> = -2..3;
        let mut poly = Poly::new();
        let seed = generate_random_seed();
        let nonce = generate_random_nonce();

        poly.derive_noise_2(&seed, nonce);

        for coeff in poly.coeffs.iter() {
            println!("{}", coeff);
            assert!(range.contains(coeff), "coefficient {} not in valid range", coeff);
        }
    }


    #[test]
    fn derive_noise_3_range_test() {
        let range: Range<i16> = -3..4;
        let mut poly = Poly::new();
        let seed = generate_random_seed();
        let nonce = generate_random_nonce();
        
        poly.derive_noise_3(&seed, nonce);

        for coeff in poly.coeffs.iter() {
            println!("{}", coeff);
            assert!(range.contains(coeff), "coefficient {} not in valid range", coeff);
        }
    }


    #[test]
    fn derive_noise_2_dist_test() {
        let expected_probabilities: HashMap<i16, f64> = [
            (-2, 1.0 / 16.0),
            (-1, 1.0 / 4.0),
            (0, 3.0 / 8.0),
            (1, 1.0 / 4.0),
            (2, 1.0 / 16.0),
        ].iter().cloned().collect();

        let mut poly = Poly::new();
        let seed = generate_random_seed();
        let nonce = generate_random_nonce();

        poly.derive_noise_2(&seed, nonce);

        let mut actual_counts: HashMap<i16, usize> = HashMap::new();
        for &coeff in poly.coeffs.iter() {
            *actual_counts.entry(coeff).or_insert(0) += 1;
        }

        for (coeff, expected_prob) in expected_probabilities.iter() {
            let actual_count = *actual_counts.get(coeff).unwrap_or(&0);
            let total_samples = poly.coeffs.len() as f64;
            let actual_prob = (actual_count as f64) / total_samples;


            assert!((actual_prob - expected_prob).abs() < EPSILON, "Actual probability {} does not match expected {} within boundries for coeff {}", actual_prob, expected_prob, coeff);
        }
    }


    #[test]
    fn derive_noise_3_dist_test() {
        let expected_probabilities: HashMap<i16, f64> = [
            (-3, 1.0 / 64.0),
            (-2, 3.0 / 32.0),
            (-1, 15.0 / 64.0),
            (0, 5.0 / 16.0),
            (1, 16.0 / 64.0),
            (2, 3.0 / 32.0),
            (3, 1.0 / 64.0),
        ].iter().cloned().collect();

        let mut poly = Poly::new();
        let seed = generate_random_seed();
        let nonce = generate_random_nonce();

        poly.derive_noise_3(&seed, nonce);

        let mut actual_counts: HashMap<i16, usize> = HashMap::new();
        for &coeff in poly.coeffs.iter() {
            *actual_counts.entry(coeff).or_insert(0) += 1;
        }

        for (coeff, expected_prob) in expected_probabilities.iter() {
            let actual_count = *actual_counts.get(coeff).unwrap_or(&0);
            let total_samples = poly.coeffs.len() as f64;
            let actual_prob = (actual_count as f64) / total_samples;


            assert!((actual_prob - expected_prob).abs() < EPSILON, "Actual probability {} does not match expected {} within boundries for coeff {}", actual_prob, expected_prob, coeff);
        }
    }
}
