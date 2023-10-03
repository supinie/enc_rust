#[cfg(test)]
mod params_tests {
    use crate::params::*;

    #[test]
    fn sec_level_test() {
        assert_eq!(
            Params::sec_level_512(),
            Params {
                k: 2,
                eta1: 3,
                eta2: 2
            }
        );
        assert_eq!(
            Params::sec_level_768(),
            Params {
                k: 3,
                eta1: 2,
                eta2: 2
            }
        );
        assert_eq!(
            Params::sec_level_1024(),
            Params {
                k: 4,
                eta1: 2,
                eta2: 2
            }
        );
    }

    #[test]
    fn poly_compressed_bytes_test() {
        assert_eq!(Params::sec_level_512().poly_compressed_bytes().unwrap(), 128);
        assert_eq!(Params::sec_level_768().poly_compressed_bytes().unwrap(), 128);
        assert_eq!(Params::sec_level_1024().poly_compressed_bytes().unwrap(), 160);
    }

    #[test]
    fn poly_vec_bytes_test() {
        assert_eq!(Params::sec_level_512().poly_vec_bytes(), 768);
        assert_eq!(Params::sec_level_768().poly_vec_bytes(), 1152);
        assert_eq!(Params::sec_level_1024().poly_vec_bytes(), 1536);
    }

    #[test]
    fn poly_vec_compressed_bytes_test() {
        assert_eq!(Params::sec_level_512().poly_vec_compressed_bytes().unwrap(), 640);
        assert_eq!(Params::sec_level_768().poly_vec_compressed_bytes().unwrap(), 960);
        assert_eq!(Params::sec_level_1024().poly_vec_compressed_bytes().unwrap(), 1408);
    }

    #[test]
    fn indcpa_public_key_bytes_test() {
        assert_eq!(Params::sec_level_512().indcpa_public_key_bytes(), 800);
        assert_eq!(Params::sec_level_768().indcpa_public_key_bytes(), 1184);
        assert_eq!(Params::sec_level_1024().indcpa_public_key_bytes(), 1568);
    }

    #[test]
    fn indcpa_private_key_bytes_test() {
        assert_eq!(Params::sec_level_512().indcpa_private_key_bytes(), 768);
        assert_eq!(Params::sec_level_768().indcpa_private_key_bytes(), 1152);
        assert_eq!(Params::sec_level_1024().indcpa_private_key_bytes(), 1536);
    }

    #[test]
    fn indcpa_bytes_test() {
        assert_eq!(Params::sec_level_512().indcpa_bytes(), 768);
        assert_eq!(Params::sec_level_768().indcpa_bytes(), 1088);
        assert_eq!(Params::sec_level_1024().indcpa_bytes(), 1568);
    }

    #[test]
    fn public_key_bytes_test() {
        assert_eq!(Params::sec_level_512().public_key_bytes(), 800);
        assert_eq!(Params::sec_level_768().public_key_bytes(), 1184);
        assert_eq!(Params::sec_level_1024().public_key_bytes(), 1568);
    }

    #[test]
    fn private_key_bytes_test() {
        assert_eq!(Params::sec_level_512().private_key_bytes(), 1632);
        assert_eq!(Params::sec_level_768().private_key_bytes(), 2400);
        assert_eq!(Params::sec_level_1024().private_key_bytes(), 3168);
    }

    #[test]
    fn cipher_text_bytes_test() {
        assert_eq!(Params::sec_level_512().cipher_text_bytes(), 768);
        assert_eq!(Params::sec_level_768().cipher_text_bytes(), 1088);
        assert_eq!(Params::sec_level_1024().cipher_text_bytes(), 1568);
    }
}
