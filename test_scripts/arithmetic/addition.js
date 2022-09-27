assert_eq(1 + 4, 5);
assert_eq(4 + 1, 5);
assert_eq(-4 + 1, -3);
assert_eq(1 + -4, -3);

assert_eq(1 + NaN, NaN);
assert_eq(NaN + 1, NaN);
assert_eq(NaN + Infinity, NaN);
assert_eq(Infinity + NaN, NaN);
assert_eq(NaN + -Infinity, NaN);
assert_eq(-Infinity + NaN, NaN);

assert_eq(Infinity + 1, Infinity);
assert_eq(1 + Infinity, Infinity);
assert_eq(1 + -Infinity, -Infinity);
assert_eq(-Infinity + 1, -Infinity);
assert_eq(Infinity + -Infinity, NaN);
assert_eq(-Infinity + Infinity, NaN);

assert_eq(Infinity + Infinity, Infinity);
assert_eq(-Infinity + -Infinity, -Infinity);
