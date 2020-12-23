# tuple

A tuple is a specialized struct with unnamed members.

```
    let x: (i32, i8) = (100, 5) // anonymous tuple holding 100 and 5
    
    println("{} is the first number.", x.0) // prints out 100
    println("{} is the second number.", x.1) // prints out 5
```

You can create a named tuple like this:

```
    struct MyTuple(i32, i8)
    
    ...
    
    let x: MyTuple = (100, 5)
    println("{} is the first number.", x.0) // prints out 100
    println("{} is the second number.", x.1) // prints out 5
    return x
```



# enum

A simple enum looks like this:
```
enum 
```

# struct, trait, and impl
primitives need to be supported by traits.

## constructors

```  
    //given a trait like this  
    trait MyTrait {
        
    }
    
    impl MyTrait on MyStruct {
    }
```

## self
## pub
## mut

# generics
NOTE: this seems simple, but I'm worried about runtime considerations. A generic object T has
no operators on it that make it useful. This is fine for containers, like lists, sets, and futures.
What about maps and sets? You'd need a trait on them that let you get a hash value. Well,
if you have a trait on it the value of the generic goes down as you could just specify that
trait name. That said, an object can have many traits, so it could have the behavior needed
for the container plus other useful traits. maybe a syntax like `<X:Hashable,Comparable>` when
you want multiple traits.

```
struct MyStruct<T> {
    x: T
    y: T
}
```

```
fn my_func<T>(val:T):T {
    return val
}
```
