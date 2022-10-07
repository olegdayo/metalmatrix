pub mod matrix {
    use std::ops::Index;

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
                    v[i].push(self[i][j]);
                }
            }

            v
        }

        pub fn new() -> Self {
            Matrix::default()
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

    impl Index<usize> for Matrix {
        type Output = [f64];

        fn index(&self, index: usize) -> &Self::Output {
            &self.matr[index]
        }
    }
}
