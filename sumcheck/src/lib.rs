use std::{f32::consts::E, vec};

use evaluation_form_poly::{
    product_poly::{ProductPolynomial, SumPolynomial},
    EvaluationFormPolynomial,
};

use ark_ff::{BigInteger, PrimeField};
use fiat_shamir::{self, FiatShamir};
use sha3::{digest::typenum::Sum, Digest, Sha3_256};

pub fn verify<F: PrimeField>(
    mut init_poly: SumPolynomial<F>,
    mut claimed_sum: F,
    mut uni_poly: Vec<Vec<F>>,
) -> F {
    let mut first_unipoly = uni_poly[0].clone();
    first_unipoly.pop().unwrap();
    let mut uni_polynomial: EvaluationFormPolynomial<F> =
        EvaluationFormPolynomial::new(&first_unipoly);
    let mut summed_poly = init_poly.clone().polyomials;
    assert_eq!(
        uni_polynomial
            .partial_evaluate(F::from(0), 0)
            .representation[0]
            + uni_polynomial
                .partial_evaluate(F::from(1), 0)
                .representation[0],
        claimed_sum.clone(),
    );
    let hash_function = Sha3_256::new();
    let mut fiat_shamir = FiatShamir::new(hash_function);
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

    for i in 0..uni_poly.len() {
        fiat_shamir.absorb(
            &uni_poly[i]
                .iter()
                .flat_map(|f| f.into_bigint().to_bits_be().into_iter().map(|b| b as u8))
                .collect::<Vec<u8>>(),
        );

        let challenge: F = fiat_shamir.squeeze();
        uni_poly[i].pop().unwrap();

        let uni_poly = SumPolynomial::new(vec![ProductPolynomial::new(vec![
            EvaluationFormPolynomial::new(&uni_poly[i]),
        ])]);

        claimed_sum = uni_poly.partial_evaluate(challenge.pow([2]), 0).polyomials[0].polyomials[0]
            .representation[0];

        init_poly = init_poly.partial_evaluate(challenge, 0);
    }
    assert_eq!(
        init_poly.reduce().polyomials[0].polyomials[0].representation[0],
        claimed_sum
    );

    claimed_sum
}

pub fn proof<F: PrimeField>(mut init_poly: SumPolynomial<F>, claimed_sum: F) -> (F, Vec<Vec<F>>) {
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
        assert_eq!(
            init_poly
                .reduce()
                .polyomials
                .iter()
                .flat_map(|poly| poly.polyomials.iter().flat_map(|p| p.representation.iter()))
                .sum::<F>(),
            *verifier_sum
        );
    }
    (claimed_sum, unipoly_vec)
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

#[cfg(test)]
mod tests {
    use super::*;
    use ark_bn254::Fq;
    use evaluation_form_poly::product_poly::ProductPolynomial;

    #[test]
    fn test_sumcheck() {
        let values: Vec<Fq> = vec![Fq::from(0), Fq::from(0), Fq::from(0), Fq::from(2)];
        let values1: Vec<Fq> = vec![Fq::from(0), Fq::from(0), Fq::from(0), Fq::from(3)];
        let values2: Vec<Fq> = vec![Fq::from(0), Fq::from(0), Fq::from(0), Fq::from(1)];
        let values3: Vec<Fq> = vec![Fq::from(0), Fq::from(0), Fq::from(0), Fq::from(7)];

        let poly = ProductPolynomial::new(vec![
            EvaluationFormPolynomial::new(&values),
            EvaluationFormPolynomial::new(&values1),
        ]);
        let poly1 = ProductPolynomial::new(vec![
            EvaluationFormPolynomial::new(&values2),
            EvaluationFormPolynomial::new(&values3),
        ]);
        let mut sum_poly = SumPolynomial::new(vec![poly, poly1]);
    
        let transcript = proof(sum_poly.clone(), Fq::from(13));
        verify(sum_poly, transcript.0, transcript.1);
    }

    // #[test]

    // fn test_sumcheck() {
    //     let values: Vec<Fq> = vec![
    //         Fq::from(0),
    //         Fq::from(0),
    //         Fq::from(0),
    //         Fq::from(2),
    //         Fq::from(0),
    //         Fq::from(10),
    //         Fq::from(0),
    //         Fq::from(17),
    //     ];
    //     let poly = EvaluationFormPolynomial::new(&values);
    //     let mut product = ProductPolynomial::new();
    //     product.add_polynomial(poly);
    //     let transcript = proof(product, Fq::from(29));

    //     verify(values, transcript.0, transcript.1);
    // }
}
