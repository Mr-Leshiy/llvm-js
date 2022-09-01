var a = true;

assert_eq(a, true);
assert_eq(!a, false);
assert_eq(!!a, true);
assert_eq(!!!a, false);
assert_eq(!!!!a, true);
assert_eq(!!!!!a, false);

var a = false;
assert_eq(a, false);
assert_eq(!a, true);
assert_eq(!!a, false);
assert_eq(!!!a, true);
assert_eq(!!!!a, false);
assert_eq(!!!!!a, true);