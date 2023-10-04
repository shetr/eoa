

pub fn one_max(bits: &[u8]) -> i32 {
    let mut one_count: i32 = 0;
    for bit in bits.iter() {
        one_count += *bit as i32;
    }
    one_count
}

fn bit_to_sign(bit: u8) -> i32 {
    (bit as i32 * 2) - 1
}

pub fn labs(bits: &[u8]) -> i32 {
    let mut es: i32 = 0;
    for k in 1..bits.len() {
        let mut ck_s = 0;
        for i in 0..bits.len()-k {
            ck_s += bit_to_sign(bits[i]) * bit_to_sign(bits[i + k])
        }
        es += ck_s * ck_s;
    }
    es
}

pub fn sphere(x: &[f64], o: &[f64]) -> f64 {
    let mut res = 0f64;
    for i in 0..x.len() {
        let diff = x[i] - o[i];
        res += diff * diff;
    }
    res
}

pub fn rosenbrock(x: &[f64]) -> f64 {
    let mut res = 0.0;
    for i in 0..x.len()-1 {
        let l = x[i + 1] - x[i]*x[i];
        let r = 1.0 - x[i];
        res += 100.0 * l * l + r * r;
    }
    res
}

pub fn perturbe_mut(bits: &mut [u8], prob: f64) {
    for bit in bits.iter_mut() {
        *bit = if rand::random::<f64>() > prob { *bit } else { *bit ^ 1 };
    }
}

pub fn perturbe(bits: &[u8], prob: f64) -> Vec<u8> {
    let mut res = Vec::from(bits);
    let l = res.len();
    perturbe_mut(&mut res[0..l], prob);
    res
}

#[cfg(test)]
mod tests {

    use super::*;

    const EPSILON: f64 = 1.0e-6;

    fn check_one_max(bits: &[u8], res: i32) {
        assert_eq!(one_max(bits), res);
    }

    fn check_labs(bits: &[u8], res: i32) {
        assert_eq!(labs(bits), res);
    }

    fn check_sphere(x: &[f64], o: &[f64], res: f64) {
        assert_eq!(sphere(x, o), res);
    }

    fn check_rosenbrock(x: &[f64], res: f64) {
        assert!((rosenbrock(x) - res).abs() < EPSILON);
    }

    #[test]
    fn tests_one_max() {
        check_one_max(&[0], 0);
        check_one_max(&[1], 1);
        check_one_max(&[0, 0], 0);
        check_one_max(&[0, 1], 1);
        check_one_max(&[1, 0], 1);
        check_one_max(&[1, 1], 2);
        check_one_max(&[0, 0, 0], 0);
        check_one_max(&[0, 0, 1], 1);
        check_one_max(&[0, 1, 0], 1);
        check_one_max(&[0, 1, 1], 2);
        check_one_max(&[1, 0, 0], 1);
        check_one_max(&[1, 0, 1], 2);
        check_one_max(&[1, 1, 0], 2);
        check_one_max(&[1, 1, 1], 3);
        check_one_max(&[0, 0, 0, 0, 0, 0, 0, 0, 0, 0], 0);
        check_one_max(&[0, 1, 0, 1, 0, 1, 0, 1, 0, 1], 5);
        check_one_max(&[0, 0, 1, 1, 0, 0, 1, 1, 0, 0], 4);
        check_one_max(&[1, 1, 0, 0, 1, 1, 0, 0, 1, 1], 6);
        check_one_max(&[0, 0, 0, 0, 0, 1, 1, 1, 1, 1], 5);
        check_one_max(&[1, 1, 1, 1, 1, 0, 0, 0, 0, 0], 5);
        check_one_max(&[1, 1, 1, 1, 1, 1, 1, 1, 1, 1], 10);
    }

    #[test]
    fn tests_labs() {
        check_labs(&[0], 0);
        check_labs(&[1], 0);
        check_labs(&[0, 0], 1);
        check_labs(&[0, 1], 1);
        check_labs(&[1, 0], 1);
        check_labs(&[1, 1], 1);
        check_labs(&[0, 0, 0], 5);
        check_labs(&[0, 0, 1], 1);
        check_labs(&[0, 1, 0], 5);
        check_labs(&[0, 1, 1], 1);
        check_labs(&[1, 0, 0], 1);
        check_labs(&[1, 0, 1], 5);
        check_labs(&[1, 1, 0], 1);
        check_labs(&[1, 1, 1], 5);
        check_labs(&[0, 0, 0, 0, 0, 0, 0, 0, 0, 0], 285);
        check_labs(&[0, 1, 0, 1, 0, 1, 0, 1, 0, 1], 285);
        check_labs(&[0, 0, 1, 1, 0, 0, 1, 1, 0, 0], 125);
        check_labs(&[1, 1, 0, 0, 1, 1, 0, 0, 1, 1], 125);
        check_labs(&[0, 0, 0, 0, 0, 1, 1, 1, 1, 1], 125);
        check_labs(&[1, 1, 1, 1, 1, 0, 0, 0, 0, 0], 125);
        check_labs(&[1, 1, 1, 1, 1, 1, 1, 1, 1, 1], 285);
    }

    #[test]
    fn test_sphere() {
        let o1 = [1.0; 1];
        let o2 = [1.0; 2];
        let o3 = [1.0; 3];
        let o10 = [1.0; 10];
        check_sphere(&[ 0.0], &o1, 1.0);
        check_sphere(&[ 0.5], &o1, 0.25);
        check_sphere(&[ 1.0], &o1, 0.0);
        check_sphere(&[-0.5], &o1, 2.25);
        check_sphere(&[-5.0], &o1, 36.0);
        check_sphere(&[ 5.0], &o1, 16.0);
        check_sphere(&[ 0.0,  0.0], &o2, 2.0);
        check_sphere(&[-1.0, -1.0], &o2, 8.0);
        check_sphere(&[-1.0,  1.0], &o2, 4.0);
        check_sphere(&[ 1.0, -1.0], &o2, 4.0);
        check_sphere(&[ 1.0,  1.0], &o2, 0.0);
        check_sphere(&[-0.1, -0.2], &o2, 2.6500000000000004);
        check_sphere(&[ 0.1,  0.2], &o2, 1.4500000000000002);
        check_sphere(&[-5.0, -5.0], &o2, 72.0);
        check_sphere(&[-5.0,  5.0], &o2, 52.0);
        check_sphere(&[ 5.0, -5.0], &o2, 52.0);
        check_sphere(&[ 5.0,  5.0], &o2, 32.0);
        check_sphere(&[ 0.0,  0.0,  0.0], &o3, 3.0);
        check_sphere(&[ 1.0,  1.0,  1.0], &o3, 0.0);
        check_sphere(&[-5.0, -5.0, -5.0], &o3, 108.0);
        check_sphere(&[-5.0, -5.0,  5.0], &o3, 88.0);
        check_sphere(&[-5.0,  5.0, -5.0], &o3, 88.0);
        check_sphere(&[-5.0,  5.0,  5.0], &o3, 68.0);
        check_sphere(&[ 5.0, -5.0, -5.0], &o3, 88.0);
        check_sphere(&[ 5.0, -5.0,  5.0], &o3, 68.0);
        check_sphere(&[ 5.0,  5.0, -5.0], &o3, 68.0);
        check_sphere(&[ 5.0,  5.0,  5.0], &o3, 48.0);
        check_sphere(&[-0.1, -1.2, -2.3, -3.4, -4.5, -5.6, -6.7, -7.8, -8.9, -9.1], &o10, 446.85999999999996);
        check_sphere(&[0.1, 1.2, 2.3, 3.4, 4.5, 5.6, 6.7, 7.8, 8.9, 9.1], &o10, 248.45999999999998);
        check_sphere(&[0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0], &o10, 10.0);
        check_sphere(&[1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0], &o10, 0.0);
    }

    #[test]
    fn test_rosenbrock() {
        check_rosenbrock(&[ 0.0], 0.0);
        check_rosenbrock(&[ 0.5], 0.0);
        check_rosenbrock(&[ 1.0], 0.0);
        check_rosenbrock(&[-0.5], 0.0);
        check_rosenbrock(&[-5.0], 0.0);
        check_rosenbrock(&[ 5.0], 0.0);
        check_rosenbrock(&[ 0.0,  0.0], 1.0);
        check_rosenbrock(&[-1.0, -1.0], 404.0);
        check_rosenbrock(&[-1.0,  1.0], 4.0);
        check_rosenbrock(&[ 1.0, -1.0], 400.0);
        check_rosenbrock(&[ 1.0,  1.0], 0.0);
        check_rosenbrock(&[-0.1, -0.2], 5.620000000000001);
        check_rosenbrock(&[ 0.1,  0.2], 4.42);
        check_rosenbrock(&[-5.0, -5.0], 90036.0);
        check_rosenbrock(&[-5.0,  5.0], 40036.0);
        check_rosenbrock(&[ 5.0, -5.0], 90016.0);
        check_rosenbrock(&[ 5.0,  5.0], 40016.0);
        check_rosenbrock(&[ 0.0,  0.0,  0.0], 2.0);
        check_rosenbrock(&[ 1.0,  1.0,  1.0], 0.0);
        check_rosenbrock(&[-5.0, -5.0, -5.0], 180072.0);
        check_rosenbrock(&[-5.0, -5.0,  5.0], 130072.0);
        check_rosenbrock(&[-5.0,  5.0, -5.0], 130052.0);
        check_rosenbrock(&[-5.0,  5.0,  5.0], 80052.0);
        check_rosenbrock(&[ 5.0, -5.0, -5.0], 180052.0);
        check_rosenbrock(&[ 5.0, -5.0,  5.0], 130052.0);
        check_rosenbrock(&[ 5.0,  5.0, -5.0], 130032.0);
        check_rosenbrock(&[ 5.0,  5.0,  5.0], 80032.0);
        check_rosenbrock(&[-0.1, -1.2, -2.3, -3.4, -4.5, -5.6, -6.7, -7.8, -8.9, -9.1], 1790768.58);
        check_rosenbrock(&[0.1, 1.2, 2.3, 3.4, 4.5, 5.6, 6.7, 7.8, 8.9, 9.1], 986898.1800000002);
        check_rosenbrock(&[0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0], 9.0);
        check_rosenbrock(&[1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0], 0.0);
    }

}