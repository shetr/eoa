use std::{f64::consts::PI, rc::Rc};

use crate::*;

const EPSILON: f64 = 0.0015;

#[derive(Clone)]
pub struct GFuncDyn {
    pub func: Rc<dyn GFunc>
}

impl FitnessFunc<FloatVec> for GFuncDyn {
    fn eval(&self, data: &FloatVec) -> f64 {
        self.func.eval(data)
    }
}

impl ConstraintsSumed<FloatVec> for GFuncDyn {
    fn violations(&self, data: &FloatVec) -> Vec<f64> {
        self.func.violations(data)
    }
}

impl GFunc for GFuncDyn {
    fn vec_size(&self) -> usize {
        self.func.vec_size()
    }
    fn optimum(&self) -> FloatVec {
        self.func.optimum()
    }
    fn bounds(&self) -> Vec<Bounds> {
        self.func.bounds()
    }
}

pub trait GFunc : FitnessFunc<FloatVec> + ConstraintsSumed<FloatVec> {
    fn vec_size(&self) -> usize;
    fn optimum(&self) -> FloatVec;
    fn bounds(&self) -> Vec<Bounds>;
}

#[derive(Clone)]
pub struct BiGFunc {
    pub g_func: Rc<dyn GFunc>
}

impl MultiObjFitnessFunc<FloatVec> for BiGFunc {
    fn eval(&self, data: &FloatVec, out: &mut Vec<f64>) {
        out.resize(2, 0.0);
        out[0] = self.g_func.eval(data);
        // TODO: call already existing function
        out[1] = self.g_func.violations(data).iter().sum::<f64>();
    }
}

#[derive(Clone)]
pub struct MultiGFunc {
    pub g_func: Rc<dyn GFunc>
}

impl MultiObjFitnessFunc<FloatVec> for MultiGFunc {
    fn eval(&self, data: &FloatVec, out: &mut Vec<f64>) {
        let violatons = self.g_func.violations(data);
        out.resize(1 + violatons.len(), 0.0);
        out[0] = self.g_func.eval(data);
        for i in 0..violatons.len() {
            out[i + 1] = violatons[i];
        }
    }
}

// Basic GFuncs

pub struct G06 {}

impl GFunc for G06 {
    fn vec_size(&self) -> usize {
        2
    }
    fn optimum(&self) -> FloatVec {
        FloatVec { values: vec![14.09500000000000064, 0.8429607892154795668] }
    }
    fn bounds(&self) -> Vec<Bounds> {
        vec![
            Bounds { lower: 0.0, upper: 100.0 },
            Bounds { lower: 0.0, upper: 100.0 }
        ]
    }
}

impl FitnessFunc<FloatVec> for G06 {
    fn eval(&self, data: &FloatVec) -> f64 {
        let x = &data.values;
        (x[0] - 10.0).powi(3) + (x[1] - 20.0).powi(3)
    }
}

impl ConstraintsSumed<FloatVec> for G06 {
    fn violations(&self, data: &FloatVec) -> Vec<f64> {
        let x = &data.values;
        vec![
            0.0f64.max(-(x[0] - 5.0).powi(2) - (x[1] - 5.0).powi(2) + 100.0),
            0.0f64.max((x[0] - 6.0).powi(2) + (x[1] - 5.0).powi(2) - 82.81)
        ]
    }
}

pub struct G08 {}

impl GFunc for G08 {
    fn vec_size(&self) -> usize {
        2
    }
    fn optimum(&self) -> FloatVec {
        FloatVec { values: vec![1.22797135260752599, 4.24537336612274885] }
    }
    fn bounds(&self) -> Vec<Bounds> {
        vec![
            Bounds { lower: 0.00001, upper: 10.0 },
            Bounds { lower: 0.0, upper: 10.0 }
        ]
    }
}

impl FitnessFunc<FloatVec> for G08 {
    fn eval(&self, data: &FloatVec) -> f64 {
        let x = &data.values;
        - (2.0 * PI * x[0]).sin().powi(3) * (2.0 * PI * x[1]).sin() / (x[0].powi(3) * (x[0] + x[1]))
    }
}

impl ConstraintsSumed<FloatVec> for G08 {
    fn violations(&self, data: &FloatVec) -> Vec<f64> {
        let x = &data.values;
        vec![
            0.0f64.max(x[0].powi(2) - x[1] + 1.0),
            0.0f64.max(1.0 - x[0] + (x[1] - 4.0).powi(2))
        ]
    }
}

pub struct G11 {}

impl GFunc for G11 {
    fn vec_size(&self) -> usize {
        2
    }
    fn optimum(&self) -> FloatVec {
        FloatVec { values: vec![-0.707036070037170616, 0.500000004333606807] }
    }
    fn bounds(&self) -> Vec<Bounds> {
        vec![
            Bounds { lower: -1.0, upper: 1.0 },
            Bounds { lower: -1.0, upper: 1.0 }
        ]
    }
}

impl FitnessFunc<FloatVec> for G11 {
    fn eval(&self, data: &FloatVec) -> f64 {
        let x = &data.values;
        x[0].powi(2) + (x[1] - 1.0).powi(2)
    }
}

impl ConstraintsSumed<FloatVec> for G11 {
    fn violations(&self, data: &FloatVec) -> Vec<f64> {
        let x = &data.values;
        vec![
            0.0f64.max((x[1] - x[0].powi(2)).abs() - EPSILON)
        ]
    }
}

pub struct G24 {}

impl GFunc for G24 {
    fn vec_size(&self) -> usize {
        2
    }
    fn optimum(&self) -> FloatVec {
        FloatVec { values: vec![2.32952019747762, 3.17849307411774] }
    }
    fn bounds(&self) -> Vec<Bounds> {
        vec![
            Bounds { lower: 0.0, upper: 3.0 },
            Bounds { lower: 0.0, upper: 4.0 }
        ]
    }
}

impl FitnessFunc<FloatVec> for G24 {
    fn eval(&self, data: &FloatVec) -> f64 {
        let x = &data.values;
        -x[0] - x[1]
    }
}

impl ConstraintsSumed<FloatVec> for G24 {
    fn violations(&self, data: &FloatVec) -> Vec<f64> {
        let x = &data.values;
        vec![
            0.0f64.max(-2.0*x[0].powi(4) + 8.0*x[0].powi(3) - 8.0*x[0].powi(2) + x[1] - 2.0),
            0.0f64.max(-4.0*x[0].powi(4) +32.0*x[0].powi(3) -88.0*x[0].powi(2) + 96.0*x[0] + x[1] - 36.0)
        ]
    }
}

// More complicated GFuncs

pub struct G04 {}

impl GFunc for G04 {
    fn vec_size(&self) -> usize {
        5
    }
    fn optimum(&self) -> FloatVec {
        FloatVec { values: vec![78.0, 33.0, 29.9952560256815985, 45.0, 36.7758129057882073] }
    }
    fn bounds(&self) -> Vec<Bounds> {
        vec![
            Bounds { lower: 78.0, upper: 102.0 },
            Bounds { lower: 33.0, upper: 45.0 },
            Bounds { lower: 27.0, upper: 45.0 },
            Bounds { lower: 27.0, upper: 45.0 },
            Bounds { lower: 27.0, upper: 45.0 }
        ]
    }
}

impl FitnessFunc<FloatVec> for G04 {
    fn eval(&self, data: &FloatVec) -> f64 {
        let x = &data.values;
        let (x1, x3, x5) = (x[0], x[2], x[4]);
        5.3578547 * x3.powi(2) + 0.8356891 * x1 * x5 + 37.293239 * x1 - 40792.141
    }
}

impl ConstraintsSumed<FloatVec> for G04 {
    fn violations(&self, data: &FloatVec) -> Vec<f64> {
        let x = &data.values;
        let (x1, x2, x3, x4, x5) = (x[0], x[1], x[2], x[3], x[4]);
        vec![
            0.0f64.max( 85.334407 + 0.0056858 * x2 * x5 + 0.0006262 * x1 * x4 - 0.0022053 * x3 * x5 - 92.0),
            0.0f64.max(-85.334407 - 0.0056858 * x2 * x5 - 0.0006262 * x1 * x4 + 0.0022053 * x3 * x5),
            0.0f64.max( 80.51249  + 0.0071317 * x2 * x5 + 0.0029955 * x1 * x2 + 0.0021813 * x3.powi(2) - 110.0),
            0.0f64.max( -80.51249 - 0.0071317 * x2 * x5 - 0.0029955 * x1 * x2 - 0.0021813 * x3.powi(2) + 90.0),
            0.0f64.max(  9.300961 + 0.0047026 * x3 * x5 + 0.0012547 * x1 * x3 + 0.0019085 * x3 * x4 - 25.0),
            0.0f64.max( -9.300961 - 0.0047026 * x3 * x5 - 0.0012547 * x1 * x3 - 0.0019085 * x3 * x4 + 20.0)
        ]
    }
}

pub struct G05 {}

impl GFunc for G05 {
    fn vec_size(&self) -> usize {
        4
    }
    fn optimum(&self) -> FloatVec {
        FloatVec { values: vec![679.945148297028709, 1026.06697600004691, 0.118876369094410433, -0.39623348521517826] }
    }
    fn bounds(&self) -> Vec<Bounds> {
        vec![
            Bounds { lower: 0.0, upper: 1200.0 },
            Bounds { lower: 0.0, upper: 1200.0 },
            Bounds { lower: -0.55, upper: 0.55 },
            Bounds { lower: -0.55, upper: 0.55 }
        ]
    }
}

impl FitnessFunc<FloatVec> for G05 {
    fn eval(&self, data: &FloatVec) -> f64 {
        let x = &data.values;
        let (x1, x2) = (x[0], x[1]);
        3.0 * x1 + 0.000001 * x1.powi(3) + 2.0 * x2 + (0.000002/3.0) * x2.powi(3)
    }
}

impl ConstraintsSumed<FloatVec> for G05 {
    fn violations(&self, data: &FloatVec) -> Vec<f64> {
        let x = &data.values;
        let (x1, x2, x3, x4) = (x[0], x[1], x[2], x[3]);
        vec![
            0.0f64.max(-x4 + x3 - 0.55),
            0.0f64.max(-x3 + x4 - 0.55),
            0.0f64.max((1000.0 * (-x3 - 0.25).sin() + 1000.0 * (   - x4 - 0.25).sin() +  894.8 - x1).abs() - EPSILON),
            0.0f64.max((1000.0 * ( x3 - 0.25).sin() + 1000.0 * (x3 - x4 - 0.25).sin() +  894.8 - x2).abs() - EPSILON),
            0.0f64.max((1000.0 * ( x4 - 0.25).sin() + 1000.0 * (x4 - x3 - 0.25).sin() + 1294.8     ).abs() - EPSILON),
        ]
    }
}

pub struct G09 {}

impl GFunc for G09 {
    fn vec_size(&self) -> usize {
        7
    }
    fn optimum(&self) -> FloatVec {
        FloatVec { values: vec![
            2.33049935147405174,
            1.95137236847114592,
            -0.477541399510615805,
            4.36572624923625874,
            -0.624486959100388983,
            1.03813099410962173,
            1.5942266780671519
        ] }
    }
    fn bounds(&self) -> Vec<Bounds> {
        vec![
            Bounds { lower: -10.0, upper: 10.0 },
            Bounds { lower: -10.0, upper: 10.0 },
            Bounds { lower: -10.0, upper: 10.0 },
            Bounds { lower: -10.0, upper: 10.0 },
            Bounds { lower: -10.0, upper: 10.0 },
            Bounds { lower: -10.0, upper: 10.0 },
            Bounds { lower: -10.0, upper: 10.0 }
        ]
    }
}

impl FitnessFunc<FloatVec> for G09 {
    fn eval(&self, data: &FloatVec) -> f64 {
        let x = &data.values;
        let (x1, x2, x3, x4, x5, x6, x7) = (x[0], x[1], x[2], x[3], x[4], x[5], x[6]);
        (x1 - 10.0).powi(2) + 5.0 * (x2 - 12.0).powi(2) + x3.powi(4) + 3.0 * (x4 - 11.0).powi(2)
        + 10.0 * x5.powi(6) + 7.0 * x6.powi(2) + x7.powi(4) - 4.0 * x6 * x7 - 10.0 * x6 - 8.0 * x7
    }
}

impl ConstraintsSumed<FloatVec> for G09 {
    fn violations(&self, data: &FloatVec) -> Vec<f64> {
        let x = &data.values;
        let (x1, x2, x3, x4, x5, x6, x7) = (x[0], x[1], x[2], x[3], x[4], x[5], x[6]);
        vec![
            0.0f64.max(-127.0 + 2.0 * x1.powi(2) + 3.0 * x2.powi(4) + x3 + 4.0 * x4.powi(2) + 5.0 * x5),
            0.0f64.max(-282.0 + 7.0 * x1 + 3.0 * x2 + 10.0 * x3.powi(2) + x4 - x5),
            0.0f64.max(-196.0 + 23.0 * x1 + x2.powi(2) + 6.0 * x6.powi(2) - 8.0 * x7),
            0.0f64.max(4.0 * x1.powi(2) + x2.powi(2) - 3.0 * x1 * x2 + 2.0 * x3.powi(2) + 5.0 * x6 - 11.0 * x7),
        ]
    }
}

pub struct G21 {}

impl GFunc for G21 {
    fn vec_size(&self) -> usize {
        7
    }
    fn optimum(&self) -> FloatVec {
        FloatVec { values: vec![
            193.724510070034967,
            5.56944131553368433e-27,
            17.3191887294084914,
            100.047897801386839,
            6.68445185362377892,
            5.99168428444264833,
            6.21451648886070451
        ] }
    }
    fn bounds(&self) -> Vec<Bounds> {
        vec![
            Bounds { lower: 0.0, upper: 1000.0 },
            Bounds { lower: 0.0, upper: 40.0 },
            Bounds { lower: 0.0, upper: 40.0 },
            Bounds { lower: 100.0, upper: 300.0 },
            Bounds { lower: 6.3, upper: 6.7 },
            Bounds { lower: 5.9, upper: 6.4 },
            Bounds { lower: 4.5, upper: 6.25 }
        ]
    }
}

impl FitnessFunc<FloatVec> for G21 {
    fn eval(&self, data: &FloatVec) -> f64 {
        let x = &data.values;
        x[0]
    }
}

impl ConstraintsSumed<FloatVec> for G21 {
    fn violations(&self, data: &FloatVec) -> Vec<f64> {
        let x = &data.values;
        let (x1, x2, x3, x4, x5, x6, x7) = (x[0], x[1], x[2], x[3], x[4], x[5], x[6]);
        vec![
            0.0f64.max(-x1 + 35.0 * x2.powf(0.6) + 35.0 * x3.powf(0.6)),
            0.0f64.max((-300.0 * x3 + 7500.0 * x5 - 7500.0 * x6 - 25.0 * x4 * x5 + 25.0 * x4 * x6 + x3 * x4).abs() - EPSILON),
            0.0f64.max((100.0 * x2 + 155.365 * x4 + 2500.0 * x7 - x2 * x4 - 25.0 * x4 * x7 - 15536.5).abs() - EPSILON),
            0.0f64.max((-x5 + (-x4 + 900.0).ln()).abs() - EPSILON),
            0.0f64.max((-x6 + (x4 + 300.0).ln()).abs() - EPSILON),
            0.0f64.max((-x7 + (-2.0 *x4 + 700.0).ln()).abs() - EPSILON),
        ]
    }
}