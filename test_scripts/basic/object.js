var a = {name: {name: {name: "Alex"}}};

assert_eq(a.name.name.name,  "Alex");

assert_eq(a.age, undefined);

var name = "name";
assert_eq(a[name]["name"].name,  "Alex");

a.name = 10;
assert_eq(a.name, 10);

a.age = 10
assert_eq(a.age, 10);

var b = a.some;
assert_eq(b, undefined);
b = 5;
assert_eq(a.some, undefined);