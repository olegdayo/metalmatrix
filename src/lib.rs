extern crate core;
extern crate core;

mod types;
mod general;
mod indexer;
mod tests;

#[cfg(test)]
mod tests_caller {
    use crate::types::{Matrix, BaseMatrix};

    #[test]
    fn check_new() {
        let m1 = BaseMatrix::new();
        let m2 = BaseMatrix::new();
        assert_eq!(m1, m2);
    }
}
