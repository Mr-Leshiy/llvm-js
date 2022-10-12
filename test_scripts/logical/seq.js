assert_eq(3 === 4, false);
assert_eq(3 === 3, true);

assert_eq(true === false, false);
assert_eq(true === true, true);

assert_eq("hello" === "hello", true);
assert_eq("hello" === "hell", false);

assert_eq({name: "Alex", age: 27} === {name: "Alex", age: 27}, false);
assert_eq({name: "Alex", age: 27} === {name: "alex", age_: 27}, false);
// var a = {name: "Alex", age: 27};
// assert_eq(a === a, true);

assert_eq(undefined === undefined, true);
assert_eq(null === null, true);
assert_eq(NaN === NaN, true);
assert_eq(Infinity === Infinity, true);

assert_eq(3 === true, false);
assert_eq(3 === undefined, false);
assert_eq(3 === null, false);
assert_eq(3 === NaN, false);
assert_eq(3 === Infinity, false);
assert_eq(undefined === null, false);
assert_eq("hello" === 3, false);
assert_eq("hello" === false, false);
assert_eq("hello" === true, false);
