pub fn sig(x: f64) -> f64 {
    return 1.0 / (1.0 + (-x).exp());
}

pub fn sig_prime(x: f64) -> f64 {
    return sig(x) * (1f64 - sig(x));
}
