use ark_ff::PrimeField;
pub fn boolean_hypercube<F: PrimeField>(no_of_variables: usize) -> Vec<String> {
    let length_of_hypercube = 2_usize.pow(no_of_variables as u32);

    let hypercube_indices: Vec<usize> = (0..length_of_hypercube).collect();

    let result: Vec<String> = (0..hypercube_indices.len())
        .map(|i| format!("{:0width$b}", i, width = no_of_variables))
        .collect();

    result
}
#[cfg(test)]
mod tests {
    use super::*;
    use ark_bn254::Fq;
    #[test]
    fn test_boolean_hypercube() {
        let result_2: Vec<String> = boolean_hypercube::<Fq>(2);
        let result: Vec<String> = boolean_hypercube::<Fq>(3);
        assert_eq!(
            result,
            vec!["000", "001", "010", "011", "100", "101", "110", "111"]
        );
        assert_eq!(result_2, vec!["00", "01", "10", "11"]);
    }
}
