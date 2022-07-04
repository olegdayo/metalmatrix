pub mod constructors {
    impl metalmatrix::Matrix<T> {
        fn sum(&mut self) -> T {
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
