use std::{f64::consts::PI, rc::Rc};

use crate::*;

const EPSILON: f64 = 0.000001;

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
    fn optimum(&self) -> f64 {
        self.func.optimum()
    }
    fn bounds(&self) -> Vec<Bounds> {
        self.func.bounds()
    }
}

pub trait GFunc : FitnessFunc<FloatVec> + ConstraintsSumed<FloatVec> {
    fn optimum(&self) -> f64;
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

pub struct G06 {}

impl GFunc for G06 {
    fn optimum(&self) -> f64 {
        -6961.81387558015
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
        (data.values[0] - 10.0).powi(3) + (data.values[1] - 20.0).powi(3)
    }
}

impl ConstraintsSumed<FloatVec> for G06 {
    fn violations(&self, data: &FloatVec) -> Vec<f64> {
        vec![
            0.0f64.max(-(data.values[0] - 5.0).powi(2) - (data.values[1] - 5.0).powi(2) + 100.0),
            0.0f64.max((data.values[0] - 6.0).powi(2) + (data.values[1] - 5.0).powi(2) - 82.81)
        ]
    }
}

pub struct G08 {}

impl GFunc for G08 {
    fn optimum(&self) -> f64 {
        -0.0958250414180359
    }
    fn bounds(&self) -> Vec<Bounds> {
        vec![
            Bounds { lower: 0.0, upper: 10.0 },
            Bounds { lower: 0.0, upper: 10.0 }
        ]
    }
}

impl FitnessFunc<FloatVec> for G08 {
    fn eval(&self, data: &FloatVec) -> f64 {
        - (2.0 * PI * data.values[0]).sin().powi(3) * (2.0 * PI * data.values[1]).sin() / (data.values[0].powi(3) * (data.values[0] + data.values[1]))
    }
}

impl ConstraintsSumed<FloatVec> for G08 {
    fn violations(&self, data: &FloatVec) -> Vec<f64> {
        vec![
            0.0f64.max(data.values[0].powi(2) - data.values[1] + 1.0),
            0.0f64.max(1.0 - data.values[0] + (data.values[1] - 4.0).powi(2))
        ]
    }
}

pub struct G11 {}

impl GFunc for G11 {
    fn optimum(&self) -> f64 {
        0.7499
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
        data.values[0].powi(2) + (data.values[1] - 1.0).powi(2)
    }
}

impl ConstraintsSumed<FloatVec> for G11 {
    fn violations(&self, data: &FloatVec) -> Vec<f64> {
        vec![
            0.0f64.max((data.values[1] - data.values[0].powi(2)).abs() - EPSILON)
        ]
    }
}

pub struct G24 {}

impl GFunc for G24 {
    fn optimum(&self) -> f64 {
        -5.50801327159536
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
        -data.values[0] - data.values[1]
    }
}

impl ConstraintsSumed<FloatVec> for G24 {
    fn violations(&self, data: &FloatVec) -> Vec<f64> {
        vec![
            0.0f64.max(-2.0*data.values[0].powi(4) + 8.0*data.values[0].powi(3) - 8.0*data.values[0].powi(2) + data.values[1] - 2.0),
            0.0f64.max(-4.0*data.values[0].powi(4) +32.0*data.values[0].powi(3) -88.0*data.values[0].powi(2) + 96.0*data.values[0] + data.values[1] - 36.0)
        ]
    }
}