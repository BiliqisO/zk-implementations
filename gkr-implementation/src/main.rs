use ark_ff::PrimeField;
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

    fn layer(self, mut gates: Vec<Gate<F>>) -> Vec<Gate<F>> {
        gates.push(self);
        gates
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ark_bn254::Fq;

    #[test]
    fn test_gate() {
        let left = Fq::from(2u64);
        let right = Fq::from(3u64);
        let gate = Gate::new(left, right, Op::Add);
        assert_eq!(gate.output, Fq::from(5u64));
    }

    #[test]
    fn test_evaluate_layers() {
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
    }
}
