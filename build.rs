fn main() {
    let features = ["accelerate", "blis", "intel-mkl", "netlib", "openblas", "r"];
    let mut count = 0;
    for feature in &features {
        if std::env::var(format!(
            "CARGO_FEATURE_{}",
            feature.to_uppercase().replace('-', "_")
        ))
        .is_ok()
        {
            count += 1;
        }
    }

    if count > 1 {
        panic!("Only one BLAS implementation feature may be enabled at a time.");
    }
}
