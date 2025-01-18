use core::num;
use std::result;
use std::ops::{Add, Mul};

#[derive(Debug, Clone, Copy)]
struct Monomial {
    exponent: u32,
    coefficients: f32,
}
impl Monomial {
    fn new(exponent: u32, coefficients: f32) -> Monomial {
        Monomial {
            exponent,
            coefficients,
        }
    }
    fn default() -> Monomial {
        Monomial {
            exponent: 0,
            coefficients: 0.0,
        }
    }
}
#[derive(Debug, Clone)]
struct Polynomial {
    monomials: Vec<Monomial>,
    degree: Option<u32>,
}

impl Polynomial {
    fn new(monomials: Vec<Monomial>) -> Polynomial {
        Polynomial {
            monomials,
            degree: None,
        }
    }
    fn default() -> Polynomial {
        Polynomial {
            monomials: Vec::new(),
            degree: None,
        }
    }
    fn evaluate(&self, x: f32) -> f32 {
        let mut result: f32 = 0.0;
        let n = self.monomials.len();
        for i in 0..n {
            result += self.monomials[i].coefficients * x.powf(self.monomials[i].exponent as f32);
        }
        return result;
    }
    fn degree(&mut self) -> Option<u32> {
        let n = self.monomials.len();
        if self.degree.is_none() {
            for i in 0..n {
                if self.monomials[i].exponent > self.degree.unwrap_or(0) {
                    self.degree = Some(self.monomials[i].exponent);
                }
            }
            return self.degree;
        } else {
            return self.degree;
        }
    }

   

fn interpolate(x: Vec<f32>, y: Vec<f32>) -> Polynomial {
    let n = x.len();
    let mut result = Polynomial::default();

    for i in 0..n {
        let mut denominator: f32 = 1.0;

        let mut numerator = Polynomial::new(vec![Monomial::new(0, 1.0)]);

        let mut a = y[i];
        for j in 0..n {
            if i != j {
                let x_n = Monomial::new(1, 1.0); // x
                let x_j = Monomial::new(0, -x[j]);
                let temp_poly = Polynomial::new(vec![x_n, x_j]);

                numerator = numerator * temp_poly;

                denominator *= x[i] - x[j];
            }
        }

        a /= denominator;

        for monomial in &mut numerator.monomials {
            monomial.coefficients *= a;
        }

        result = result + numerator;
    }

    result
}

   
}

impl Mul for Polynomial {
    type Output = Polynomial;

    fn mul(self, p2:Polynomial) -> Self {
        let p1: Vec<Monomial> = self.monomials;
          let  p2: Vec<Monomial> = p2.monomials;

        let mut polynomial: Vec<Monomial> = Vec::new();
        for i in 0..p1.len() {
            for j in 0..p2.len() {
                polynomial.push(Monomial {
                    coefficients: p1[i].coefficients * p2[j].coefficients,
                    exponent: p1[i].exponent.wrapping_add(p2[j].exponent),
                });
            }
        }
       
        for i in 0..polynomial.len() {
            let mut j = i + 1;
            while j < polynomial.len() {
                if polynomial[i].exponent == polynomial[j].exponent {
                    polynomial[i].coefficients += polynomial[j].coefficients;
                    polynomial.remove(j);
                } else {
                    j += 1;
                }
            }
        }
        Polynomial {
            monomials: polynomial,
            degree: None,
        }
    }
}

impl Add for Polynomial {
    type Output = Polynomial;

     fn add(self,  p2: Polynomial) -> Self {
        let p1: Vec<Monomial> = self.monomials;
        let  p2: Vec<Monomial> = p2.monomials;
        let mut polynomial: Vec<Monomial> = Vec::new();
        polynomial = [p1, p2].concat();

        for i in 0..polynomial.len() {
            let mut j = i + 1;
            while j < polynomial.len() {
                if polynomial[i].exponent == polynomial[j].exponent {
                    polynomial[i].coefficients += polynomial[j].coefficients;
                    polynomial.remove(j);
                } else {
                    j += 1;
                }
            }
        }
        Polynomial {
            monomials: polynomial,
            degree: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_evaluate() {
        let default = Polynomial::default();
        let m1 = Monomial::new(2, 3.0);
        let m2 = Monomial::new(1, 2.0);
        let m3 = Monomial::new(0, 5.0);

        let p = Polynomial {
            monomials: vec![m1, m2, m3],
            ..default
        };
        let result = p.evaluate(4.0);
        assert_eq!(result, 61.0);
    }

    #[test]
    fn test_degree() {
        let default = Polynomial::default();
        let m1 = Monomial::new(2, 3.0);
        let m2 = Monomial::new(1, 2.0);

        let mut p = Polynomial {
            monomials: vec![m1, m2],
            ..default
        };
        let result = p.degree();
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_multiplication() {
        let default = Polynomial::default();
        let m1 = Monomial::new(3, 4.0);
        let m2 = Monomial::new(2, 3.0);
        let m5 = Monomial::new(1, 3.0);
        let m3 = Monomial::new(2, 5.0);
        let m4 = Monomial::new(1, 7.0);

        let mut p1 = Polynomial {
            monomials: vec![m1, m2, m5],
            ..default
        };
        let mut p2 = Polynomial {
            monomials: vec![m3, m4],
            ..default
        };
        let result = p1 * p2;
        assert_eq!(result.monomials[0].coefficients, 20.0);
        assert_eq!(result.monomials[1].coefficients, 43.0);
        assert_eq!(result.monomials[2].coefficients, 36.0);
        assert_eq!(result.monomials[3].coefficients, 21.0);
        // assert_eq!(result[0].exponent, 6);
        // assert_eq!(result[1].exponent, 4);
        // assert_eq!(result[2].exponent, 5);
        // assert_eq!(result[3].exponent, 3);
    }
    #[test]
    fn test_interpolate() {
        let default = Polynomial::default();
        let x = vec![1.0, 2.0, 3.0];
        let y = vec![1.0, 4.0, 9.0];
        let result = Polynomial::interpolate(x, y);
        assert_eq!(result.monomials[0].coefficients, 1.0);
        assert_eq!(result.monomials[0].exponent, 2);
    
    }
}
