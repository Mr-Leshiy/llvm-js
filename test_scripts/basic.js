function foo(arg1, arg2) {}

{
    var a = 5;
    var b = 6;
    {
        a = b;
        b = 7;
        var c = "hello";
    }
    foo(a, b);
}
