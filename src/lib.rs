mod matrix;
mod diagonal_matrix;

#[cfg(test)]
mod tests {
    use crate::matrix::matrix;
    use crate::diagonal_matrix::diagonal_matrix;

    #[test]
    fn it_works() {
        let m1 = matrix::Matrix::new();
        let m2 = matrix::Matrix::new();
        assert_eq!(m1, m2);
    }
}
