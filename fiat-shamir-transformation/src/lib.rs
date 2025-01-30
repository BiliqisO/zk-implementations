use ark_ff::PrimeField;
use sha3::Digest;


#[derive(Debug)]
pub struct FiatShamir<K, F: PrimeField> {
   pub hash_function: K,
   pub transcript: Vec<F>, //all the random challenges
}
impl<K: Digest + Clone, F: PrimeField> FiatShamir<K, F> {
   pub fn new(hash_function: K) -> Self {
        FiatShamir {
            hash_function,
            transcript: Vec::new(),
        }
    }
   pub fn absorb(&self, input: &[u8]) -> &Self {
        let mut hash_function = self.hash_function.clone();
        hash_function.update(input);
        &self
    }
   pub fn squeeze(&mut self) -> &Self {
        let hash_function = self.hash_function.clone();
        let result = hash_function.finalize();

        let result_bytes: Vec<u8> = result.to_vec();
        println!(" result_bytes{:?}", &result_bytes);
        let result_field = F::from_le_bytes_mod_order(&result_bytes);
        println!(" result_field{:?}", &result_field);
        self.transcript.push(result_field);
        self
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
        let mut first: FiatShamir<sha3::digest::core_api::CoreWrapper<sha3::Sha3_256Core>, Fq> =
            FiatShamir::new(hash_function);
        let input = b"biliqis";
        let input1: &[u8; 7] = b"onikoyi";
        first.absorb(input);
        first.absorb(input1);
        first.squeeze();

        assert_eq!(first.transcript.len(), 1);
    }
}
