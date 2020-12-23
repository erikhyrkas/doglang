# Dog

While the initial implementation of Dog is written in Rust, the `dog` folder holds the implementation of Dog is
written in Dog. It might take a long time before Dog is fully self-compiling, but this is the optimistic plan 
to support it.

The `std` folder is needed even before Dog self compiles, as it represents the standard library of functions,
structures, enums, and traits provide high-level support to users rather than interacting with their system's
libc by hand. A special flag in the config tells the compiler to not automatically link `std` as we are in the
process of building it. 

```
THOUGHT: Can we simply not auto-link a library if the name is the same as our current
project? This may be complex, especially when versions are taken into account. Also,
there are litterally only two use cases where this needs to be done.
```

The `internal` folder holds a library of the lowest level interactions with the OS and CPU and building blocks 
for the `std` library. A special flag in the config tells the compiler to not include any feature
that relies on `std`, since this library is built before `std` exists, and to not link the `internal` library
since that is the library we are building.

So, `internal` is compiled first, then `std`, and finally `dog`. All projects written in dog, even if compiled
with the Rust version of the compiler, still need `internal` and `std`, but they are included automatically.