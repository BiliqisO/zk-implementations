use ark_ff::{PrimeField, Zero};
use ark_ec::{pairing::Pairing, PrimeGroup};
use multilinear_polynomial::EvaluationFormPolynomial;
mod trusted_setup;


fn commit<F: PrimeField, P:Pairing>(poly_values:Vec<F>,  lagrange_basis:Vec<<P>::G1>)-> <P>::G1{    
    assert_eq!(poly_values.len(), lagrange_basis.len(), "len of values of poly should be equal to len of lagrange basis");
    let mut sum = P::G1::zero();
    for i in 0..poly_values.len(){
         sum += lagrange_basis[i].mul_bigint(poly_values[i].into_bigint());
    }
    sum
}
fn open<F:PrimeField>(poly_values:&Vec<F>, open_values:&Vec<F>)-> F{
    let variables_len: F = F::from(open_values.len() as u64);
    assert_eq!(F::from(poly_values.len() as  u64), F::from(2).pow(variables_len.into_bigint()));
    let mut partial_evaluation = F::zero();
    let mut poly = EvaluationFormPolynomial::new(&poly_values);
    for i in 0..open_values.len(){
        poly = poly.partial_evaluate(open_values[i], 0);
   
    }
    
    poly.representation[0]   

}

pub fn blowup_sub<F:PrimeField>(poly1:&Vec<F>, poly2:&Vec<F>) -> EvaluationFormPolynomial<F> {    
     let mut result =  vec![];
        for i in 0..poly1.len() {
            let first_poly = poly1[i];
            for j in 0..poly2.len() {
                let second_poly = first_poly + poly2[j];
                result.push(second_poly);
            }
        }
          EvaluationFormPolynomial::new(&result)
    }

fn generate_proofs<F:PrimeField>(poly_values:&Vec<F>, open_values:&Vec<F>){
    let open = open(poly_values, open_values);
    let poly_minus_open = poly_values.iter().map(|x| *x - open).collect::<Vec<F>>();
    let mut poly = EvaluationFormPolynomial::new(&poly_minus_open);
    // let mid = poly.representation.len() / 2;
    // let (first_half, second_half) = poly.representation.split_at(mid);

    // let univariate_polynomial: Vec<F> = vec![first_half_sum, second_half_sum];
    let mut quotient_evals_vec = vec![];
        for j in 0..open_values.len() -1{
            
            let mut eval_at_1 = poly.partial_evaluate(F::from(1), 0);
            let eval_at_0 = eval_at_1.partial_evaluate(F::from(0), 0);
            // println!("eval_at_1 {:?} j {}",  eval_at_1, j);
            // println!("eval_at_0  {:?} j {}",  eval_at_0, j);
         
            let mut quotient_poly = blowup_sub(&eval_at_1.representation, &eval_at_0.representation);
            println!("quotient_poly {:?}", quotient_poly);

            let quotient_eval = quotient_poly.partial_evaluate(open_values[j], 0);  
            quotient_evals_vec.push(quotient_eval);

            poly = poly.partial_evaluate(open_values[j], 0);
        }
        println!("quotient_evals_vec {:?}", quotient_evals_vec);



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
     #[test]
     fn test_generate_proofs(){
    let poly_values = vec![ 
                Fq::from(0),
                Fq::from(4),
                Fq::from(0),
                Fq::from(4),
                Fq::from(0),
                Fq::from(4),
                Fq::from(3),
                Fq::from(7),];
        let open_values = vec![ 
                Fq::from(6),
                Fq::from(4),
                Fq::from(0),];
        let open = generate_proofs(&poly_values, &open_values);
        // assert_eq!(open, Fq::from(72));
     }
     #[test]
    fn test_open(){
        let poly_values = vec![ 
                Fq::from(0),
                Fq::from(4),
                Fq::from(0),
                Fq::from(4),
                Fq::from(0),
                Fq::from(4),
                Fq::from(3),
                Fq::from(7),];
        let open_values = vec![ 
                Fq::from(6),
                Fq::from(4),
                Fq::from(0),];
        let open = open(&poly_values, &open_values);
        assert_eq!(open, Fq::from(72));
    }

}