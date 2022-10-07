use std::ops::Index;
use crate::types::{BaseMatrix, DiagonalMatrix, SquareMatrix, LowerTriangularMatrix};

impl Index<usize> for BaseMatrix {
    type Output = [f64];

    fn index(&self, index: usize) -> &Self::Output {
        todo!()
    }
}

impl Index<usize> for DiagonalMatrix {
    type Output = [f64];

    fn index(&self, index: usize) -> &Self::Output {
        todo!()
    }
}

impl Index<usize> for SquareMatrix {
    type Output = [f64];

    fn index(&self, index: usize) -> &Self::Output {
        todo!()
    }
}

impl Index<usize> for LowerTriangularMatrix {
    type Output = [f64];

    fn index(&self, index: usize) -> &Self::Output {
        todo!()
    }
}
