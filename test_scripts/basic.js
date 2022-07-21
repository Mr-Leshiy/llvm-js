function foo(arg1, arg2) {
    arg1 = 12;
}

{
    var a = 5;
    var b = 6;
    foo(a, b);
    {
        a = b;
        b = 7;
        var c = "hello";
    }
    foo(a, b);
}
