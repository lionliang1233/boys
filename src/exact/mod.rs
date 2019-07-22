pub fn boys(n: u64, x: f64) -> f64 {
    let n = n as f64;
    if x > 0.0 {
        let f = 2.0 * x.powf(n + 0.5);
        let g = rgsl::gamma_beta::gamma::gamma(n + 0.5);
        // need the "upper" incomplete gamma function, integrate from x to infty
        // regularized -> divide by gamma function
        let gi = rgsl::gamma_beta::incomplete_gamma::gamma_inc_P_e(n + 0.5, x)
            .1
            .val;
        g * gi / f
    } else {
        1.0 / (n * 2.0 + 1.0)
    }
}

#[cfg(test)]
mod tests {
    use super::boys;

    #[test]
    fn test_boys() {
        let thresh = 1.0e-16;
        let ref_2_2p0 = 0.0529428148329765;
        let ref_14_42p6 = 2.6578172951711814e-14;
        assert!(boys(2, 2.0) - ref_2_2p0 < thresh);
        assert!(boys(14, 42.67768466983068) - ref_14_42p6 < thresh);
    }
}
