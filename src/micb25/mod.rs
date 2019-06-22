use std::f64::consts::PI;

use rgsl::error::erf;

mod data;

pub fn boys(n: u64, t: f64) -> f64 {
    let eps = 1.0e-12;
    if n == 0 && t < eps {
        1.0
    } else if n == 0 {
        (PI / (4.0 * t)).sqrt() * erf(t.sqrt())
    } else if t < eps {
        1.0 / ((2.0 * n as f64) + 1.0)
    } else if t > 30.0 {
        N_FAC2_DBLE[(2 * (n - 1) + 2) as usize] / 2.0_f64.powi(n as i32 + 1)
            * (PI / t.powi(2 * n as i32 + 1)).sqrt()
    } else if t > 10.0 {
        let j = ((t - 9.95) * 10.0) as usize;
        let dt = data::BoysFuncValuesL[j][0] - t as f64;
        let mut dti = dt;
        let mut lres = data::BoysFuncValuesL[j][n as usize + 1];
        let epsrel = lres * eps;
        for i in 0..6 {
            let sfac = data::BoysFuncValuesL[j][n as usize + 2 + i] * dti / N_FAC_DBLE[i];
            lres += sfac;
            if sfac.abs() < epsrel {
                return lres;
            }
            dti *= dt;
        }
        lres
    } else if t > 5.0 {
        let j = ((t - 4.975) * 20.0) as usize;
        let dt = data::BoysFuncValuesM[j][0] - t as f64;
        let mut dti = dt;
        let mut lres = data::BoysFuncValuesM[j][n as usize + 1];
        let epsrel = lres * eps;
        for i in 0..6 {
            let sfac = data::BoysFuncValuesM[j][n as usize + 2 + i] * dti / N_FAC_DBLE[i];
            lres += sfac;
            if sfac.abs() < epsrel {
                return lres;
            }
            dti *= dt;
        }
        lres
    } else {
        let j = ((t * 40.0) + 0.5) as usize;
        let dt = data::BoysFuncValuesS[j][0] - t as f64;
        let mut dti = dt;
        let mut lres = data::BoysFuncValuesS[j][n as usize + 1];
        let epsrel = lres * eps;
        for i in 0..6 {
            let sfac = data::BoysFuncValuesS[j][n as usize + 2 + i] * dti / N_FAC_DBLE[i];
            lres += sfac;
            if sfac.abs() < epsrel {
                return lres;
            }
            dti *= dt;
        }
        lres
    }
}

#[cfg(test)]
mod tests {
    use std::fs;
    use super::boys;
    #[test]
    fn boys_test() {
        let data = fs::read_to_string("./benchmark_values.txt").expect("unable to read file");

        println!("-----------------------------------------------------------------------------------------------");
        println!("{:>5} {:>22} {:>22} {:>22} {:>22}", "N", "X", "calc.value", "ref. value", "diff");
        println!("-----------------------------------------------------------------------------------------------");
        for line in data.lines() {
            let tokens: Vec<&str> = line.split_whitespace().collect();
            // Allow comments
            if tokens[0] != "#" {
                let n: u64 = tokens[0].parse().unwrap();
                let t: f64 = tokens[1].parse().unwrap();
                let m: f64 = tokens[2].parse().unwrap();
                let res = boys(n, t);
                println!("{:5} {:22.16} {:22.16} {:22.16} {:22.16}", n, t, res, m, res - m);
                assert!((res - m).abs() < 1.0e-12);
            }
        }
        println!("-----------------------------------------------------------------------------------------------");
    }
}

const N_FAC_DBLE: [f64; 6] = [1.0, 2.0, 6.0, 24.0, 120.0, 720.0];

const N_FAC2_DBLE: [f64; 31] = [
    1.0,
    1.0,
    1.0,
    2.0,
    3.0,
    8.0,
    15.0,
    48.0,
    105.0,
    384.0,
    945.0,
    3840.0,
    10395.0,
    46080.0,
    135135.0,
    645120.0,
    2027025.0,
    10321920.0,
    34459425.0,
    185794560.0,
    654729075.0,
    3715891200.0,
    13749310575.0,
    81749606400.0,
    316234143225.0,
    1961990553600.0,
    7905853580625.0,
    51011754393600.0,
    213458046676875.0,
    1428329123020800.0,
    6190283353629375.0,
];
