use evaluation_form_poly::EvaluationFormPolynomial;

use ark_ff::{BigInteger, PrimeField};
use fiat_shamir::{self, FiatShamir};
use sha3::{Digest, Sha3_256};
fn verify<F: PrimeField>(polynomial: Vec<F>, transcript: (Vec<F>, F)) -> bool {
    let challenges = transcript.0;
    let mut poly = EvaluationFormPolynomial::new(&polynomial);
    for i in 0..challenges.len() {
        poly = poly.partial_evaluate(challenges[i], 0);
    }
    if poly.representation.iter().copied().sum::<F>() == transcript.1 {
        true
    } else {
        false
    }
}
fn proof<F: PrimeField>(init_polynomial: &Vec<F>, claimed_sum: F) -> (Vec<F>, F) {
    let no_of_variables = init_polynomial.len().ilog2() - 1;

    let mut transcript_vec = Vec::new();
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

    //the very first evaluation/setup
    let mut uni_polynomial_eval = proof_engine(&init_polynomial);
    fiat_shamir.absorb(
        &uni_polynomial_eval
            .iter()
            .flat_map(|f| f.into_bigint().to_bits_be().into_iter().map(|b| b as u8))
            .collect::<Vec<u8>>(),
    );
    let mut challenge: F = fiat_shamir.squeeze();
    let mut multilinear_poly = EvaluationFormPolynomial::new(&init_polynomial);
    transcript_vec.push(challenge);
    let mut uni_polynomial: EvaluationFormPolynomial<F>;
    let mut verifier_sum = F::zero();

    for i in 0..no_of_variables {
        //unipoly from prover
        uni_polynomial = EvaluationFormPolynomial::new(&uni_polynomial_eval);

        //sum evaluated by verifier
        let verifier_sum: &F = &uni_polynomial.partial_evaluate(challenge, 0).representation[0];
        //new multilinear poly from challenge
        multilinear_poly = multilinear_poly.partial_evaluate(challenge, 0);

        assert_eq!(
            multilinear_poly.representation.iter().copied().sum::<F>(),
            *verifier_sum
        );

        uni_polynomial_eval = proof_engine(&multilinear_poly.representation);
        fiat_shamir.absorb(
            &uni_polynomial_eval
                .iter()
                .flat_map(|f: &F| f.into_bigint().to_bits_be().into_iter().map(|b| b as u8))
                .collect::<Vec<u8>>(),
        );

        challenge = fiat_shamir.squeeze();

        transcript_vec.push(challenge);
    }

    verifier_sum = multilinear_poly
        .partial_evaluate(challenge, 0)
        .representation[0];

    (transcript_vec, verifier_sum)
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
    println!("{:?}", univariate_polynomial);
    univariate_polynomial
}

#[cfg(test)]
mod tests {
    use super::*;
    use ark_bn254::Fq;


    #[test]
    fn test_sumcheck() {
        let values: Vec<Fq> = vec![
            Fq::from(0),
            Fq::from(0),
            Fq::from(0),
            Fq::from(2),
            Fq::from(0),
            Fq::from(10),
            Fq::from(0),
            Fq::from(17),
        ];
        let transcript = proof(&values, Fq::from(29));
        let iop = verify(values, transcript);
        assert_eq!(iop, true);
    }
}
