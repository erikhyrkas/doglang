# Getting Started

## Overview

The fundamental constructs you'd expect with any language are part of Dog, streamlined for ease of use and to minimize
the syntax one has to learn to be effective.

Creating a simple application that prints out `Hello World!`.

```
app HelloWorld() {
  println("Hello world!") 
}
``` 

`println` is a default function in the `std` library that prints text to the console.

Let's create and use our own function. When run, this program prints out `I did it!`

```
fn saySomething(something: string) {
    println(something)
}

app HelloWorld() {
  saySomething("I did it!") 
}
```

We'll do one more example with variables, loops, and conditional logic.

```
fn requestValue(): string {
    println("Type a number and hit enter:")
    return readln()
}

fn compute(values: [int]): int {
    let result: mut int
    for value: values {
        result += value
    }
    return result
}

app Compute() {
    let value1: string = requestValue()
    let value2: string = requestValue()
    let result: int = compute( (int or 0) value1, (int or 0) value2 )
    println("Your result is {}", result)
    if result == 42 {
        println("You win!")
    }
}
```