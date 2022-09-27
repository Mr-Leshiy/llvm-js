var a = true || false;
var a = true || false;
var a = true || false;

assert_eq(true || true, true);
assert_eq(true || false, true);
assert_eq(false || true, true);
assert_eq(false || false, false);

assert_eq("" || "foo", "foo");
assert_eq(2 || 0, 2);
assert_eq("foo" || 4, "foo");
assert_eq(0 || true, true);
assert_eq(0 || false, false);
assert_eq(false || 0, 0);
assert_eq(undefined || 0, 0);
assert_eq(null || 0, 0);
assert_eq(false || undefined, undefined);
assert_eq(false || null, null);
assert_eq(true || "foo", true);

assert_eq(true || 1 || "foo", true);
assert_eq(0 || false || "foo", "foo");
