var a = true;
assert_eq(!a, false);
assert_eq(!!a, true);
assert_eq(!!!a, false);
assert_eq(!!!!a, true);
assert_eq(!!!!!a, false);

var a = false;
assert_eq(!a, true);
assert_eq(!!a, false);
assert_eq(!!!a, true);
assert_eq(!!!!a, false);
assert_eq(!!!!!a, true);

var a = 5;
assert_eq(!a, false);
assert_eq(!!a, true);
assert_eq(!!!a, false);
assert_eq(!!!!a, true);
assert_eq(!!!!!a, false);

var a = -5;
assert_eq(!a, false);
assert_eq(!!a, true);
assert_eq(!!!a, false);
assert_eq(!!!!a, true);
assert_eq(!!!!!a, false);

var a = 0;
assert_eq(!a, true);
assert_eq(!!a, false);
assert_eq(!!!a, true);
assert_eq(!!!!a, false);
assert_eq(!!!!!a, true);