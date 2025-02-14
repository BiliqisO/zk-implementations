use crate::EvaluationFormPolynomial;
use ark_ff::PrimeField;
use std::vec;
#[derive(Debug, Clone)]
pub struct SumPolynomial<F: PrimeField> {
    pub polyomials: Vec<ProductPolynomial<F>>,
}
impl<F: PrimeField> SumPolynomial<F> {
    pub fn new(polyomials: Vec<ProductPolynomial<F>>) -> Self {
        SumPolynomial { polyomials }
    }
    pub fn add_polynomial(&mut self, poly: ProductPolynomial<F>) {
        self.polyomials.push(poly);
    }
    pub fn partial_evaluate(&self, value: F, position: usize) -> SumPolynomial<F> {
        let mut result = SumPolynomial::new(vec![]);
    for mut poly in self.polyomials.clone() {  
    // Handle each ProductPolynomial's partial evaluation
    let evaluated = poly.partial_evaluate(value, position);
    result.add_polynomial(evaluated);
}

        result
    }
    pub fn reduce(&self) -> SumPolynomial<F> {
        if self.polyomials.len() == 1 {
            return self.clone();
        }
        if self.polyomials.len() != 2 {
            panic!("The number of polynomials should be 2");
        } else if self.polyomials[0].polyomials[0].representation.len()
            != self.polyomials[1].polyomials[0].representation.len()
        {
            panic!("The number of monomials in the polynomials should be equal");
        }

        let mut result = ProductPolynomial::new(vec![EvaluationFormPolynomial::default()]);
        let mut result1 = ProductPolynomial::new(vec![EvaluationFormPolynomial::default()]);
        let mut total_result = SumPolynomial::new(vec![ProductPolynomial::new(vec![
            EvaluationFormPolynomial::default(),
        ])]);
        for i in 0..self.polyomials[0].polyomials[0].representation.len() {
            let eval1 = self.polyomials[0].polyomials[0].representation[i]
                * self.polyomials[0].polyomials[1].representation[i];
            let eval2 = self.polyomials[1].polyomials[0].representation[i]
                * self.polyomials[1].polyomials[1].representation[i];

            result.polyomials[0].representation.push(eval1);
            result1.polyomials[0].representation.push(eval2);

            let total_eval =
                result.polyomials[0].representation[i] + result1.polyomials[0].representation[i];
            total_result.polyomials[0].polyomials[0]
                .representation
                .push(total_eval);
        }

        total_result
    }
}
#[derive(Debug, Clone)]
pub struct ProductPolynomial<F: PrimeField> {
    pub polyomials: Vec<EvaluationFormPolynomial<F>>,
}
impl<F: PrimeField> ProductPolynomial<F> {
    pub fn new(polyomials: Vec<EvaluationFormPolynomial<F>>) -> Self {
        ProductPolynomial { polyomials }
    }
    pub fn add_polynomial(&mut self, poly: EvaluationFormPolynomial<F>) {
        self.polyomials.push(poly);
    }
    pub fn add_polynomials(&mut self, polys: Vec<EvaluationFormPolynomial<F>>) {
        for i in 0..polys.len() {
            self.polyomials.push(polys[i].clone());
        }
    }
    pub fn partial_evaluate(&mut self, value: F, position: usize) -> ProductPolynomial<F> {
        let mut result = ProductPolynomial::new(vec![]);
        for i in 0..self.polyomials.len() {
            let mut poly = self.polyomials[i].clone();
            let poly = poly.partial_evaluate(value, position);
            result.add_polynomial(poly);
        }
        result
    }

    pub fn evaluate(self, values: Vec<F>) -> F {
        let mut result = F::from(1u32);
        for i in 0..self.polyomials.len() {
            let mut poly = self.polyomials[i].clone();
            for j in 0..values.len() {
                let value = values[j];
                poly = poly.partial_evaluate(value, 0);
            }
            result = result * poly.representation[0];
        }
        //   for i in 0..poly.representation.len() {
        //  result.push(poly.representation[i] * poly1.representation[i]);

        // }
        result
    }

    pub fn reduce(&mut self) -> ProductPolynomial<F> {
        if self.polyomials.len() == 1 {
            return self.clone();
        }
        if self.polyomials.len() != 2 {
            panic!("The number of polynomials should be 2");
        } else if self.polyomials[0].representation.len() != self.polyomials[1].representation.len()
        {
            panic!("The number of monomials in the polynomials should be equal");
        }
        let mut result = EvaluationFormPolynomial::default();
        for i in 0..self.polyomials[0].representation.len() {
            let poly = self.polyomials[0].representation[i] * self.polyomials[1].representation[i];

            result.representation.push(poly);
        }
        ProductPolynomial::new(vec![result])
    }
    pub fn sum_poly(self) -> EvaluationFormPolynomial<F> {
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

    pub fn degree(&self) -> usize {
        let degree = self.polyomials.len();
        degree
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use ark_bn254::Fq;
    #[test]
    fn test_sumpolynomial_reduce() {
        let values: Vec<Fq> = vec![Fq::from(0), Fq::from(3), Fq::from(2), Fq::from(5)];
        let poly = EvaluationFormPolynomial::new(&values);
        let values1: Vec<Fq> = vec![Fq::from(0), Fq::from(0), Fq::from(0), Fq::from(4)];
        let poly1 = EvaluationFormPolynomial::new(&values1);
        let mut product = ProductPolynomial::new(vec![poly, poly1]);

        let values2: Vec<Fq> = vec![Fq::from(0), Fq::from(3), Fq::from(2), Fq::from(5)];
        let poly2 = EvaluationFormPolynomial::new(&values2);
        let values3: Vec<Fq> = vec![Fq::from(0), Fq::from(0), Fq::from(0), Fq::from(4)];
        let poly3 = EvaluationFormPolynomial::new(&values3);
        let mut product1 = ProductPolynomial::new(vec![poly2, poly3]);

        let sum = SumPolynomial::new(vec![product, product1]);

        let result = sum.reduce();
        assert_eq!(
            result.polyomials[0].polyomials[0].representation,
            vec![Fq::from(0), Fq::from(0), Fq::from(0), Fq::from(40)]
        );
    }
    #[test]
    fn test_sumpolynomial_partial_evaluate() {
        let values: Vec<Fq> = vec![Fq::from(0), Fq::from(3), Fq::from(2), Fq::from(5)];
        let poly = EvaluationFormPolynomial::new(&values);

        let values1: Vec<Fq> = vec![Fq::from(0), Fq::from(0), Fq::from(0), Fq::from(4)];
        let poly1 = EvaluationFormPolynomial::new(&values1);
        //write poly3 and poly4
        let values2: Vec<Fq> = vec![Fq::from(0), Fq::from(3), Fq::from(2), Fq::from(5)];
        let poly2 = EvaluationFormPolynomial::new(&values2);
        let values3: Vec<Fq> = vec![Fq::from(0), Fq::from(0), Fq::from(0), Fq::from(4)];
        let poly3 = EvaluationFormPolynomial::new(&values3);

        let sum = SumPolynomial::new(vec![
            ProductPolynomial::new(vec![poly, poly1]),
            ProductPolynomial::new(vec![poly2, poly3]),
        ]);

        let result = sum.partial_evaluate(Fq::from(5), 0);

        assert_eq!(
            result.polyomials[0].polyomials[0].representation,
            vec![Fq::from(10), Fq::from(13)]
        );
        assert_eq!(
            result.polyomials[0].polyomials[1].representation,
            vec![Fq::from(0), Fq::from(20)]
        );
        assert_eq!(
            result.polyomials[1].polyomials[0].representation,
            vec![Fq::from(10), Fq::from(13)]
        );
        assert_eq!(
            result.polyomials[1].polyomials[1].representation,
            vec![Fq::from(0), Fq::from(20)]
        );
    }

    #[test]
    fn test_product_poly_partial_eval() {
        let values: Vec<Fq> = vec![Fq::from(0), Fq::from(3), Fq::from(2), Fq::from(5)];
        let poly = EvaluationFormPolynomial::new(&values);
        let values1: Vec<Fq> = vec![Fq::from(0), Fq::from(0), Fq::from(0), Fq::from(4)];
        let poly1 = EvaluationFormPolynomial::new(&values1);
        let mut product = ProductPolynomial::new(vec![poly, poly1]);

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
        let product = ProductPolynomial::new(vec![poly, poly1]);
        let result = product.evaluate(vec![Fq::from(5), Fq::from(2)]);
        assert_eq!(result, Fq::from(56));
    }
    #[test]
    fn test_product_poly_degree() {
        let values: Vec<Fq> = vec![Fq::from(0), Fq::from(3), Fq::from(2), Fq::from(5)];
        let poly = EvaluationFormPolynomial::new(&values);
        let values1: Vec<Fq> = vec![Fq::from(0), Fq::from(0), Fq::from(0), Fq::from(4)];
        let poly1 = EvaluationFormPolynomial::new(&values1);
        let product = ProductPolynomial::new(vec![poly, poly1]);
        let result = product.degree();
        assert_eq!(result, 2);
    }
    #[test]

    fn test_sum_poly() {
        let values: Vec<Fq> = vec![Fq::from(3), Fq::from(3), Fq::from(3), Fq::from(5)];
        let poly = EvaluationFormPolynomial::new(&values);
        let values1: Vec<Fq> = vec![Fq::from(6), Fq::from(8)];
        let poly1 = EvaluationFormPolynomial::new(&values1);
        let product = ProductPolynomial::new(vec![poly, poly1]);
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
    // #[test]
    // fn test_reduce_add() {
    //     let values: Vec<Fq> = vec![Fq::from(24), Fq::from(36)];
    //     let poly = EvaluationFormPolynomial::new(&values);
    //     let values1: Vec<Fq> = vec![Fq::from(9), Fq::from(33)];
    //     let poly1 = EvaluationFormPolynomial::new(&values1);
    //     let  product = ProductPolynomial::new(vec![poly, poly1]);

    //     let result = product.reduce_mul();
    //     assert_eq!(
    //         result.polyomials[0].representation,
    //         vec![Fq::from(33), Fq::from(69)]
    //     );
    // }
}
