var a;
assert_eq(a, undefined);

var a = undefined;
assert_eq(a, undefined);

var a = null;
assert_eq(a, null);

var a = NaN;
assert_eq(a, NaN);

var a = Infinity;
assert_eq(a, Infinity);

var a = 0;
assert_eq(a, 0);

var a1 = 5;
assert_eq(a1, 5);

var a1 = 12;
assert_eq(a1, 12);

var a2 = 12;
assert_eq(a1, a2);

var a2 = "Hello world";

var a2 = {name: "Alex", age: 13};

var a3 = true;
assert_eq(a3, true);

var a4 = false;
assert_eq(a4, false);

a3 = 6.421;
assert_eq(a3, 6.421);

a3 = a3;
assert_eq(a3, 6.421);

a3 = a1;
assert_eq(a3, a1);

var a5 = a4;
assert_eq(a5, a4);

a4 = 0;
assert_eq(a4, 0);
assert_eq(a5, false);

a5 = 0;
assert_eq(a5, 0);

var _a_1 = true;
assert_eq(_a_1, true);
