#[cfg(test)]
mod it_macro {
    use ::testcat::*;

    it!("example should assert true", test_assert_true);
    test!("example a is less than b", test_a_and_b);

    fn test_assert_true() {
        assert_eq!(true, true)
    }

    fn test_a_and_b() {
        let a = 3;
        let b = 9;

        assert!(a < b)
    }
}
