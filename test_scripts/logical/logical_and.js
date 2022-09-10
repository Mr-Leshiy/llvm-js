assert_eq(true && true, true);
assert_eq(true && false, false);
assert_eq(false && true, false);
assert_eq(false && false, false);

assert_eq("" && "foo", "");
assert_eq(2 && 0, 0);
assert_eq("foo" && 4, 4);
assert_eq(0 && true, 0);
assert_eq(0 && false, 0);
assert_eq(false && 0, false);
assert_eq(true && "foo", "foo");

assert_eq(true && 1 && "foo", "foo");
assert_eq(0 && false && "", 0);
