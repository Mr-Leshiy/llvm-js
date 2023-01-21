var a1;
var a2 = a1;
var a3 = undefined;
var a4 = null;
var a5 = 1;
var a6 = NaN;
var a7 = Infinity;
var a8 = -Infinity;
var a9 = "Hello";
var a10 = { name: {name: "Alex", surname: "Smith"}, age: 1, say: a9 };
var a11 = [1, 2, a10];

a10.name = "Alex";
a11[2] = 3;