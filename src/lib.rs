#[cfg(test)]
mod tests {
    #[test]
    fn f() {
        let expected: i32 = 4;
        let got: i32 = 2 + 2;
        assert_eq!(expected, got)
    }
}
