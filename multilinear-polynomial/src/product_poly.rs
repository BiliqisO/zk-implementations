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
                println!("Value: {:?}", value);
                println!("J: {:?}", j);
                poly = poly.partial_evaluate(value, 0);
            }

            result = result + poly.representation[0];
        }
        result
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
}
