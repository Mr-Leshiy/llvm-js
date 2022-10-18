let a;
assert_eq(a, undefined);

let a = undefined;
assert_eq(a, undefined);

let a = null;
assert_eq(a, null);

let a = NaN;
assert_eq(a, NaN);

let a = Infinity;
assert_eq(a, Infinity);

let a = 0;
assert_eq(a, 0);

let a1 = 5;
assert_eq(a1, 5);

let a1 = 12;
assert_eq(a1, 12);

let a2 = 12;
assert_eq(a1, a2);

let a2 = "Hello world";

var a2 = {name: "Alex", age: 13};

let a3 = true;
assert_eq(a3, true);

let a4 = false;
assert_eq(a4, false);

a3 = 6.421;
assert_eq(a3, 6.421);

a3 = a3;
assert_eq(a3, 6.421);

a3 = a1;
assert_eq(a3, a1);

let a5 = a4;
assert_eq(a5, a4);

a4 = 0;
assert_eq(a4, 0);
assert_eq(a5, false);

a5 = 0;
assert_eq(a5, 0);

let _a_1 = true;
assert_eq(_a_1, true);
