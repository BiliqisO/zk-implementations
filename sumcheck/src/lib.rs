use std::f32::consts::E;

use evaluation_form_poly::{product_poly::ProductPolynomial, EvaluationFormPolynomial};

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

//claimed sum and polynomilas already sent in eval form,
fn proof<F: PrimeField>(mut init_poly: ProductPolynomial<F>, claimed_sum: F) -> (F, Vec<Vec<F>>) {
    let init_poly_rep = &init_poly.polyomials[0].representation;
    let no_of_variables = init_poly_rep.len().ilog2();

    let summed_poly = init_poly.same_vars_sum_poly();
    let mut summed_poly = summed_poly.representation;

    let hash_function = Sha3_256::new();
    let mut fiat_shamir = FiatShamir::new(hash_function);
    let init_polynomial_bytes: Vec<u8> = summed_poly
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

    let mut unipoly_vec = vec![];

    for _ in 0..no_of_variables {
        let uni_polynomial_eval = proof_engine(&summed_poly);
        fiat_shamir.absorb(
            &uni_polynomial_eval
                .iter()
                .flat_map(|f: &F| f.into_bigint().to_bits_be().into_iter().map(|b| b as u8))
                .collect::<Vec<u8>>(),
        );
        let challenge = fiat_shamir.squeeze();

        let mut multilinear_poly = ProductPolynomial::new();
        multilinear_poly.add_polynomials(init_poly.polyomials);

        let mut uni_polynomial = EvaluationFormPolynomial::new(&uni_polynomial_eval);

        let verifier_sum: &F = &uni_polynomial.partial_evaluate(challenge, 0).representation[0];

        init_poly = multilinear_poly.partial_evaluate(challenge, 0);

        summed_poly = init_poly.same_vars_sum_poly().representation;

        assert_eq!(
            init_poly
                .polyomials
                .iter()
                .map(|poly| poly.representation.iter().sum::<F>())
                .sum::<F>(),
            *verifier_sum
        );
        unipoly_vec.push(uni_polynomial_eval.clone());
    }

    (claimed_sum, unipoly_vec)
}

fn proof_engine<F: PrimeField>(evaluation_form_vec: &Vec<F>) -> Vec<F> {
    let mut init_poly = ProductPolynomial::new();
    init_poly.add_polynomial(EvaluationFormPolynomial::new(evaluation_form_vec));
    let degree = init_poly.degree() + 1;

    let mid = evaluation_form_vec.len() / 2;
    let first_half_sum: F = evaluation_form_vec[..mid]
        .iter()
        .map(|monomial| monomial)
        .sum();
    let second_half_sum: F = evaluation_form_vec[mid..]
        .iter()
        .map(|monomial| monomial)
        .sum();
    let mut univariate_polynomial: Vec<F> = vec![first_half_sum, second_half_sum];

    let mut res_vec = vec![];
    for i in 0..degree {
        let mut uni_poly = ProductPolynomial::new();
        uni_poly.add_polynomial(EvaluationFormPolynomial::new(&univariate_polynomial));

        let res = uni_poly.evaluate([F::from(i as u32)].to_vec());

        res_vec.push(res);
    }

    res_vec
}

#[cfg(test)]
mod tests {
    use super::*;
    use ark_bn254::Fq;
    use evaluation_form_poly::product_poly::ProductPolynomial;

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
        let poly = EvaluationFormPolynomial::new(&values);
        let mut product = ProductPolynomial::new();
        product.add_polynomial(poly);
        let transcript = proof(product, Fq::from(29));

        verify(values, transcript.0, transcript.1);
    }
}
