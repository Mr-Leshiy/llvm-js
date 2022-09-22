assert_eq(2 * 5, 10);
assert_eq(5 * 2, 10);

assert_eq(NaN * 5, NaN);
assert_eq(5 * NaN, NaN);
assert_eq(Infinity * NaN, NaN);
assert_eq(NaN * Infinity, NaN);

assert_eq(1 * Infinity, Infinity);
assert_eq(Infinity * 1, Infinity);
assert_eq(Infinity * Infinity, Infinity);