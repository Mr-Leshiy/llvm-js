assert_eq(3 !== 4, true);
assert_eq(3 !== 3, false);

assert_eq(true !== false, true);
assert_eq(true !== true, false);

assert_eq("hello" !== "hello", false);
assert_eq("hello" !== "hell", true);

assert_eq(undefined !== undefined, false);
assert_eq(null !== null, false);
assert_eq(NaN !== NaN, false);
assert_eq(Infinity !== Infinity, false);

assert_eq(3 !== true, true);
assert_eq(3 !== undefined, true);
assert_eq(3 !== null, true);
assert_eq(3 !== NaN, true);
assert_eq(3 !== Infinity, true);
assert_eq(undefined !== null, true);
assert_eq("hello" !== 3, true);
assert_eq("hello" !== false, true);
assert_eq(true !== "hello", true);
