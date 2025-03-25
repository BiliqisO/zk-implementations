// File: trusted_setup.rs
use ark_ff::PrimeField;
use ark_ec::pairing::Pairing;
use ark_ec::PrimeGroup;
use multilinear_polynomial::boolean_hypercube::boolean_hypercube;

 pub struct Tau<P: Pairing> {  
    pub lagrange_basis: Vec<P::G1>,
    pub g2_tau:Vec<P::G2>,  

}
impl<P:Pairing> Tau<P> {
    pub fn initialise<F: PrimeField>(taus:Vec<F>)->Self{
        let generator1 =P::G1::generator();
        let generator2 =P::G2::generator();

        
        let no_of_variables = taus.len();
        let hypercube = boolean_hypercube::<F>(no_of_variables).into_iter().collect::<Vec<_>>();
        let mut lagrange_basis = Vec::new();
        for point in &hypercube {
            let mut value = F::one();
            for (i, bit) in point.chars().enumerate() {
            if bit == '1' {
                value *= taus[i];
            } else {
                value *= F::one() - taus[i];
            }
            }
            lagrange_basis.push(generator1.mul_bigint(value.into_bigint()));
        }
        let mut g2_tau = Vec::new();
        g2_tau = taus.iter().map(|i| generator2.mul_bigint(i.into_bigint())).collect::<Vec<_>>();
     
        // println!("lagrange_basis {:?}", lagrange_basis);
        // println!("g2_tau {:?}", g2_tau);
        Tau { lagrange_basis, g2_tau}
       
    } 
}
#[cfg(test)]
mod tests { 
    use super::*;
    use ark_bn254::Fq;
    #[test]
        fn test_initialise(){  
        let taus = vec![Fq::from(45u64), Fq::from(2u64), Fq::from(3u64)];
        Tau::< ark_bn254::Bn254>::initialise(taus);        
        }
 


}