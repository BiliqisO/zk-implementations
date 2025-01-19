# Polynomial Functions

A Rust library for polynomial operations, including addition, multiplication, evaluation, and Lagrange interpolation for univariate polynomials.

## Features

- **Addition**: Add two polynomials together.
- **Multiplication**: Multiply two polynomials.
- **Evaluation**: Evaluate a polynomial at a given value of `x`.
- **Lagrange Interpolation**: Perform Lagrange interpolation to find a polynomial that passes through given points.

## Usage

Add the following to your `Cargo.toml`:

```toml
[dependencies]
 zkpolynomial = "0.1.0"
```

## Example

Here's an example of how to use the library:

```rust
use polynomials::{Monomial, UnivariatePolynomial};

fn main() {
    // Create some monomials
    let m1 = Monomial::new(2, 3.0);
    let m2 = Monomial::new(1, 2.0);
    let m3 = Monomial::new(0, 5.0);

    // Create a polynomial
    let p = UnivariatePolynomial::new(vec![m1, m2, m3]);

    // Evaluate the polynomial at x = 4.0
    let result = p.evaluate(4.0);
    println!("P(4.0) = {}", result);

    // Perform Lagrange interpolation
    let x = vec![1.0, 2.0, 3.0];
    let y = vec![1.0, 4.0, 9.0];
    let interpolated_poly = UnivariatePolynomial::interpolate(x, y);
    println!("Interpolated Polynomial: {:?}", interpolated_poly);
}
```

## Documentation

### Monomial

Represents a single term in a polynomial, consisting of an exponent and a coefficient.

#### Methods

- new(exponent: u32, coefficients: f32) -> Monomial: Creates a new Monomial with the given exponent and coefficient.

- default() -> Monomial: Creates a default Monomial with an exponent of 0 and a coefficient of 0.0.

### UnivariatePolynomial

Represents a polynomial, which is a sum of monomials.

#### Methods

- new(monomials: Vec<Monomial>) -> UnivariatePolynomial: Creates a new UnivariatePolynomial with the given monomials.

- default() -> UnivariatePolynomial: Creates a default

UnivariatePolynomial with no monomials and no degree.

- evaluate(&self, x: f32) -> f32: Evaluates the polynomial at a given value of x.

- degree(&mut self) -> Option<u32>: Returns the degree of the polynomial, if known.

- interpolate(x: Vec<f32>, y: Vec<f32>) -> UnivariatePolynomial: Performs Lagrange interpolation to find a polynomial that passes through the given points.

### Trait Implementations

#### Mul for UnivariatePolynomial

Multiplies two polynomials and returns the result.

```rust
impl Mul for UnivariatePolynomial {
    type Output = UnivariatePolynomial;

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
```

#### Add for UnivariatePolynomial

Adds two polynomials and returns the result.

```rust
impl Add for UnivariatePolynomial {
    type Output = UnivariatePolynomial;

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
```

## Tests

### Evaluate Method

Tests the evaluate method of the UnivariatePolynomial struct.

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
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
}
```

### Degree Method

Tests the degree method of the UnivariatePolynomial struct.

```rust
#[cfg(test)]
mod tests {
    use super::*;

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
}
```

### Multiplication Method

Tests the multiplication of two

UnivariatePolynomial

s.

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_multiplication() {
        let default = UnivariatePolynomial::default();
        let m1 = Monomial::new(3, 4.0);
        let m2 = Monomial::new(2, 3.0);
        let m5 = Monomial::new(1, 3.0);
        let m3 = Monomial::new(2, 5.0);
        let m4 = Monomial::new(1, 7.0);

        let mut p1 = UnivariatePolynomial {
            monomials: vec![m1, m2, m5],
            ..default
        };
        let mut p2 = UnivariatePolynomial {
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
}
```

### Lagrange Interpolation Method

Tests the Lagrange interpolation method.

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_interpolate() {
        let default = UnivariatePolynomial::default();
        let x = vec![1.0, 2.0, 3.0];
        let y = vec![1.0, 4.0, 9.0];
        let result = UnivariatePolynomial::interpolate(x, y);
        assert_eq!(result.monomials[0].coefficients, 1.0);
        assert_eq!(result.monomials[0].exponent, 2);
    }
}
```

## License

This project is licensed under the MIT License. See the LICENSE file for details.

## Contributing

Contributions are welcome! Please open an issue or submit a pull request on GitHub.

## Repository

[GitHub Repository](https://github.com/BiliqisO/zk-implementations/tree/main/polynomial-functions)
