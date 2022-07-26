{
  var a = 5;
  var b = 6;
  {
    a = b;
    b = 7;
    var c = "hello";
    {
      b = true;
      c = a;
    }
  }
}
