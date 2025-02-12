use crate::EvaluationFormPolynomial;
use ark_ff::PrimeField;
use std::vec;

pub struct ProductPolynomial<F: PrimeField> {
    pub polyomials: Vec<EvaluationFormPolynomial<F>>,
}
impl<F: PrimeField> ProductPolynomial<F> {
    pub fn new() -> Self {
        ProductPolynomial { polyomials: vec![] }
    }
    pub fn add_polynomial(&mut self, poly: EvaluationFormPolynomial<F>) {
        self.polyomials.push(poly);
    }
    pub fn add_polynomials(&mut self, polys: Vec<EvaluationFormPolynomial<F>>) {
        for i in 0..polys.len() {
            self.polyomials.push(polys[i].clone());
        }
    }
    pub fn partial_evaluate(&mut self, value: F, position: usize) -> Self {
        let mut result = ProductPolynomial::new();
        for i in 0..self.polyomials.len() {
            let mut poly = self.polyomials[i].clone();
            poly = poly.partial_evaluate(value, position);
            result.add_polynomial(poly);
        }
        result
    }
    pub fn evaluate(&mut self, values: Vec<F>) -> F {
        let mut result = F::from(0u32);
        for i in 0..self.polyomials.len() {
            let mut poly = self.polyomials[i].clone();
            for j in 0..values.len() {
                let value = values[j];
                poly = poly.partial_evaluate(value, 0);
            }

            result = result + poly.representation[0];
        }
        result
    }
    pub fn same_vars_sum_poly(&mut self) -> EvaluationFormPolynomial<F> {
        let mut result = EvaluationFormPolynomial::default();
        for i in 0..self.polyomials[0].representation.len()  {
            let poly = self.polyomials[0].representation[i] + self.polyomials[1].representation[i]; 
        
                result.representation.push(poly);
            
        }
        result

    }
   pub fn sum_poly(mut self) -> EvaluationFormPolynomial<F> {
        let mut result = EvaluationFormPolynomial::default();
    
        for i in 0..self.polyomials[0].representation.len() {
            let first_poly = self.polyomials[0].representation[i];
            for j in 0..self.polyomials[1].representation.len() {
                let second_poly = first_poly + self.polyomials[1].representation[j];
                result.representation.push(second_poly);
            }
        }
        result
    }

    fn degree(&self) -> usize {
        let mut max_degree = 0;
        for i in 0..self.polyomials.len() {
            let poly = &self.polyomials[i];
            let degree = poly.hypercube.len();
            if degree > max_degree {
                max_degree = degree;
            }
        }
        max_degree
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use ark_bn254::Fq;

    #[test]
    fn test_product_poly_partial_eval() {
        let values: Vec<Fq> = vec![Fq::from(0), Fq::from(3), Fq::from(2), Fq::from(5)];
        let poly = EvaluationFormPolynomial::new(&values);
        let values1: Vec<Fq> = vec![Fq::from(0), Fq::from(0), Fq::from(0), Fq::from(4)];
        let poly1 = EvaluationFormPolynomial::new(&values1);
        let mut product = ProductPolynomial::new();
        product.add_polynomial(poly);
        product.add_polynomial(poly1);
        let result = product.partial_evaluate(Fq::from(5), 0);
        assert_eq!(
            result.polyomials[0].representation,
            vec![Fq::from(10), Fq::from(13)]
        );
        assert_eq!(
            result.polyomials[1].representation,
            vec![Fq::from(0), Fq::from(20)]
        );
    }
    #[test]
    fn test_product_poly_evaluate() {
        let values: Vec<Fq> = vec![Fq::from(0), Fq::from(3), Fq::from(2), Fq::from(5)];
        let poly = EvaluationFormPolynomial::new(&values);
        let values1: Vec<Fq> = vec![Fq::from(0), Fq::from(0), Fq::from(0), Fq::from(4)];
        let poly1 = EvaluationFormPolynomial::new(&values1);
        let mut product = ProductPolynomial::new();
        product.add_polynomial(poly);
        product.add_polynomial(poly1);
        let result = product.evaluate(vec![Fq::from(5), Fq::from(2)]);
        assert_eq!(result, Fq::from(56));
    }
    #[test]
    fn test_product_poly_degree() {
        let values: Vec<Fq> = vec![Fq::from(0), Fq::from(3), Fq::from(2), Fq::from(5)];
        let poly = EvaluationFormPolynomial::new(&values);
        let values1: Vec<Fq> = vec![Fq::from(0), Fq::from(0), Fq::from(0), Fq::from(4)];
        let poly1 = EvaluationFormPolynomial::new(&values1);
        let mut product = ProductPolynomial::new();
        product.add_polynomial(poly);
        product.add_polynomial(poly1);
        let result = product.degree();
        assert_eq!(result, 4);
    }
    #[test]

    fn test_sum_poly() {
        let values: Vec<Fq> = vec![Fq::from(3), Fq::from(3), Fq::from(3), Fq::from(5)];
        let poly = EvaluationFormPolynomial::new(&values);
        let values1: Vec<Fq> = vec![Fq::from(6), Fq::from(8)];
        let poly1 = EvaluationFormPolynomial::new(&values1);
        let mut product = ProductPolynomial::new();
        product.add_polynomial(poly);
        product.add_polynomial(poly1);
        let result = product.sum_poly();
        assert_eq!(
            result.representation,
            vec![
                Fq::from(9),
                Fq::from(11),
                Fq::from(9),
                Fq::from(11),
                Fq::from(11),
                Fq::from(11),
                Fq::from(13)
            ]
        );
    }
    #[test]
    fn test_univariate_sum_poly() {
        let values: Vec<Fq> = vec![Fq::from(24), Fq::from(36)];
        let poly = EvaluationFormPolynomial::new(&values);
        let values1: Vec<Fq> = vec![Fq::from(9), Fq::from(33)];
        let poly1 = EvaluationFormPolynomial::new(&values1);
        let mut product = ProductPolynomial::new();
        product.add_polynomial(poly);
        product.add_polynomial(poly1);
        let result = product.same_vars_sum_poly();
        assert_eq!(
            result.representation,
            vec![Fq::from(33), Fq::from(69)]
        );
    }       
}
