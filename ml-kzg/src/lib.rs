use ark_ff::{PrimeField, Zero};
use ark_ec::{pairing::Pairing, PrimeGroup};
mod trusted_setup;


fn commit<F: PrimeField, P:Pairing>(poly_values:Vec<F>,  lagrange_basis:Vec<<P>::G1>)-> <P>::G1{    
    assert_eq!(poly_values.len(), lagrange_basis.len(), "len of values of poly should be equal to len of lagrange basis");
    let mut sum = P::G1::zero();
    for i in 0..poly_values.len(){
         sum += lagrange_basis[i].mul_bigint(poly_values[i].into_bigint());
    }
    sum

}
#[cfg(test)]
mod tests {
    use crate::trusted_setup::Tau;

    use super::*;
    use ark_bn254::Fq;

    #[test]
    fn test_commit(){
        let poly_values =  vec![
                Fq::from(1),
                Fq::from(2),
                Fq::from(3),
                Fq::from(4),
        ];
        let taus = vec![Fq::from(45u64), Fq::from(2u64)];
       let powers_of_tau  =  Tau::< ark_bn254::Bn254>::initialise(taus);   
       let lagrange_basis = powers_of_tau.lagrange_basis; 

       let commit =  commit::<ark_bn254::Fq, ark_bn254::Bn254>(poly_values, lagrange_basis);
       println!("commit {}", commit);


    }
}