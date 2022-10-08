use crate::types::{Matrix, BaseMatrix, DiagonalMatrix, SquareMatrix, LowerTriangularMatrix};

impl Matrix for BaseMatrix {
    fn rows(&self) -> usize {
        self.rows
    }

    fn cols(&self) -> usize {
        self.cols
    }

    fn shape(&self) -> (usize, usize) {
        (self.rows, self.cols)
    }

    fn to_vec(&self) -> Vec<Vec<f64>> {
        let mut v = Vec::new();

        for i in 0..self.rows {
            v.push(Vec::new());
            for j in 0..self.cols {
                v[i].push(self[i][j]);
            }
        }

        v
    }

    fn get_row(&self, index: usize) -> &[f64] {
        self.matr[index].as_slice()
    }

    fn new() -> Self {
        BaseMatrix::default()
    }

    fn from(matrix: Vec<Vec<f64>>) -> Result<Self, String> {
        if matrix.len() == 0 {
            return Err("Cannot create 0-row matrix".to_string())
        }

        if matrix[0].len() == 0 {
            return Err("Cannot create 0-column matrix".to_string())
        }

        let mut m = BaseMatrix::new();
        m.matr = matrix;

        Ok(m)
    }
}

impl Matrix for SquareMatrix {
    fn rows(&self) -> usize {
        self.side
    }

    fn cols(&self) -> usize {
        self.side
    }

    fn shape(&self) -> (usize, usize) {
        (self.side, self.side)
    }

    fn to_vec(&self) -> Vec<Vec<f64>> {
        todo!()
    }

    fn get_row(&self, index: usize) -> &[f64] {
        self.matr[index].as_slice()
    }

    fn new() -> Self {
        SquareMatrix::default()
    }

    fn from(matrix: Vec<Vec<f64>>) -> Result<Self, String> {
        todo!()
    }
}

impl Matrix for DiagonalMatrix {
    fn rows(&self) -> usize {
        self.side
    }

    fn cols(&self) -> usize {
        self.side
    }

    fn shape(&self) -> (usize, usize) {
        (self.side, self.side)
    }

    fn to_vec(&self) -> Vec<Vec<f64>> {
        todo!()
    }

    fn get_row(&self, index: usize) -> &[f64] {
        todo!()
    }

    fn new() -> Self {
        DiagonalMatrix::default()
    }

    fn from(matrix: Vec<Vec<f64>>) -> Result<Self, String> {
        todo!()
    }
}

impl Matrix for LowerTriangularMatrix {
    fn rows(&self) -> usize {
        self.rows
    }

    fn cols(&self) -> usize {
        self.cols
    }

    fn shape(&self) -> (usize, usize) {
        (self.rows, self.cols)
    }

    fn to_vec(&self) -> Vec<Vec<f64>> {
        todo!()
    }

    fn get_row(&self, index: usize) -> &[f64] {
        self.matr[index].as_slice()
    }

    fn new() -> Self {
        LowerTriangularMatrix::default()
    }

    fn from(matrix: Vec<Vec<f64>>) -> Result<Self, String> {
        todo!()
    }
}
