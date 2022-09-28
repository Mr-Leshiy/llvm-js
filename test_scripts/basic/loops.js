var a = 0;
while(a !== 10) {
    a = a + 1;
}
assert_eq(a, 10);

function foo() {
    var a = 0;
    while(a !== 10) {
        a = a + 1;
    }
    return a;
}
assert_eq(foo(), 10);

function foo() {
    var a = 0;
    while(a !== 10) {
        a = a + 1;
        return a;
    }
    return a;
}
assert_eq(foo(), 1);
