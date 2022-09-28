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

function foo(a) {
  if(a) {
    if(a || true) {
      if(!a) {
        return "here 1";
      } else {
        return "here 2";
      }
      return "here 3";
    } else {
      return "here 4";
    }
  } else {
    return "here 5";
  }
  return "does not reach";
}
assert_eq(foo(true), "here 2");
assert_eq(foo(false), "here 5");

