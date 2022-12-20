assert_eq(gb_variables_count(), 0);

var a1;
assert_eq(gb_variables_count(), 1);

var a2 = a1;
assert_eq(gb_variables_count(), 2);

var a3 = undefined;
assert_eq(gb_variables_count(), 3);

var a4 = null;
assert_eq(gb_variables_count(), 4);

var a5 = 1;
assert_eq(gb_variables_count(), 5);

var a6 = NaN;
assert_eq(gb_variables_count(), 6);

var a7 = Infinity;
assert_eq(gb_variables_count(), 7);

var a8 = -Infinity;
assert_eq(gb_variables_count(), 8);

var a9 = "Hello";
assert_eq(gb_variables_count(), 9);

var a10 = { name: "Alex", age: 1, say: a9 };
assert_eq(gb_variables_count(), 12);

var a11 = [1, 2, a10];
assert_eq(gb_variables_count(), 15);