use std::ops::{Add, Mul};

/// Represents a single term in a polynomial, consisting of an exponent and a coefficient.
#[derive(Debug, Clone, Copy)]
pub struct Monomial {
    /// The exponent of the monomial.
   pub exponent: u32,
    /// The coefficient of the monomial.
   pub coefficients: f32,
}
impl Monomial {
    /// Creates a new `Monomial` with the given exponent and coefficient.
    ///
    /// # Arguments
    ///
    /// * `exponent` - The exponent of the monomial.
    /// * `coefficients` - The coefficient of the monomial.
    ///
    /// # Returns
    ///
    /// A new `Monomial` instance.
    pub fn new(exponent: u32, coefficients: f32) -> Monomial {
        Monomial {
            exponent,
            coefficients,
        }
    }
    /// Creates a default `Monomial` with an exponent of 0 and a coefficient of 0.0.
    ///
    /// # Returns
    ///
    /// A default `Monomial` instance.
    pub fn default() -> Monomial {
        Monomial {
            exponent: 0,
            coefficients: 0.0,
        }
    }
}
/// Represents a polynomial, which is a sum of monomials.
#[derive(Debug, Clone)]
pub struct UnivariatePolynomial {
    /// The list of monomials that make up the polynomial.
    monomials: Vec<Monomial>,
    /// The degree of the polynomial, if known.
    pub degree: Option<u32>,
}
impl UnivariatePolynomial {
    /// Adds a monomial to the polynomial. If a monomial with the same exponent already exists,
    /// their coefficients are combined.
    ///
    /// # Arguments
    ///
    /// * `exponent` - The exponent of the monomial to add.
    /// * `coefficients` - The coefficient of the monomial to add.

    pub fn new(monomials: Vec<Monomial>) -> UnivariatePolynomial {
        UnivariatePolynomial {
            monomials,
            degree: None,
        }
    }
    /// Creates a default `Polynomial` with no monomials and no degree.
    ///
    /// # Returns
    ///
    /// A default `Polynomial` instance.
   pub fn default() -> UnivariatePolynomial {
        UnivariatePolynomial {
            monomials: Vec::new(),
            degree: None,
        }
    }
    /// Evaluates the polynomial at a given value of `x`.
    ///
    /// # Arguments
    ///
    /// * `x` - The value at which to evaluate the polynomial.
    ///
    /// # Returns
    ///
    /// The result of evaluating the polynomial at `x`.
   pub  fn evaluate(&self, x: f32) -> f32 {
        let mut result: f32 = 0.0;
        let n = self.monomials.len();
        for i in 0..n {
            result += self.monomials[i].coefficients * x.powf(self.monomials[i].exponent as f32);
        }
        return result;
    }

    /// Returns the degree of the polynomial.
    ///
    /// # Returns
    ///
    /// The degree of the polynomial, if known.
   pub fn degree(&mut self) -> Option<u32> {
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
    /// Performs Lagrange interpolation to find a polynomial that passes through the given points.
    ///
    /// # Arguments
    ///
    /// * `x` - A vector of x-coordinates of the points.
    /// * `y` - A vector of y-coordinates of the points.
    ///
    /// # Returns
    ///
    /// A `UnivariatePolynomial` that passes through the given points.
    pub fn interpolate(x: Vec<f32>, y: Vec<f32>) -> UnivariatePolynomial {
        let n = x.len();
        let mut result = UnivariatePolynomial::default();

        for i in 0..n {
            let mut denominator: f32 = 1.0;

            let mut numerator = UnivariatePolynomial::new(vec![Monomial::new(0, 1.0)]);

            let mut a = y[i];
            for j in 0..n {
                if i != j {
                    let x_n = Monomial::new(1, 1.0); // x
                    let x_j = Monomial::new(0, -x[j]);
                    let temp_poly = UnivariatePolynomial::new(vec![x_n, x_j]);

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

impl Mul for UnivariatePolynomial {
    type Output = UnivariatePolynomial;
    /// Multiplies two polynomials and returns the result.
    ///
    /// # Arguments
    ///
    /// * `p2` - The polynomial to multiply by.
    ///
    /// # Returns
    ///
    /// A new `Polynomial` representing the product of the two polynomials.

    fn mul(self, p2: UnivariatePolynomial) -> Self {
        let p1: Vec<Monomial> = self.monomials;
        let p2: Vec<Monomial> = p2.monomials;

        let mut polynomial: Vec<Monomial> = Vec::new();
        for i in 0..p1.len() {
            for j in 0..p2.len() {
                polynomial.push(Monomial {
                    coefficients: p1[i].coefficients * p2[j].coefficients,
                    exponent: p1[i].exponent.wrapping_add(p2[j].exponent),
                });
            }
        }
        // Combine monomials with the same exponent

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
        UnivariatePolynomial {
            monomials: polynomial,
            degree: None,
        }
    }
}

impl Add for UnivariatePolynomial {
    type Output = UnivariatePolynomial;
    /// Adds two polynomials and returns the result.
    ///
    /// # Arguments
    ///
    /// * `p2` - The polynomial to add.
    ///
    /// # Returns
    ///
    /// A new `Polynomial` representing the sum of the two polynomials.

    fn add(self, p2: UnivariatePolynomial) -> Self {
        let p1: Vec<Monomial> = self.monomials;
        let p2: Vec<Monomial> = p2.monomials;
        let mut polynomial: Vec<Monomial> = Vec::new();
        polynomial = [p1, p2].concat();
        // Combine monomials with the same exponent

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
        UnivariatePolynomial {
            monomials: polynomial,
            degree: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    /// Tests the `evaluate` method of the `Polynomial` struct.
    fn test_evaluate() {
        let default = UnivariatePolynomial::default();
        let m1 = Monomial::new(2, 3.0);
        let m2 = Monomial::new(1, 2.0);
        let m3 = Monomial::new(0, 5.0);

        let p = UnivariatePolynomial {
            monomials: vec![m1, m2, m3],
            ..default
        };
        let result = p.evaluate(4.0);
        assert_eq!(result, 61.0);
    }

    /// Tests the `degree` method of the `UnivariatePolynomial` struct.

    #[test]
    fn test_degree() {
        let default = UnivariatePolynomial::default();
        let m1 = Monomial::new(2, 3.0);
        let m2 = Monomial::new(1, 2.0);

        let mut p = UnivariatePolynomial {
            monomials: vec![m1, m2],
            ..default
        };
        let result = p.degree();
        assert_eq!(result, Some(2));
    }

    /// Tests the multiplication of two UnivariatePolynomials.

    #[test]
    fn test_multiplication() {
        let default = UnivariatePolynomial::default();
        let m1 = Monomial::new(3, 4.0);
        let m2 = Monomial::new(2, 3.0);
        let m5 = Monomial::new(1, 3.0);
        let m3 = Monomial::new(2, 5.0);
        let m4 = Monomial::new(1, 7.0);

        let  p1 = UnivariatePolynomial {
            monomials: vec![m1, m2, m5],
            ..default
        };
        let  p2 = UnivariatePolynomial {
            monomials: vec![m3, m4],
            ..default
        };
        let result = p1 * p2;
        assert_eq!(result.monomials[0].coefficients, 20.0);
        assert_eq!(result.monomials[1].coefficients, 43.0);
        assert_eq!(result.monomials[2].coefficients, 36.0);
        assert_eq!(result.monomials[3].coefficients, 21.0);
        assert_eq!(result.monomials[0].exponent, 5);
        assert_eq!(result.monomials[1].exponent, 4);
        assert_eq!(result.monomials[2].exponent, 3);
        assert_eq!(result.monomials[3].exponent, 2);
    }

    /// Tests the Lagrange interpolation method.
    #[test]
    fn test_interpolate() {
        let x = vec![1.0, 2.0, 3.0];
        let y = vec![1.0, 4.0, 9.0];
        let result = UnivariatePolynomial::interpolate(x, y);
        assert_eq!(result.monomials[0].coefficients, 1.0);
        assert_eq!(result.monomials[0].exponent, 2);
    }
}
