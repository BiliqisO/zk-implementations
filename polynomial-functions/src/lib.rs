use core::num;
use std::ops::{Add, Mul};
fn modulo(a: i32, n: i32) -> i32 {
    ((a % n) + n) % n
}
fn mod_inverse(b: i64, p: i64) -> i64 {
    let mut a = b;
    let mut m = p;
    let mut u = 1;
    let mut v = 0;

    while a != 0 {
        let t = m / a;
        m -= t * a;
        std::mem::swap(&mut a, &mut m);
        v -= t * u;
        std::mem::swap(&mut u, &mut v);
    }

    (v + p) % p
}
/// Represents a single term in a polynomial, consisting of an exponent and a coefficient.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Monomial {
    /// The exponent of the monomial.
    pub exponent: u32,
    /// The coefficient of the monomial.
    pub coefficients: i64,
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
    pub fn new(exponent: u32, coefficients: i64) -> Monomial {
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
            coefficients: 0,
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
    pub prime_modulo: Option<u32>,
}
impl UnivariatePolynomial {
    /// Adds a monomial to the polynomial. If a monomial with the same exponent already exists,
    /// their coefficients are combined.
    ///
    /// # Arguments
    ///
    /// * `exponent` - The exponent of the monomial to add.
    /// * `coefficients` - The coefficient of the monomial to add.

    pub fn new(monomials: Vec<Monomial>, prime_modulo: u32) -> UnivariatePolynomial {
        UnivariatePolynomial {
            monomials,
            degree: None,
            prime_modulo: Some(prime_modulo),
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
            prime_modulo: Some(1),
        }
    }
    pub fn set_prime_modulo(&mut self, prime_modulo: u32) {
        self.prime_modulo = Some(prime_modulo);
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
    pub fn evaluate(&self, x: i64) -> i64 {
        let mut result: i64 = 0;
        let n = self.monomials.len();
        for i in 0..n {
            result += self.monomials[i].coefficients * x.pow(self.monomials[i].exponent as u32);
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
    //add prime modulo
    pub fn interpolate(x: Vec<i64>, y: Vec<i64>, p: u32) -> UnivariatePolynomial {
        let n = x.len();
        let mut result = UnivariatePolynomial::default();

        result.set_prime_modulo(p);

        for i in 0..n {
            let mut denominator: i64 = 1;

            let mut numerator = UnivariatePolynomial::new(vec![Monomial::new(0, 1)], p);

            let mut a = y[i];
            for j in 0..n {
                if i != j {
                    let x_n = Monomial::new(1, 1); // x
                    let x_j =
                        Monomial::new(0, modulo((-x[j]).try_into().unwrap(), p as i32) as i64);
                    let temp_poly = UnivariatePolynomial::new(vec![x_n, x_j], p);

                    numerator = numerator * temp_poly;

                    denominator *= x[i] - x[j];
                    denominator = modulo(denominator as i32, p as i32) as i64;
                }
            }

            a = a * mod_inverse(denominator, p as i64);

            for monomial in &mut numerator.monomials {
                monomial.coefficients = modulo((monomial.coefficients * a) as i32, p as i32) as i64;
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
        let p: u32 = self.prime_modulo.unwrap();

        let mut polynomial: Vec<Monomial> = Vec::new();
        for i in 0..p1.len() {
            for j in 0..p2.len() {
                let coefficients = modulo(
                    (modulo(p1[i].coefficients as i32, p as i32) as i64
                        * modulo(p2[j].coefficients as i32, p as i32) as i64)
                        as i32,
                    p as i32,
                ) as i64;
                polynomial.push(Monomial {
                    coefficients: (coefficients % p as i64 + p as i64) % p as i64,

                    exponent: p1[i].exponent.wrapping_add(p2[j].exponent),
                });
            }
        }
        // Combine monomials with the same exponent
        for i in 0..polynomial.len() {
            let mut j = i + 1;
            while j < polynomial.len() {
                if polynomial[i].exponent == polynomial[j].exponent {
                    polynomial[i].coefficients =
                        polynomial[i].coefficients + polynomial[j].coefficients;
                    polynomial[i].coefficients =
                        modulo((polynomial[i].coefficients) as i32, p as i32) as i64;

                    polynomial.remove(j);
                } else {
                    j += 1;
                }
            }
        }
        polynomial.sort();

        UnivariatePolynomial {
            monomials: polynomial,
            degree: None,
            prime_modulo: Some(p),
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
        let p: u32 = self.prime_modulo.unwrap();
        polynomial = [p1, p2].concat();
        // Combine monomials with the same exponent

        for i in 0..polynomial.len() {
            let mut j = i + 1;
            while j < polynomial.len() {
                if polynomial[i].exponent == polynomial[j].exponent {
                    polynomial[i].coefficients =
                        polynomial[i].coefficients + polynomial[j].coefficients;
                    polynomial[i].coefficients =
                        modulo((polynomial[i].coefficients) as i32, p as i32) as i64;

                    polynomial.remove(j);
                } else {
                    j += 1;
                }
            }
        }
        polynomial.sort();
        UnivariatePolynomial {
            monomials: polynomial,
            degree: None,
            prime_modulo: Some(p),
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
        let m1 = Monomial::new(2, 3);
        let m2 = Monomial::new(1, 2);
        let m3 = Monomial::new(0, 5);

        let p = UnivariatePolynomial {
            monomials: vec![m1, m2, m3],
            ..default
        };
        let result = p.evaluate(4);
        assert_eq!(result, 61);
    }

    /// Tests the `degree` method of the `UnivariatePolynomial` struct.

    #[test]
    fn test_degree() {
        let default = UnivariatePolynomial::default();
        let m1 = Monomial::new(2, 3);
        let m2 = Monomial::new(1, 2);

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
        let m1 = Monomial::new(2, 5);
        let m2 = Monomial::new(1, -4);
        let m5 = Monomial::new(0, 2);
        let m3 = Monomial::new(3, 1);
        let m4 = Monomial::new(2, -2);
        let m6 = Monomial::new(0, 5);

        let p1 = UnivariatePolynomial {
            monomials: vec![m1, m2, m5],
            prime_modulo: Some(6),
            ..default
        };
        let p2 = UnivariatePolynomial {
            monomials: vec![m3, m4, m6],
            ..default
        };
        let result = p1 * p2;
        assert_eq!(result.monomials[5].coefficients, 5);
        assert_eq!(result.monomials[4].coefficients, 4);
        assert_eq!(result.monomials[3].coefficients, 4);
        assert_eq!(result.monomials[2].coefficients, 3);
        assert_eq!(result.monomials[1].coefficients, 4);
        assert_eq!(result.monomials[0].coefficients, 4);
        assert_eq!(result.monomials[5].exponent, 5);
        assert_eq!(result.monomials[4].exponent, 4);
        assert_eq!(result.monomials[3].exponent, 3);
        assert_eq!(result.monomials[2].exponent, 2);
        assert_eq!(result.monomials[1].exponent, 1);
        assert_eq!(result.monomials[0].exponent, 0);
    }

    /// Tests the Lagrange interpolation method.
    #[test]
    fn test_interpolate() {
        let x = vec![0, -2, 2];
        let y = vec![4, 1, 3];
        let p = 5;
        let result = UnivariatePolynomial::interpolate(x, y, p);
        assert_eq!(result.monomials[0].coefficients, 4);
        assert_eq!(result.monomials[0].exponent, 0);
        assert_eq!(result.monomials[1].coefficients, 3);
        assert_eq!(result.monomials[1].exponent, 1);
        assert_eq!(result.monomials[2].coefficients, 2);
        assert_eq!(result.monomials[2].exponent, 2);
    }
}
