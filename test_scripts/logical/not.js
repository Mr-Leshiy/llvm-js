var a = undefined;
assert_eq(!a, true);
assert_eq(!!a, false);
assert_eq(!!!a, true);
assert_eq(!!!!a, false);
assert_eq(!!!!!a, true);

var a = null;
assert_eq(!a, true);
assert_eq(!!a, false);
assert_eq(!!!a, true);
assert_eq(!!!!a, false);
assert_eq(!!!!!a, true);

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

var a = 0;
assert_eq(!a, true);
assert_eq(!!a, false);
assert_eq(!!!a, true);
assert_eq(!!!!a, false);
assert_eq(!!!!!a, true);

var a = "Hello world";
assert_eq(!a, false);
assert_eq(!!a, true);
assert_eq(!!!a, false);
assert_eq(!!!!a, true);
assert_eq(!!!!!a, false);

var a = "";
assert_eq(!a, true);
assert_eq(!!a, false);
assert_eq(!!!a, true);
assert_eq(!!!!a, false);
assert_eq(!!!!!a, true);
