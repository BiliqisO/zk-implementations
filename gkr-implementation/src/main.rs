mod gkr_sumcheck;
use ark_ff::Field;
use gkr_sumcheck as sumcheck;

use ark_ff::{BigInteger, PrimeField};
use fiat_shamir::{self, FiatShamir};
use multilinear_polynomial::{
    product_poly::{ProductPolynomial, SumPolynomial},
    EvaluationFormPolynomial,
};
use polynomials::UnivariatePolynomial;
use sha3::{digest::typenum::Sum, Digest, Sha3_256};
use std::ops::Add;

fn main() {
    println!("Hello, world!");
}
#[derive(Debug, Clone, Copy)]
enum Op {
    Add,
    Mul,
}
#[derive(Debug, Clone)]
struct Gate<F: PrimeField> {
    left: F,
    right: F,
    output: F,
    op: Op,
}
#[derive(Debug, Clone)]
struct Layer<F: PrimeField> {
    gates: Vec<Gate<F>>,
}
#[derive(Debug, Clone)]
struct Gkrproof<F: PrimeField> {
    output_mle: Vec<F>,
    sumcheck_proof: Vec<(F, Vec<Vec<F>>)>,
    w_s: Vec<(F, F)>,
}
#[derive(Debug, Clone)]
struct Circuit<F: PrimeField> {
    layers: Vec<Layer<F>>,
}
impl<F: PrimeField> Circuit<F> {
    fn new() -> Self {
        Self { layers: vec![] }
    }
    fn add_layer(&mut self, layer: Layer<F>) {
        self.layers.push(layer);
    }
    fn add_i(indices: Vec<F>, values: Vec<F>) -> F {
        let mut res = F::from(0);
        for i in 0..indices.len() {
            if indices[i] == F::from(0) {
                res = res * F::from(1) - values[i]
            } else if indices[i] == F::from(1) {
                res = res * values[i];
            }
        }
        res
    }
    fn w_i(values: Vec<F>) -> EvaluationFormPolynomial<F> {
        let poly = EvaluationFormPolynomial::new(&values);
        poly
    }

    fn add_i_or_mul_i(&self, layer: usize) -> (Vec<F>, Vec<F>) {
        let layers: Vec<Layer<F>> = self.layers.iter().rev().cloned().collect();
        let indices = self.clone().generate_gate_indices_for_layer_i(layer);

        let num_bits = indices.first().map(|idx| idx.len()).unwrap_or(0);
        let num_combinations = 2usize.pow(num_bits as u32);

        let mut add_i_vec = vec![0; num_combinations];
        let mut mul_i_vec = vec![0; num_combinations];

        for (i, gate) in layers[layer].gates.iter().enumerate() {
            let binary_string = &indices[i];
            let decimal_value = usize::from_str_radix(&binary_string, 2).unwrap_or(0);

            match gate.op {
                Op::Add => add_i_vec[decimal_value] = 1,
                Op::Mul => mul_i_vec[decimal_value] = 1,
                _ => continue,
            }
        }
        (
            add_i_vec.iter().map(|&x| F::from(x)).collect(),
            mul_i_vec.iter().map(|&x| F::from(x)).collect(),
        )
    }
    fn get_ws(&self, layer: usize) -> Vec<F> {
        let layers: Vec<Layer<F>> = self.layers.iter().rev().cloned().collect();
        let mut w = vec![];
        let x_s: Vec<F> = (0..=layer).map(|i| F::from(i as u64)).collect();
        w.push(vec![layers[0].gates[0].output]);
        for i in 0..layers.len() {
            let w0 = layers[i]
                .gates
                .iter()
                .flat_map(|gate| vec![gate.left, gate.right])
                .collect::<Vec<F>>();
            w.push(w0);
        }
        w[layer].clone()
    }
    fn verifier(self, gkr_proof: Gkrproof<F>) {
        let layers: Vec<Layer<F>> = self.layers.iter().rev().cloned().collect();

        let hash_function = Sha3_256::new();
        let mut fiat_shamir: FiatShamir<
            sha3::digest::core_api::CoreWrapper<sha3::Sha3_256Core>,
            F,
        > = FiatShamir::new(hash_function);
        let output_mle = gkr_proof.output_mle;
        let output_mle_bytes: Vec<u8> = output_mle
            .iter()
            .flat_map(|f| f.into_bigint().to_bits_be().into_iter().map(|b| b as u8))
            .collect();
        fiat_shamir.absorb(&output_mle_bytes);
        let r_1 = fiat_shamir.squeeze();
        let r_s = vec![r_1];




        
        let init_claim = EvaluationFormPolynomial::new(&output_mle)
            .partial_evaluate(r_1, 0)
            .representation[0];
        println!("init_claim {:?}", init_claim);

        let (add_i, mul_i) = self.add_i_or_mul_i(0);

        let mut new_add_poly = EvaluationFormPolynomial::new(&add_i);
        let mut new_mul_poly = EvaluationFormPolynomial::new(&mul_i);

        for i in 0..r_s.len() {
            new_add_poly = new_add_poly.partial_evaluate(r_s[i], 0);

            new_mul_poly = new_mul_poly.partial_evaluate(r_s[i], 0);
        }

        let mut init_f_bc: SumPolynomial<F> =
            self.generate_fbc(0, new_add_poly.clone(), new_mul_poly.clone());

        let ( claimed_sum,  _round_polys, mut random_challenges) =
            sumcheck::proof(init_f_bc, init_claim);
            assert_eq!(claimed_sum, init_claim);

        let (mut r_b, mut r_c) = random_challenges.split_at(random_challenges.len() / 2);
        for i in 1..layers.len() {
            let (w_rb, w_rc) = gkr_proof.w_s[i-1];
            let w_rb_poly = EvaluationFormPolynomial::new(&vec![w_rb]);
            let w_rc_poly = EvaluationFormPolynomial::new(&vec![w_rc]);
          
            let w_rb_bytes : Vec<u8>  = w_rb_poly.representation.iter()
            .flat_map(|f| f.into_bigint().to_bits_be().into_iter().map(|b| b as u8))
            .collect();
            let w_rc_bytes : Vec<u8>  = w_rc_poly.representation.iter()
            .flat_map(|f| f.into_bigint().to_bits_be().into_iter().map(|b| b as u8))
            .collect();
            fiat_shamir.absorb(&w_rb_bytes);
            let alpha = fiat_shamir.squeeze();
            fiat_shamir.absorb(&w_rc_bytes);
            let beta = fiat_shamir.squeeze();;
            let (alpha_add_i, alpha_mul_i) = self.add_i_or_mul_i(i);

            let mut alpha_add_i = EvaluationFormPolynomial::new(&alpha_add_i);
            let mut alpha_mul_i = EvaluationFormPolynomial::new(&alpha_mul_i);

            for &r_b_value in r_b {
                alpha_add_i = alpha_add_i.partial_evaluate(r_b_value, 0);
                alpha_mul_i = alpha_mul_i.partial_evaluate(r_b_value, 0);
            }

            alpha_add_i
                .representation
                .iter_mut()
                .for_each(|coeff| *coeff *= alpha);
            alpha_add_i = EvaluationFormPolynomial::new(&alpha_add_i.representation);
            alpha_mul_i
                .representation
                .iter_mut()
                .for_each(|coeff: &mut F| *coeff *= alpha);
            alpha_mul_i = EvaluationFormPolynomial::new(&alpha_mul_i.representation);

            let (beta_add_i, beta_mul_i) = self.add_i_or_mul_i(i);

            let mut beta_add_i = EvaluationFormPolynomial::new(&beta_add_i);
            let mut beta_mul_i = EvaluationFormPolynomial::new(&beta_mul_i);

            for &r_c_value in r_c {
                beta_add_i = beta_add_i.partial_evaluate(r_c_value, 0);
                beta_mul_i = beta_mul_i.partial_evaluate(r_c_value, 0);
            }

            beta_add_i
                .representation
                .iter_mut()
                .for_each(|coeff| *coeff *= beta);

            beta_add_i = EvaluationFormPolynomial::new(&beta_add_i.representation);

            beta_mul_i
                .representation
                .iter_mut()
                .for_each(|coeff| *coeff *= beta);

            beta_mul_i = EvaluationFormPolynomial::new(&beta_mul_i.representation);
            new_add_poly = alpha_add_i.add(beta_add_i);
            new_mul_poly = alpha_mul_i.add(beta_mul_i);
            let sum_check_res = &gkr_proof.sumcheck_proof[i];

            let x_s: Vec<F> = (0..=2).map(|i| F::from(i as u64)).collect();
            let y_s = &sum_check_res.1;

            let uni_polynomial = UnivariatePolynomial::interpolate(x_s, y_s[0].clone());
            let eval_at_0 = uni_polynomial.evaluate(F::zero());
            let eval_at_1 = uni_polynomial.evaluate(F::one());

            let verifier_sum = eval_at_0 + eval_at_1;
          

            init_f_bc = self.generate_fbc(i, new_add_poly.clone(), new_mul_poly.clone());
          

            let sumcheck_res = sumcheck::verify(init_f_bc, claimed_sum,y_s.to_vec());
            random_challenges = sumcheck_res.1; 
          

            let res: (&[F], &[F]) = random_challenges.split_at(random_challenges.len() / 2);
            r_b = res.0;
            r_c = res.1;
        

            for i in 0..random_challenges.len() {
                new_add_poly = new_add_poly.partial_evaluate(random_challenges[i], 0);

                new_mul_poly = new_mul_poly.partial_evaluate(random_challenges[i], 0);
            }
            let claim = alpha * (w_rb) + beta * (w_rc);
        
            assert_eq!(verifier_sum, claim);

            // let res = (new_add_poly_clone.representation[0] * (w_s.0 + w_s.1)) + (new_mul_poly.clone().representation[0] * (w_s.0 * w_s.1));

        }
    }
    fn proof(&self) -> Gkrproof<F> {
        let layers: Vec<Layer<F>> = self.layers.iter().rev().cloned().collect();
            let alpha = F::zero();
            let beta = F::zero();

        let hash_function = Sha3_256::new();
        let mut fiat_shamir: FiatShamir<
            sha3::digest::core_api::CoreWrapper<sha3::Sha3_256Core>,
            F,
        > = FiatShamir::new(hash_function);
        let mut output_mle = layers[0]
            .gates
            .iter()
            .map(|gate| gate.output)
            .collect::<Vec<F>>();
        if output_mle.len() == 1 {
            output_mle.push(F::from(0));
        }

        let output_mle_bytes: Vec<u8> = output_mle
            .iter()
            .flat_map(|f| f.into_bigint().to_bits_be().into_iter().map(|b| b as u8))
            .collect();
        fiat_shamir.absorb(&output_mle_bytes);

        // TODO: r_1 should depend on the number of variables for the output polynomial
        let r_1 = fiat_shamir.squeeze();
        let r_s = vec![r_1];

        let init_claim = EvaluationFormPolynomial::new(&output_mle)
            .partial_evaluate(r_1, 0)
            .representation[0];

        let (add_i, mul_i) = self.add_i_or_mul_i(0);

        let mut new_add_poly = EvaluationFormPolynomial::new(&add_i);
        let mut new_mul_poly = EvaluationFormPolynomial::new(&mul_i);

        for i in 0..r_s.len() {
            new_add_poly = new_add_poly.partial_evaluate(r_s[i], 0);

            new_mul_poly = new_mul_poly.partial_evaluate(r_s[i], 0);
        }
        let mut init_f_bc: SumPolynomial<F> =
            self.generate_fbc(0, new_add_poly.clone(), new_mul_poly.clone());

        let mut challenges_vec = vec![];
        let mut claimed_sum_vec = vec![];
        let (mut claimed_sum, mut round_polys, mut random_challenges) =
            sumcheck::proof(init_f_bc, init_claim);

        claimed_sum_vec.push(claimed_sum);

        let (mut r_b, mut r_c) = random_challenges.split_at(random_challenges.len() / 2);

        let mut w = self.get_ws(1);
        let mut w_i = EvaluationFormPolynomial::new(&w);
        let mut w_rb = w_i.clone();
        let mut w_rc = w_i.clone();

        for i in 0..r_b.len() {
            w_rb = w_rb.partial_evaluate(r_b[i], 0);
            w_rc = w_rc.partial_evaluate(r_c[i], 0);
        }

        let mut gkr_proof = Gkrproof {
            output_mle,
            sumcheck_proof: vec![(claimed_sum, round_polys)],
            w_s: vec![(w_rb.representation[0], w_rc.representation[0])],
        };

        challenges_vec.push(random_challenges.clone());
        for i in 1..layers.len() {
            w = self.get_ws(i + 1);
     
            w_i = EvaluationFormPolynomial::new(&w);
            
            let w_rb_bytes : Vec<u8>  = w_rb.representation.iter()
            .flat_map(|f| f.into_bigint().to_bits_be().into_iter().map(|b| b as u8))
            .collect();
            let w_rc_bytes : Vec<u8>  = w_rc.representation.iter()
            .flat_map(|f| f.into_bigint().to_bits_be().into_iter().map(|b| b as u8))
            .collect();
            fiat_shamir.absorb(&w_rb_bytes);
            let alpha = fiat_shamir.squeeze();
            fiat_shamir.absorb(&w_rc_bytes);
            let beta = fiat_shamir.squeeze();;
            let (alpha_add_i, alpha_mul_i) = self.add_i_or_mul_i(i);

            let mut alpha_add_i = EvaluationFormPolynomial::new(&alpha_add_i);
            let mut alpha_mul_i = EvaluationFormPolynomial::new(&alpha_mul_i);

            for &r_b_value in r_b {
                alpha_add_i = alpha_add_i.partial_evaluate(r_b_value, 0);
                alpha_mul_i = alpha_mul_i.partial_evaluate(r_b_value, 0);
            }

            alpha_add_i
                .representation
                .iter_mut()
                .for_each(|coeff| *coeff *= alpha);
            alpha_add_i = EvaluationFormPolynomial::new(&alpha_add_i.representation);
            alpha_mul_i
                .representation
                .iter_mut()
                .for_each(|coeff: &mut F| *coeff *= alpha);
            alpha_mul_i = EvaluationFormPolynomial::new(&alpha_mul_i.representation);

            let (beta_add_i, beta_mul_i) = self.add_i_or_mul_i(i);

            let mut beta_add_i = EvaluationFormPolynomial::new(&beta_add_i);
            let mut beta_mul_i = EvaluationFormPolynomial::new(&beta_mul_i);

            for &r_c_value in r_c {
                beta_add_i = beta_add_i.partial_evaluate(r_c_value, 0);
                beta_mul_i = beta_mul_i.partial_evaluate(r_c_value, 0);
            }

            beta_add_i
                .representation
                .iter_mut()
                .for_each(|coeff| *coeff *= beta);

            beta_add_i = EvaluationFormPolynomial::new(&beta_add_i.representation);

            beta_mul_i
                .representation
                .iter_mut()
                .for_each(|coeff| *coeff *= beta);

            beta_mul_i = EvaluationFormPolynomial::new(&beta_mul_i.representation);
            new_add_poly = alpha_add_i.add(beta_add_i);
            new_mul_poly = alpha_mul_i.add(beta_mul_i);

            init_f_bc = self.generate_fbc(i, new_add_poly, new_mul_poly);
            

            w_rb = w_i.clone();
            w_rc = w_i.clone();

            let sumcheck_res = sumcheck::proof(init_f_bc, init_claim);
            random_challenges = sumcheck_res.2;
            round_polys = sumcheck_res.1;
            claimed_sum = sumcheck_res.0;
            claimed_sum_vec.push(claimed_sum);

            let res: (&[F], &[F]) = random_challenges.split_at(random_challenges.len() / 2);
            r_b = res.0;
            r_c = res.1;
            for i in 0..r_b.len() {
                w_rb = w_rb.partial_evaluate(r_b[i], 0);
                w_rc = w_rc.partial_evaluate(r_c[i], 0);
            }

            challenges_vec.push(random_challenges.clone());

            gkr_proof.sumcheck_proof.push((claimed_sum, round_polys));
            gkr_proof
                .w_s
                .push((w_rb.representation[0], w_rc.representation[0]));
        }
        // println!("gkr_proof {:?}", gkr_proof);
        gkr_proof
    }

    fn generate_fbc(
        &self,
        layer: usize,
        add_i_poly: EvaluationFormPolynomial<F>,
        mul_i_poly: EvaluationFormPolynomial<F>,
    ) -> SumPolynomial<F> {
        let w = self.get_ws(layer + 1);

        let w_i = EvaluationFormPolynomial::new(&w);

        let w_bc = ProductPolynomial::new(vec![w_i.clone(), w_i.clone()]);

        let w_add_bc: EvaluationFormPolynomial<F> = w_bc.sum_poly();

        let w_mul_bc: EvaluationFormPolynomial<F> = w_bc.mul_poly();

        let fbc = SumPolynomial::new(vec![
            ProductPolynomial::new(vec![w_add_bc, add_i_poly]),
            ProductPolynomial::new(vec![w_mul_bc, mul_i_poly]),
        ]);

        fbc
    }

    fn generate_gate_indices_for_layer_i(self, layer: usize) -> Vec<String> {
        let layers: Vec<Layer<F>> = self.layers.iter().rev().cloned().collect();

        let outputs: Vec<F> = layers[layer].gates.iter().map(|gate| gate.output).collect();
        let output_binary: Vec<String> = (0..outputs.len())
            .map(|i| format!("{:0width$b}", i, width = layer))
            .collect();

        let left_right: Vec<F> = layers[layer]
            .gates
            .iter()
            .flat_map(|gate| vec![gate.left, gate.right])
            .collect();
        let left_right_binary: Vec<String> = (0..left_right.len())
            .map(|i| format!("{:0width$b}", i, width = layer + 1))
            .collect();

        let gate_indices: Vec<String> = output_binary
            .iter()
            .enumerate()
            .map(|(i, output)| {
                vec![
                    output.clone(),
                    left_right_binary[i * 2].clone(),
                    left_right_binary[i * 2 + 1].clone(),
                ]
                .join("")
            })
            .collect();

        gate_indices
    }
}

impl<F: PrimeField> Layer<F> {
    fn new() -> Self {
        Self { gates: Vec::new() }
    }
    fn add_gate(&mut self, gate: Gate<F>) {
        self.gates.push(gate);
    }
    fn evaluate_layer(&mut self, ops: Vec<Op>) -> Vec<F> {
        let output: Vec<F> = self.gates.iter().map(|gate| gate.output).collect();

        self.gates.drain(..);

        for i in 0..ops.len() {
            if i * 2 + 1 < output.len() {
                let new_gate = Gate::new(output[i * 2], output[i * 2 + 1], ops[i]);
                self.gates.push(new_gate);
            }
        }

        output
    }
}
impl<F: PrimeField> Gate<F> {
    fn new(left: F, right: F, op: Op) -> Self {
        let output = match op {
            Op::Add => left + right,
            Op::Mul => left * right,
        };
        Self {
            left,
            right,
            output,
            op,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ark_bn254::Fq;

    #[test]
    fn test_add_i_or_mul_i() {
        let left = Fq::from(1u64);
        let right = Fq::from(2u64);
        let gate1 = Gate::new(left, right, Op::Add);

        let left = Fq::from(3u64);
        let right = Fq::from(4u64);
        let gate2 = Gate::new(left, right, Op::Mul);

        let left = Fq::from(5u64);
        let right = Fq::from(6u64);
        let gate3 = Gate::new(left, right, Op::Add);

        let left = Fq::from(7u64);
        let right = Fq::from(8u64);
        let gate4 = Gate::new(left, right, Op::Add);

        let mut layer = Layer::new();
        layer.add_gate(gate1);
        layer.add_gate(gate2);
        layer.add_gate(gate3);
        layer.add_gate(gate4);

        let mut circuit = Circuit::new();
        circuit.add_layer((layer.clone()));

        let layer1_output = layer.evaluate_layer(vec![Op::Add, Op::Add]);
        circuit.add_layer((layer.clone()));
        let layer2_output = layer.evaluate_layer(vec![Op::Add]);
        circuit.add_layer((layer.clone()));

        println!(" circuit{:?}", circuit);
        let v = circuit.layers[0].clone();

        circuit.add_i_or_mul_i(0);
        // circuit.generate_fbc(1, vec![Fq::from(2), Fq::from(5)]);
        let gkr_proof = circuit.proof();
        circuit.verifier(gkr_proof);
    }
    #[test]
    fn test_gate() {
        let left = Fq::from(2u64);
        let right = Fq::from(3u64);
        let gate = Gate::new(left, right, Op::Add);
        assert_eq!(gate.output, Fq::from(5u64));
    }

    #[test]
    fn test_compute_circuits() {
        let left = Fq::from(1u64);
        let right = Fq::from(2u64);
        let gate1 = Gate::new(left, right, Op::Add);

        let left = Fq::from(3u64);
        let right = Fq::from(4u64);
        let gate2 = Gate::new(left, right, Op::Mul);

        let left = Fq::from(5u64);
        let right = Fq::from(6u64);
        let gate3 = Gate::new(left, right, Op::Add);

        let left = Fq::from(7u64);
        let right = Fq::from(8u64);
        let gate4 = Gate::new(left, right, Op::Add);

        let mut layer = Layer::new();
        layer.add_gate(gate1);
        layer.add_gate(gate2);
        layer.add_gate(gate3);
        layer.add_gate(gate4);

        let mut circuit = Circuit::new();
        circuit.add_layer((layer.clone()));

        let layer1_output = layer.evaluate_layer(vec![Op::Add, Op::Add]);
        circuit.add_layer((layer.clone()));
        let layer2_output = layer.evaluate_layer(vec![Op::Add]);
        circuit.add_layer((layer.clone()));

        // println!(" circuit{:?}", circuit);
        let v = circuit.layers[0].clone();
        // println!("v {:?}", v);
        circuit.generate_gate_indices_for_layer_i(1);
    }
    #[test]
    fn test_addi() {
        let indices = vec![Fq::from(0), Fq::from(0), Fq::from(1)];
        let values = vec![Fq::from(4), Fq::from(2), Fq::from(3)];
        let res = Circuit::add_i(indices, values);
        // println!("res {:?}", res)
    }
}
