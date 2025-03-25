mod gkr_sumcheck;
use evaluation_form_poly::EvaluationFormPolynomial;

use ark_ff::{BigInteger, PrimeField};
use fiat_shamir::{self, FiatShamir};
use sha3::{Digest, Sha3_256};

fn verify<F: PrimeField>(init_polynomial: Vec<F>, mut claimed_sum: F, uni_poly: Vec<Vec<F>>) -> F {
    let mut uni_polynomial: EvaluationFormPolynomial<F> =
        EvaluationFormPolynomial::new(&uni_poly[0]);
    assert_eq!(
        uni_polynomial
            .partial_evaluate(F::from(0), 0)
            .representation[0]
            + uni_polynomial
                .partial_evaluate(F::from(1), 0)
                .representation[0],
        claimed_sum.clone()
    );
    let hash_function = Sha3_256::new();
    let mut fiat_shamir = FiatShamir::new(hash_function);
    let init_polynomial_bytes: Vec<u8> = init_polynomial
        .iter()
        .flat_map(|f| f.into_bigint().to_bits_be().into_iter().map(|b| b as u8))
        .collect();
    fiat_shamir.absorb(&init_polynomial_bytes);
    let claimed_sum_bytes: Vec<u8> = claimed_sum
        .into_bigint()
        .to_bits_be()
        .into_iter()
        .map(|b| b as u8)
        .collect();
    fiat_shamir.absorb(&claimed_sum_bytes);

    let mut init_poly = EvaluationFormPolynomial::new(&init_polynomial);

    for i in 0..uni_poly.len() {
        fiat_shamir.absorb(
            &uni_poly[i]
                .iter()
                .flat_map(|f| f.into_bigint().to_bits_be().into_iter().map(|b| b as u8))
                .collect::<Vec<u8>>(),
        );

        let challenge = fiat_shamir.squeeze();

        uni_polynomial = EvaluationFormPolynomial::new(&uni_poly[i]);

        claimed_sum = uni_polynomial.partial_evaluate(challenge, 0).representation[0];

        init_poly = init_poly.partial_evaluate(challenge, 0);
    }

    assert_eq!(init_poly.representation[0], claimed_sum);

    claimed_sum
}

fn proof<F: PrimeField>(mut init_polynomial: Vec<F>, claimed_sum: F) -> (F, Vec<Vec<F>>) {
    let mut unipoly_vec = vec![];
    let no_of_variables = init_polynomial.len().ilog2();

    let hash_function = Sha3_256::new();
    let mut fiat_shamir = FiatShamir::new(hash_function);
    let init_polynomial_bytes: Vec<u8> = init_polynomial
        .iter()
        .flat_map(|f| f.into_bigint().to_bits_be().into_iter().map(|b| b as u8))
        .collect();
    fiat_shamir.absorb(&init_polynomial_bytes);
    let claimed_sum_bytes: Vec<u8> = claimed_sum
        .into_bigint()
        .to_bits_be()
        .into_iter()
        .map(|b| b as u8)
        .collect();
    fiat_shamir.absorb(&claimed_sum_bytes);

    for _ in 0..no_of_variables {
        let uni_polynomial_eval = proof_engine(&init_polynomial);
        fiat_shamir.absorb(
            &uni_polynomial_eval
                .iter()
                .flat_map(|f: &F| f.into_bigint().to_bits_be().into_iter().map(|b| b as u8))
                .collect::<Vec<u8>>(),
        );
        let challenge = fiat_shamir.squeeze();

        let mut multilinear_poly = EvaluationFormPolynomial::new(&init_polynomial);

        let mut uni_polynomial = EvaluationFormPolynomial::new(&uni_polynomial_eval);

        let verifier_sum: &F = &uni_polynomial.partial_evaluate(challenge, 0).representation[0];

        init_polynomial = multilinear_poly
            .partial_evaluate(challenge, 0)
            .representation;

        assert_eq!(init_polynomial.iter().copied().sum::<F>(), *verifier_sum);
        unipoly_vec.push(uni_polynomial_eval.clone());
    }

    (claimed_sum, unipoly_vec)
}

fn proof_engine<F: PrimeField>(evaluation_form_vec: &Vec<F>) -> Vec<F> {
    let mid = evaluation_form_vec.len() / 2;
    let first_half_sum: F = evaluation_form_vec[..mid]
        .iter()
        .map(|monomial| monomial)
        .sum();
    let second_half_sum: F = evaluation_form_vec[mid..]
        .iter()
        .map(|monomial| monomial)
        .sum();
    let univariate_polynomial: Vec<F> = vec![first_half_sum, second_half_sum];
    univariate_polynomial
}

#[cfg(test)]
mod tests {
    use super::*;
    use field_tracker::{Ft, start_tscope, end_tscope, print_summary, summary};

    type Fr = Ft!(ark_bn254::Fq);

    #[test]
    fn test_sumcheck() {
        let values: Vec<Fr> = vec![Fr::from(2); 1<<20];
        // vec![
        //     Fr::from(0),
        //     Fr::from(0),
        //     Fr::from(0),
        //     Fr::from(2),
        //     Fr::from(0),
        //     Fr::from(10),
        //     Fr::from(0),
        //     Fr::from(17),
        // ];
        let transcript = proof(values.clone(), Fr::from(65536));
    
        verify(values, transcript.0, transcript.1);
        print_summary!()
    }
}