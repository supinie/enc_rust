#[cfg(test)]
mod matrix_tests {
    use crate::{
        matrix::*,
        params::{SecurityLevel, K},
    };

    static TEST_PARAMS: [SecurityLevel; 3] = [
        SecurityLevel::new(K::Two),
        SecurityLevel::new(K::Three),
        SecurityLevel::new(K::Four),
    ];
}
