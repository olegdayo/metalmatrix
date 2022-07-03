pub mod metal {
    pub struct Matrix<T> {
        rows_number: usize,
        columns_number: usize,
        matr: Vec<Vec<T>>,
    }
    impl Matrix<T> {
        fn sum(&self) -> T {
            let mut ans: T = 0;
            for i in 0..self.rows_number {
                for j in 0..self.columns_number {
                    ans += self.matr[i][j];
                }
            }
            return ans;
        }
    }
}
