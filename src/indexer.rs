use std::ops::Index;
use crate::types::{BaseMatrix, SquareMatrix, DiagonalMatrix, LowerTriangularMatrix, Matrix};

impl Index<usize> for BaseMatrix {
    type Output = [f64];

    fn index(&self, index: usize) -> &Self::Output {
        self.get_row(index)
    }
}

impl Index<usize> for SquareMatrix {
    type Output = [f64];

    fn index(&self, index: usize) -> &Self::Output {
        self.get_row(index)
    }
}

impl Index<usize> for DiagonalMatrix {
    type Output = [f64];

    fn index(&self, index: usize) -> &Self::Output {
        self.get_row(index)
    }
}

impl Index<usize> for LowerTriangularMatrix {
    type Output = [f64];

    fn index(&self, index: usize) -> &Self::Output {
        self.get_row(index)
    }
}
