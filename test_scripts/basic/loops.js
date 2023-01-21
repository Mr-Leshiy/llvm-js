var a = 0;
while(a != 10) {
    a = a + 1;
}
assert_eq(a, 10);

var a = 0;
while(false) {
    a = a + 1;
}
assert_eq(a, 0);

function foo() {
    var a = 0;
    while(a != 10) {
        a = a + 1;
    }
    return a;
}
assert_eq(foo(), 10);

function foo() {
    var a = 0;
    while(a != 10) {
        a = a + 1;
        return a;
    }
    return a;
}
assert_eq(foo(), 1);

var a = 0;
do {
    a = a + 1;
} while(a != 10);
assert_eq(a, 10);

var a = 0;
do {
    a = a + 1;
} while(false)
assert_eq(a, 1);

function foo() {
    var a = 0;
    do {
        a = a + 1;
    } while(a != 10)
    return a;
}
assert_eq(foo(), 10);

function foo() {
    var a = 0;
    do {
        a = a + 1;
        return a;
    } while(a != 10);
    return a;
}
assert_eq(foo(), 1);
