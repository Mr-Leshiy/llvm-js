var a = [1, "Hello", false];

assert_eq(a[0], 1);
assert_eq(a[1], "Hello");
assert_eq(a[2], false);

a[0] = 13;

assert_eq(a[0], 13);

assert_eq(a[10], undefined);