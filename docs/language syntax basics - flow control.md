# Flow Control

## if and else
The `if` statement is the most fundamental flow control statement.

A simple example that checks if the variable `x` is greater than `10`, and then executes
the code within the curly brackets. If `x` is less than 10, the code is in the curly brackets.
```
    if x > 10 {
        // this code runs if X is greater than 10.
    }
```

We can extend this example to run different code when `x` is 10 or less.
```
    if x > 10 {
        // this code runs if X is greater than 10.
    } else {
        // this code runs if x is 10 or less.
    }
```


## for, in, break, and continue

The ability to repeat code multiple times is essential to most programs. 

Here is an example of a for loop:
```
    // a loop from 0 inclusive through 10 exclusive
    // equivalent to java `for (int x=0; x < 10; x++)`
    for x:int in [0..10] {
        // do something with x
        // the first time through the loop it is 0
        // this block will repeat with x being one bigger until we reach 9
    } 
```

Here's a slight variation:
```
    // a loop from 0 inclusive through 10 inclusive
    // equivalent to java `for (int x=0; x <= 10; x++)`
    for x:int in [0..=10] {
        // do something with x
        // the first time through the loop it is 0
        // this block will repeat with x being one bigger until we reach 10
    } 
```

You can also loop over any Dog collection of objects:
```
    // iterate over all objects in `mystructlist`
    // equivalent to java `for (Mystruct x : mystructlist)`
    for x:MyStruct in mystructlist {
        // do something with each object in mystructlist
    } 
```

## switch and case

Many languages have the equivalent of switch statement, so it may be familiar.

The switch statement will evaluate a variable against multiple options and then runs the block of code
for that option.

Example:
```
    switch x {
        1: {
            // run this code if x is 1
        }
        2: {
            // run this code if x is 2
        }
        default: {
            // run this code if x is not 1 or 2
        }
    }
```

Unlike some languages, code never flows from one option block to another. This prevents difficult to track bugs.
If you need to reuse code, put that code in a function and call it from both blocks.

## with
`with` creates a special block that calls `acquire()` and `release()` on 
structures that implement the `resource` trait.

Useful for files, sockets, mutexes, conditions, and semaphores. 

```
    with file { // file opened
        // read the file
    } otherwise { // file will be closed automatically
        // happy path failed, but we can do something here
    }
```

## do-while emulation

Multiple languages have a do-while loop structure, in Dog, you can achieve something similar using
the `while` loop with a `break` statement.

```
while {
    // at least once, and then repeat while x is less than 10
    if x < 10 {
        break;
    }
} 
```

There are a number of respected sources that dislike the do-while structure because the exit condition
isn't obvious, that said, the above syntax is no better. Arguing about do-while vs while loops is pretty
much like arguing about spaces vs tabs. Everybody has an opinion. We didn't add the `do` keyword, but
the functionality is still possible, so everybody wins?