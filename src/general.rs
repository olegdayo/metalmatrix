use crate::matrix_types::{Matrix, BaseMatrix, DiagonalMatrix, SquareMatrix};
use crate::vector_types::{CompressedVector, FullVector};

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
        for i in 0..matrix.len() {
            if matrix[i].len() != bm.cols {
                return Err("Matrix rows are not the same length".to_string());
            }

            bm.matr.push(
                FullVector{
                    index: i,
                    row: matrix[i].clone(),
                    len: bm.cols,
                }
            );
        }

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
        for i in 0..matrix.len() {
            sm.matr.push(
                FullVector{
                    index: i,
                    row: matrix[i].clone(),
                    len: sm.side,
                }
            );
        }

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
            dm.diag.push(
                CompressedVector{
                    index: i,
                    val: matrix[i][i],
                    len: dm.side,
                }
            );
        }

        Ok(dm)
    }
}
