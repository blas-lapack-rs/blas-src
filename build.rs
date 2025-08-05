const FEATURES: &[&str] = &["accelerate", "blis", "intel-mkl", "netlib", "openblas", "r"];

fn main() {
    if FEATURES
        .iter()
        .map(|name| name.to_uppercase().replace('-', "_"))
        .map(|name| format!("CARGO_FEATURE_{name}"))
        .filter(|name| std::env::var(name).is_ok())
        .count()
        > 1
    {
        panic!("At most one BLAS implementation may be enabled at a time.");
    }
}
