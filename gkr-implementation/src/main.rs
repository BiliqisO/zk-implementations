use std::ops::Index;

use ark_ff::PrimeField;
use evaluation_form_poly::{product_poly::{ProductPolynomial, SumPolynomial}, EvaluationFormPolynomial};
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
    //indices for this should be gotten from
    fn init_add_i(indices: Vec<F>) {}
    // fn mul_i(indices: Vec<F>, values: Vec<F>) -> F {

      
    // }
    fn w_i(values: Vec<F>) -> EvaluationFormPolynomial<F> {
        let poly = EvaluationFormPolynomial::new(&values);
        poly
    }
  

    fn add_i_or_mul_i(&self, layer: usize) -> (Vec<F>, Vec<F>) {
    // Extract layer information
    let layers: Vec<Layer<F>> = self.layers.iter().rev().cloned().collect();
    let indices = self.clone().generate_gate_indices_for_layer_i(layer);

    // Compute the number of bits in an index (assuming all are the same length)
    let num_bits = indices.first().map(|idx| idx.len()).unwrap_or(0);
    let num_combinations = 2usize.pow(num_bits as u32); // 2^num_bits

    // Initialize add_i and mul_i vectors with all zeros
    let mut add_i_vec = vec![0; num_combinations];
    let mut mul_i_vec = vec![0; num_combinations];

    for (i, gate) in layers[layer].gates.iter().enumerate() {
        // Convert gate index to a binary string and then to decimal
        let binary_string = &indices[i]; 
        let decimal_value = usize::from_str_radix(&binary_string, 2).unwrap_or(0);

        match gate.op {
            Op::Add => add_i_vec[decimal_value] = 1, // Set the corresponding index
            Op::Mul => mul_i_vec[decimal_value] = 1, // Same for mul_i
            _ => continue, // Ignore other operations
        }
    }
    

    println!("add_i_vec: {:?}", add_i_vec);
    println!("mul_i_vec: {:?}", mul_i_vec);
    (add_i_vec.iter().map(|&x| F::from(x)).collect(), mul_i_vec.iter().map(|&x| F::from(x)).collect())
}
fn generate_fbc(self, layer:usize, r_s:Vec<F>){
    let layers: Vec<Layer<F>> = self.layers.iter().rev().cloned().collect();
    let mut add_i_poly = EvaluationFormPolynomial::new(&add_i);
    let mut mul_i_poly = EvaluationFormPolynomial::new(&mul_i);


    let w: Vec<F> = layers[layer]
            .gates
            .iter()
            .flat_map(|gate| vec![gate.left, gate.right])
            .collect();
    let w_i = EvaluationFormPolynomial::new(&w);
    let (add_i, mul_i) = self.add_i_or_mul_i(layer);

    let w =  ProductPolynomial::new(vec![w_i, w_i]);
    let w_add_bc = w.sum_poly();
    let w_mul_bc = w.mul_poly();

    let mut add_bc = EvaluationFormPolynomial::default();
    let mut mul_bc = EvaluationFormPolynomial::default();
    for i in 0..r_s.len(){
        add_bc = add_i_poly.partial_evaluate(r_s[i], 0);
        mul_bc = mul_i_poly.partial_evaluate(r_s[i], 0);
        
    }
    let fbc =SumPolynomial::new(vec![ProductPolynomial::new(vec![w_add_bc, add_bc]), ProductPolynomial::new( vec![w_mul_bc, mul_bc])])
    //continue from here 


}


    fn generate_gate_indices_for_layer_i(self, layer: usize) -> Vec<String> {
        let layers: Vec<Layer<F>> = self.layers.iter().rev().cloned().collect();

        let outputs: Vec<F> = layers[layer].gates.iter().map(|gate| gate.output).collect();
        let output_binary: Vec<String> = (0..outputs.len())
            .map(|i| format!("{:0width$b}", i, width = layer))
            .collect();
        println!("output_binary{:?}", output_binary);

        let left_right: Vec<F> = layers[layer]
            .gates
            .iter()
            .flat_map(|gate| vec![gate.left, gate.right])
            .collect();
        let left_right_binary: Vec<String> = (0..left_right.len())
            .map(|i| format!("{:0width$b}", i, width = layer + 1))
            .collect();
        println!("left_right_binary{:?}", left_right_binary);
        let gate_indices: Vec<String> = output_binary
            .iter()
            .enumerate()
            .map(|(i, output)| {
                vec![
                    output.clone(),
                    left_right_binary[i * 2].clone(),
                    left_right_binary[i * 2 + 1].clone(),
                ].join("")
            })
            .collect();
        println!("gate_indices{:?}", gate_indices);
      
        gate_indices
    }
    //values is w for that layer
    fn generate_fb(self, i: usize, op: Op, b: F, c: F, values: Vec<F>) {
        // let layer_op = self.layers
        // let layer_indices =
        // This should go out of this fuction

        for gate in &self.layers[i].gates {
            match gate.op {
                Op::Add => {

                    // let add_sum: F = Circuit::add_i(indices, values) ;
                    // let fbc =

                    //this is valid but not in the right place
                    // let w0  = Circuit::w_i(values.clone()).partial_evaluate(b, 0).representation;
                    // let w1 = Circuit::w_i(values).partial_evaluate(c, 0).representation;
                    // let w:Vec<F>  =     w0.iter().zip(w1.iter()).map(|(a, b)| *a + *b).collect();
                }

                Op::Mul => todo!(),
            }
        }
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
        let mut output = vec![];

        output = self.gates.iter().map(|out| out.output).collect();
        println!("output{:?}", output);
        self.gates.drain(..);
        for i in 0..ops.len() {
            let new_gate = Gate::new(output[i], output[i + 1], ops[i]);

            self.gates.push(new_gate);
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
    fn test_add_i_or_mul_i(){
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

        println!(" circuit{:?}", circuit);
        let v = circuit.layers[0].clone();
        println!("v {:?}", v);
        circuit.generate_gate_indices_for_layer_i(1);
    }
    #[test]
    fn test_addi() {
        let indices = vec![Fq::from(0), Fq::from(0), Fq::from(1)];
        let values = vec![Fq::from(4), Fq::from(2), Fq::from(3)];
        let res = Circuit::add_i(indices, values);
        println!("res {:?}", res)
    }
}
