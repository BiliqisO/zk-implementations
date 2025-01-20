use ark_ff::PrimeField;

use std::ops::{Add, Mul};

/// Represents a single term in a polynomial, consisting of an exponent and a coefficient.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Monomial<F: PrimeField> {
    /// The exponent of the monomial.
    pub exponent: usize,
    /// The coefficient of the monomial.
    pub coefficients: F,
}
impl<F: PrimeField> Monomial<F> {
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
    pub fn new(exponent: usize, coefficients: F) -> Monomial<F> {
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
    pub fn default() -> Monomial<F> {
        Monomial {
            exponent: 0,
            coefficients: F::zero(),
        }
    }
}
/// Represents a polynomial, which is a sum of monomials.
#[derive(Debug, Clone)]
pub struct UnivariatePolynomial<F: PrimeField> {
    /// The list of monomials that make up the polynomial.
    monomials: Vec<Monomial<F>>,
    /// The degree of the polynomial, if known.
    pub degree: Option<u32>,
}
impl<F: PrimeField> UnivariatePolynomial<F> {
    /// Adds a monomial to the polynomial. If a monomial with the same exponent already exists,
    /// their coefficients are combined.
    ///
    /// # Arguments
    ///
    /// * `exponent` - The exponent of the monomial to add.
    /// * `coefficients` - The coefficient of the monomial to add.

    pub fn new(monomials: Vec<Monomial<F>>) -> UnivariatePolynomial<F> {
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
    pub fn default() -> UnivariatePolynomial<F> {
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
    pub fn evaluate(&self, x: i64) -> F {
        let mut result: F = F::zero();
        let n = self.monomials.len();
        for i in 0..n {
            result += self.monomials[i].coefficients
                * F::from(x).pow(&[self.monomials[i].exponent as u64]);
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
                if self.monomials[i].exponent as u32 > self.degree.unwrap_or(0) {
                    self.degree = Some(self.monomials[i].exponent as u32);
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
    //add prime modulo
    pub fn interpolate(x: Vec<F>, y: Vec<F>) -> UnivariatePolynomial<F> {
        let n = x.len();
        let mut result = UnivariatePolynomial::default();

        // result.set_prime_modulo(p);

        for i in 0..n {
            let mut denominator = F::from(1);

            let mut numerator = UnivariatePolynomial::new(vec![Monomial::new(0, F::one())]);

            let mut a = y[i];
            for j in 0..n {
                if i != j {
                    let x_n = Monomial::new(1, F::one()); // x
                    let x_j = Monomial::new(0, F::from(-x[j]));
                    let temp_poly = UnivariatePolynomial::new(vec![x_n, x_j]);

                    numerator = numerator * temp_poly;

                    denominator = denominator * (F::from(x[i]) - F::from(x[j]));
                }
            }

            a /= denominator;

            for monomial in &mut numerator.monomials {
                monomial.coefficients *= F::from(a);
            }

            result = result + numerator;
        }

        result
    }
}

impl<F: PrimeField> Mul for UnivariatePolynomial<F> {
    type Output = UnivariatePolynomial<F>;
    /// Multiplies two polynomials and returns the result.
    ///
    /// # Arguments
    ///
    /// * `p2` - The polynomial to multiply by.
    ///
    /// # Returns
    ///
    /// A new `Polynomial` representing the product of the two polynomials.

    fn mul(self, p2: UnivariatePolynomial<F>) -> Self {
        let p1: Vec<Monomial<F>> = self.monomials;
        let p2: Vec<Monomial<F>> = p2.monomials;

        let mut polynomial: Vec<Monomial<F>> = Vec::new();
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
                    let (left, right) = polynomial.split_at_mut(j);
                    left[i].coefficients += right[0].coefficients;
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

impl<F: PrimeField> Add for UnivariatePolynomial<F> {
    type Output = UnivariatePolynomial<F>;
    /// Adds two polynomials and returns the result.
    ///
    /// # Arguments
    ///
    /// * `p2` - The polynomial to add.
    ///
    /// # Returns
    ///
    /// A new `Polynomial` representing the sum of the two polynomials.

    fn add(self, p2: UnivariatePolynomial<F>) -> Self {
        let p1: Vec<Monomial<F>> = self.monomials;
        let p2: Vec<Monomial<F>> = p2.monomials;
        let mut polynomial: Vec<Monomial<F>> = Vec::new();
        polynomial = [p1, p2].concat();
        // Combine monomials with the same exponent

        for i in 0..polynomial.len() {
            let mut j = i + 1;
            while j < polynomial.len() {
                if polynomial[i].exponent == polynomial[j].exponent {
                    let (left, right) = polynomial.split_at_mut(j);
                    left[i].coefficients += right[0].coefficients;
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
    use ark_bn254::Fq;

    #[test]
    /// Tests the `evaluate` method of the `Polynomial` struct.
    fn test_evaluate() {
        let default = UnivariatePolynomial::default();
        let m1 = Monomial::new(2, Fq::from(3u32));
        let m2 = Monomial::new(1, Fq::from(2u32));
        let m3 = Monomial::new(0, Fq::from(5u32));

        let p = UnivariatePolynomial {
            monomials: vec![m1, m2, m3],
            ..default
        };
        let result = p.evaluate(4);
        assert_eq!(result, Fq::from(61u32));
    }

    /// Tests the `degree` method of the `UnivariatePolynomial` struct.

    #[test]
    fn test_degree() {
        let default = UnivariatePolynomial::<Fq>::default();
        let m1 = Monomial::new(2, Fq::from(3u32));
        let m2 = Monomial::new(1, Fq::from(2u32));

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
        let m1 = Monomial::new(3, Fq::from(4u32));
        let m2 = Monomial::new(2, Fq::from(3u32));
        let m5 = Monomial::new(1, Fq::from(3u32));
        let m3 = Monomial::new(2, Fq::from(5u32));
        let m4 = Monomial::new(1, Fq::from(7u32));

        let p1 = UnivariatePolynomial {
            monomials: vec![m1, m2, m5],
            ..default
        };
        let p2 = UnivariatePolynomial {
            monomials: vec![m3, m4],
            ..default
        };
        let result = p1 * p2;
        assert_eq!(result.monomials[0].coefficients, Fq::from(20u32));
        assert_eq!(result.monomials[1].coefficients, Fq::from(43u32));
        assert_eq!(result.monomials[2].coefficients, Fq::from(36u32));
        assert_eq!(result.monomials[3].coefficients, Fq::from(21u32));
        assert_eq!(result.monomials[0].exponent, 5);
        assert_eq!(result.monomials[1].exponent, 4);
        assert_eq!(result.monomials[2].exponent, 3);
        assert_eq!(result.monomials[3].exponent, 2);
    }

    /// Tests the Lagrange interpolation method.
    #[test]
    fn test_interpolate() {
        let x = vec![Fq::from(1), Fq::from(2), Fq::from(3)];
        let y = vec![Fq::from(1), Fq::from(4), Fq::from(9)];
        let result = UnivariatePolynomial::<Fq>::interpolate(x, y);
        assert_eq!(result.monomials[0].coefficients, Fq::from(1u32));
        assert_eq!(result.monomials[0].exponent, 2);
    }
}
