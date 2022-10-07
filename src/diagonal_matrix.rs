pub mod diagonal_matrix {
    #[derive(PartialEq, Debug)]
    pub struct DiagonalMatrix {
        side: usize,
        diag: Vec<f64>,
    }

    impl DiagonalMatrix {
        pub fn new() -> Self {
            DiagonalMatrix{
                side: 0,
                diag: Vec::new(),
            }
        }
    }
}
