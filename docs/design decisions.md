# Design Decisions

Every design decision needs to back the goals of: safe, easy, fast, and flexible. Roughly in that order.

Safe is non-negotiable, as a language used for working with large amounts of data. You have
to trust that it will be stable and accurate. As a language, safe also means that it makes engineer errors
less likely.

Easy and fast are trickier. Sometimes we need to lean one way or another, and it's about walking the line.

The word "easy" and "fast" are both traps. Easy can mean that it takes less code or less learning to do something.
Fast can mean that it takes less time to write the application or that the resulting code does its job fast. To
all of these statements, we say: yes, let's do that.

If you want the ultimate in fast and flexible, you could already implement literally anything in C if you don't mind 
overcoming difficult problems with metric tons of code.

If you want the ultimate in easy, there are tools with user interfaces that manipulate data, but they are not
fast or flexible.

Dog's niche is in being faster than Python and Scala and roughly as easy as Python and Scala.

Wait! Why not be even easier?

Well, first of all, "easy" is subjective. All languages can be intimidating to people who aren't familiar with them.
Familiarity makes things easier to learn, but it also means that by offering constructs like SQL and DataFrames, it
isn't "easier", it is the same. To be easier, we need novel constructs that aren't in other languages that take less
code to do common tasks and are also easy to learn and intuitive.

You could argue that making SQL compile-time checked is novel, but it's more convenient than it is anything else, since
both Scala and Python can run SQL against files if they use Spark.

If we did come up with a novel construct that was easy, that construct also has to be fast and flexible.

Flexibility gives us the power to do what we might not be able to do in a tool, but it eats away at our easy goal. So,
we need to balance that flexibility and give it where we can, because that makes our language useful.

Okay, so what if we did come up with that novel construct? Then great! But people will still start using Dog with
constructs that they are already familiar with and only use something novel after they have experience. If the
construct was good enough, you'd likely see it also ported to other languages unless it was somehow fundamental to
the language design.

So, let's go over some of the big design decisions.

## Type Inference

Type inference is the idea that the compiler can tell what type a variable should be by simply looking at what is
being passed to it.
* variable declarations
* loop declarations
* configuration declarations
* structure default initialization
* lambda calls

Look at these two code blocks:

Option A
```
  let x = y.do_something()
  let z = x.do_other_thing()
```
vs

Option B
```
  ResultStruct x = y.do_something()
  int z = x.do_other_thing()
```

Option A is tidy and neat and requires very little typing. It has a very consistent format and it is pleasing 
to the eye. Without an IDE, we don't know the types of x or z, so this is not very legible.

Option B gives us detailed information about what was returned in each method call without going to the
definition of the method. Without an IDE, the Option B is legible.

What if we compromised at did this:

Option C
```
  let x: ResultStruct = y.do_something()
  let z: int = x.do_other_thing()
```

Option C has the same amount of information as Option B, so it is legible. Is it neater? There is some visual appeal to
consistency, but not as neat as Option A. Still, neater than Option B.

Even if go with Option A, most modern IDEs will make it look like Option C, simply with less typing. 

So, Option A is only the neatest if we don't use a modern IDE, but it's also not very legible.
Option B is equivalent in legibility to Option C.
Option C is only needed if we don't have a modern IDE.

Dog went with Option A when possible and Option C when the developer is needs or wants to be explicit, because it was 
the neatest in all situations, required the least typing, and was only difficult to less legible when a modern IDE 
wasn't used.

Giving people the choice of using Option C if they don't use an IDE seemed like a good course of action, while 
permitting Option A in cases that aren't ambiguous allows people to choose a more concise format to type.

In the code examples and in the standard library, we will prefer Option C for clarity, but we won't begrudge your
use of Option A for maximum aesthetics and ergonomics.

Dog
```
  let x = y.do_something() 
  let z = x.do_other_thing() 
  let w: float = z
```
## Parenthesis
We could have if/while/switch statements use parenthesis for the expression they are evaluating, like:
```
  if (a || b) {
    x.do_something()
  } 
```

This is something that Java and C both do, but at some point, you end up with a sea of parenthesis.
```
  if ((a || b) && (f && (c && d))) {
    x.do_something()
  } 
```

Without them, it looks *slightly* neater and more legible:
```
  if a || b {
    x.do_something()
  } 
  
  if (a || b) && (f && (c && d)) {
    x.do_something()
  } 
```

## Semicolons

I feel like semicolons for terminating instructions exists in many languages because it made
it easier to parse and also made possible to have if/while/for statements that were single lines.

I do not like the idea of if statements that don't have their body enclosed in parenthesis,
as that style often leads to bugs. Most style checkers for languages that allow that syntax
will flag the use of that style, so why propagate such a thing?

Languages like Groovy and JavaScript don't require semicolons, but you still need them from
time-to-time. In JavaScript's case, you can treat anything like a function you can also have lines like this:

```
const x = something
(async () => {... do something})()
```

Unfortunately, javascript interprets this as an attempt to call something as a function with the hopes of calling
its result as a function as well. Disaster.

This exact situation could happen in Dog if we aren't careful. I considered using a lambda
syntax that led with parenthesis that would look like this:

```
let x:int = y
(|x:int, y:int| -> {... do something })() // create and invoke a lambda in one line... why? I don't know.
```
Why doesn't it break? Because Dog knows about newlines. It will still allow a newline between a variable
and a period, so that you can write a statement like:

```
let x: int = y
    .call_func1()
    .call_func2()
```    
And x will be equal to the result of call_fun2(). You cannot however do this with dog:

```
let x: int = y.call_func1
    ().call_func2()
```
Putting the newline there would cause the parser to loose it on you. Who even writes code like that?
don't do that. Newlines before or after boolean operators, or inside of parenthetical clauses, are
all fine. Put a newline in a place that would make the code ambiguous and we're not fine.

Dog way:
```
  if u == 10 {
    y.do_something()
  }
  x.do_other_thing()
```

## Curly brackets

This brings us to curly brackets. There are two big options for supporting blocks of code: curly
brackets (or some other similar delimiter) or indentation. Basically, the C style or the Python style.
(And with that statement, the Basic, Assembly, Fortran, Cobol, and Lisp developers are now all up in arms 
that they don't know what a curly bracket does either.)

We should be consistent across the language, if we use spaces for blocks of code, we should use them for
all blocks of code, not just if statements, but loops and scope blocks.

Option A: Curlies only
```
  if u == 10 {
    y.do_something()
  }
  x.do_other_thing()
```
Option B: Neither
```
  if u == 10
    y.do_something()
  x.do_other_thing()
```

Having no curlies looks neat, subjectively, but what are the consequences?

We have to look past aesthetics here to help us with our decision.

Without curlies, you have to rely on whitespace to determine where a code block starts or ends,
which is something that Python does. There are languages that use keywords instead of curlies, but 
it feels like more typing for little gain since there are so many languages that use curlies already
that developers are familiar with them. There are also languages that use other symbols instead of 
curlies, but that's less common and no more or less typing.

Are whitespaces more ergonomic and aesthetically appealing? Probably. You have to type them either way
and they already have meaning to humans as a grouping of related lines. 

There is a practical issues though: tabs and spaces. You are already cringing. I can feel it.

A tab looks like spaces, but it can be represented as having a different size based on the editor. If
we allow tabs, there will not work as intended and be confusing to troubleshoot.

What if we simply didn't allow the tab character for indents? We could give a clear compile error in this
case and it wouldn't be confusing. We'll probably get half of the known coding community banging on our door
that we chose spaces over tabs, but the code wouldn't likely be buggy. We could have also chosen tabs over 
spaces with mostly the same effect.

There's a problem though, this whole time we've been looking at if statements and not scope blocks. A scope 
block is simply a block of code in a function that has its own scope and variables within it are cleaned up when 
the scope ends.
```
  x.in_func_scope()
  
    let z = y.in_sub_scope()
    x.use(z)
  
  x.do_something // z is gone if x didn't hold a copy of it
```
What happens when you jam a sub scope next to an if?

For example:
```
  if x == 10 
    do_something() // do_something will happen only if x is 10
     
    let y = get_y() // is this a scope block or is this part of the if?
    y.another_thing()
      
  do_something_new() // this is definitely not in the if statement or a scope block
```

We could make it so that extra new lines means that the block ended, but that would make formatting blocks
less appealing. We could make a new keyword for scope blocks. Why not ditch scope blocks altogether or use a 
function? Scope blocks are faster and use less stack than a function. There are situations where you allocate 
something memory intensive, but only need it for a short time and releasing it before a function ends helps 
the code run well. It is an advanced feature, but something we'll want for internal code even if users of the 
language rarely use it.

The other fringe argument against spaces for blocks are the annoyances with refactoring or moving blocks of 
code. Good IDEs will do this okay, but occasionally they still have issues.

Curlies don't require as much explanation. They are explicit in nature, and their only real downside is that 
they make code look more cluttered.

After much consideration, the Dog way is to choose curlies because, while we want to value neatness, we don't 
want to do it at the expense of functionality.

```
  if x == 10 { 
    do_something()
  }
  
  {
    let y = get_y()
    
    y.another_thing()
  }
       
  do_something_new()
```

## Keywords

Most of the keywords should be familiar to people from Java or C, with the exception of a few that
are similar to keywords in Rust or Scala.

The goal was to keep the keyword count low, so that there wasn't much to learn, which probably immediately
makes you ask about the following seemingly extraneous keywords:
* app
* ui
* lib
* service
* test
* config

A few more that are addressed in other sections:
* mut (see [Ownership and Mutability](#Ownership and Mutability))
* const  (see [const](#const))
* let (see [Type Inference](#Type Inference))

### app, ui, lib, test, and service
Let's start with app, ui, lib, test, and service as they exist for the same reason. They could have simply
been a function with an attribute on them.

Instead of:
```
app MyApp() {
}
```
It could have been:
```
#app("MyApp")
pub fn MyApp() {
}
```

However, the major drawback of doing this is that you might have code that decided to call
the program entrypoint, which can be dicey. People write a main function with some assumptions,
including that it will be called once. 

Why not rely on the function name, like the C, Java, etc. `main` function? Because you might have 
one project that generates multiple libraries or executables and we wouldn't want naming collisions. 

Also, it doesn't look very tidy, which is the primary reason for keywords in this case.

There is a part of me that could be swayed to ditch these keywords in favor of attribute tags. As an
attribute tag though, it's not technical an official keyword, but it more or less serves the same 
purpose and takes up as much brain space. You need to somehow tell Dog that this is the entry point
for an app, and I think that fewer symbols is better from neatness perspective.

The Dog way:
```
app MyApp() {
}
```

### config
Isn't config just populating a struct?

Config is parsed before any compilation happens. It has properties in it, but it can also have code.
What is more, you are both defining custom properties while also assigning to them during construction.

Even more, configs can do inheritance! You can make a config that pulls in all of the values
of an existing config and then override a single value. Nowhere else in Dog can you do inheritance.

And at this point, you are probably wondering why dog doesn't have inheritance like this. You'll have to read 
the section on that. An exception was made here for the sake of being concise enough to fit in a single file
and easy to read.

Effectively, the config file is a script that runs at build time. It can run simple expressions of its own 
if it needs to. It isn't just assigning variables to a struct, effectively, it is a special form of
an application.

And when the actual program is run, everything in the config is effectively a `const`, so the values
can be inserted directly rather than by reference if this performs better.

Why not just use a property file or a yaml file or whatever the newest hottest format is?

Because property files are declarative in nature, and I wanted to give developers the power to
customize the build process. I also want them to be able inherit properties.

During the initial development, it'll probably be nothing more than a glorified Python dependencies.txt,
however over time I would like to see it transform into a more robust development build scripting language.

## Delegation, Inheritance, and Traits

The relationships between different elements of data has meaning to us as humans and be useful in writing concise, 
reusable code that is easy to understand and intuitive. This has led us from languages like C that have structures, 
pure data, and functions, pure logic, to more modern languages that have concepts of objects that combine logic and
data. In recent years, there has been a re-balancing from object-only languages to languages that support both. 
In this section we're going to cover objects-oriented patterns specifically.

There are two primary approaches to representing complicated relationships:
* Inheritance
* Composition

Inheritance represents "is a" relationships. A dog "is a" mammal. Also a dog "is a" pet.

Composition represents "has a" relationships. A dog "has a" owner. Also a dog "has a" favorite napping spot. 

You can bend one approach work well enough for the other. For example, instead of "A dog is a mammal" you could say 
"A dog has a classification of mammal." From a code perspective the difference is `dog.getClassificationName()` vs
`dog.getClassification().getName()`. We can do the same thing in the other direction.

Why am I telling you this? Well, we're getting to that.

Languages have gone different directions with representing objects and for the most part, each has worked well.

Something happens though when we it comes to inheritance. Objects often need to fulfill more than one unrelated role
and things get ugly.

The three common options are:
* Multiple Inheritance (C++/Python) 
* Interfaces (Java -- though interfaces in the newest Java act more like Traits)
* Traits (Scala/Python)

Multiple inheritance has the problem of being ambiguous to humans. It's not always apparent what function will be 
called in a given situation, and it leads to bugs.

Interfaces solve the problem of an object having multiple implementations of the same method, so you know what will
happen, but your object can't behave differently given the context of how it was called. Also, you can't add an 
interface to an existing object if you don't have the source code for it -- or do byte-code manipulation.

Traits solve the problem by defining an object's behavior for a concrete role, but offer the ability to give a default
behavior.

The first two options rely on inheritance, but Traits rely on delegation. 

Wait, what?!

Traits aren't part of the object's definition. They are defined on their own and contain a reference to the specific 
structure they were instantiated for. A trait "has a" object. If I have a list of objects with a shared trait, I have
a list of trait objects that each reference an object that may or may not be backed by the same structure.

As a user of a language, you think the dog "has a" trait or maybe "has a" set of traits, which is technically true, 
but from an implementation perspective, a trait "has an" implementation and "has a" struct.

I guess when we talk implementation, you should know what that looks like.

A C++ or Java class, at a super high level, are very similar (yes, there are major differences, ignore them) for our 
purposes. Each class as a mapping between the a method's signature and the memory address of that method. (This is only
true of methods that can be overridden -- in C++ they are called `virtual` methods.) I'm going to gloss over 
dynamic-dispatch vs multiple-dispatch vs single-dispatch. You need this table because at compile time, you can't
directly jump to a function's memory location if you don't know which function will be used for this instance of a object. 
(Imagine you have a list of animals, some of which are dogs and some of which are cats. You can't assume with 
inheritance that animal.speak() will making a barking or meowing sound, but at run time, animal.speak() could delegate 
to either dog.bark() or cat.meow(). It gets complicated.)

A trait has a *reference* to something like dispatch table for the impl and a *reference* to a structure. Compared that 
to a C++ which has *MULTIPLE* built-in dispatch tables (one for each object it extends) as part of its 
data structure. 

Dog uses traits because they address problems without requiring an understanding of complicated function resolution rules 
and can be added to existing structures, even if you don't control the source of those structures. This does use more 
memory because we need a trait object that has two pointers in it, so another 16 bytes. Performance-wise, it should be
roughly as fast or faster than C++ depending on the use case.

## Shadowing

Shadowing leads to bugs, but there are situations that simple require it. If you disallow all shadowing but pull in a 
library that has a variable you declared as a global, things will not end well for you.

A global reference in a file that it was not declared in must be in a use statement, where it could be aliased if needed.

You cannot have any shadowing of variables in a file. If you `use` a global, it must be aliased if it would collide.

In the case of a trait, it can have functions that have parameter names that collide with a `struct`'s member name, which 
is allowed and requires a `self` reference.

In short:

For a given scope, only a parameter's name may shadow another variable.

## Ownership and Mutability

Safe: High
Easy: Low
Fast: High
Flexible: Low

When you allocate an object in most languages, it goes on the heap and lives through that languages object
life-cycle. Depending on the language, there may be many references to that object, but anybody that has a
reference can probably modify it.

discussion on const and


Your first thought might be: I need to declare that my variables are mutable?!?!

If you've used Rust or Scala, this may not be as big of a shock to the system, but for everybody else, you
are screaming WTH at the top of your lungs.

It is a poor programing practice to reassign variables in many cases. Not all cases, and I'm not going to
enumerate the times it is good and bad because that is simply too much work. However, there are times when
you definitely will need to, so we need to give that option.

That said, immutability helps make concurrency safer. Threaded code that takes in immutable input and gives
out immutable results is less likely to suffer from certain types of race condition bugs and forces
better program design.

That reminds me, when a variable assigned to a structure isn't mutable, that means that the whole structure isn't
mutable from this location. You could still call a function on a trait of that structure to change it, but
the structure's members will only be mutable to itself if they were explicitly set as mutable.

So, `mut` largely exists to help with concurrency, but it also helps a little with bug prevention.

## Pass by value and reference

And `ref`? Why does that even exist? Shouldn't everything be pass-by-reference?

I decided to make Dog pass-by-value, as this allows the compiler to prefer using the stack over the heap in
many use cases. This is usually good for performance if variables being passed aren't too big and it also helps
with performance that you can't have two different locations modify the same object.

That said, some objects will use the heap, or they will reference objects that use the heap, because their size isn't
fixed. Walking the heap to copy them and their child references would be brutal. A shallow copy could be done which would
be fast, but then the behavior would be inconsistent: if I modify the child of an object passed to me, it will have side
effects on the caller's copy, but changes to the top level object are not reflected to the caller.

Why not do what Java did and pass primitives by-value, but objects by-ref? Because not all objects need to be on the
heap and Java's approach is slow in those cases.

So, an object that is passed by-value in Dog cannot be modified, nor can it be passed by-ref to another function, which
would create a loophole.

An object passed by `ref` can only be modified if the function declares it can be using the `mut` keyword, and it can only
passed to other functions that are also `ref` and `mut`.

... hmm.. now that i think of it, if the `mut` keyword is on a function you are about to pass a value to, I could do two
things:
* verify that the object came in as a parameter with `mut`
* or verify that i created the object on the heap in this function -- the compiler can be sure to put
  it on the heap if it knows we'll use it this way in this function in the future.

... hmm... is this good enough... must think

## Code before entry points

Some languages allow you to have code that starts before the program starts, while other languages declare "no code
before main". The problem with creating semantics for code to start running before the start of the program is that
you are moving the goal posts. There's simple a new starting point, but one that isn't apparent to all developers
and makes the whole process more mysterious and difficult.

If you know that there is no code executing before the entry point, then you as the developer have full control of
the initialization of your application.

What's the drawback of this? Well, it means that global variables can't call functions, otherwise they would have to
be initialized before main so you could safely access them. Calling functions from global variables is problematic 
from a predictability perspective as they may not be initialized in the order you expect. While it is legal to have
globals, you will need to initialize them within a function.

## const

I was hesitant to add `const` because it was another keyword and I weighed whether it was better to
have 'static' and 'final' keywords, which offer more control at the expense of more reserved words.

I hope that most people who use Dog aren't trying to have ultimate control over their variables, I hope
they want to quickly and easily manipulate data. While `const` helps with quickly, it has some nuances
that may make people not find it so easy.

Constants need to be evaluated at compile-time for two reasons:
* Dog doesn't execute user code before an entry point: see section [Code before entry points](#Code before entry points) for reasons
* Constants can be inlined if they are defined at compile time

## constructors and deconstructors

There is only one way to initialize a structure:
```
  let a: MyStruct = MyStruct {
    prop1 = 0
    prop2 = "hi"
  }
```
When you create it, you have to set every property on it, no exceptions. This makes the creation explict and obvious.

That doesn't mean you can't make a method that is similar to a constructor, however.
```
pub struct MyStruct {
  prop1: int
  prop2: String
}

impl MyStruct {
  pub const fn new(): MyStruct {
    return MyStruct {
      prop1 = 0
      prop2 = "hi"
    }
  }
}

  ... somewhere later ...

  let a: MyStruct = MyStruct::new()
```

A `const` function cannot be overridden and is not backed by a struct. Also notice that the `impl` above is a public 
scope trait implementation (all `impl`'s are public) and the return is the public structure, but the fields are private. 
To read the values out, you'd need to add functions that access them in your `impl` or another `trait`.

There are no deconstructors in Dog. This is because Dog does garbage collection and does not guarantee when or even if 
an object will be cleaned up. This requires you to manage your object's lifecycle.

If you implement the `Resource` trait and define the `acquire()` and `release()` functions, you can then do:

```
  with my_resource { // acquire is called here
  
  } // release is called here even if there was an exception
```

## Memory

static, stack, and heap



## Concurrency
NOTE: still working through design and details here

Trying to decide what should use keywords and what should be base library.

The base library needs this functionality, as will the user, and it can
either be through keywords that do it, or it can be through objects that
take in lambdas.

`atomic` and `volatile` are also an issue as they require detailed knowledge that
most people don't know. Volatile isn't even really useful in the way that C refers
to it on modern architectures. Atomic is useful and required in some way. I don't
want to see every variable be atomic, that'd be too slow, but it also might be
hard to use the language if users need to declare atomic themselves.  Current best
bet is to have an `Atomic` trait and encourage forms of concurrency that don't
usually rely on shared state -- so, immutable inputs and then doing the processing of
outputs in a single thread.

I would like to provide built in syntax for a concurrent loop, but maybe doing
something through standard language conventions is fine. This goal might be met
like `mylist.parallel().foreach(|x|->{...})`, but it creates two conventions
for doing loops that would not look similar.
Maybe `parallel for x: int in mylist {}`, as an attribute like `parallel` could
be applied to `while` loops. I had created a parallel loop in smirk, and I called
it 'with', but I belatedly realized that the keyword was confusing to those
who came form python.

Also the ability to create futures from any method seems pretty convenient.
Current syntax goal looks something like `let x: future int = obj!do_something()` or
if it isn't a keyword, it could be a generic like `let x: future<int> = obj!do_something()`
This would be similar to some syntax like `let x: future<int> = threads::invoke(() -> obj.do_something())`
if we used a lambda and passed it to a function that kicked off a thread and returned a
future. As a starting point, a `Future` trait will need to exist and so will a function
that kicks off a thread and returns that Future. If we eventually put syntactical
sugar on it, then that is fine.

## Exceptions

In an effort to not design a language that permits what we jokingly refer to as exception-oriented-code,
Dog allows you to throw and catch exceptions using `fail` and `otherwise`, but does not create
specialized exception types or place data in the exception beyond a text message. This is to prevent
developers from using exceptions for program flow control in non-error scenarios -- like an 
old-school goto command.

Code that jumps from one method to a method three calls before is hard to read and not intuitive,
so catch errors near where they happen if you need to catch them at all.

If an exception can only happen due to a coding mistake, do not attempt to handle them. Let the
program fail in testing and add the appropriate code to avoid an exceptional condition in
the future.

The otherwise block can make your code look cleaner when values are expected to be null some of the
time. See below for an example.
```
fn will_fail() {
  fail("I don't need a reason, but I'll give you one.")
}

  ...

  will_fail() otherwise {
    println("We ignored the failure.")
  } 
  
  {
    // do some things
    will_fail()
    // do more things
  } otherwise {
    println("We ignored the failure.")
  }
```

You'll notice that the otherwise block doesn't provide a copy of the message. This is intentional to
prevent it from becoming a delivery mechanism for data that would be used for flow control.

By not catching the exception, the program will print the message out when it fails.

In your config, you can add the variable `print_debug: true` and when an exception happens, it will print
the message and trace information even if the exception is caught in an otherwise block.

There are times, that an exception isn't actually thrown but the otherwise block is executed.
```
    let x: int = obj1.getObj2().getObj3().getInt() otherwise 0
```
If obj1 or any of the objects after that are null, the otherwise block is called, but the compiler 
recognizes this scenerio and since it is already checking for null (because a native program not checking
for null is going to crash), it simply skips to the otherwise block if a section is null.

Ideally, I'd like to implement the Itanium C++ ABI: Exception Handling standard, but would settle on sjlj.

[LLVM exception handling details](https://llvm.org/docs/ExceptionHandling.html)

[Read the Docs examples of exception handling](https://mapping-high-level-constructs-to-llvm-ir.readthedocs.io/en/latest/exception-handling/index.html)
