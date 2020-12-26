# Exceptions

## fail and otherwise
Any code block can call fail like this:
`fail('This is the reason something bad happened')`

You can catch the failures with an otherwise statement.

The first is inline like this:
```
    let x: int = obj.method_will_fail() otherwise 0
```
This will set the variable `x` to the result of `obj.method_will_fail()` or if it did fail, it will set `x`
to `0`. The otherwise block is also called if `obj` is null.

Warning: long chains of methods where any one of them can return a null is more difficult to debug
when you use otherwise. For example:
```
    // x is 0 if obj1, obj2, or obj3 are null, which may not be your intent, so be careful
    // when chaining together long lists of function calls before an otherwise.
    let x: int = obj1.getObj2().getObj3().getInt() otherwise 0
```
You can also place an otherwise on a state block, like this:
```
fn will_fail() {
  fail("I don't need a reason, but I'll give you one.")
}

  ...
  
  {
    // do some things
    will_fail()
    // do more things
  } otherwise {
    println('We ignored the failure.')
  }
```