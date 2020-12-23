
# Code Blocks and Scope


## scope

# Queues and Streams

# Advanced Config

```
NOTICE: Most of the stuff in this section is exceptionally tentative and may change dramatically as
implementation and reality collide with early thoughts.

```

## Disabling linking of default libraries
When compiling `internal`, we can't link `std` because it won't exist yet. It would create a circular
dependency.
Also, when we compile `std`, we don't want to link to a different external version of ourself.

## Disabling default use statements
Whe compiling the `internal` library, we can't automatically `use std`. There may be other use cases 
for this.

# Concurrency 

```
NOTE: Most of this section is a work in progress!!
```

```
Note: 
need to decide on details for
* mutexes
* semaphores
* conditional variables
```

## threadlocal



## future and !

## once (singletons)

This might be function like this if a trait is used:
```
static one_time = Once()

fn my_func(): MyTrait {
    return one_time.call(() -> {
        return MyTrait {
            prop1: "I'm a singleton"
            prop2: 42
        }
    })
}
```

A keyword would be handy because it could let you do things like this:
```
// by declaring "once" this function's result is cached for the given parameters (value:int)
once fn my_func(value: int): MyTrait {
    return MyTrait {
        prop1: "I'm a singleton"
        prop2: value
    }
}
```

## synchronized blocks
Two parameters:
* target
* timeout
  
Example with an otherwise block:
```
    // attempt to exclusively access X, but give up after 1,000 miliseconds
    syncrhonized (x, 1000) {
        // only one thread at a time in this block
    } otherwise {
        // this block only happens if there is an exception or timeout
    }
```
## synchronized variable, function, or trait
An entire mutable variable, function, or trait can be synchronized, which means only one thread can 
access it at a time.

If the function is a member of a trait, then the function is synchronized for
the exact instance of the structure it is used with.


## throttled
Throttled limits the maximum number of threads that can access a block. Setting the throttle 
to 1 is identical to synchronized.
```
    // first 10 threads can access X, but this thread gives up after 1,000 miliseconds
    throttled (x, 10, 1000) {
    } otherwise {
    }
```
## wait notify

Thread 1:
```
    // wait on notification to x
    wait(x, 1000) {
    } otherwise {
    } 
```
Thread 2:
```
    notify(x) // notify all threads waiting on x 
```
Thread 3:
```
    notify(x) // notify up to 2 threads waiting on x 
```


# Attributes
Attributes are similar to Java Annotations or Rust Attributes.

The purpose is to provide metadata about entities that can be acted upon at compile time,
but that action may add code to the program that happens at runtime.

An attribute might give hints to the compiler about how to handle a situation or it may 
trigger code to run before and after the creation of a structure, the call of a function, 
or it may generate trait implementations based on a template.

In some ways, Dog attributes help fulfill the need for C/C++ preprocessor instructions and macros.

## attribute

```
#my_attribute
```

# unsafe
## functions
```
fn unsafe myfunc(): int {
    ... call unsafe code
}
```
## blocks
```
unsafe {
    ... call unsafe function()
}
```

# extern and libraries

Native libraries may have different calling conventions.

Options include:
* system: stdcall on win32, cdecl for everything else
* cdecl: c calling convention (caller clears the stack)
* stdcall: microsoft win32 api calling convention (callee clears the stack)
* win64: microsoft win64 default C calling convention
* sysv64: non-windows default C calling convention
* aapcs: the default for ARM
* fastcall: MSVC `__fastcall` and gcc/clang `__attribute__((fastcall))`
* vectorcall: MSVC `vectorcall` and clang `__attribute__((vectorcall))`


Dog assumes "cdecl" if a calling convention isn't specified.

## static
In config, example 1:
```
config MyConfig {
    static_native_libraries: [
        does_something: {
            lib_name: "something",
            functions: {
                "system" lib_func1(text:string):int,
                lib_func2(num1:int, num2:int):string
            }
        }
    ]
}    
```
`lib_name` refers to a library name and will receive the appropriate extension for the compile target. 
Standard paths will be checked for the library. (Common extensions are `.a`, `.lib` for static libraries.) 
and `.dll`, `.so`, `.dylyb` for dynamic libraries.)

Using it in code:
```
    ...
    let result1: int = does_something::lib_fun1("hello library")
    let resutl2: string = does_something::lib_fun2(1, 2)
    ...
```

While not recommended, you can explicitly specify the path of a library like this:
```
config MyConfig {
    static_native_libraries: [
        does_something: {
            path: {
                windows: "something.lib",
                linux: "something.a",
                macox: "something.a"
            },
            functions: {
                "system" lib_func1(text:string):int,
                lib_func2(num1:int, num2:int):string
            }
        }
    ]
}    
```

If a path isn't specified for an operating system, then it will not be linked.

### default libraries

`std` is statically linked by default.

## Dynamic
In config, example 1:
```
config MyConfig {
    dynamic_native_libraries: [
        dyn_does_something: {
            lib_name: "something",
            functions: {
                "system" lib_func1(text:string):int,
                lib_func2(num1:int, num2:int):string
            }
        }
    ]
}    
```
`lib_name` refers to a library name and will receive the appropriate extension for the compile target.
Standard paths will be checked for the library. (Common extensions are `.dll`, `.so`, `.dylyb` for dynamic libraries.)

While not recommended, you can, just like with static libraries, specify the path explicitly. 
```
config MyConfig {
    dynamic_native_libraries: [
        dyn_does_something: {
            path: {
                windows: "something.dll"
                linux: "something.so"
                macox: "something.dylib"
            }
            functions: {
                "system" lib_func1(text:string):int
                lib_func2(num1:int, num2:int):string
            }
        }
    ]
}    
```

Using it in code:
```
    ...
    dyn_does_something::load() // if you don't load first, the functions below will fail.
    let result1: int = dyn_does_something::lib_fun1("hello library")
    let resutl2: string = dyn_does_something::lib_fun2(1, 2)
    ...
```

# Config specific Modules

The first line of any dog file can have a `mod` line that declares the module it is a member of.

On this line, you can specify to only compile that file for specific configuration values, which is useful
if you need something operating system specific.

Example:
```
mod my_mod_name (os:"windows")
```

In general, it is advised to only use this for situations that are absolutely necessary, or your code 
may not work for certain configuration values.