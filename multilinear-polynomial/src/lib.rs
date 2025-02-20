use ark_ff::PrimeField;
use std::vec;
use std::ops::{Index, Add};
pub mod boolean_hypercube;
pub mod product_poly;
use boolean_hypercube::*;

#[derive(Debug, Clone, PartialEq)]
pub struct EvaluationFormPolynomial<F: PrimeField> {
    pub representation: Vec<F>,
    pub hypercube: Vec<String>,
}
impl<F: PrimeField> Add for EvaluationFormPolynomial<F> {
    type Output = Self;

    pub fn add(self, other: Self) -> Self {
        let mut result = self.clone();
        for (i, coeff) in other.representation.iter().enumerate() {
            result.representation[i] += coeff;
        }
        result
    }
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
        let hypercube: Vec<String> = boolean_hypercube::<F>(hypercube_size as usize);

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
        let mut poly: Vec<(String, F)> = Vec::new();
        let mut rep = Vec::new();

        for i in 0..self_vec_len {
            let mut hypercube = self.hypercube[i].clone();

            hypercube.remove(position);

            poly.push((hypercube, evaluation_form_vec[i]));
        }
        let mut merged_poly: Vec<(String, F)> = vec![];

        for eval in poly {
            if let Some(existing) = merged_poly.iter_mut().find(|e| e.0 == eval.0) {
                existing.1 = existing.1 * (F::from(1u32) - values) + eval.1 * values;
                rep.push(existing.1);
            } else {
                merged_poly.push(eval);
            }
        }
        let (hypercube, eval_rep): (Vec<String>, Vec<F>) = merged_poly.into_iter().unzip();

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

#[cfg(test)]
mod tests {
    use super::*;
    use ark_bn254::Fq;
     #[test]
    fn test_add_polynomials(){
         let values: Vec<Fq> = vec![Fq::from(0), Fq::from(2), Fq::from(0), Fq::from(5)];
         let values1: Vec<Fq> = vec![Fq::from(0), Fq::from(2), Fq::from(0), Fq::from(5)];
        let  poly = EvaluationFormPolynomial::new(&values);
        let  poly1 = EvaluationFormPolynomial::new(&values1);
     
      
       let addition = poly.add(poly1);
         assert_eq!(
            addition.representation,
            vec![(Fq::from(0)), (Fq::from(4)), (Fq::from(0)), (Fq::from(10))]
        );
      
       println!("addition {:?}", addition);

    }

    #[test]
    fn test_evaluation_form_partial_evaluation() {
        let values: Vec<Fq> = vec![Fq::from(0), Fq::from(2), Fq::from(0), Fq::from(5)];
        let mut poly = EvaluationFormPolynomial::new(&values);
        assert_eq!(
            poly.representation,
            vec![(Fq::from(0)), (Fq::from(2)), (Fq::from(0)), (Fq::from(5))]
        );
        assert_eq!(poly.hypercube, vec!["00", "01", "10", "11"]);
        let mut pol = poly.partial_evaluate(Fq::from(5), 0);
        assert_eq!(pol.representation, vec![(Fq::from(0)), (Fq::from(17))]);

        let result = pol.partial_evaluate(Fq::from(2), 0);
        assert_eq!(result.representation, vec![Fq::from(34)]);
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
