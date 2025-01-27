use  ark_bn254::Fq;
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use multilinear_polynomial::{boolean_hypercube, multilinear_monomial, multilinear_polynomial_sparse, sparse_partial_evalauation, EvaluationFormPolynomial};


pub fn bench_evaluation_form_partial(c: &mut Criterion) {
    let values: Vec<Fq> = vec![Fq::from(0), Fq::from(2), Fq::from(0), Fq::from(5)];
    let  poly = EvaluationFormPolynomial::new(&values);

    c.bench_function("Partial Evaluation", |b| {
        b.iter(|| {
            let mut result = black_box(poly.clone());
            result = result.partial_evaluate(black_box(Fq::from(5)), black_box(0));
            result.partial_evaluate(black_box(Fq::from(2)), black_box(0));
        });
    });
}
pub fn bench_sparse_partial_evaluation(c: &mut Criterion) {
    let m_1 = multilinear_monomial(Fq::from(3), vec![Fq::from(0), Fq::from(1), Fq::from(1)]);
    let m_2 = multilinear_monomial(Fq::from(4), vec![Fq::from(1), Fq::from(1), Fq::from(0)]);
    let m_3 = multilinear_monomial(Fq::from(5), vec![Fq::from(1), Fq::from(1), Fq::from(1)]);
    let poly = vec![m_1, m_2, m_3];
    let p = multilinear_polynomial_sparse(poly);

    c.bench_function("Sparse Partial Evaluation", |b| {
        b.iter(|| {
            let result = sparse_partial_evalauation(black_box(p.clone()), black_box(Fq::from(5)), black_box(1));
            sparse_partial_evalauation(result, black_box(Fq::from(5)), black_box(0));
        });
    });
}

pub fn bench_boolean_hypercube(c: &mut Criterion) {
    c.bench_function("Boolean Hypercube Generation", |b| {
        b.iter(|| {
            black_box(boolean_hypercube::<Fq>(black_box(10)));
        });
    });
}

criterion_group!(
    benches,
    bench_evaluation_form_partial,
    bench_sparse_partial_evaluation,
    bench_boolean_hypercube
);
criterion_main!(benches);
