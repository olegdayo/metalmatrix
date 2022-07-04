mod metal;
#[cfg(test)]
mod tests {
    #[test]
    fn f() {
        let x = Matrix{rows_number: 2, columns_number: 3, matr: vec![vec![1, 2, 3], vec![4, 5, 6]]};
        assert_eq!(x.sum(), 21);
    }
}
