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
        let mut v = Vec::with_capacity(self.rows);
        v.resize(self.rows, Vec::with_capacity(self.cols));

        for i in 0..self.rows {
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
            return Err("Cannot create 0-row matrix".to_string());
        }

        if matrix[0].len() == 0 {
            return Err("Cannot create 0-column matrix".to_string());
        }

        let mut bm = BaseMatrix::new();
        bm.rows = matrix.len();
        bm.cols = matrix[0].len();
        bm.matr = matrix;

        Ok(bm)
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
        let mut v = Vec::with_capacity(self.side);
        v.resize(self.side, Vec::with_capacity(self.side));

        for i in 0..self.side {
            for j in 0..self.side {
                v[i].push(self[i][j]);
            }
        }

        v
    }

    fn get_row(&self, index: usize) -> &[f64] {
        self.matr[index].as_slice()
    }

    fn new() -> Self {
        SquareMatrix::default()
    }

    fn from(matrix: Vec<Vec<f64>>) -> Result<Self, String> {
        if matrix.len() == 0 {
            return Err("Cannot create 0-row matrix".to_string());
        }

        if matrix[0].len() == 0 {
            return Err("Cannot create 0-column matrix".to_string());
        }

        if matrix.len() != matrix[0].len() {
            return Err("Not a square matrix".to_string());
        }

        let mut sm = SquareMatrix::new();
        sm.side = matrix.len();
        sm.matr = matrix;

        Ok(sm)
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
        let mut v = Vec::with_capacity(self.side);
        v.resize(self.side, Vec::with_capacity(self.side));

        for i in 0..self.side {
            v[i].resize(self.side, 0f64);
            v[i][i] = self[i][i];
        }

        v
    }

    fn get_row(&self, index: usize) -> &[f64] {
        let mut v = Vec::with_capacity(self.side);
        v.resize(self.side, 0f64);
        v[index] = self.diag[index];
        v.as_slice()
    }

    fn new() -> Self {
        DiagonalMatrix::default()
    }

    fn from(matrix: Vec<Vec<f64>>) -> Result<Self, String> {
        if matrix.len() == 0 {
            return Err("Cannot create 0-row matrix".to_string());
        }

        if matrix[0].len() == 0 {
            return Err("Cannot create 0-column matrix".to_string());
        }

        if matrix.len() != matrix[0].len() {
            return Err("Not a square matrix".to_string());
        }

        let mut dm = DiagonalMatrix::new();
        dm.side = matrix.len();
        dm.diag = Vec::with_capacity(dm.side);
        for i in 0..dm.side {
            dm.diag.push(matrix[i][i]);
        }

        Ok(dm)
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
