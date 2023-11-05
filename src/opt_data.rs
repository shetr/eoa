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

pub trait VecOptData<T : Clone> {
    fn new() -> Self;
    fn with_capacity(capacity: usize) -> Self;
    fn get(&self) -> &Vec<T>;
    fn get_mut(&mut self) -> &mut Vec<T>;
}

impl VecOptData<f64> for FloatVec {
    fn new() -> Self {
        FloatVec { values: Vec::new() }
    }
    fn with_capacity(capacity: usize) -> Self {
        FloatVec { values: Vec::with_capacity(capacity) }
    }
    fn get(&self) -> &Vec<f64> {
        &self.values
    }
    fn get_mut(&mut self) -> &mut Vec<f64> {
        &mut self.values
    }
}

impl VecOptData<u8> for NaiveBitVec {
    fn new() -> Self {
        NaiveBitVec { bits: Vec::new() }
    }
    fn with_capacity(capacity: usize) -> Self {
        NaiveBitVec { bits: Vec::with_capacity(capacity) }
    }
    fn get(&self) -> &Vec<u8> {
        &self.bits
    }
    fn get_mut(&mut self) -> &mut Vec<u8> {
        &mut self.bits
    }
}

pub struct Solution<T: OptData> {
    pub value: T,
    pub fitness: f64
}

#[derive(Clone)]
pub struct Statistics {
    pub fitness: Vec<f64>
}