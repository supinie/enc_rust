use crate::{
    matrix::{Mat512, MatOperations, New},
    params::{GetSecLevel, POLYBYTES},
    polynomials::Poly,
    vectors::{LinkSecLevel, PolyVecOperations},
};
use sha3::{Digest, Sha3_512};

#[derive(Default)]
pub struct PrivateKey<PV: PolyVecOperations + GetSecLevel + Default> {
    secret: PV,
}

#[derive(Default)]
pub struct PublicKey<PV: PolyVecOperations + GetSecLevel + Default, M: MatOperations + GetSecLevel + LinkSecLevel<PV> + New> {
    rho: [u8; 32],
    noise: PV,
    a_t: M,
}

impl<PV: PolyVecOperations + GetSecLevel + Default> PrivateKey<PV> {
    pub fn pack(&self, buf: &mut [u8]) {
        self.secret.pack(buf);
    }

    pub fn unpack(&mut self, buf: &[u8]) {
        self.secret.unpack(buf);
        self.secret.normalise();
    }
}

impl<PV: PolyVecOperations + GetSecLevel + Default, M: MatOperations + GetSecLevel + LinkSecLevel<PV> + New> PublicKey<PV, M> {
    pub fn pack(&self, buf: &mut [u8]) {
        self.noise.pack(buf);
        let k_value: u8 = M::sec_level().k().into();
        let start_index = usize::from(k_value) * POLYBYTES;
        buf[start_index..].copy_from_slice(&self.rho[..]);
    }

    pub fn unpack(&mut self, buf: &[u8]) {
        self.noise.unpack(buf);
        self.noise.normalise();
        let k_value: u8 = M::sec_level().k().into();
        let start_index = usize::from(k_value) * POLYBYTES;
        self.rho[..].copy_from_slice(&buf[start_index..]);
        self.a_t = M::derive(&self.rho, true);
    }
}

pub fn generate_key_pair<PV, M>(seed: &[u8]) -> (PrivateKey<PV>, PublicKey<PV, M>)
where
    PV: PolyVecOperations + GetSecLevel + Default + Iterator<Item = Poly> + Copy,
    M: MatOperations + GetSecLevel + LinkSecLevel<PV> + New + Iterator<Item = PV> + Copy,
{
    let mut pub_key = PublicKey {
        rho: [0u8; 32],
        noise: PV::default(),
        a_t: M::new()
    };
    let mut priv_key = PrivateKey {
        secret: PV::default()
    };

    let mut expanded_seed = [0u8; 64];
    let mut hash = Sha3_512::new();
    hash.update(seed);

    expanded_seed.copy_from_slice(&hash.finalize());

    pub_key.rho[..].copy_from_slice(&expanded_seed[..32]);
    pub_key.a_t = M::derive(&pub_key.rho, false);
    let sigma = &expanded_seed[32..]; // seed for noise
    
    priv_key.secret.derive_noise(sigma, 0, PV::sec_level().eta_1());
    priv_key.secret.ntt();
    priv_key.secret.normalise();

    for (mut poly, vec) in pub_key.noise.zip(pub_key.a_t) {
        poly.inner_product_pointwise(vec, priv_key.secret);
        poly.mont_form();
    }

    let mut error = PV::default();
    let k_value: u8 = M::sec_level().k().into();
    error.derive_noise(sigma, k_value, M::sec_level().eta_1());
    error.ntt();

    pub_key.noise.add(error);
    pub_key.noise.normalise();

    pub_key.a_t.transpose();

    (priv_key, pub_key)
}
    

// fn test() {
//     let pub_key = PublicKey {
//         rho: [0u8; 32],
//         noise: PolyVec512::from([Poly::new(); 2]),
//         a_t: [PolyVec512::from([Poly::new(); 2]); 2]
//     };

//     let invalid_key = PublicKey {
//         rho: [0u8; 32],
//         noise: PolyVec768::from([Poly::new(); 3]),
//         a_t: [PolyVec512::from([Poly::new(); 2]); 2]
//     };
// }
