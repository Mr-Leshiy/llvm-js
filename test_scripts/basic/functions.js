function foo(arg1, arg2) {
    arg1 = 12;
    var a = 3;
    a = arg2;
}

var a = 4;
var b = 3;

foo(a, b);
foo(4, true);
foo("hello", a);