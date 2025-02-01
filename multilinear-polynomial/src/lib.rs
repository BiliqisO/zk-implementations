use ark_ff::PrimeField;
use std::vec;


#[derive( Debug, Clone, PartialEq)]
pub struct EvaluationFormPolynomial<F: PrimeField> {
    pub representation: Vec<F>,
    pub hypercube: Vec<Vec<F>>,
}

impl<F: PrimeField> EvaluationFormPolynomial<F> {
 

    pub fn default() -> Self {
        EvaluationFormPolynomial {
            representation: vec![],
            hypercube: vec![],
        }
    }



    pub fn new(values: &Vec<F>) -> Self {
        // Check if the length of values is a power of 2
        assert!(
            values.len().is_power_of_two(),
            "Length of values must be a power of 2"
        );

        let value = values.len();
        let hypercube_size = value.ilog2();
        let hypercube = boolean_hypercube(hypercube_size as usize);

        let evaluation = EvaluationFormPolynomial::default();
        let mut data = evaluation.representation;
        for i in 0..value {
            data.push(values[i]);
        }

        EvaluationFormPolynomial {
            representation: data,
            hypercube,
        }
    }

    pub fn partial_evaluate(&mut self, values: F, position: usize) -> Self {
        let evaluation_form_vec = &self.representation;
        let self_vec_len = evaluation_form_vec.len();
        let mut poly: Vec<(Vec<F>, F)> = Vec::new();
        let mut rep = Vec::new();

        for i in 0..self_vec_len {
            let mut hypercube = self.hypercube[i].clone();

            hypercube.remove(position);

            poly.push((hypercube, evaluation_form_vec[i]));
        }
        let mut merged_poly: Vec<(Vec<F>, F)> = vec![];

        for eval in poly {
            if let Some(existing) = merged_poly.iter_mut().find(|e| e.0 == eval.0) {
                existing.1 = existing.1 * (F::from(1u32) - values) + eval.1 * values;
                rep.push(existing.1);
            } else {
                merged_poly.push(eval);
            }
        }
        let (hypercube, eval_rep): (Vec<Vec<F>>, Vec<F>) = merged_poly.into_iter().unzip();

        EvaluationFormPolynomial {
            representation: eval_rep,
            hypercube,
        }
    }
}
#[derive(Debug, PartialEq, Clone)]
pub struct MultilinearPolynomialSparse<F: PrimeField> {
    polynomial: Vec<(F, Vec<F>)>,
}
impl<F: PrimeField> MultilinearPolynomialSparse<F> {

    pub fn multilinear_monomial(coeff: F, variables: Vec<F>) -> (F, Vec<F>) {
        let monomial: (F, Vec<F>) = (coeff, variables);
        return monomial;
    }

    pub fn new(self) -> Self {
        self
    }

    pub fn evaluation(&mut self, values: F, position: usize) -> Self {
        let mut result = vec![];
        for i in 0..self.polynomial.len() {
            let coeff = self.polynomial[i].0.clone();
            let expo = self.polynomial[i].1[position].clone();
            let ans = coeff * values.pow(expo.into_bigint());

            // Convert the 1 in the position evaluated to 0 after evaluation
            self.polynomial[i].0 = ans;
            self.polynomial[i].1[position] = F::from(0u64);
            result = self.polynomial.clone();
            if self.polynomial[i].1.iter().all(|&x| x == F::from(0u64)) {
                let sum: F = result.iter().map(|(coeff, _)| *coeff).sum();
                result = vec![(sum, vec![F::from(0u64); self.polynomial[i].1.len()])];
            }
        }
        MultilinearPolynomialSparse { polynomial: result }
    }

    pub fn sum(&self, other: &Self) -> Self {
        let mut polynomial = self.polynomial.clone();
        let mut result = vec![];

        for (coeff, vars) in &other.polynomial {
            if let Some(existing) = polynomial.iter_mut().find(|(_, v)| v == vars) {
                existing.0 += *coeff;
            } else {
                result.push((*coeff, vars.clone()));
            }
        }

        MultilinearPolynomialSparse { polynomial: result }
    }
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
            poly.representation,
            vec![(Fq::from(0)), (Fq::from(2)), (Fq::from(0)), (Fq::from(5))]
        );
        assert_eq!(
            poly.hypercube,
            vec![
                (vec![Fq::from(0), Fq::from(0)]),
                (vec![Fq::from(0), Fq::from(1)]),
                (vec![Fq::from(1), Fq::from(0)]),
                (vec![Fq::from(1), Fq::from(1)])
            ]
        );
        let mut pol = poly.partial_evaluate(Fq::from(5), 0);
        assert_eq!(pol.representation, vec![(Fq::from(0)), (Fq::from(17))]);

        let result = pol.partial_evaluate(Fq::from(2), 0);
        assert_eq!(result.representation, vec![Fq::from(34)]);
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
        let m_1 = MultilinearPolynomialSparse::multilinear_monomial(
            Fq::from(3),
            vec![Fq::from(0), Fq::from(1), Fq::from(1)],
        );
        let m_2 = MultilinearPolynomialSparse::multilinear_monomial(
            Fq::from(4),
            vec![Fq::from(1), Fq::from(1), Fq::from(0)],
        );
        let m_3 = MultilinearPolynomialSparse::multilinear_monomial(
            Fq::from(5),
            vec![Fq::from(1), Fq::from(1), Fq::from(1)],
        );
        let poly = vec![m_1, m_2, m_3];
        let mut p =
            MultilinearPolynomialSparse::new(MultilinearPolynomialSparse { polynomial: poly });
        let mut ans = p.evaluation(Fq::from(5), 1);
        assert_eq!(
            ans.polynomial,
            vec![
                (Fq::from(15), vec![Fq::from(0), Fq::from(0), Fq::from(1)]),
                (Fq::from(20), vec![Fq::from(1), Fq::from(0), Fq::from(0)]),
                (Fq::from(25), vec![Fq::from(1), Fq::from(0), Fq::from(1)])
            ]
        );
        let mut ans1 = ans.evaluation(Fq::from(5), 0);
        assert_eq!(
            ans1.polynomial,
            vec![
                (Fq::from(15), vec![Fq::from(0), Fq::from(0), Fq::from(1)]),
                (Fq::from(100), vec![Fq::from(0), Fq::from(0), Fq::from(0)]),
                (Fq::from(125), vec![Fq::from(0), Fq::from(0), Fq::from(1)])
            ]
        );
        let ans2 = ans1.evaluation(Fq::from(5), 2);
        assert_eq!(
            ans2.polynomial,
            vec![(Fq::from(800), vec![Fq::from(0), Fq::from(0), Fq::from(0)])]
        )
    }
}
