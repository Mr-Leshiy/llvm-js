function foo(arg1, arg2) {
  arg1 = 12;
  var a = 3;
  a = arg2;
}

function bar(a) {
  return a;
}

var a = 4;
var b = 3;

assert_eq(foo(a, b), undefined);
assert_eq(foo(4, true), undefined);
assert_eq(foo("hello", a), undefined);

assert_eq(a, 4);
assert_eq(b, 3);

function foo(arg1, arg2) {
  return 5;
}

assert_eq(foo(a, b), 5);
assert_eq(foo(4, true), 5);
assert_eq(foo("hello", a), 5);

var res = foo(4, 5);
assert_eq(res, 5);

assert_eq(bar(3), 3);
assert_eq(bar(true), true);
assert_eq(bar("Hello"), "Hello");

function foo(arg) {
  return arg;
}
assert_eq(foo(), undefined);
assert_eq(foo(5), 5);

var bar = foo;
assert_eq(bar(), undefined);
assert_eq(bar(5), 5);

