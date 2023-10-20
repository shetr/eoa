use crate::opt_traits::*;
use crate::opt_data::*;
use crate::utils::*;

pub fn naive_one_max(bits: &[u8]) -> i32 {
    let mut one_count: i32 = 0;
    for bit in bits.iter() {
        one_count += *bit as i32;
    }
    one_count
}

fn bit_to_sign(bit: u8) -> i32 {
    (bit as i32 * 2) - 1
}

pub fn naive_labs(bits: &[u8]) -> i32 {
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

pub fn sphere_bin(bits: &[u8], bounds: &[Bounds], o: &[f64]) -> f64 {
    sphere(&bin_to_real(bits, bounds), o)
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

pub fn rosenbrock_bin(bits: &[u8], bounds: &[Bounds]) -> f64 {
    rosenbrock(&bin_to_real(bits, bounds))
}

pub fn linear(x: &[f64], a: f64, b: &[f64]) -> f64 {
    let mut res = a;
    for i in 0..x.len() {
        res += x[i] * b[i];
    }
    res
}

pub fn step(x: &[f64], a: f64, b: &[f64]) -> f64 {
    let mut res = a;
    for i in 0..x.len() {
        res += (x[i] * b[i]).floor();
    }
    res
}

pub fn rastrigin(x: &[f64]) -> f64 {
    let mut res = 10.0 * (x.len() as f64);
    for i in 0..x.len() {
        res += x[i] * x[i] - 10.0 * (2.0 * std::f64::consts::PI * x[i]).cos();
    }
    res
}

pub fn griewank(x: &[f64]) -> f64 {
    let mut sum = 0.0;
    let mut prod = 0.0;
    for i in 0..x.len() {
        sum += x[i] * x[i];
        prod *= (x[i] / (i as f64).sqrt()).cos();
    }
    1.0 + (1.0/4000.0) * sum - prod
}

pub fn schwefel(x: &[f64]) -> f64 {
    let mut sum = 0.0;
    for i in 0..x.len() {
        sum += x[i] * x[i].abs().sqrt().sin();
    }
    -sum
}

pub struct OneMaxFunc { }

pub struct LabsFunc { }

pub struct NaiveBitRealFunc<RealFunc: FitnessFunc<FloatVec>> {
    pub real_func: RealFunc,
    pub bounds: Vec<Bounds>,
    temp_data: FloatVec
}

impl<RealFunc: FitnessFunc<FloatVec>> NaiveBitRealFunc<RealFunc> {
    pub fn new(real_func: RealFunc, bounds: Vec<Bounds>) -> Self {
        let len = bounds.len();
        NaiveBitRealFunc { real_func: real_func, bounds: bounds, temp_data: FloatVec { values: vec![0.0; len] } }
    }
}

impl<RealFunc: FitnessFunc<FloatVec>> FitnessFunc<NaiveBitVec> for NaiveBitRealFunc<RealFunc> {
    fn eval(&mut self, data: &NaiveBitVec) -> f64 {
        bin_to_real_mut(&data.bits, &self.bounds, &mut self.temp_data.values);
        self.real_func.eval(&self.temp_data)
    }
}

pub struct SphereFunc {
    pub o: Vec<f64>
}

pub struct RosenbrockFunc { }

pub struct LinearFunc {
    a: f64,
    b: Vec<f64>
}

pub struct StepFunc {
    a: f64,
    b: Vec<f64>
}

pub struct RastriginFunc {}

pub struct GriewankFunc {}

pub struct SchwefelFunc {}

impl FitnessFunc<NaiveBitVec> for OneMaxFunc {
    fn eval(&mut self, data: &NaiveBitVec) -> f64 {
        naive_one_max(&data.bits) as f64
    }
}

impl FitnessFunc<NaiveBitVec> for LabsFunc {
    fn eval(&mut self, data: &NaiveBitVec) -> f64 {
        naive_labs(&data.bits) as f64
    }
}

impl FitnessFunc<FloatVec> for SphereFunc {
    fn eval(&mut self, data: &FloatVec) -> f64 {
        sphere(&data.values, &self.o)
    }
}

impl FitnessFunc<FloatVec> for RosenbrockFunc {
    fn eval(&mut self, data: &FloatVec) -> f64 {
        rosenbrock(&data.values)
    }
}

impl FitnessFunc<FloatVec> for LinearFunc {
    fn eval(&mut self, data: &FloatVec) -> f64 {
        linear(&data.values, self.a, &self.b)
    }
}

impl FitnessFunc<FloatVec> for StepFunc {
    fn eval(&mut self, data: &FloatVec) -> f64 {
        step(&data.values, self.a, &self.b)
    }
}

impl FitnessFunc<FloatVec> for RastriginFunc {
    fn eval(&mut self, data: &FloatVec) -> f64 {
        rastrigin(&data.values)
    }
}

impl FitnessFunc<FloatVec> for GriewankFunc {
    fn eval(&mut self, data: &FloatVec) -> f64 {
        griewank(&data.values)
    }
}

impl FitnessFunc<FloatVec> for SchwefelFunc {
    fn eval(&mut self, data: &FloatVec) -> f64 {
        schwefel(&data.values)
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    const EPSILON: f64 = 1.0e-6;

    fn check_one_max(bits: &[u8], res: i32) {
        assert_eq!(naive_one_max(bits), res);
    }

    fn check_labs(bits: &[u8], res: i32) {
        assert_eq!(naive_labs(bits), res);
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