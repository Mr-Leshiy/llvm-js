assert_eq(2 >= 0, true);
assert_eq(Infinity >= 2, true);
assert_eq(Infinity >= -2, true);
assert_eq(Infinity >= -Infinity, true);
assert_eq(-Infinity >= -Infinity, true);
assert_eq(Infinity >= Infinity, true);

assert_eq(2 >= NaN, false);
assert_eq(NaN >= 2, false);
assert_eq(NaN >= Infinity, false);
assert_eq(Infinity >= NaN, false);
assert_eq(NaN >= -Infinity, false);
assert_eq(-Infinity >= NaN, false);
assert_eq(NaN >= NaN, false);

assert_eq(-2 >= 2, false);
assert_eq(2 >= Infinity, false);
assert_eq(-2 >= Infinity, false);
assert_eq(-Infinity >= Infinity, false);
assert_eq(-Infinity >= 2, false);
assert_eq(-Infinity >= -2, false);

assert_eq(2 >= -Infinity, true);
assert_eq(-2 >= -Infinity, true);
assert_eq(2 >= true, true);
assert_eq(true >= 0, true);
assert_eq(true >= -1, true);
assert_eq(1 >= null, true);
assert_eq(true >= null, true);
assert_eq("b" >= "a", true);
assert_eq(2 >= 2, true);
assert_eq(true >= true, true);
assert_eq(false >= false, true);
assert_eq("hello" >= "hello", true);
assert_eq(null >= null, true);

assert_eq(2 >= undefined, false);
assert_eq(undefined >= 2, false);
assert_eq(2 >= "hello", false);
assert_eq("hello" >= 2, false);
assert_eq(2 >= {}, false);
assert_eq({} >= 2, false);
assert_eq(2 >= [], false);
assert_eq([] >= 2, false);

assert_eq(true >= undefined, false);
assert_eq(undefined >= true, false);
assert_eq(true >= "hello", false);
assert_eq("hello" >= true, false);
assert_eq(true >= {}, false);
assert_eq({} >= true, false);
assert_eq(true >= [], false);
assert_eq([] >= true, false);

assert_eq(false >= undefined, false);
assert_eq(undefined >= false, false);
assert_eq(false >= "hello", false);
assert_eq("hello" >= false, false);
assert_eq(false >= {}, false);
assert_eq({} >= false, false);
assert_eq(false >= [], false);
assert_eq([] >= false, false);

assert_eq(null >= undefined, false);
assert_eq(undefined >= null, false);
assert_eq(null >= "hello", false);
assert_eq("hello" >= null, false);
assert_eq(null >= {}, false);
assert_eq({} >= null, false);
assert_eq(null >= [], false);
assert_eq([] >= null, false);

assert_eq(undefined >= undefined, false);
assert_eq({} >= {}, false);
assert_eq([] >= [], false);