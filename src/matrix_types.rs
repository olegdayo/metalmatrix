use std::ops::Index;
use crate::vector_types::{CompressedVector, FullVector, Vector};

pub trait Matrix: Index<usize, Output=dyn Vector> {
    fn rows(&self) -> usize;
    fn cols(&self) -> usize;
    fn shape(&self) -> (usize, usize);
    fn to_vec(&self) -> Vec<Vec<f64>>;

    fn new() -> Self;
    fn from(matrix: Vec<Vec<f64>>) -> Result<Self, String> where Self: Sized;
}

#[derive(Debug, Default, PartialEq)]
pub struct BaseMatrix {
    pub(crate) rows: usize,
    pub(crate) cols: usize,
    pub(crate) matr: Vec<FullVector>,
}

#[derive(Debug, Default, PartialEq)]
pub struct SquareMatrix {
    pub(crate) side: usize,
    pub(crate) matr: Vec<FullVector>,
}

#[derive(Debug, Default, PartialEq)]
pub struct DiagonalMatrix {
    pub(crate) side: usize,
    pub(crate) diag: Vec<CompressedVector>,
}
