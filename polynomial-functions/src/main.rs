use std::result;

fn main() {
    println!("Hello, world!");
}
#[derive(Debug)]
struct Monomial {
    exponent: u32,
    coefficients: u32,
}
impl Monomial {
    fn new(exponent: u32, coefficients: u32) -> Monomial {
        Monomial {
            exponent,
            coefficients,
        }
    }
    fn default() -> Monomial {
        Monomial {
            exponent: 0,
            coefficients: 0,
        }
    }
}
struct Polyomial {
    monomials: Vec<Monomial>,
    degree: Option<u32>,
}

impl Polyomial {
    fn new(monomials: Vec<Monomial>) -> Polyomial {
        Polyomial {
            monomials,
            degree: None,
        }
    }
    fn default() -> Polyomial {
        Polyomial {
            monomials: Vec::new(),
            degree: None,
        }
    }
    fn evaluate(&self, x: u32) -> u32 {
        let mut result: u32 = 0;
        let n = self.monomials.len();
        for i in 0..n {
            result += self.monomials[i].coefficients * x.pow(self.monomials[i].exponent);
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

    fn multiplication(self, p2: Vec<Monomial>) -> Vec<Monomial> {
        let p1 = self.monomials;
        let mut polynomial: Vec<Monomial> = Vec::new();
        for i in 0..p1.len() {
            for j in 0..p2.len() {
                polynomial.push(Monomial {
                    coefficients: p1[i].coefficients * p2[j].coefficients,
                    exponent: p1[i].exponent + p2[j].exponent,
                });
            }
        }
        println!("{:?}", polynomial);
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
        return polynomial;
    }
    // fn interpolate(&self, x:Vec<u32>, y:Vec<u32>) {
    //     let n = x.len();
    //     let mut result: u32 = 0;
    //     for i in 0..n{
    //        y[i] *  - x[i+1] * y[i+1] / x[i] - x[i+1];
    //     }

    // }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_evaluate() {
        let default = Polyomial::default();
        let m1 = Monomial::new(2, 3);
        let m2 = Monomial::new(1, 2);
        let m3 = Monomial::new(0, 5);

        let p = Polyomial {
            monomials: vec![m1, m2, m3],
            ..default
        };
        let result = p.evaluate(4);
        assert_eq!(result, 61);
    }

    #[test]
    fn test_degree() {
        let default = Polyomial::default();
        let m1 = Monomial::new(2, 3);
        let m2 = Monomial::new(1, 2);

        let mut p = Polyomial {
            monomials: vec![m1, m2],
            ..default
        };
        let result = Polyomial::degree(&mut p);
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_multiplication() {
        let default = Polyomial::default();
        let m1 = Monomial::new(3, 4);
        let m2 = Monomial::new(2, 3);
        let m5 = Monomial::new(1, 3);
        let m3 = Monomial::new(2, 5);
        let m4 = Monomial::new(1, 7);

        let mut p1 = Polyomial {
            monomials: vec![m1, m2, m5],
            ..default
        };
        let mut p2 = Polyomial {
            monomials: vec![m3, m4],
            ..default
        };
        let result = Polyomial::multiplication(p1, p2.monomials);
        assert_eq!(result[0].coefficients, 20);
        assert_eq!(result[1].coefficients, 43);
        assert_eq!(result[2].coefficients, 36);
        assert_eq!(result[3].coefficients, 21);
        // assert_eq!(result[0].exponent, 6);
        // assert_eq!(result[1].exponent, 4);
        // assert_eq!(result[2].exponent, 5);
        // assert_eq!(result[3].exponent, 3);
    }
    #[test]
    fn test_interpolate() {}
}
