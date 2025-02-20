use std::{f32::consts::E, vec};

use multilinear_polynomial::{
    product_poly::{ProductPolynomial, SumPolynomial},
    EvaluationFormPolynomial,
};

use ark_ff::{BigInteger, PrimeField};
use fiat_shamir::{self, FiatShamir};
use sha3::{digest::typenum::Sum, Digest, Sha3_256};
pub fn proof<F: PrimeField>(mut init_poly: SumPolynomial<F>, claimed_sum: F) -> (F, Vec<F>) {
    let init_poly_rep = &init_poly.polyomials[0].polyomials[0].representation;
    let no_of_variables = init_poly_rep.len().ilog2();

    let mut summed_poly = init_poly.clone().polyomials;

    let hash_function = Sha3_256::new();
    let mut fiat_shamir: FiatShamir<sha3::digest::core_api::CoreWrapper<sha3::Sha3_256Core>, F> =
        FiatShamir::new(hash_function);
    let init_polynomial_bytes: Vec<u8> = summed_poly
        .iter()
        .flat_map(|f| {
            f.polyomials.iter().flat_map(|p| {
                p.representation
                    .iter()
                    .flat_map(|elem| elem.into_bigint().to_bits_be().into_iter().map(|b| b as u8))
            })
        })
        .collect();
    fiat_shamir.absorb(&init_polynomial_bytes);
    let claimed_sum_bytes: Vec<u8> = claimed_sum
        .into_bigint()
        .to_bits_be()
        .into_iter()
        .map(|b| b as u8)
        .collect();
    fiat_shamir.absorb(&claimed_sum_bytes);

    let mut unipoly_vec = vec![];
    let mut challenge_vec = vec![]; 

    for _ in 0..no_of_variables {
        let mut uni_polynomial_eval = proof_engine(&init_poly);
        unipoly_vec.push(uni_polynomial_eval.clone());

        fiat_shamir.absorb(
            &uni_polynomial_eval
                .iter()
                .flat_map(|f: &F| f.into_bigint().to_bits_be().into_iter().map(|b| b as u8))
                .collect::<Vec<u8>>(),
        );
        uni_polynomial_eval.pop().unwrap();
        let challenge = fiat_shamir.squeeze();
        challenge_vec.push(challenge);  

        let evaluation_polys: Vec<EvaluationFormPolynomial<F>> = init_poly
            .polyomials
            .iter()
            .map(|poly| EvaluationFormPolynomial::new(&poly.polyomials[0].representation))
            .collect();

        let mut multilinear_poly = init_poly;

        let mut uni_polynomial: SumPolynomial<F> = SumPolynomial::new(vec![
            ProductPolynomial::new(vec![EvaluationFormPolynomial::new(&uni_polynomial_eval)]),
        ]);

        let verifier_sum: &F = &uni_polynomial
            .partial_evaluate(challenge.pow([2]), 0)
            .polyomials[0]
            .polyomials[0]
            .representation[0];

        init_poly = multilinear_poly.partial_evaluate(challenge, 0);
        // assert_eq!(
        //     init_poly
        //         .reduce()
        //         .polyomials
        //         .iter()
        //         .flat_map(|poly| poly.polyomials.iter().flat_map(|p| p.representation.iter()))
        //         .sum::<F>(),
        //     *verifier_sum
        // );
    }
    (claimed_sum, challenge_vec)
}

fn proof_engine<F: PrimeField>(mut poly: &SumPolynomial<F>) -> Vec<F> {
    let init_poly = poly.polyomials.clone();

    let degree = init_poly[0].degree() + 1;
    println!("degree {:?}", degree);
    let mut res_vec = SumPolynomial::new(vec![]);

    for i in 0..degree {
        let res = poly.partial_evaluate(F::from(i as u32), 0);
        let reduced = res.reduce();
        res_vec.add_polynomials(reduced.polyomials);
    }
    let mut result = Vec::new();
    for product_poly in &res_vec.polyomials {
        let sum = product_poly.polyomials[0]
            .representation
            .iter()
            .fold(F::zero(), |acc, x| acc.add(x));

        result.push(sum);
    }

    result
}