use ark_ff::PrimeField;
use std::vec;

#[derive(Debug, Clone, PartialEq)]
pub struct EvaluationFormMonomial<F: PrimeField> {
    pub hypercube: Vec<F>,
    pub value: F,
}
#[derive(Debug)]
pub struct EvaluationFormPolynomial<F: PrimeField> {
    pub evaluation: Vec<EvaluationFormMonomial<F>>,
}
impl<F: PrimeField> EvaluationFormPolynomial<F> {
    //evaluation form representation
    pub fn default() -> Self {
        EvaluationFormPolynomial { evaluation: vec![] }
    }

    pub fn new(values: &Vec<F>) -> Self {
        let value = values.len();
        let hypercube_size = value.ilog2();
        let hypercube = boolean_hypercube(hypercube_size as usize);

        let evaluation = EvaluationFormPolynomial::default();
        let mut data = evaluation.evaluation.clone();
        for i in 0..value {
            data.push(EvaluationFormMonomial {
                hypercube: hypercube[i].clone(),
                value: values[i],
            });
        }
        EvaluationFormPolynomial { evaluation: data }
    }
    //partial evaluation
    pub fn partial_evaluate(&mut self, values: F, position: usize) -> Self {
        let evaluation_form_vec = &self.evaluation;
        let self_vec_len = evaluation_form_vec.len();
        let mut poly = EvaluationFormPolynomial::default().evaluation;

        for i in 0..self_vec_len {
            let mut hypercube = evaluation_form_vec[i].hypercube.clone();

            hypercube.remove(position);

            poly.push(EvaluationFormMonomial {
                hypercube: hypercube,
                value: evaluation_form_vec[i].value,
            });
        }
        let mut merged_poly: Vec<EvaluationFormMonomial<F>> = vec![];

        for eval in poly {
            if let Some(existing) = merged_poly
                .iter_mut()
                .find(|e| e.hypercube == eval.hypercube)
            {
                existing.value = existing.value * (F::from(1u32) - values) + eval.value * values;
            } else {
                merged_poly.push(eval);
            }
        }
        EvaluationFormPolynomial {
            evaluation: merged_poly,
        }
    }
}
pub fn multilinear_monomial<F: PrimeField>(coeff: F, variables: Vec<F>) -> (F, Vec<F>) {
    let monomial: (F, Vec<F>) = (coeff, variables);
    return monomial;
}
pub fn multilinear_polynomial_sparse<F: PrimeField>(
    monomial: Vec<(F, Vec<F>)>,
) -> Vec<(F, Vec<F>)> {
    monomial
}
pub fn sparse_partial_evalauation<F: PrimeField>(
    mut polyomial: Vec<(F, Vec<F>)>,
    values: F,
    position: usize,
) -> Vec<(F, Vec<F>)> {
    let mut result = vec![];
    for i in 0..polyomial.len() {
        let coeff = polyomial[i].0.clone();
        let expo = polyomial[i].1[position].clone();
        let ans = coeff * values.pow(expo.into_bigint());

        // Convert the 1 in the position evaluated to 0 after evaluation
        polyomial[i].0 = ans;
        polyomial[i].1[position] = F::from(0u64);
        result = polyomial.clone();
        if polyomial[i].1.iter().all(|&x| x == F::from(0u64)) {
            let sum: F = result.iter().map(|(coeff, _)| *coeff).sum();
            result = vec![(sum, vec![F::from(0u64); polyomial[i].1.len()])];
        }
    }
    result
}
pub fn boolean_hypercube<F: PrimeField>(no_of_variables: usize) -> Vec<Vec<F>> {
    let length_of_hypercube = 2_usize.pow(no_of_variables as u32);

    let mut result = vec![];

    for i in 0..length_of_hypercube {
        let mut term = vec![];
        for j in (0..no_of_variables).rev() {
            term.push(F::from(((i >> j) & 1usize) as u64));
        }
        result.push(term);
    }

    result
}
#[cfg(test)]
mod tests {
    use super::*;
    use ark_bn254::Fq;

    #[test]
    fn test_evaluation_form_partial_evaluation() {
        let values: Vec<Fq> = vec![Fq::from(0), Fq::from(2), Fq::from(0), Fq::from(5)];
        let mut poly = EvaluationFormPolynomial::new(&values);
        assert_eq!(
            poly.evaluation,
            vec![
                EvaluationFormMonomial {
                    hypercube: vec![Fq::from(0), Fq::from(0)],
                    value: Fq::from(0)
                },
                EvaluationFormMonomial {
                    hypercube: vec![Fq::from(0), Fq::from(1)],
                    value: Fq::from(2)
                },
                EvaluationFormMonomial {
                    hypercube: vec![Fq::from(1), Fq::from(0)],
                    value: Fq::from(0)
                },
                EvaluationFormMonomial {
                    hypercube: vec![Fq::from(1), Fq::from(1)],
                    value: Fq::from(5)
                }
            ]
        );
        let mut pol = poly.partial_evaluate(Fq::from(5), 0);
        assert_eq!(
            pol.evaluation,
            vec![
                EvaluationFormMonomial {
                    hypercube: vec![Fq::from(0)],
                    value: Fq::from(0)
                },
                EvaluationFormMonomial {
                    hypercube: vec![Fq::from(1)],
                    value: Fq::from(17)
                }
            ]
        );
        let result = pol.partial_evaluate(Fq::from(2), 0);
        assert_eq!(
            result.evaluation,
            vec![EvaluationFormMonomial {
                hypercube: vec![],
                value: Fq::from(34)
            }]
        );
    }

    #[test]
    fn test_boolean_hypercube() {
        let cube = boolean_hypercube::<Fq>(3);
        let bh_3 = vec![
            vec![Fq::from(0), Fq::from(0), Fq::from(0)],
            vec![Fq::from(0), Fq::from(0), Fq::from(1)],
            vec![Fq::from(0), Fq::from(1), Fq::from(0)],
            vec![Fq::from(0), Fq::from(1), Fq::from(1)],
            vec![Fq::from(1), Fq::from(0), Fq::from(0)],
            vec![Fq::from(1), Fq::from(0), Fq::from(1)],
            vec![Fq::from(1), Fq::from(1), Fq::from(0)],
            vec![Fq::from(1), Fq::from(1), Fq::from(1)],
        ];
        assert_eq!(cube, bh_3);
    }
    #[test]
    fn test_sparse_partial_evaluation() {
        let m_1 = multilinear_monomial(Fq::from(3), vec![Fq::from(0), Fq::from(1), Fq::from(1)]);
        let m_2 = multilinear_monomial(Fq::from(4), vec![Fq::from(1), Fq::from(1), Fq::from(0)]);
        let m_3 = multilinear_monomial(Fq::from(5), vec![Fq::from(1), Fq::from(1), Fq::from(1)]);
        let poly = vec![m_1, m_2, m_3];
        let p = multilinear_polynomial_sparse(poly);
        let ans = sparse_partial_evalauation(p, Fq::from(5), 1);
        assert_eq!(
            ans,
            vec![
                (Fq::from(15), vec![Fq::from(0), Fq::from(0), Fq::from(1)]),
                (Fq::from(20), vec![Fq::from(1), Fq::from(0), Fq::from(0)]),
                (Fq::from(25), vec![Fq::from(1), Fq::from(0), Fq::from(1)])
            ]
        );
        let ans1 = sparse_partial_evalauation(ans, Fq::from(5), 0);
        assert_eq!(
            ans1,
            vec![
                (Fq::from(15), vec![Fq::from(0), Fq::from(0), Fq::from(1)]),
                (Fq::from(100), vec![Fq::from(0), Fq::from(0), Fq::from(0)]),
                (Fq::from(125), vec![Fq::from(0), Fq::from(0), Fq::from(1)])
            ]
        );
        let ans2 = sparse_partial_evalauation(ans1, Fq::from(5), 2);
        assert_eq!(
            ans2,
            vec![(Fq::from(800), vec![Fq::from(0), Fq::from(0), Fq::from(0)])]
        )
    }
}
