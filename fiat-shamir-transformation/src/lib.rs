use std::marker::PhantomData;

use ark_ff::PrimeField;
use sha3::Digest;


#[derive(Debug)]
pub struct FiatShamir<K, F: PrimeField> {
   pub hash_function: K,
   pub transcript: PhantomData<F>, //all the random challenges
}
impl<K: Digest + Clone, F: PrimeField> FiatShamir<K, F> {
   pub fn new(hash_function: K) -> Self {
        FiatShamir {
            hash_function,
            transcript: PhantomData,
        }
    }
   pub fn absorb(&mut self, input: &[u8]) {
       self.hash_function.update(input);
    
    }
    pub fn squeeze(&mut self) -> F{
        let result = self.hash_function.clone().finalize();

        let result_bytes: Vec<u8> = result.to_vec();
        self.absorb(&result_bytes);
    
        let result_field = F::from_le_bytes_mod_order(&result_bytes);
        result_field
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use ark_bn254::Fq;
    use sha3::{Digest, Sha3_256};

    #[test]
    fn test_fiat_shamir() {
        let hash_function = Sha3_256::new();
        let hash_function2 = Sha3_256::new();
        let mut  first: FiatShamir<sha3::digest::core_api::CoreWrapper<sha3::Sha3_256Core>, Fq> =
            FiatShamir::new(hash_function2);
        let  mut  second: FiatShamir<sha3::digest::core_api::CoreWrapper<sha3::Sha3_256Core>, Fq> =
            FiatShamir::new(hash_function);
        let input = b"biliqis";
        let input1: &[u8; 7] = b"onikoyi";
        let input2: &[u8;10] = b"onikashoyi";
        first.absorb(input);
        second.absorb(input2);
       first.squeeze();
        // second.absorb(input2);
       second.squeeze();

        // assert_eq!(squeeze2.transcript.len(), 1);
    }
}
