use std::{f64::consts::PI, marker::PhantomData, rc::Rc, borrow::BorrowMut, cell::RefCell};

use crate::*;

#[derive(Clone)]
pub struct GFuncDyn {
    pub func: Rc<RefCell<dyn GFunc>>
}

impl FitnessFunc<FloatVec> for GFuncDyn {
    fn eval(&mut self, data: &FloatVec) -> f64 {
        let mut func_ref = self.func.borrow_mut();
        
        *reference = false;
        self.func.to_owned().borrow_mut().eval(data)
    }
}

impl ConstraintsCounted<FloatVec> for GFuncDyn {
    fn constraints_fulfilled(&self, data: &FloatVec) -> Vec<bool> {
        self.func.constraints_fulfilled(data)
    }
}

pub trait GFunc : FitnessFunc<FloatVec> + ConstraintsCounted<FloatVec> {
    fn optimum(&self) -> f64; 
}

#[derive(Clone)]
pub struct BiGFunc {
    pub g_func: Rc<dyn GFunc>
}

impl MultiObjFitnessFunc<FloatVec> for BiGFunc {
    fn eval(&mut self, data: &FloatVec, out: &mut Vec<f64>) {
        out.resize(2, 0.0);
        out[0] = self.g_func.eval(data);
        out[1] = self.g_func.constraints_fulfilled(data).iter().map(|x| if *x { 0.0 } else { 1.0 }).sum::<f64>();
    }
}

pub struct G06 {}

impl GFunc for G06 {
    fn optimum(&self) -> f64 {
        -6961.81387558015
    }
}

impl FitnessFunc<FloatVec> for G06 {
    fn eval(&mut self, data: &FloatVec) -> f64 {
        (data.values[0] - 10.0).powi(3) + (data.values[1] - 20.0).powi(3)
    }
}

impl ConstraintsCounted<FloatVec> for G06 {
    fn constraints_fulfilled(&self, data: &FloatVec) -> Vec<bool> {
        vec![
            -(data.values[0] - 5.0).powi(2) - (data.values[1] - 5.0).powi(2) + 100.0 <= 0.0,
            (data.values[0] - 6.0).powi(2) + (data.values[1] - 5.0).powi(2) - 82.81 <= 0.0,
            0.0 <= data.values[0] && data.values[0] <= 100.0 && 0.0 <= data.values[1] && data.values[1] <= 100.0
        ]
    }
}

pub struct G08 {}

impl GFunc for G08 {
    fn optimum(&self) -> f64 {
        -0.0958250414180359
    }
}

impl FitnessFunc<FloatVec> for G08 {
    fn eval(&mut self, data: &FloatVec) -> f64 {
        - (2.0 * PI * data.values[0]).sin().powi(3) * (2.0 * PI * data.values[1]).sin() / (data.values[0].powi(3) * (data.values[0] + data.values[1]))
    }
}

impl ConstraintsCounted<FloatVec> for G08 {
    fn constraints_fulfilled(&self, data: &FloatVec) -> Vec<bool> {
        vec![
            data.values[0].powi(2) - data.values[1] + 1.0 <= 0.0,
            1.0 - data.values[0] + (data.values[1] - 4.0).powi(2) <= 0.0,
            0.0 <= data.values[0] && data.values[0] <= 10.0 && 0.0 <= data.values[1] && data.values[1] <= 10.0
        ]
    }
}

pub struct G11 {}

impl GFunc for G11 {
    fn optimum(&self) -> f64 {
        0.7499
    }
}

impl FitnessFunc<FloatVec> for G11 {
    fn eval(&mut self, data: &FloatVec) -> f64 {
        data.values[0].powi(2) + (data.values[1] - 1.0).powi(2)
    }
}

impl ConstraintsCounted<FloatVec> for G11 {
    fn constraints_fulfilled(&self, data: &FloatVec) -> Vec<bool> {
        vec![
            data.values[1] - data.values[0].powi(2) == 0.0,
            -1.0 <= data.values[0] && data.values[0] <= 1.0 && -1.0 <= data.values[1] && data.values[1] <= 1.0
        ]
    }
}

pub struct G24 {}

impl GFunc for G24 {
    fn optimum(&self) -> f64 {
        -5.50801327159536
    }
}

impl FitnessFunc<FloatVec> for G24 {
    fn eval(&mut self, data: &FloatVec) -> f64 {
        -data.values[0] - data.values[1]
    }
}

impl ConstraintsCounted<FloatVec> for G24 {
    fn constraints_fulfilled(&self, data: &FloatVec) -> Vec<bool> {
        vec![
            -2.0*data.values[0].powi(4) + 8.0*data.values[0].powi(3) - 8.0*data.values[0].powi(2) + data.values[1] - 2.0 <= 0.0,
            -4.0*data.values[0].powi(4) +32.0*data.values[0].powi(3) -88.0*data.values[0].powi(2) + 96.0*data.values[0] + data.values[1] - 36.0 <= 0.0,
            0.0 <= data.values[0] && data.values[0] <= 3.0 && 0.0 <= data.values[1] && data.values[1] <= 4.0
        ]
    }
}