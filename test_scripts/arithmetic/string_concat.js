assert_eq("Hello " + "world", "Hello world");

assert_eq(2 + " world", "2.000000 world");
assert_eq("Hello " + 2, "Hello 2.000000");

assert_eq(undefined + " world", "undefined world");
assert_eq(false + " world", "false world");
assert_eq(true + " world", "true world");
assert_eq(NaN + " world", "NaN world");
assert_eq(null + " world", "null world");
assert_eq(Infinity + " world", "Infinity world");
assert_eq(-Infinity + " world", "-Infinity world");
