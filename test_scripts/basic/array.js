var a = [1, "Hello", false];
assert_eq(a.length, 3);
assert_eq(a[0], 1);
assert_eq(a[1], "Hello");
assert_eq(a[2], false);

a[0] = 13;
assert_eq(a[0], 13);

a[4] = 14;
assert_eq(a[4], 14);
assert_eq(a.length, 5);

assert_eq(a[10], undefined);

a.name = "John";
assert_eq(a.name, "John");