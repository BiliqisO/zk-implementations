use ark_ff::PrimeField;
fn main() {
    println!("Hello, world!");
}
#[derive(Debug, Clone, Copy)]
enum Op {
    Add,
    Mul,
}
#[derive(Debug)]
struct Gate<F: PrimeField> {
    left: F,
    right: F,
    output: F,
    op: Op,
}
#[derive(Debug)]
struct Layer<F: PrimeField> {
    gates: Vec<Gate<F>>,
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

        self.gates.drain(..);

        for i in 0..output.len() - 1 {
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
   
        let mut layer = Layer::new();
        layer.add_gate(gate1);
        layer.add_gate(gate2);
        layer.add_gate(gate3);
        
        println!("layerr {:?}", layer);
        let layer1_output = layer.evaluate_layer(vec![Op::Add, Op::Add]);
        println!("layer1_output {:?}", layer1_output);
        let layer2_output = layer.evaluate_layer(vec![Op::Add]);
        println!("layer2_output {:?}", layer2_output);
        let layer3_output = layer.evaluate_layer(vec![Op::Add]);
        println!("layer3_output {:?}", layer3_output);
       
    }
}
