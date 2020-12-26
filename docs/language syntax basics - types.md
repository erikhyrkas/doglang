# Types

## str

A string is an immutable array of characters with the equivalent of a `trait` to make it friendly
to use.

## uint
`uint` is an alias for the system `u32` on a 32-bit processor and `u64` on a 64-bit processor.

You can explicitly specify the size in bits like this: `u8`, `u16`, `u32`, and `u64`. 

## int
`int` is an alias for the system `i32` on a 32-bit processor and `i64` on a 64-bit processor.

You can explicitly specify the size in bits like this: `i8`, `i16`, `i32`, and `i64`.

## float
`float` is an alias for the system `f32` on a 32-bit processor and `f64` on a 64-bit processor.

You can explicitly specify the size in bits like this: `f32` and `f64`.

## bool
A boolean type that can have a value of `true` or `false`.

## char
`char` represents a multi-byte utf8 character and is equivalent to a `trait` on a u32

## Immutable and mut

## Ref
Primitives must have a value, but you can treat them as nullable by using `ref`.

```
    let x: mut ref int = null // must be a mutable reference to be re-assignable
    if x == null {
        x = 0         
    }
    // x will be equal to 0 here.
```

Dog will automatically dereference x, but you can also test it against null.

## casting

```
    let y: float = 1.2
    let x: int = y as int // explicit 
    let z: float = x as float // explicit
    
    let u: f32 = 1.4
    let v: f64 = u  // gaining precision, fine
    let q: f32 = v as f32 // losing precision, must be explicit
```

Cast otherwise is a syntax that allows you to handle the situation where a variable cannot be cast,
so you want a default.
```    
    let my_string: "I'm not a number"
    let my_num: 42
    let result = num_num + my_string as int otherwise 0
    println('Result is: {}`, result) // 42 because 42 + 0  
```

## Common Collections

Common collections
* Array
* List
* Map
* Set

### Arrays
Declared like this:
```
    let x:[char] = ['1', '2', '3'] // declares an initialized array of 3 elements
    let x:[char; 32] // declares an empty array of length 32
```

The size of an array is immutable, unlike a list.

### Lists
Declared like this:
```
    let x:List<MyTrait> = List<MyTrait>::from([
        x: val1,
        y: val2
    ])
```
or this:
```
    let x:List<MyTrait> = List<MyTrait>()
    x.add(MyTrait {
        prop1: 'my property value',
        prop2: 1000
    })
```

# Semi-structured data

A `Semistructured` trait exists that sits over some common types used in JSON and Yaml.

* i64
* f64
* bool
* string
* list
* map

The trait's methods allow for the easy conversion back and forth between common semi-structured
text representations while being backed by concrete binary types.