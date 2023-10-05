

pub fn perturbe_mut(bits: &mut [u8], prob: f64) {
    for bit in bits.iter_mut() {
        *bit = if rand::random::<f64>() > prob { *bit } else { *bit ^ 1 };
    }
}

pub fn perturbe(bits: &[u8], prob: f64) -> Vec<u8> {
    let mut res = Vec::from(bits);
    perturbe_mut(&mut res[..], prob);
    res
}

#[derive(Debug, Copy, Clone)]
pub struct Bounds {
    pub upper: f64,
    pub lower: f64
}

pub fn bin_to_real(bits: &[u8], bounds: &[Bounds]) -> Vec<f64> {
    let mut res = vec![0.0; bounds.len()];
    let chunk_size = bits.len() / bounds.len();
    for i in 0..bounds.len() {
        let mut acc: i32 = 0;
        let mut pow: i32 = 1;
        let from = i * chunk_size;
        let to = std::cmp::min((i + 1) * chunk_size, bits.len());
        for b in (from..to).rev() {
            acc += bits[b] as i32 * pow;
            pow <<= 1;
        }
        let bound_size = bounds[i].upper - bounds[i].lower;
        res[i] = bounds[i].lower + bound_size * (acc as f64) / ((pow - 1) as f64);
    }
    res
}

pub fn bin_to_real_uniform(bits: &[u8], bounds: &[Bounds]) -> Option<Vec<f64>> {
    if bits.len() % bounds.len() == 0 {
        Some(bin_to_real(bits, bounds))
    } else {
        None
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    
    const EPSILON: f64 = 1.0e-6;

    const BITS01: [u8; 1]  = [0];
    const BITS02: [u8; 1]  = [1];
    const BITS03: [u8; 2]  = [0, 0];
    const BITS04: [u8; 2]  = [0, 1];
    const BITS05: [u8; 2]  = [1, 0];
    const BITS06: [u8; 2]  = [1, 1];
    const BITS07: [u8; 3]  = [0, 0, 0];
    const BITS08: [u8; 3]  = [0, 0, 1];
    const BITS09: [u8; 3]  = [0, 1, 0];
    const BITS10: [u8; 3]  = [0, 1, 1];
    const BITS11: [u8; 3]  = [1, 0, 0];
    const BITS12: [u8; 3]  = [1, 0, 1];
    const BITS13: [u8; 3]  = [1, 1, 0];
    const BITS14: [u8; 3]  = [1, 1, 1];
    const BITS15: [u8; 12] = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    const BITS16: [u8; 12] = [1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1];
    const BITS17: [u8; 12] = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1];
    const BITS18: [u8; 12] = [0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1];
    const BITS19: [u8; 12] = [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    const BITS20: [u8; 12] = [1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0];
    const BITS21: [u8; 12] = [0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1];
    const BITS22: [u8; 12] = [0, 1, 1, 1, 1, 1, 0, 1, 1, 1, 1, 1];
    const BITS23: [u8; 12] = [1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0];
    const BITS24: [u8; 12] = [1, 1, 1, 1, 1, 0, 1, 1, 1, 1, 1, 0];
    const BITS25: [u8; 12] = [0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 1];
    const BITS26: [u8; 12] = [0, 1, 1, 1, 0, 1, 1, 1, 0, 1, 1, 1];
    const BITS27: [u8; 12] = [1, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0];
    const BITS28: [u8; 12] = [1, 1, 1, 0, 1, 1, 1, 0, 1, 1, 1, 0];
    const BITS29: [u8; 12] = [0, 0, 1, 0, 0, 1, 0, 0, 1, 0, 0, 1];
    const BITS30: [u8; 12] = [0, 1, 1, 0, 1, 1, 0, 1, 1, 0, 1, 1];
    const BITS31: [u8; 12] = [1, 0, 0, 1, 0, 0, 1, 0, 0, 1, 0, 0];
    const BITS32: [u8; 12] = [1, 1, 0, 1, 1, 0, 1, 1, 0, 1, 1, 0];
    const BITS33: [u8; 12] = [0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1];
    const BITS34: [u8; 12] = [1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0];

    fn check_bin_to_real(bits: &[u8], bounds: &[Bounds], expected: &[f64]) {
        let res = bin_to_real(bits, bounds);
        assert_eq!(res.len(), expected.len());
        for i in 0..res.len() {
            assert!((res[i] - expected[i]).abs() < EPSILON, "say something");
        }
    }

    #[test]
    fn test_bin_to_real_1d_1() {
        let bounds = [Bounds { lower: 0.0, upper: 1.0 }];
        check_bin_to_real(&BITS01, &bounds, &[0.0]);
        check_bin_to_real(&BITS02, &bounds, &[1.0]);
        check_bin_to_real(&BITS03, &bounds, &[0.0]);
        check_bin_to_real(&BITS04, &bounds, &[0.3333333333333333]);
        check_bin_to_real(&BITS05, &bounds, &[0.6666666666666666]);
        check_bin_to_real(&BITS06, &bounds, &[1.0]);
        check_bin_to_real(&BITS07, &bounds, &[0.0]);
        check_bin_to_real(&BITS08, &bounds, &[0.14285714285714285]);
        check_bin_to_real(&BITS09, &bounds, &[0.2857142857142857]);
        check_bin_to_real(&BITS10, &bounds, &[0.42857142857142855]);
        check_bin_to_real(&BITS11, &bounds, &[0.5714285714285714]);
        check_bin_to_real(&BITS12, &bounds, &[0.7142857142857143]);
        check_bin_to_real(&BITS13, &bounds, &[0.8571428571428571]);
        check_bin_to_real(&BITS14, &bounds, &[1.0]);
        check_bin_to_real(&BITS15, &bounds, &[0.0]);
        check_bin_to_real(&BITS16, &bounds, &[1.0]);
        check_bin_to_real(&BITS17, &bounds, &[0.0002442002442002442]);
        check_bin_to_real(&BITS18, &bounds, &[0.4998778998778999]);
        check_bin_to_real(&BITS19, &bounds, &[0.5001221001221001]);
        check_bin_to_real(&BITS20, &bounds, &[0.9997557997557998]);
        check_bin_to_real(&BITS21, &bounds, &[0.015873015873015872]);
        check_bin_to_real(&BITS22, &bounds, &[0.49206349206349204]);
        check_bin_to_real(&BITS23, &bounds, &[0.5079365079365079]);
        check_bin_to_real(&BITS24, &bounds, &[0.9841269841269841]);
        check_bin_to_real(&BITS25, &bounds, &[0.06666666666666667]);
        check_bin_to_real(&BITS26, &bounds, &[0.4666666666666667]);
        check_bin_to_real(&BITS27, &bounds, &[0.5333333333333333]);
        check_bin_to_real(&BITS28, &bounds, &[0.9333333333333333]);
        check_bin_to_real(&BITS29, &bounds, &[0.14285714285714285]);
        check_bin_to_real(&BITS30, &bounds, &[0.42857142857142855]);
        check_bin_to_real(&BITS31, &bounds, &[0.5714285714285714]);
        check_bin_to_real(&BITS32, &bounds, &[0.8571428571428571]);
        check_bin_to_real(&BITS33, &bounds, &[0.3333333333333333]);
        check_bin_to_real(&BITS34, &bounds, &[0.6666666666666666]);
    }

    #[test]
    fn test_bin_to_real_1d_2() {
        let bounds = [Bounds { lower: 0.0, upper: 4095.0 }];
        check_bin_to_real(&BITS01, &bounds, &[0.0]);
        check_bin_to_real(&BITS02, &bounds, &[4095.0]);
        check_bin_to_real(&BITS03, &bounds, &[0.0]);
        check_bin_to_real(&BITS04, &bounds, &[1365.0]);
        check_bin_to_real(&BITS05, &bounds, &[2730.0]);
        check_bin_to_real(&BITS06, &bounds, &[4095.0]);
        check_bin_to_real(&BITS07, &bounds, &[0.0]);
        check_bin_to_real(&BITS08, &bounds, &[585.0]);
        check_bin_to_real(&BITS09, &bounds, &[1170.0]);
        check_bin_to_real(&BITS10, &bounds, &[1755.0]);
        check_bin_to_real(&BITS11, &bounds, &[2340.0]);
        check_bin_to_real(&BITS12, &bounds, &[2925.0]);
        check_bin_to_real(&BITS13, &bounds, &[3510.0]);
        check_bin_to_real(&BITS14, &bounds, &[4095.0]);
        check_bin_to_real(&BITS15, &bounds, &[0.0]);
        check_bin_to_real(&BITS16, &bounds, &[4095.0]);
        check_bin_to_real(&BITS17, &bounds, &[1.0]);
        check_bin_to_real(&BITS18, &bounds, &[2047.0]);
        check_bin_to_real(&BITS19, &bounds, &[2048.0]);
        check_bin_to_real(&BITS20, &bounds, &[4094.0]);
        check_bin_to_real(&BITS21, &bounds, &[65.0]);
        check_bin_to_real(&BITS22, &bounds, &[2015.0]);
        check_bin_to_real(&BITS23, &bounds, &[2080.0]);
        check_bin_to_real(&BITS24, &bounds, &[4030.0]);
        check_bin_to_real(&BITS25, &bounds, &[273.0]);
        check_bin_to_real(&BITS26, &bounds, &[1911.0]);
        check_bin_to_real(&BITS27, &bounds, &[2184.0]);
        check_bin_to_real(&BITS28, &bounds, &[3822.0]);
        check_bin_to_real(&BITS29, &bounds, &[585.0]);
        check_bin_to_real(&BITS30, &bounds, &[1755.0]);
        check_bin_to_real(&BITS31, &bounds, &[2340.0]);
        check_bin_to_real(&BITS32, &bounds, &[3510.0]);
        check_bin_to_real(&BITS33, &bounds, &[1365.0]);
        check_bin_to_real(&BITS34, &bounds, &[2730.0]);
    }

    #[test]
    fn test_bin_to_real_1d_3() {
        let bounds = [Bounds { lower: -5.0, upper: 5.0 }];
        check_bin_to_real(&BITS01, &bounds, &[-5.0]);
        check_bin_to_real(&BITS02, &bounds, &[5.0]);
        check_bin_to_real(&BITS03, &bounds, &[-5.0]);
        check_bin_to_real(&BITS04, &bounds, &[-1.6666666666666665]);
        check_bin_to_real(&BITS05, &bounds, &[1.666666666666667]);
        check_bin_to_real(&BITS06, &bounds, &[5.0]);
        check_bin_to_real(&BITS07, &bounds, &[-5.0]);
        check_bin_to_real(&BITS08, &bounds, &[-3.571428571428571]);
        check_bin_to_real(&BITS09, &bounds, &[-2.142857142857143]);
        check_bin_to_real(&BITS10, &bounds, &[-0.7142857142857144]);
        check_bin_to_real(&BITS11, &bounds, &[0.7142857142857144]);
        check_bin_to_real(&BITS12, &bounds, &[2.1428571428571432]);
        check_bin_to_real(&BITS13, &bounds, &[3.571428571428571]);
        check_bin_to_real(&BITS14, &bounds, &[5.0]);
        check_bin_to_real(&BITS15, &bounds, &[-5.0]);
        check_bin_to_real(&BITS16, &bounds, &[5.0]);
        check_bin_to_real(&BITS17, &bounds, &[-4.997557997557998]);
        check_bin_to_real(&BITS18, &bounds, &[-0.0012210012210012167]);
        check_bin_to_real(&BITS19, &bounds, &[0.0012210012210012167]);
        check_bin_to_real(&BITS20, &bounds, &[4.997557997557998]);
        check_bin_to_real(&BITS21, &bounds, &[-4.841269841269841]);
        check_bin_to_real(&BITS22, &bounds, &[-0.07936507936507908]);
        check_bin_to_real(&BITS23, &bounds, &[0.07936507936507908]);
        check_bin_to_real(&BITS24, &bounds, &[4.841269841269842]);
        check_bin_to_real(&BITS25, &bounds, &[-4.333333333333333]);
        check_bin_to_real(&BITS26, &bounds, &[-0.33333333333333304]);
        check_bin_to_real(&BITS27, &bounds, &[0.33333333333333304]);
        check_bin_to_real(&BITS28, &bounds, &[4.333333333333334]);
        check_bin_to_real(&BITS29, &bounds, &[-3.571428571428571]);
        check_bin_to_real(&BITS30, &bounds, &[-0.7142857142857144]);
        check_bin_to_real(&BITS31, &bounds, &[0.7142857142857144]);
        check_bin_to_real(&BITS32, &bounds, &[3.571428571428571]);
        check_bin_to_real(&BITS33, &bounds, &[-1.6666666666666665]);
        check_bin_to_real(&BITS34, &bounds, &[1.666666666666667]);
    }

    #[test]
    fn test_bin_to_real_2d_1() {
        let bounds = [Bounds { lower: 0.0, upper: 1.0 }, Bounds { lower: 0.0, upper: 1.0 }];
        check_bin_to_real(&BITS03, &bounds, &[0.0, 0.0]);
        check_bin_to_real(&BITS04, &bounds, &[0.0, 1.0]);
        check_bin_to_real(&BITS05, &bounds, &[1.0, 0.0]);
        check_bin_to_real(&BITS06, &bounds, &[1.0, 1.0]);
        check_bin_to_real(&BITS15, &bounds, &[0.0, 0.0]);
        check_bin_to_real(&BITS16, &bounds, &[1.0, 1.0]);
        check_bin_to_real(&BITS17, &bounds, &[0.0, 0.015873015873015872]);
        check_bin_to_real(&BITS18, &bounds, &[0.49206349206349204, 1.0]);
        check_bin_to_real(&BITS19, &bounds, &[0.5079365079365079, 0.0]);
        check_bin_to_real(&BITS20, &bounds, &[1.0, 0.9841269841269841]);
        check_bin_to_real(&BITS21, &bounds, &[0.015873015873015872, 0.015873015873015872]);
        check_bin_to_real(&BITS22, &bounds, &[0.49206349206349204, 0.49206349206349204]);
        check_bin_to_real(&BITS23, &bounds, &[0.5079365079365079, 0.5079365079365079]);
        check_bin_to_real(&BITS24, &bounds, &[0.9841269841269841, 0.9841269841269841]);
        check_bin_to_real(&BITS25, &bounds, &[0.06349206349206349, 0.2698412698412698]);
        check_bin_to_real(&BITS26, &bounds, &[0.4603174603174603, 0.873015873015873]);
        check_bin_to_real(&BITS27, &bounds, &[0.5396825396825397, 0.12698412698412698]);
        check_bin_to_real(&BITS28, &bounds, &[0.9365079365079365, 0.7301587301587301]);
        check_bin_to_real(&BITS29, &bounds, &[0.14285714285714285, 0.14285714285714285]);
        check_bin_to_real(&BITS30, &bounds, &[0.42857142857142855, 0.42857142857142855]);
        check_bin_to_real(&BITS31, &bounds, &[0.5714285714285714, 0.5714285714285714]);
        check_bin_to_real(&BITS32, &bounds, &[0.8571428571428571, 0.8571428571428571]);
        check_bin_to_real(&BITS33, &bounds, &[0.3333333333333333, 0.3333333333333333]);
        check_bin_to_real(&BITS34, &bounds, &[0.6666666666666666, 0.6666666666666666]);
    }

    #[test]
    fn test_bin_to_real_2d_2() {
        let bounds = [Bounds { lower: 0.0, upper: 63.0 }, Bounds { lower: -32.0, upper: 31.0 }];
        check_bin_to_real(&BITS03, &bounds, &[0.0, -32.0]);
        check_bin_to_real(&BITS04, &bounds, &[0.0, 31.0]);
        check_bin_to_real(&BITS05, &bounds, &[63.0, -32.0]);
        check_bin_to_real(&BITS06, &bounds, &[63.0, 31.0]);
        check_bin_to_real(&BITS15, &bounds, &[0.0, -32.0]);
        check_bin_to_real(&BITS16, &bounds, &[63.0, 31.0]);
        check_bin_to_real(&BITS17, &bounds, &[0.0, -31.0]);
        check_bin_to_real(&BITS18, &bounds, &[31.0, 31.0]);
        check_bin_to_real(&BITS19, &bounds, &[32.0, -32.0]);
        check_bin_to_real(&BITS20, &bounds, &[63.0, 30.0]);
        check_bin_to_real(&BITS21, &bounds, &[1.0, -31.0]);
        check_bin_to_real(&BITS22, &bounds, &[31.0, -1.0]);
        check_bin_to_real(&BITS23, &bounds, &[32.0, 0.0]);
        check_bin_to_real(&BITS24, &bounds, &[62.0, 30.0]);
        check_bin_to_real(&BITS25, &bounds, &[4.0, -15.0]);
        check_bin_to_real(&BITS26, &bounds, &[29.0, 23.0]);
        check_bin_to_real(&BITS27, &bounds, &[34.0, -24.0]);
        check_bin_to_real(&BITS28, &bounds, &[59.0, 14.0]);
        check_bin_to_real(&BITS29, &bounds, &[9.0, -23.0]);
        check_bin_to_real(&BITS30, &bounds, &[27.0, -5.0]);
        check_bin_to_real(&BITS31, &bounds, &[36.0, 4.0]);
        check_bin_to_real(&BITS32, &bounds, &[54.0, 22.0]);
        check_bin_to_real(&BITS33, &bounds, &[21.0, -11.0]);
        check_bin_to_real(&BITS34, &bounds, &[42.0, 10.0]);
    }

    #[test]
    fn test_bin_to_real_2d_3() {
        let bounds = [Bounds { lower: -5.0, upper: 5.0 }, Bounds { lower: 0.0, upper: 10.0 }];
        check_bin_to_real(&BITS03, &bounds, &[-5.0, 0.0]);
        check_bin_to_real(&BITS04, &bounds, &[-5.0, 10.0]);
        check_bin_to_real(&BITS05, &bounds, &[5.0, 0.0]);
        check_bin_to_real(&BITS06, &bounds, &[5.0, 10.0]);
        check_bin_to_real(&BITS15, &bounds, &[-5.0, 0.0]);
        check_bin_to_real(&BITS16, &bounds, &[5.0, 10.0]);
        check_bin_to_real(&BITS17, &bounds, &[-5.0, 0.15873015873015872]);
        check_bin_to_real(&BITS18, &bounds, &[-0.07936507936507908, 10.0]);
        check_bin_to_real(&BITS19, &bounds, &[0.07936507936507908, 0.0]);
        check_bin_to_real(&BITS20, &bounds, &[5.0, 9.841269841269842]);
        check_bin_to_real(&BITS21, &bounds, &[-4.841269841269841, 0.15873015873015872]);
        check_bin_to_real(&BITS22, &bounds, &[-0.07936507936507908, 4.920634920634921]);
        check_bin_to_real(&BITS23, &bounds, &[0.07936507936507908, 5.079365079365079]);
        check_bin_to_real(&BITS24, &bounds, &[4.841269841269842, 9.841269841269842]);
        check_bin_to_real(&BITS25, &bounds, &[-4.365079365079366, 2.6984126984126986]);
        check_bin_to_real(&BITS26, &bounds, &[-0.3968253968253972, 8.73015873015873]);
        check_bin_to_real(&BITS27, &bounds, &[0.3968253968253972, 1.2698412698412698]);
        check_bin_to_real(&BITS28, &bounds, &[4.365079365079366, 7.301587301587301]);
        check_bin_to_real(&BITS29, &bounds, &[-3.571428571428571, 1.4285714285714286]);
        check_bin_to_real(&BITS30, &bounds, &[-0.7142857142857144, 4.285714285714286]);
        check_bin_to_real(&BITS31, &bounds, &[0.7142857142857144, 5.714285714285714]);
        check_bin_to_real(&BITS32, &bounds, &[3.571428571428571, 8.571428571428571]);
        check_bin_to_real(&BITS33, &bounds, &[-1.6666666666666665, 3.3333333333333335]);
        check_bin_to_real(&BITS34, &bounds, &[1.666666666666667, 6.666666666666667]);
    }

    #[test]
    fn test_bin_to_real_3d_1() {
        let bounds = [Bounds { lower: 0.0, upper: 1.0 }; 3];
        check_bin_to_real(&BITS07, &bounds, &[0.0, 0.0, 0.0]);
        check_bin_to_real(&BITS08, &bounds, &[0.0, 0.0, 1.0]);
        check_bin_to_real(&BITS09, &bounds, &[0.0, 1.0, 0.0]);
        check_bin_to_real(&BITS10, &bounds, &[0.0, 1.0, 1.0]);
        check_bin_to_real(&BITS11, &bounds, &[1.0, 0.0, 0.0]);
        check_bin_to_real(&BITS12, &bounds, &[1.0, 0.0, 1.0]);
        check_bin_to_real(&BITS13, &bounds, &[1.0, 1.0, 0.0]);
        check_bin_to_real(&BITS14, &bounds, &[1.0, 1.0, 1.0]);
        check_bin_to_real(&BITS15, &bounds, &[0.0, 0.0, 0.0]);
        check_bin_to_real(&BITS16, &bounds, &[1.0, 1.0, 1.0]);
        check_bin_to_real(&BITS17, &bounds, &[0.0, 0.0, 0.06666666666666667]);
        check_bin_to_real(&BITS18, &bounds, &[0.4666666666666667, 1.0, 1.0]);
        check_bin_to_real(&BITS19, &bounds, &[0.5333333333333333, 0.0, 0.0]);
        check_bin_to_real(&BITS20, &bounds, &[1.0, 1.0, 0.9333333333333333]);
        check_bin_to_real(&BITS21, &bounds, &[0.0, 0.26666666666666666, 0.06666666666666667]);
        check_bin_to_real(&BITS22, &bounds, &[0.4666666666666667, 0.8666666666666667, 1.0]);
        check_bin_to_real(&BITS23, &bounds, &[0.5333333333333333, 0.13333333333333333, 0.0]);
        check_bin_to_real(&BITS24, &bounds, &[1.0, 0.7333333333333333, 0.9333333333333333]);
        check_bin_to_real(&BITS25, &bounds, &[0.06666666666666667, 0.06666666666666667, 0.06666666666666667]);
        check_bin_to_real(&BITS26, &bounds, &[0.4666666666666667, 0.4666666666666667, 0.4666666666666667]);
        check_bin_to_real(&BITS27, &bounds, &[0.5333333333333333, 0.5333333333333333, 0.5333333333333333]);
        check_bin_to_real(&BITS28, &bounds, &[0.9333333333333333, 0.9333333333333333, 0.9333333333333333]);
        check_bin_to_real(&BITS29, &bounds, &[0.13333333333333333, 0.26666666666666666, 0.6]);
        check_bin_to_real(&BITS30, &bounds, &[0.4, 0.8666666666666667, 0.7333333333333333]);
        check_bin_to_real(&BITS31, &bounds, &[0.6, 0.13333333333333333, 0.26666666666666666]);
        check_bin_to_real(&BITS32, &bounds, &[0.8666666666666667, 0.7333333333333333, 0.4]);
        check_bin_to_real(&BITS33, &bounds, &[0.3333333333333333, 0.3333333333333333, 0.3333333333333333]);
        check_bin_to_real(&BITS34, &bounds, &[0.6666666666666666, 0.6666666666666666, 0.6666666666666666]);
    }

    #[test]
    fn test_bin_to_real_3d_2() {
        let bounds = [Bounds { lower: 0.0, upper: 15.0 }, Bounds { lower: -8.0, upper: 7.0 }, Bounds { lower: -8.0, upper: 8.0 }];
        check_bin_to_real(&BITS07, &bounds, &[0.0, -8.0, -8.0]);
        check_bin_to_real(&BITS08, &bounds, &[0.0, -8.0, 8.0]);
        check_bin_to_real(&BITS09, &bounds, &[0.0, 7.0, -8.0]);
        check_bin_to_real(&BITS10, &bounds, &[0.0, 7.0, 8.0]);
        check_bin_to_real(&BITS11, &bounds, &[15.0, -8.0, -8.0]);
        check_bin_to_real(&BITS12, &bounds, &[15.0, -8.0, 8.0]);
        check_bin_to_real(&BITS13, &bounds, &[15.0, 7.0, -8.0]);
        check_bin_to_real(&BITS14, &bounds, &[15.0, 7.0, 8.0]);
        check_bin_to_real(&BITS15, &bounds, &[0.0, -8.0, -8.0]);
        check_bin_to_real(&BITS16, &bounds, &[15.0, 7.0, 8.0]);
        check_bin_to_real(&BITS17, &bounds, &[0.0, -8.0, -6.933333333333334]);
        check_bin_to_real(&BITS18, &bounds, &[7.0, 7.0, 8.0]);
        check_bin_to_real(&BITS19, &bounds, &[8.0, -8.0, -8.0]);
        check_bin_to_real(&BITS20, &bounds, &[15.0, 7.0, 6.933333333333334]);
        check_bin_to_real(&BITS21, &bounds, &[0.0, -4.0, -6.933333333333334]);
        check_bin_to_real(&BITS22, &bounds, &[7.0, 5.0, 8.0]);
        check_bin_to_real(&BITS23, &bounds, &[8.0, -6.0, -8.0]);
        check_bin_to_real(&BITS24, &bounds, &[15.0, 3.0, 6.933333333333334]);
        check_bin_to_real(&BITS25, &bounds, &[1.0, -7.0, -6.933333333333334]);
        check_bin_to_real(&BITS26, &bounds, &[7.0, -1.0, -0.5333333333333332]);
        check_bin_to_real(&BITS27, &bounds, &[8.0, 0.0, 0.5333333333333332]);
        check_bin_to_real(&BITS28, &bounds, &[14.0, 6.0, 6.933333333333334]);
        check_bin_to_real(&BITS29, &bounds, &[2.0, -4.0, 1.5999999999999996]);
        check_bin_to_real(&BITS30, &bounds, &[6.0, 5.0, 3.7333333333333325]);
        check_bin_to_real(&BITS31, &bounds, &[9.0, -6.0, -3.7333333333333334]);
        check_bin_to_real(&BITS32, &bounds, &[13.0, 3.0, -1.5999999999999996]);
        check_bin_to_real(&BITS33, &bounds, &[5.0, -3.0, -2.666666666666667]);
        check_bin_to_real(&BITS34, &bounds, &[10.0, 2.0, 2.666666666666666]);
    }

    #[test]
    fn test_bin_to_real_4d_1() {
        let bounds = [Bounds { lower: 0.0, upper: 1.0 }; 4];
        check_bin_to_real(&BITS15, &bounds, &[0.0, 0.0, 0.0, 0.0]);
        check_bin_to_real(&BITS16, &bounds, &[1.0, 1.0, 1.0, 1.0]);
        check_bin_to_real(&BITS17, &bounds, &[0.0, 0.0, 0.0, 0.14285714285714285]);
        check_bin_to_real(&BITS18, &bounds, &[0.42857142857142855, 1.0, 1.0, 1.0]);
        check_bin_to_real(&BITS19, &bounds, &[0.5714285714285714, 0.0, 0.0, 0.0]);
        check_bin_to_real(&BITS20, &bounds, &[1.0, 1.0, 1.0, 0.8571428571428571]);
        check_bin_to_real(&BITS21, &bounds, &[0.0, 0.14285714285714285, 0.0, 0.14285714285714285]);
        check_bin_to_real(&BITS22, &bounds, &[0.42857142857142855, 1.0, 0.42857142857142855, 1.0]);
        check_bin_to_real(&BITS23, &bounds, &[0.5714285714285714, 0.0, 0.5714285714285714, 0.0]);
        check_bin_to_real(&BITS24, &bounds, &[1.0, 0.8571428571428571, 1.0, 0.8571428571428571]);
        check_bin_to_real(&BITS25, &bounds, &[0.0, 0.5714285714285714, 0.2857142857142857, 0.14285714285714285]);
        check_bin_to_real(&BITS26, &bounds, &[0.42857142857142855, 0.7142857142857143, 0.8571428571428571, 1.0]);
        check_bin_to_real(&BITS27, &bounds, &[0.5714285714285714, 0.2857142857142857, 0.14285714285714285, 0.0]);
        check_bin_to_real(&BITS28, &bounds, &[1.0, 0.42857142857142855, 0.7142857142857143, 0.8571428571428571]);
        check_bin_to_real(&BITS29, &bounds, &[0.14285714285714285, 0.14285714285714285, 0.14285714285714285, 0.14285714285714285]);
        check_bin_to_real(&BITS30, &bounds, &[0.42857142857142855, 0.42857142857142855, 0.42857142857142855, 0.42857142857142855]);
        check_bin_to_real(&BITS31, &bounds, &[0.5714285714285714, 0.5714285714285714, 0.5714285714285714, 0.5714285714285714]);
        check_bin_to_real(&BITS32, &bounds, &[0.8571428571428571, 0.8571428571428571, 0.8571428571428571, 0.8571428571428571]);
        check_bin_to_real(&BITS33, &bounds, &[0.2857142857142857, 0.7142857142857143, 0.2857142857142857, 0.7142857142857143]);
        check_bin_to_real(&BITS34, &bounds, &[0.7142857142857143, 0.2857142857142857, 0.7142857142857143, 0.2857142857142857]);
    }

    #[test]
    fn test_bin_to_real_4d_2() {
        let bounds = [
            Bounds { lower:  0.0, upper: 7.0 },
            Bounds { lower: -4.0, upper: 3.0 },
            Bounds { lower: -4.0, upper: 4.0 },
            Bounds { lower: -8.0, upper: 0.0 },
        ];
        check_bin_to_real(&BITS15, &bounds, &[0.0, -4.0, -4.0, -8.0]);
        check_bin_to_real(&BITS16, &bounds, &[7.0, 3.0, 4.0, 0.0]);
        check_bin_to_real(&BITS17, &bounds, &[0.0, -4.0, -4.0, -6.857142857142858]);
        check_bin_to_real(&BITS18, &bounds, &[3.0, 3.0, 4.0, 0.0]);
        check_bin_to_real(&BITS19, &bounds, &[4.0, -4.0, -4.0, -8.0]);
        check_bin_to_real(&BITS20, &bounds, &[7.0, 3.0, 4.0, -1.1428571428571432]);
        check_bin_to_real(&BITS21, &bounds, &[0.0, -3.0, -4.0, -6.857142857142858]);
        check_bin_to_real(&BITS22, &bounds, &[3.0, 3.0, -0.5714285714285716, 0.0]);
        check_bin_to_real(&BITS23, &bounds, &[4.0, -4.0, 0.5714285714285712, -8.0]);
        check_bin_to_real(&BITS24, &bounds, &[7.0, 2.0, 4.0, -1.1428571428571432]);
        check_bin_to_real(&BITS25, &bounds, &[0.0, 0.0, -1.7142857142857144, -6.857142857142858]);
        check_bin_to_real(&BITS26, &bounds, &[3.0, 1.0, 2.8571428571428568, 0.0]);
        check_bin_to_real(&BITS27, &bounds, &[4.0, -2.0, -2.857142857142857, -8.0]);
        check_bin_to_real(&BITS28, &bounds, &[7.0, -1.0, 1.7142857142857144, -1.1428571428571432]);
        check_bin_to_real(&BITS29, &bounds, &[1.0, -3.0, -2.857142857142857, -6.857142857142858]);
        check_bin_to_real(&BITS30, &bounds, &[3.0, -1.0, -0.5714285714285716, -4.571428571428571]);
        check_bin_to_real(&BITS31, &bounds, &[4.0, 0.0, 0.5714285714285712, -3.428571428571429]);
        check_bin_to_real(&BITS32, &bounds, &[6.0, 2.0, 2.8571428571428568, -1.1428571428571432]);
        check_bin_to_real(&BITS33, &bounds, &[2.0, 1.0, -1.7142857142857144, -2.2857142857142856]);
        check_bin_to_real(&BITS34, &bounds, &[5.0, -2.0, 1.7142857142857144, -5.714285714285714]);
    }

    #[test]
    fn test_bin_to_real_6d_1() {
        let bounds = [Bounds { lower: 0.0, upper: 1.0 }; 6];
        check_bin_to_real(&BITS15, &bounds, &[0.0, 0.0, 0.0, 0.0, 0.0, 0.0]);
        check_bin_to_real(&BITS16, &bounds, &[1.0, 1.0, 1.0, 1.0, 1.0, 1.0]);
        check_bin_to_real(&BITS17, &bounds, &[0.0, 0.0, 0.0, 0.0, 0.0, 0.3333333333333333]);
        check_bin_to_real(&BITS18, &bounds, &[0.3333333333333333, 1.0, 1.0, 1.0, 1.0, 1.0]);
        check_bin_to_real(&BITS19, &bounds, &[0.6666666666666666, 0.0, 0.0, 0.0, 0.0, 0.0]);
        check_bin_to_real(&BITS20, &bounds, &[1.0, 1.0, 1.0, 1.0, 1.0, 0.6666666666666666]);
        check_bin_to_real(&BITS21, &bounds, &[0.0, 0.0, 0.3333333333333333, 0.0, 0.0, 0.3333333333333333]);
        check_bin_to_real(&BITS22, &bounds, &[0.3333333333333333, 1.0, 1.0, 0.3333333333333333, 1.0, 1.0]);
        check_bin_to_real(&BITS23, &bounds, &[0.6666666666666666, 0.0, 0.0, 0.6666666666666666, 0.0, 0.0]);
        check_bin_to_real(&BITS24, &bounds, &[1.0, 1.0, 0.6666666666666666, 1.0, 1.0, 0.6666666666666666]);
        check_bin_to_real(&BITS25, &bounds, &[0.0, 0.3333333333333333, 0.0, 0.3333333333333333, 0.0, 0.3333333333333333]);
        check_bin_to_real(&BITS26, &bounds, &[0.3333333333333333, 1.0, 0.3333333333333333, 1.0, 0.3333333333333333, 1.0]);
        check_bin_to_real(&BITS27, &bounds, &[0.6666666666666666, 0.0, 0.6666666666666666, 0.0, 0.6666666666666666, 0.0]);
        check_bin_to_real(&BITS28, &bounds, &[1.0, 0.6666666666666666, 1.0, 0.6666666666666666, 1.0, 0.6666666666666666]);
        check_bin_to_real(&BITS29, &bounds, &[0.0, 0.6666666666666666, 0.3333333333333333, 0.0, 0.6666666666666666, 0.3333333333333333]);
        check_bin_to_real(&BITS30, &bounds, &[0.3333333333333333, 0.6666666666666666, 1.0, 0.3333333333333333, 0.6666666666666666, 1.0]);
        check_bin_to_real(&BITS31, &bounds, &[0.6666666666666666, 0.3333333333333333, 0.0, 0.6666666666666666, 0.3333333333333333, 0.0]);
        check_bin_to_real(&BITS32, &bounds, &[1.0, 0.3333333333333333, 0.6666666666666666, 1.0, 0.3333333333333333, 0.6666666666666666]);
        check_bin_to_real(&BITS33, &bounds, &[0.3333333333333333, 0.3333333333333333, 0.3333333333333333, 0.3333333333333333, 0.3333333333333333, 0.3333333333333333]);
        check_bin_to_real(&BITS34, &bounds, &[0.6666666666666666, 0.6666666666666666, 0.6666666666666666, 0.6666666666666666, 0.6666666666666666, 0.6666666666666666]);
    }

    #[test]
    fn test_bin_to_real_6d_2() {
        let bounds = [
            Bounds { lower: 0.0, upper: 2.0 },
            Bounds { lower: 1.0, upper: 4.0 },
            Bounds { lower: 2.0, upper: 6.0 },
            Bounds { lower: 3.0, upper: 8.0 },
            Bounds { lower: 4.0, upper: 10.0 },
            Bounds { lower: 5.0, upper: 12.0 },
        ];
        check_bin_to_real(&BITS15, &bounds, &[0.0, 1.0, 2.0, 3.0, 4.0, 5.0]);
        check_bin_to_real(&BITS16, &bounds, &[2.0, 4.0, 6.0, 8.0, 10.0, 12.0]);
        check_bin_to_real(&BITS17, &bounds, &[0.0, 1.0, 2.0, 3.0, 4.0, 7.333333333333334]);
        check_bin_to_real(&BITS18, &bounds, &[0.6666666666666666, 4.0, 6.0, 8.0, 10.0, 12.0]);
        check_bin_to_real(&BITS19, &bounds, &[1.3333333333333333, 1.0, 2.0, 3.0, 4.0, 5.0]);
        check_bin_to_real(&BITS20, &bounds, &[2.0, 4.0, 6.0, 8.0, 10.0, 9.666666666666668]);
        check_bin_to_real(&BITS21, &bounds, &[0.0, 1.0, 3.333333333333333, 3.0, 4.0, 7.333333333333334]);
        check_bin_to_real(&BITS22, &bounds, &[0.6666666666666666, 4.0, 6.0, 4.666666666666667, 10.0, 12.0]);
        check_bin_to_real(&BITS23, &bounds, &[1.3333333333333333, 1.0, 2.0, 6.333333333333334, 4.0, 5.0]);
        check_bin_to_real(&BITS24, &bounds, &[2.0, 4.0, 4.666666666666666, 8.0, 10.0, 9.666666666666668]);
        check_bin_to_real(&BITS25, &bounds, &[0.0, 2.0, 2.0, 4.666666666666667, 4.0, 7.333333333333334]);
        check_bin_to_real(&BITS26, &bounds, &[0.6666666666666666, 4.0, 3.333333333333333, 8.0, 6.0, 12.0]);
        check_bin_to_real(&BITS27, &bounds, &[1.3333333333333333, 1.0, 4.666666666666666, 3.0, 8.0, 5.0]);
        check_bin_to_real(&BITS28, &bounds, &[2.0, 3.0, 6.0, 6.333333333333334, 10.0, 9.666666666666668]);
        check_bin_to_real(&BITS29, &bounds, &[0.0, 3.0, 3.333333333333333, 3.0, 8.0, 7.333333333333334]);
        check_bin_to_real(&BITS30, &bounds, &[0.6666666666666666, 3.0, 6.0, 4.666666666666667, 8.0, 12.0]);
        check_bin_to_real(&BITS31, &bounds, &[1.3333333333333333, 2.0, 2.0, 6.333333333333334, 6.0, 5.0]);
        check_bin_to_real(&BITS32, &bounds, &[2.0, 2.0, 4.666666666666666, 8.0, 6.0, 9.666666666666668]);
        check_bin_to_real(&BITS33, &bounds, &[0.6666666666666666, 2.0, 3.333333333333333, 4.666666666666667, 6.0, 7.333333333333334]);
        check_bin_to_real(&BITS34, &bounds, &[1.3333333333333333, 3.0, 4.666666666666666, 6.333333333333334, 8.0, 9.666666666666668]);
    }

}