var a;

if (true) {
  a = "hello";
}
assert_eq(a, "hello");

if (false) {
  a = "world";
}
assert_eq(a, "hello");

if (true) {
  a = "world";
} else {
  a = "world!!!!";
}
assert_eq(a, "world");

if (false) {
  a = "world";
} else {
  a = "world!!!!";
}
assert_eq(a, "world!!!!");
