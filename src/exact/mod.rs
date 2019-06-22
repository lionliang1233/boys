fn boys(n: u64, x: f64) -> f64 {
    let n = n as f64;
    if x > 0.0 {
        let f = 2.0 * x.powf(n + 0.5);
        let g = rgsl::gamma_beta::gamma::gamma(n + 0.5);
        // need the "upper" incomplete gamma function, integrate from x to infty
        // regularized -> divide by gamma function
        let gi = rgsl::gamma_beta::incomplete_gamma::gamma_inc_P_e(n + 0.5, x)
            .1
            .val;
        return g * gi / f;
    } else {
        return 1.0 / (n * 2.0 + 1.0);
    }
}

#[cfg(test)]
mod tests {
    use super::boys;

    #[test]
    fn test_boys() {
        let thresh = 1.0e-16;

        assert!(boys(2, 2.0) - 0.0529428148329765 < thresh);
    }
}
