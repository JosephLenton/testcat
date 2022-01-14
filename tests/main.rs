#[cfg(test)]
mod it_macro {
    use ::testcat::*;

    describe!("assertion examples", {
        it!("should assert true", test_assert_true);
        it!("should assert a is less than b", test_a_and_b);
    });

    test!("5 is less than 9", test_5_and_9);

    fn test_assert_true() {
        assert_eq!(true, true)
    }

    fn test_a_and_b() {
        let a = 3;
        let b = 9;

        assert!(a < b)
    }

    fn test_5_and_9() {
        assert!(5 < 9)
    }
}
