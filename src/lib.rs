mod matrix;
mod types;
mod general;
mod indexer;

#[cfg(test)]
mod tests {
    use crate::types;

    #[test]
    fn check_new() {
        let m1 = types::Matrix::new();
        let m2 = types::Matrix::new();
        assert_eq!(m1, m2);
    }
}
