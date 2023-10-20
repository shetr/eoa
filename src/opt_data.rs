use crate::opt_traits::*;

#[derive(Debug, Copy, Clone)]
pub struct Bounds {
    pub upper: f64,
    pub lower: f64
}

#[derive(Clone)]
pub struct FloatVec {
    pub values: Vec<f64>
}

#[derive(Clone)]
pub struct NaiveBitVec {
    pub bits: Vec<u8>
}

impl OptData for FloatVec {
    fn dim(&self) -> usize {
        self.values.len()
    }
}

impl OptData for NaiveBitVec {
    fn dim(&self) -> usize {
        self.bits.len()
    }
}

pub struct Solution<T: OptData> {
    pub value: T,
    pub fitness: f64
}

pub struct Statistics {
    pub fitness: Vec<f64>
}