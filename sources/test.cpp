var aaa = 1.0;
var bbb = true;
var ccc = "aaa";
{
  aaa = aaa + 1.0;
  aaa = aaa * 2.0;
  print aaa;

  
  var bbb = false;
  {
    var bbb = 10.0;
    print bbb;
  }
  print bbb;

  print ccc == "aaa";
  print ccc;
}
print bbb;

{
  var aaa = aaa * 2.0;
  print aaa;
  
}
