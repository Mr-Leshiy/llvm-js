var a = {name: {name: {name: "Alex"}}};

assert_eq(a.name.name.name,  "Alex");

assert_eq(a.age, undefined);

var name = "name";
assert_eq(a[name]["name"].name,  "Alex");