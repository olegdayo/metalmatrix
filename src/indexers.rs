mod matrices {
    use std::ops::Index;
    use crate::matrix_types::{BaseMatrix, DiagonalMatrix, SquareMatrix};
    use crate::vector_types::{Vector};

    impl Index<usize> for BaseMatrix {
        type Output = dyn Vector<Output=f64>;

        fn index(&self, index: usize) -> &Self::Output {
            &self.matr[index]
        }
    }

    impl Index<usize> for SquareMatrix {
        type Output = dyn Vector<Output=f64>;

        fn index(&self, index: usize) -> &Self::Output {
            &self.matr[index]
        }
    }

    impl Index<usize> for DiagonalMatrix {
        type Output = dyn Vector<Output=f64>;

        fn index(&self, index: usize) -> &Self::Output {
            &self.diag[index]
        }
    }
}

mod vectors {
    use std::ops::Index;
    use crate::vector_types::{FullVector, CompressedVector};

    static ZERO: f64 = 0f64;

    impl Index<usize> for FullVector {
        type Output = f64;

        fn index(&self, index: usize) -> &Self::Output {
            &self.row[index]
        }
    }

    impl Index<usize> for CompressedVector {
        type Output = f64;

        fn index(&self, index: usize) -> &Self::Output {
            if self.index == index {
                return &self.val
            }
            &ZERO
        }
    }
}
