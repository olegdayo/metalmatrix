pub mod matrix {
    #[derive(PartialEq, Debug)]
    pub struct Matrix {
        rows: usize,
        cols: usize,
        matr: Vec<Vec<f64>>,
    }

    impl Matrix {
        pub fn rows(&self) -> usize {
            self.rows
        }

        pub fn cols(&self) -> usize {
            self.cols
        }

        pub fn to_vec(&self) -> Vec<Vec<f64>> {
            let mut v = Vec::new();

            for i in 0..self.rows {
                v.push(Vec::new());
                for j in 0..self.cols {
                    v[i].push(self.matr[i][j]);
                }
            }

            v
        }

        pub fn new() -> Self {
            Matrix{
                rows: 0,
                cols: 0,
                matr: Vec::new(),
            }
        }

        pub fn from(matrix: Vec<Vec<f64>>) -> Result<Self, &'static str> {
            if matrix.len() == 0 {
                return Err("Cannot create 0-row matrix")
            }

            if matrix[0].len() == 0 {
                return Err("Cannot create 0-column matrix")
            }

            let mut m = Matrix::new();
            m.matr = matrix;

            Ok(m)
        }
    }
}

pub mod general {
    use crate::matrix::matrix::Matrix;
}
