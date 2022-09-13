assert_eq(3 === 4 , false);
assert_eq(3 === 3 , true);

assert_eq(true === false , false);
assert_eq(true === true , true);

assert_eq("hello" === "hello" , true);
assert_eq("hello" === "hell" , false);

assert_eq(undefined === undefined , true);
assert_eq(null === null , true);

assert_eq(3 === true , false);
assert_eq(3 === undefined , false);
assert_eq(3 === null , false);
assert_eq(undefined === null , false);
assert_eq("hello" === 3 , false);
assert_eq("hello" === false , false);
assert_eq("hello" === true , false);
