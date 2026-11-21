fn factorial(n) {
    if n <= 0 {
        return 1;
    }
    return n * factorial(n-1);
}
assert_eq(factorial(0), 1);
assert_eq(factorial(1), 1);
assert_eq(factorial(8), 40320);
