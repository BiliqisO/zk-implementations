use ark_ff::PrimeField;
use ark_std;
use polynomials::{Monomial, UnivariatePolynomial};
/// Sets up the Shamir Secret Sharing scheme.
///
/// This function generates `n` shares from a given secret using a polynomial of degree `threshold - 1`.
///
/// # Arguments
///
/// * `secret` - The secret to be shared.
/// * `threshold` - The minimum number of shares required to reconstruct the secret.
/// * `n` - The total number of shares to generate.
///
/// # Returns
///
/// A vector of tuples where each tuple contains a share (x, y).

pub fn setup<F: PrimeField>(secret: F, threshold: usize, n: usize) -> Vec<(F, F)> {
    let mut monomials = Vec::new();
    let m1 = Monomial::new(0, secret);
    monomials.push(m1);

    for i in 1..threshold {
        let mut rng = ark_std::rand::thread_rng();

        let coeff = F::rand(&mut rng);

        let monomial = Monomial::new(i, coeff);
        monomials.push(monomial);
    }
    let polynomial = UnivariatePolynomial::new(monomials);
    let mut shares: Vec<(F, F)> = Vec::new();

    for _ in 0..n {
        let mut rng = ark_std::rand::thread_rng();
        let shares_x = F::rand(&mut rng);
        let shares_y = polynomial.evaluate(shares_x);
        shares.push((shares_x, shares_y));
    }

    shares
}

/// Reconstructs the secret from the given shares using the Shamir Secret Sharing scheme.
///
/// This function interpolates a polynomial from the given shares and evaluates it at zero to recover the secret.
///
/// # Arguments
///
/// * `x` - A vector of x-coordinates of the shares.
/// * `y` - A vector of y-coordinates of the shares.
///
/// # Returns
///
/// The reconstructed secret.
pub fn reconstruct_data<F: PrimeField>(x: Vec<F>, y: Vec<F>) -> F {
    let data_poly: UnivariatePolynomial<F> = UnivariatePolynomial::interpolate(x, y);

    let data = data_poly.evaluate(F::from(0));

    return data;
}

/// Sets up the Shamir Secret Sharing scheme with an additional password.
///
/// This function generates `n` shares from a given secret and password using a polynomial of degree `threshold - 1`.
///
/// # Arguments
///
/// * `secret` - The secret to be shared.
/// * `threshold` - The minimum number of shares required to reconstruct the secret.
/// * `n` - The total number of shares to generate.
/// * `password` - An additional password used in the share generation.
///
/// # Returns
///
/// A vector of tuples where each tuple contains a share (x, y).
pub fn passworded_setup<F: PrimeField>(
    secret: F,
    threshold: usize,
    n: usize,
    password: F,
) -> Vec<(F, F)> {
    let mut x_s = Vec::new();
    let mut y_s = Vec::new();
    let mut shares: Vec<(F, F)> = Vec::new();
    x_s.push(password);
    y_s.push(secret);

    for _ in 1..threshold {
        let mut rng = ark_std::rand::thread_rng();
        let x = F::rand(&mut rng);
        let y = F::rand(&mut rng);
        x_s.push(x);
        y_s.push(y);
    }
    let polynomial = UnivariatePolynomial::interpolate(x_s, y_s);
    for _ in 0..n {
        let mut rng = ark_std::rand::thread_rng();
        let shares_x = F::rand(&mut rng);
        let shares_y = polynomial.evaluate(shares_x);
        shares.push((shares_x, shares_y));
    }
    return shares;
}

/// Reconstructs the secret from the given shares and password using the Shamir Secret Sharing scheme.
///
/// This function interpolates a polynomial from the given shares and evaluates it at the password to recover the secret.
///
/// # Arguments
///
/// * `x` - A vector of x-coordinates of the shares.
/// * `y` - A vector of y-coordinates of the shares.
/// * `password` - The password used in the share generation.
///
/// # Returns
///
/// The reconstructed secret.

pub fn reconstruct_data_with_password<F: PrimeField>(x: Vec<F>, y: Vec<F>, password: F) -> F {
    println!("x{}", x.len());

    let data_poly: UnivariatePolynomial<F> = UnivariatePolynomial::interpolate(x, y);
    println!("data_poly {:?}", data_poly);
    let data = data_poly.evaluate(password);
    println!("data {:?}", data);
    return data;
}

#[cfg(test)]
mod tests {
    use super::*;
    use ark_bn254::Fq;

    #[test]
    fn test_shamir() {
        let points = setup(Fq::from(355), 4, 10);

        let point_1_x = points[0].0;
        let point_2_x = points[1].0;
        let point_3_x = points[2].0;
        let point_4_x = points[3].0;
        let point_1_y = points[0].1;
        let point_2_y = points[1].1;
        let point_3_y = points[2].1;
        let point_4_y = points[3].1;
        let x = vec![point_1_x, point_2_x, point_3_x, point_4_x];
        let y = vec![point_1_y, point_2_y, point_3_y, point_4_y];
        let secret = reconstruct_data::<Fq>(x, y);

        assert_eq!(secret, Fq::from(355))
    }

    #[test]
    fn test_shamir_passworded() {
        let points = passworded_setup(Fq::from(3000000), 4, 10, Fq::from(40));

        let point_1_x = points[0].0;
        let point_2_x = points[1].0;
        let point_3_x = points[2].0;
        let point_4_x = points[3].0;
        let point_1_y = points[0].1;
        let point_2_y = points[1].1;
        let point_3_y = points[2].1;
        let point_4_y = points[3].1;
        let x = vec![point_1_x, point_2_x, point_3_x, point_4_x];
        let y = vec![point_1_y, point_2_y, point_3_y, point_4_y];
        let secret = reconstruct_data_with_password::<Fq>(x, y, Fq::from(40));
        assert_eq!(secret, Fq::from(3000000))
    }
}
