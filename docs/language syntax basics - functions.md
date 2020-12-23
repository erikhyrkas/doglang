# Functions
Dog is a pass-by-value language, so when you call a method, it copies the original value to
the stack for a function to use. That is not to say that you cannot pass by reference, but you 
must explicitly label arguments when they are by-reference with the `ref` qualifier. 

## Simple example
```
// simple function that adds two numbers and returns the result
fn my_add(a:int, b:int): int {
    return a + b
}
```

## Parameters
Parameters are not reassignable, but they can be mutable.

```
// making this structure and properties public to avoid confusion
pub struct MyStruct {
    pub prop1: mut int  // mutable
    pub prop2: int      // read-only
}

impl MyStruct {
    fn add_one() {
        prop1++
    }
    fn get_sum(): int {
        return prop1 + prop2
    }
}

fn my_func1(a: int, b: MyStruct, c: mut MyStruct) {
    a = 10 // fail: this is worng and will not compile
    let a: int = 10 // fail: shadowing a parameter is not allowed 
    
    b.prop1 = 10 // fail: b is not mutable, so you can't change it
    b.prop2 = 10 // fail: b and prop2 are not mutable, so you can't change them
    b.add_one() // fail: b is not mutable and the compiler knows that this function could change it
    let b_val: int = b.get_sum() // success: nothing was modified
    let b_prop1: int = b.prop1  // success: nothing was modified
    let b_prop2: int = b.prop2  // success: nothing was modified
    change_my_struct( b ) // fail: b is not mutable and the compiler knows this function could change it
    
    c.prop1 = 10 // success: c and prop1 are mutable
    c.prop2 = 10 // fail: prop2 is not mutable
    c.add_one() // success: c and prop1 are mutable
    let c_val: int = c.get_sum() // success: nothing was modified
    let c_prop1: int = c.prop1  // success: nothing was modified
    let c_prop2: int = c.prop2  // success: nothing was modified
    change_my_struct( c ) // success: c is mutable
    
    let d: mut MyStruct = MyStruct {
        prop1 = 12 // success: we can always set a property during construction
        prop2 = 15 // success: we can always set a property during construction
    }
    
    d.prop1 = 10 // success: d and prop1 are mutable
    d.prop2 = 10 // fail: prop2 is not mutable
    d.add_one() // success: d and prop1 are mutable
    let c_val: int = d.get_sum() // success: nothing was modified
    let c_prop1: int = d.prop1  // success: nothing was modified
    let c_prop2: int = d.prop2  // success: nothing was modified
    change_my_struct( c ) // success: d is mutable
    
    
}

fn change_my_struct(x: mut MyStruct) {
    x.prop1 = 15 // success: x and prop1 are mutable
    // x.prop2 = 25 // would fail: prop2 is not mutable
    x.add_one() // success: prop1 and x are mutable
}


```


## pub
A function is not visible outside of the current file unless you declare it to be public.
```
pub fn my_add(a:int, b:int): int {
    return a + b
}
```

## return

Functions can return a value as multiple examples have shown:
```
pub fn return_a_value(): int { // notice the return type in the function definition is int
    return 3 // returns the int 3
}
```
Or you can return nothing:
```
pub fn return_no_value() { // notice no return type in the function definition
    // do something here    
}
```

Or you can return multiple values in a tuple:
```
pub fn return_a_value(): (int, float) { // notice the return types in the function definition is a tuple
    return (3, 1.2) // returns the int 3 and the float 1.2 in a tuple
}
```


## lambdas (named and unnamed functions as parameters)

Example of a lambda as a variable:
```
fn call_lambda(my_callback: |int, float| -> float) {
    return fn(5, 1.2);
}

fn my_func() {
    // explict function
    fn my_lambda(x:int, y:float): float {
        return x+y 
    }
    let result1: float = call_lambda(my_lambda) // result1 = 6.2
    
    // verbose lambda
    let result2: float = call_lambda(|i, j| -> {
        return i*j
    }) // result2 = 6 
    
    // simple lambda without curlies or return
    let result3: float = call_lambda(|i, j| -> i-j) // result2 = 4.8 
    
}
```

