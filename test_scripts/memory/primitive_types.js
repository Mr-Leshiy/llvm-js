assert_eq(gb_variables_count(), 0);
{
  var a1;
  var a2 = a1;
  var a3 = undefined;
  var a4 = null;
  var a5 = 1;
  var a6 = NaN;
  var a7 = Infinity;
  var a8 = -Infinity;
  var a9 = "Hello";
  var a10 = { name: "Alex", age: 1 };
  var a11 = [1, 2];
}
assert_eq(gb_variables_count(), 0);
