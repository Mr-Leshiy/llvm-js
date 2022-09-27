assert_eq(10 / 2, 5);
assert_eq(-10 / 2, -5);
assert_eq(-10 / -2, 5);
assert_eq(0 / 2, 0);

assert_eq(NaN / 2, NaN);
assert_eq(1 / NaN, NaN);
assert_eq(NaN / Infinity, NaN);
assert_eq(Infinity / NaN, NaN);

assert_eq(Infinity / 1, Infinity);
assert_eq(Infinity / -1, -Infinity);
assert_eq(-Infinity / 1, -Infinity);
assert_eq(-Infinity / -1, Infinity);
assert_eq(Infinity / 0, Infinity);

assert_eq(Infinity / Infinity, NaN);
assert_eq(1 / 0, Infinity);
assert_eq(-1 / 0, -Infinity);
assert_eq(0 / 0, NaN);
