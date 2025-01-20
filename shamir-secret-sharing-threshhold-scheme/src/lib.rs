use ark_ff::PrimeField;
use ark_std;
use polynomials::{Monomial, UnivariatePolynomial};

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
    println!(" polynomial {:?}", polynomial);
    let mut shares: Vec<(F, F)> = Vec::new();

    for _ in 0..n {
        let mut rng = ark_std::rand::thread_rng();
        let shares_x = F::rand(&mut rng);
        let shares_y = polynomial.evaluate(shares_x);
        shares.push((shares_x, shares_y));
    }
    println!(" polynomialqwer1 {:?}", polynomial);
    println!("shares , {:?}", shares);

    shares
}
pub fn reconstruct_data<F: PrimeField>(x: Vec<F>, y: Vec<F>) -> F {
    println!("x{}", x.len());

    let data_poly: UnivariatePolynomial<F> = UnivariatePolynomial::interpolate(x, y);
    println!("data_poly {:?}", data_poly);
    let data = data_poly.evaluate(F::from(0));
    println!("data {:?}", data);
    return data;
}

#[cfg(test)]
mod tests {
    use super::*;
    use ark_bn254::Fq;

    #[test]
    fn test() {
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
       let secret =  reconstruct_data::<Fq>(x, y);
       assert_eq!(secret, Fq::from(355))
    }
}
