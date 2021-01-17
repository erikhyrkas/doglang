# Design Decisions

Every design decision needs to back the goals of: safe, easy, fast, and flexible. Roughly in that order.

Safe is non-negotiable, as a language used for working with large amounts of data. You have to trust that it will be
stable and accurate. As a language, safe also means that it makes engineer errors less likely.

Easy and fast are trickier. Sometimes we need to lean one way or another, and it's about walking the line.

The word "easy" and "fast" are both traps. Easy can mean that it takes less code or less learning to do something. Fast
can mean that it takes less time to write the application or that the resulting code does its job fast. To all of these
statements, we say: yes, let's do that.

If you want the ultimate in fast and flexible, you could already implement literally anything in C if you don't mind
overcoming difficult problems with metric tons of code.

If you want the ultimate in easy, there are tools with user interfaces that manipulate data, but they are not fast or
flexible.

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
constructs that they are already familiar with and only use something novel after they have experience. If the construct
was good enough, you'd likely see it also ported to other languages unless it was somehow fundamental to the language
design.

So, let's go over some of the big design decisions.

## Type Inference

Type inference is the idea that the compiler can tell what type a variable should be by simply looking at what is being
passed to it.

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

Option A is tidy and neat and requires very little typing. It has a very consistent format and it is pleasing to the
eye. Without an IDE, we don't know the types of x or z, so this is not very legible.

Option B gives us detailed information about what was returned in each method call without going to the definition of
the method. Without an IDE, the Option B is legible.

What if we compromised at did this:

Option C

```
  let x: ResultStruct = y.do_something()
  let z: int = x.do_other_thing()
```

Option C has the same amount of information as Option B, so it is legible. Is it neater? There is some visual appeal to
consistency, but not as neat as Option A. Still, neater than Option B.

Even if go with Option A, most modern IDEs will make it look like Option C, simply with less typing.

So, Option A is only the neatest if we don't use a modern IDE, but it's also not very legible. Option B is equivalent in
legibility to Option C. Option C is only needed if we don't have a modern IDE.

Dog went with Option A when possible and Option C when the developer is needs or wants to be explicit, because it was
the neatest in all situations, required the least typing, and was only difficult to less legible when a modern IDE
wasn't used.

Giving people the choice of using Option C if they don't use an IDE seemed like a good course of action, while
permitting Option A in cases that aren't ambiguous allows people to choose a more concise format to type.

In the code examples and in the standard library, we will prefer Option C for clarity, but we won't begrudge your use of
Option A for maximum aesthetics and ergonomics.

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

I feel like semicolons for terminating instructions exists in many languages because it made it easier to parse and also
made possible to have if/while/for statements that were single lines. I do not like the idea of if statements that don't
have their body enclosed in parenthesis, as that style often leads to bugs. Most style checkers for languages that allow
that syntax will flag the use of that style, so why propagate such a thing?

Languages like Groovy and JavaScript don't require semicolons, but you still need them from time-to-time. In
JavaScript's case, you can treat anything like a function you can also have lines like this:

```
const x = something
(async () => {... do something})()
```

Unfortunately, javascript interprets this as an attempt to call something as a function with the hopes of calling its
result as a function as well. Disaster.

This exact situation could happen in Dog if we aren't careful. I considered using a lambda syntax that led with
parenthesis that would look like this:

```
let x:int = y
(|x:int, y:int| -> {... do something })() // create and invoke a lambda in one line... why? I don't know.
```

Why doesn't it break? Because Dog knows about newlines. It will still allow a newline between a variable and a period,
so that you can write a statement like:

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

Putting the newline there would cause the parser to loose it on you. Who even writes code like that? don't do that.
Newlines before or after boolean operators, or inside of parenthetical clauses, are all fine. Put a newline in a place
that would make the code ambiguous and we're not fine.

After more thought, I decided to allow semicolons as optional line endings, not because I think they are a syntactical
requirement -- because while I think we can avoid those issues, but from an ergonomics perspective if somebody feels
more productive or prefers that code style, it costs very little to accommodate their style. Semicolons won't be
considered idiomatic.

Dog way:

```
  if u == 10 {
    y.do_something()
  }
  x.do_other_thing()
```

## Colon vs Equal

When it comes to assigning values during construction of a struct, we could use colons or equals:

```
    let mystruct = MyStruct {
        prop1 = 'val'
    }
```

or with colon

```
    let mystruct = MyStruct {
        prop1: 'val'
    }
```

The colon looks neat and is very similar to the JSON syntax. There is no penalty to supporting both syntaxes, but we'll
pick the colon as the idiomatic approach.

```
    let mystruct = MyStruct {
        prop1: 'val'
    }
```

## Curly brackets

This brings us to curly brackets. There are two big options for supporting blocks of code: curly brackets (or some other
similar delimiter) or indentation. Basically, the C style or the Python style.
(And with that statement, the Basic, Assembly, Fortran, Cobol, and Lisp developers are now all up in arms that they
don't know what a curly bracket does either.)

We should be consistent across the language, if we use spaces for blocks of code, we should use them for all blocks of
code, not just if statements, but loops and scope blocks.

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

Without curlies, you have to rely on whitespace to determine where a code block starts or ends, which is something that
Python does. There are languages that use keywords instead of curlies, but it feels like more typing for little gain
since there are so many languages that use curlies already that developers are familiar with them. There are also
languages that use other symbols instead of curlies, but that's less common and no more or less typing.

Are whitespaces more ergonomic and aesthetically appealing? Probably. You have to type them either way and they already
have meaning to humans as a grouping of related lines.

There is a practical issues though: tabs and spaces. You are already cringing. I can feel it.

A tab looks like spaces, but it can be represented as having a different size based on the editor. If we allow tabs,
there will not work as intended and be confusing to troubleshoot.

What if we simply didn't allow the tab character for indents? We could give a clear compile error in this case and it
wouldn't be confusing. We'll probably get half of the known coding community banging on our door that we chose spaces
over tabs, but the code wouldn't likely be buggy. We could have also chosen tabs over spaces with mostly the same
effect.

There's a problem though, this whole time we've been looking at if statements and not scope blocks. A scope block is
simply a block of code in a function that has its own scope and variables within it are cleaned up when the scope ends.

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

We could make it so that extra new lines means that the block ended, but that would make formatting blocks less
appealing. We could make a new keyword for scope blocks. Why not ditch scope blocks altogether or use a function? Scope
blocks are faster and use less stack than a function. There are situations where you allocate something memory
intensive, but only need it for a short time and releasing it before a function ends helps the code run well. It is an
advanced feature, but something we'll want for internal code even if users of the language rarely use it.

The other fringe argument against spaces for blocks are the annoyances with refactoring or moving blocks of code. Good
IDEs will do this okay, but occasionally they still have issues.

Curlies don't require as much explanation. They are explicit in nature, and their only real downside is that they make
code look more cluttered.

After much consideration, the Dog way is to choose curlies because, while we want to value neatness, we don't want to do
it at the expense of functionality.

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

## Single or Double Quotes

Different languages use either single or double quotes around strings and characters.

Java, C, and others will use single quotes around characters and double quotes around strings.

```Java
    char c = 'a';
    String string = "string";
    
```

Python has no concept of a char. Something is a string or it isn't, and it will let you use single or double quotes.

```Python
    c = 'a'
string = 'string'
another_string = "this is also a string"
another_c = "a"
```

Where things get more interesting is with embedding quotes within your strings or characters.

Java needs escaping fairly often:

```Java
    char single_quote = '\''; 
    String string = "I asked, \"How are you?\"";  
```

Python can often avoid escaping by simply using the appropriate enclosing quote:

```Python
    single_quote = "'";
string = 'I asked, "How are you?"';
```

From a purely aesthetic perspective, single quotes are less noisy than double quotes. (I know, this is subjective. So,
fight me.) And being able to pick either a single quote or a double quote as needed allows you to forgo escaping in many
cases.

From a technical perspective, the char data type exists to declare expectations (this will only be 1 character long)
and more importantly, to save memory. Strings simply have much more overhead associated with them because even when they
hold one character they are still an array and a length.

On the whole strings are much more common in day-to-day programming and chars are largely used when we get into the
nitty-gritty. Nobody moving data from databases or from files is using a char data type very often.

So, even though this is controversial to me, I've decided to support the Python approach of allowing either a single or
double quote.

What does this mean for the char data type? It is still there and fine. Most of the time, we can infer whether a piece
of text is a string or char, but in rare cases the compiler can't infer it, the developer can cast the value.

Simple Rules:

* The char data type cannot have more than one character
* Compiler will default to a char data type if it cannot infer the data type, because a char can be cast to a string
  later and a char uses less memory

## Multiline strings

I see no good argument against multi-line strings.

The only nuance is the embedding of the \r character, which Dog will strip, so that newlines are always represented as
\n. In the rare case you need both \r\n, then you can explicitly put them in and not use multi-line strings, or use them
and then do a replace. In most cases, a \n works fine, even on Windows for console output.

For file output, specify your preference with a file encoder to translate \n into \r\n if you want the file to be
Windows specific when you write it.

Dog strings are inherently multi-line.

```
    let my_multi_line_string = "
This is a great line of text.
This is another one.
Scott hates multi-line strings.
Erik thinks he is crazy."    
```

## Keywords

Most of the keywords should be familiar to people from Java or C, with the exception of a few that are similar to
keywords in Rust or Scala.

The goal was to keep the keyword count low, so that there wasn't much to learn, which probably immediately makes you ask
about the following seemingly extraneous keywords:

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

Let's start with app, ui, lib, test, and service as they exist for the same reason. They could have simply been a
function with an attribute on them.

Instead of:

```
app fn MyApp() {
}
```

It could have been:

```
_app("MyApp")
pub fn MyApp() {
}
```

Or they could have simply been functions without annotations that were referenced in the config.

However, entry points don't behave like other functions. A console entry point has inherently different setup than a 
user interface, in terms of what is initialized when. There is boiler plate code that goes into each of these types of
declarations under the cover that a developer shouldn't have to write in a high-level language. A console application,
what we've given the `app` keyword is the simplest type of entry point, but still has very specific requirements around
handling arguments that is different than a library.

Endpoints imply that this additional logic exists and having keywords acknowledges this and shows that the developer
is doing this purposefully. Entry points are also designed to be called only once per execution, which is something 
that functions are not designed to do.

Why not rely on the function name, like the C, Java, etc. `main` function? Because you might have one project that
generates multiple libraries or executables and we wouldn't want naming collisions.

It is also tidy to use keywords over additional tagging or to need to reference an external file to find out that
a method was designed to be an entry point.

There is a part of me that could be swayed to ditch these keywords in favor of attribute tags. As an attribute tag
though, it's not technical an official keyword, but it more or less serves the same purpose and takes up as much brain
space. You need to somehow tell Dog that this is the entry point for an app, and I think that fewer symbols is better
from neatness perspective.

I weighed specifying entry points through config, but having to go elsewhere to determine that a function was designed
to be a test or a application, seems like it would make the code less apparent. I weighed treating test entry points 
differently than app end points, but it felt inconsistent to me.

What is more, by using keywords, we are communicating that this is an important feature of the language and not just an
afterthought or a bolt-on. A low level language is extremely minimal in keywords and forces the developer to know
patterns. A high level language like Dog wants to create clear and easy ways of doing common tasks that also read
without difficulty.

I was going to drop the `fn` aspect of the declaration, however it led to the `test` declaration being inconsistent,
and being consistent helps people learn faster. (Yeah, there's part of me that is now debating some of the 
optional syntax that may make the language less consistent. I'm looking at you, semicolons.)

The Dog way:

```
app fn MyApp() {
}
```

### config

Isn't config just populating a struct?

Config is parsed before any compilation happens. It has properties in it, but it can also have code. What is more, you
are both defining custom properties while also assigning to them during construction.

Even more, configs can do inheritance! You can make a config that pulls in all of the values of an existing config and
then override a single value. Nowhere else in Dog can you do inheritance.

And at this point, you are probably wondering why dog doesn't have inheritance like this. You'll have to read the
section on that. An exception was made here for the sake of being concise enough to fit in a single file and easy to
read.

Effectively, the config file is a script that runs at build time. It can run simple expressions of its own if it needs
to. It isn't just assigning variables to a struct, effectively, it is a special form of an application.

And when the actual program is run, everything in the config is effectively a `const`, so the values can be inserted
directly rather than by reference if this performs better.

Why not just use a property file or a yaml file or whatever the newest hottest format is?

Because property files are declarative in nature, and I wanted to give developers the power to customize the build
process. I also want them to be able inherit properties.

During the initial development, it'll probably be nothing more than a glorified Python dependencies.txt, however over
time I would like to see it transform into a more robust development build scripting language.

## Delegation, Inheritance, and Traits

The relationships between different elements of data has meaning to us as humans and be useful in writing concise,
reusable code that is easy to understand and intuitive. This has led us from languages like C that have structures, pure
data, and functions, pure logic, to more modern languages that have concepts of objects that combine logic and data. In
recent years, there has been a re-balancing from object-only languages to languages that support both. In this section
we're going to cover objects-oriented patterns specifically.

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

Something happens though when we it comes to inheritance. Objects often need to fulfill more than one unrelated role and
things get ugly.

The three common options are:

* Multiple Inheritance (C++/Python)
* Interfaces (Java -- though interfaces in the newest Java act more like Traits)
* Traits (Scala/Python)

Multiple inheritance has the problem of being ambiguous to humans. It's not always apparent what function will be called
in a given situation, and it leads to bugs.

Interfaces solve the problem of an object having multiple implementations of the same method, so you know what will
happen, but your object can't behave differently given the context of how it was called. Also, you can't add an
interface to an existing object if you don't have the source code for it -- or do byte-code manipulation.

Traits solve the problem by defining an object's behavior for a concrete role, but offer the ability to give a default
behavior.

The first two options rely on inheritance, but Traits rely on delegation.

Wait, what?!

Traits aren't part of the object's definition. They are defined on their own and contain a reference to the specific
structure they were instantiated for. A trait "has a" object. If I have a list of objects with a shared trait, I have a
list of trait objects that each reference an object that may or may not be backed by the same structure.

As a user of a language, you think the dog "has a" trait or maybe "has a" set of traits, which is technically true, but
from an implementation perspective, a trait "has an" implementation and "has a" struct.

I guess when we talk implementation, you should know what that looks like.

A C++ or Java class, at a super high level, are very similar (yes, there are major differences, ignore them) for our
purposes. Each class as a mapping between the a method's signature and the memory address of that method. (This is only
true of methods that can be overridden -- in C++ they are called `virtual` methods.) I'm going to gloss over
dynamic-dispatch vs multiple-dispatch vs single-dispatch. You need this table because at compile time, you can't
directly jump to a function's memory location if you don't know which function will be used for this instance of a
object.
(Imagine you have a list of animals, some of which are dogs and some of which are cats. You can't assume with
inheritance that animal.speak() will making a barking or meowing sound, but at run time, animal.speak() could delegate
to either dog.bark() or cat.meow(). It gets complicated.)

A trait has a *reference* to something like dispatch table for the impl and a *reference* to a structure. Compared that
to a C++ which has *MULTIPLE* built-in dispatch tables (one for each object it extends) as part of its data structure.

Dog uses traits because they address problems without requiring an understanding of complicated function resolution
rules and can be added to existing structures, even if you don't control the source of those structures. This does use
more memory because we need a trait object that has two pointers in it, so another 16 bytes. Performance-wise, it should
be roughly as fast or faster than C++ depending on the use case.

## Shadowing

Shadowing leads to bugs, but there are situations that simple require it. If you disallow all shadowing but pull in a
library that has a variable you declared as a global, things will not end well for you.

A global reference in a file that it was not declared in must be in a use statement, where it could be aliased if
needed.

You cannot have any shadowing of variables in a file. If you `use` a global, it must be aliased if it would collide.

In the case of a trait, it can have functions that have parameter names that collide with a `struct`'s member name,
which is allowed and requires a `self` reference.

In short:

For a given scope, only a parameter's name may shadow another variable.

## Ownership and Mutability

When you allocate an object in most languages, it goes on the heap and lives through that languages object life-cycle.
Depending on the language, there may be many references to that object, but anybody that has a reference can probably
modify it.

discussion on const and

Your first thought might be: I need to declare that my variables are mutable?!?!

If you've used Rust or Scala, this may not be as big of a shock to the system, but for everybody else, you are screaming
WTH at the top of your lungs.

It is a poor programing practice to reassign variables in many cases. Not all cases, and I'm not going to enumerate the
times it is good and bad because that is simply too much work. However, there are times when you definitely will need
to, so we need to give that option.

That said, immutability helps make concurrency safer. Threaded code that takes in immutable input and gives out
immutable results is less likely to suffer from certain types of race condition bugs and forces better program design.

That reminds me, when a variable assigned to a structure isn't mutable, that means that the whole structure isn't
mutable from this location. You could still call a function on a trait of that structure to change it, but the
structure's members will only be mutable to itself if they were explicitly set as mutable.

So, `mut` largely exists to help with concurrency, but it also helps a little with bug prevention.

## Pass by value and reference

This has been a hard internal struggle for me as I weigh performance and language ergonomics.

Pass-by-value copies parameters to a function on the stack. If that parameter was mutable, any change you made to it
would not impact the original value. However, if we do flag the variable as mutable, it is intuitive to me that we are
saying we want to change the original variable, so that leads us to...

Pass-by-reference sends a pointer to the original object so that any operations done to it are on the original copy.

Within a function itself, how the code is generated when it interacts with a structure or primitive changes depending on
whether it represents a pointer or a local variable.

I had been considering a `ref` keyword, but it seems redundant to me, since I don't think code should reassign the value
of a parameter unless their intent was to change the original value.

For variable assignment, I believe primitives need to always be by-value. We'd need a more expressive syntax that would
be harder to use if we wanted to have structure members that referred to members of other structures and the behavior
would not be safe. I think that object references are common enough in languages that people will assume them to exist.
Primitive references, that would take some C like syntax and maybe then we'd need that `ref` keyword.

On the whole, I don't want users to think much about this. The standard pattern should be to have immutable parameters,
and only make them mutable if they really need to be. This gives the compiler the option to pass-by-value or reference,
depending on the size of the object, and potentially help thread safety and performance.

TODO: I'm trying to decide if there is ever a case where you pass a structure's primitive member to a function
by-reference, only to have the object garbage collected. I think to prevent this, you can't pass a primitive into a new
thread by-reference. It has to be by value, or you need to pass the parent object. This is because garbage collectors
look at full structures, not sub-parts of their allocation. To it, it sees an array of bytes with a little metadata, not
individual fields.

## Code before entry points

Some languages allow you to have code that starts before the program starts, while other languages declare "no code
before main". The problem with creating semantics for code to start running before the start of the program is that you
are moving the goal posts. There's simple a new starting point, but one that isn't apparent to all developers and makes
the whole process more mysterious and difficult.

If you know that there is no code executing before the entry point, then you as the developer have full control of the
initialization of your application.

What's the drawback of this? Well, it means that global variables can't call functions, otherwise they would have to be
initialized before main so you could safely access them. Calling functions from global variables is problematic from a
predictability perspective as they may not be initialized in the order you expect. While it is legal to have globals,
you will need to initialize them within a function.

## const

I was hesitant to add `const` because it was another keyword and I weighed whether it was better to have 'static' and '
final' keywords, which offer more control at the expense of more reserved words.

I hope that most people who use Dog aren't trying to have ultimate control over their variables, I hope they want to
quickly and easily manipulate data. While `const` helps with quickly, it has some nuances that may make people not find
it so easy.

Constants need to be evaluated at compile-time for two reasons:

* Dog doesn't execute user code before an entry point: see section [Code before entry points](#Code before entry points)
  for reasons
* Constants can be inlined if they are defined at compile time

## global state and singletons

Public functions are effectively global, but what about variables? We said that constants are evaluated at compile time.

Globals cause many race-condition bugs for new programmers when they need to do anything with concurrency. They forget 
that state is shared and they put data in the wrong scope, which works with one thread, but not with two or more. Still,
applications need to have state that lives for the duration of the code's execution.

How do we store application state that has a lifecycle that matches the application?

Option 1: From within the entry point

The entry point (the `app` or `lib` or whichever) could create one or more state objects. They are explicitly declared 
within that function and it is apparent that they will be cleaned up at the end of the function.
```
app fn main() {
  let x: MyState = createState()
  doLogic(x)
}
```
The biggest problem with only using this approach is that you can't have a true singleton if you have no global state.

There are times when only one instance of something should exist, and it should be for the entire duration of the 
code's execution. Also, by needing to pass around global state, function signatures become heavily burdened by 
long parameter lists or custom parameter objects need to be created to hold the application state and be passed
from function to function. These parameter objects become either complicated with many optional members or generic 
with maps of string keys to objects. 

Option 2: Singletons

We could create a singleton type. For example, Scala has the `object` keyword, which is a way of declaring a class as a 
singleton and defining a global that is lazily initialized. By being lazily initialized, you don't have code running 
before the entry point, but the execution of the code isn't super apparent. When will this object be initialized?
It may be conditionally referenced, so maybe never? Maybe an hour from now.

Singletons are a mixed bag, as they represent a potentially mutable state shared by multiple threads. They solve the
issue of having global state that doesn't require passing it from function to function, however.

Maybe something like:
```
singleton my_singleton: MyStruct = MyStruct::new() // the const method new() isn't called until the first use of my_singleton 
```

Rather than the `singleton` keyword, I think the word `lazy` would work just as well. Because either way the functionality
is the same, but `lazy` is fewer letters to type and maybe a better description.

```
lazy my_singleton: MyStruct = MyStruct::new() // same end result, different keyword 
```

Here a variable is acting like a function under the covers, where that function is checking to see if the structure
has been created, and if it has not, then it is creating it in a thread-safe way. 

Either way, you can technically create MyStruct as many times as you want, so it isn't really a singleton.

The biggest issue? What if MyStruct::new() references a different singleton/lazy variable that then references back to my_singleton?

Sure, that's an error to have a circular reference, but it won't be apparent why things are broken.


Option 3: Globals

Many languages simply let you declare variables outside of a function or class and their scope is global. Initialization
happens from the top of the file to the bottom. However, this opens the door to code running before the entry point,
which can have different implications depending on the type of entry point. 

Most of the time, having initialization happen in an arbitrary oder doesn't hurt, but having it potentially happen 
in an order that you didn't anticipate will cause hard to track bugs.

```
  let my_struct: MyStruct = MyStruct::new() // this will get called when the app loads, but you can't be sure when
```

Option 4: Cached function results

Similar to the singleton data type, but rather than having a named variable act as a function call to potentially
initialize a variable, the function is synchronized and returns a cached result if it has one, otherwise it 
runs and then stores the result internally.

```
  // in a function before you use it, not outside of a function:
  let my_struct: MyStruct = MyStruct::singleton() 
  
impl MyStruct {
  const once fn singleton(): MyStruct {
      return MyStruct {}; // the first time we are called, we create the structure, but future calls will return the originally created structure
  } 
}
```

The function could be part of a trait or impl, or be standalone. This syntax is very similar in functionality to the
`singleton` keyword functionality, but makes it apparent that logic will happen when you want to reference it, compared
to referencing a variable, which doesn't feel like it should contain potential logic that could be slow. Even with a
debugger, stepping to a variable usage and finding yourself in a method feels weird to me.

If `once` is used on a non-const function, what are the consequences? It means we have an object instance that can
have cached values, this seems useful in situations where a computation is expensive.

Can a trait define a `once` function? I think so, but it would impact how a function signature is defined.

Can a lambda be define as `once`? Is there a use case that this would help? How would it be bad?

To prevent hanging, we either need cycle detection or the synchronization needs to be re-entrant, but it then needs to 
know that initialization is in progress and that this is an error, so it must exit the program. I'm not sure at this
point how feasible compile-time cycle detection is if anything we refer to is potentially using late-binding.

Sifting through the options

We've only covered globals at this point, but not thread locals, but we need to be aware that our pattern may
influence what thread locals look like. If we make a `singleton` keyword, we may want a `threadlocal` keyword
for a singleton of that scope. If we use the `once` keyword, we may want a `thread_once` keyword. We haven't decided
whether to support thread locals yet, but they are a powerful feature in concurrency and we should at least
keep the door open to them.

Option 1 is a given. We get it by simply having the ability to have local variables and pass them around. However, 
supporting a global state option seems like a way to alleviate ugly code and inconvenience. Option 4 feels right to
me, though it isn't the way that other languages typically work (which is either option 3 and potentially something 
like option 2.) You know with Option 4 that a function is running -- even if the results may be cached -- and it
is apparent that this is when the initialization happens. You might use this function in many places, but it is your
code and you could control when the first invocation happens for performance or sequencing. It also avoids the issues
of an initialization referencing another global that will fail in a non-apparent way.

Dog way:
```
  // in a function before you use it, not outside of a function:
  let my_struct: MyStruct = MyStruct::singleton() 
  
impl MyStruct {
  const once fn singleton(): MyStruct {
      return MyStruct {}; // the first time we are called, we create the structure, but future calls will return the originally created structure
  }
  once fn cached_member(): int {
    //... slow complicated logic here where the result won't change...
    return result; 
  } 
}
```

## constructors and deconstructors

There is only one way to initialize a structure:

```
  let a: MyStruct = MyStruct {
    prop1: 0
    prop2: 'hi'
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
      prop1: 0
      prop2: 'hi'
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

Static memory is simply the memory that the application takes up when it is loaded. You can reference it, and you can
get in trouble by writing to it and then overwriting bits of code. Static memory makes sense for constants and for
global values.

Stack memory is allocated for each thread, including the main thread. The linker that compiled the application decides
the default stack size for the application's main thread, which is typically 1 megabytes. This can be changes. Each
thread that is allocated gets it's own stack, which is typically 1 megabyte. Because the chunk of memory is reserved
when the thread is created, and because of the way that allocations and deallocations work (last in first out), it is
very fast to use stack memory. The only problem with it is that it is relatively small and allocations on it only live
for the duration of their scope. Because it is so fast though, many languages attempt to maximize their use of it. Rust
is an example of a language that requires you to explicitly allocate structures on the heap.

The Java JVM will do what is called "escape analysis" and if it determines that an object won't live past the current
thread's scope, then it will use the stack rather than the heap to allocate the object. (Escape analysis also helps it
determine if it can remove synchronization because the object isn't accessed by multiple threads and it can also help it
determine whether parts of the object can be stored in registers or whether it needs to remain in memory for safer
concurrent access.)

Heap memory is that which is allocated and freed by the program. It is expensive to allocate because it requires a
search for a large enough chunk of memory for an allocation, and when memory fragmentation occurs, it becomes more and
more difficult over the life of the application. When we discuss garbage collection, it is the heap space that we are
referring to. By default any object that has to live beyond the current scope it was created in must be on the heap.

For Dog, the goal is design the syntax in such a way that escape analysis is possible and safe and that interactions
with native code is also possible.

For the `internal` package, all allocations will need to be static or stack because there will be no garbage collector,
since it is the code defining the garbage collector! For the `std` package, structs will all use the heap initially
until we have strong enough escape analysis to use the stack.

## Concurrency

NOTE: still working through design and details here

Trying to decide what should use keywords and what should be base library.

The base library needs this functionality, as will the user, and it can either be through keywords that do it, or it can
be through objects that take in lambdas.

`atomic` and `volatile` are also an issue as they require detailed knowledge that most people don't know. Volatile isn't
even really useful in the way that C refers to it on modern architectures. Atomic is useful and required in some way. I
don't want to see every variable be atomic, that'd be too slow, but it also might be hard to use the language if users
need to declare atomic themselves. Current best bet is to have an `Atomic` trait and encourage forms of concurrency that
don't usually rely on shared state -- so, immutable inputs and then doing the processing of outputs in a single thread.

I would like to provide built in syntax for a concurrent loop, but maybe doing something through standard language
conventions is fine. This goal might be met like `mylist.parallel().foreach(|x|->{...})`, but it creates two conventions
for doing loops that would not look similar. Maybe `parallel for x: int in mylist {}`, as an attribute like `parallel`
could be applied to `while` loops. I had created a parallel loop in smirk, and I called it 'with', but I belatedly
realized that the keyword was confusing to those who came form python.

Also the ability to create futures from any method seems pretty convenient. Current syntax goal looks something
like `let x: future int = obj!do_something()` or if it isn't a keyword, it could be a generic
like `let x: future<int> = obj!do_something()`
This would be similar to some syntax like `let x: future<int> = threads::invoke(|| -> obj.do_something())`
if we used a lambda and passed it to a function that kicked off a thread and returned a future. As a starting point,
a `Future` trait will need to exist and so will a function that kicks off a thread and returns that Future. If we
eventually put syntactical sugar on it, then that is fine.

## Exceptions

In an effort to not design a language that permits what we jokingly refer to as exception-oriented-code, Dog allows you
to throw and catch exceptions using `fail` and `otherwise`, but does not create specialized exception types or place
data in the exception beyond a text message. This is to prevent developers from using exceptions for program flow
control in non-error scenarios -- like an old-school goto command.

Code that jumps from one method to a method three calls before is hard to read and not intuitive, so catch errors near
where they happen if you need to catch them at all.

If an exception can only happen due to a coding mistake, do not attempt to handle them. Let the program fail in testing
and add the appropriate code to avoid an exceptional condition in the future.

The otherwise block can make your code look cleaner when values are expected to be null some of the time. See below for
an example.

```
fn will_fail() {
  fail('I don't need a reason, but I'll give you one.')
}

  ...

  will_fail() otherwise {
    println('We ignored the failure.')
  } 
  
  {
    // do some things
    will_fail()
    // do more things
  } otherwise {
    println('We ignored the failure.')
  }
```

You'll notice that the otherwise block doesn't provide a copy of the message. This is intentional to prevent it from
becoming a delivery mechanism for data that would be used for flow control.

By not catching the exception, the program will print the message out when it fails.

In your config, you can add the variable `print_debug: true` and when an exception happens, it will print the message
and trace information even if the exception is caught in an otherwise block.

There are times, that an exception isn't actually thrown but the otherwise block is executed.

```
    let x: int = obj1.getObj2().getObj3().getInt() otherwise 0
```

If obj1 or any of the objects after that are null, the otherwise block is called, but the compiler recognizes this
scenario and since it is already checking for null (because a native program not checking for null is going to crash),
it simply skips to the otherwise block if a section is null.

This brings us briefly in contact with the "billion dollar mistake", as Tony Hoare puts it. Check out the section on 
[Why null?](#why-null).

Ideally, I'd like to implement the Itanium C++ ABI: Exception Handling standard, but would settle on sjlj.

[LLVM exception handling details](https://llvm.org/docs/ExceptionHandling.html)

[Read the Docs examples of exception handling](https://mapping-high-level-constructs-to-llvm-ir.readthedocs.io/en/latest/exception-handling/index.html)

# Why null?

I know, I know, you are super hoping that I don't use the word Monad here. 

First of all, I want to acknowledge that null references are a huge deal. In native code, they cause huge amounts
of heartache. In higher level languages, they are annoyances. Code must check for null or the code will fail -- but
at least it will fail in a way that the developer can determine what happened and add a new check.

The constant need to check for null is something of an anti-pattern. We want our code to flow cleanly and consistently,
so you'll see languages came up with the concept of `Optional` or `Maybe`, which is really just a wrapper object
around the original that you can reliably reference, and it gives you some conveniences around checking for and handling
the scenario of data not being available.

To accommodate this, I have two plans:
1. The otherwise keyword, which isn't only for exception handling -- it is a null check.
2. Compile-time enforcement of using null checks (like `otherwise`) when referencing a potentially null object.

For #2, you can always choose to explicitly call `fail()` if that is what you want, but by forcing you to be explicit,
we avoid unintentional consequences of failing at a point we didn't expect to.

Let's compare Rust-ish:
```
  if let Some(my_struct) = get_my_struct() {
    return my_struct.fetch_val();
  }
  return 0;
```

vs Dog:
```
  return get_my_struct().fetch_val() otherwise 0 
```

Each of the entities in our expression are automatically null-checked and we can easily see what will happen.

Languages like Java that added Optional in later only made things worse. The Optional itself can be null, and it 
then to pull out the value, you still need to check if something is present. Yes, it can help with legibility,
but it feels like a half-measure. Optional cannot be passed by reference to work -- it must always be by value
so that it can never be null. Java simply doesn't support pass by value for objects, and at the end of the day,
Optional is an object.

Now I'll hear, by option/optional/maybe give us great syntactical sugar to apply functions to data. It's burnt sugar.

You add a lot of syntax to the language that makes it harder to do well, in the hopes that once people know that
syntax they will do it well, but they may never learn and they still do it poorly.

By not having null-pointer exceptions, and forcing you to always handle null with `otherwise`, we don't add more 
syntax than we have to. You still can check if something is `null` using the `==` operator, but you aren't allowed
to dereference a null object without handling the result.

Every object reference is effectively optional in Dog without needing the extra syntax to say that it is optional and
without letting you get away with not handling null in any situation.

Parting shot at Scala and Rust: Putting a function like get() or unwrap() on a Option object that throws an exception 
if the value is null? The whole point was to force people to handle null safely but to then throw an exception anyway
--you did nothing! The shortest and easiest syntax runs into the exact problem you were trying to avoid. The syntax
people want to use should be the syntax that is safe and explicit. In Dog, you could `fail()` as part of your otherwise
but you are doing so explicitly--and I hope that this pattern is rare because exception-oriented code is something
I don't want to see in Dog.

## Object

We need a top level "Object" trait that helps us support reflection.

There will be some popular traits like "Hashable", "Comparable" (?), and "Textual" (Is there a better name for a trait
that has a as_string() function?), but the goal will be to keep Object for reflection and unknown type purposes only.

## Traits on primitives

There are so many times it will be useful to put a trait on a primitive or to handle passing a primitive by reference.

I think that each primitive will need a structure equivalent that we box to and unbox from to make traits work smoothly
with the least amount of custom code. We can add optimizations around boxing and unboxes for performance.

Auto-boxing can only happen to an immutable primitive wrapper. Mutable wrappers around an integer will exist for
calling to C functions, but you'll need to explicitly create them since their value may change during the call.

## Runtime Type Information

I think the most elegant (?!) way to handle this is to make a trait like `TypeInfo`.

Maybe?!?! Automatically generate it for all types that don't define it for themselves. I know that in the
[Object](#Object) section, I said there wouldn't be an Object class, and suddenly here is a class that would likely be
on everything. It could simply be optional, but it is so boiler plate that it would be easy to generate. Even more, it
might allow for methods that return the type traits more accurately than a user could. Like size of the object in
bytes (although this is a trap because of memory alignment -- so there are at least two sizes, one for how much it is
taking up in memory and one for how many bytes it represents. There would likely need to be a different size for how big
it was to serialize... though maybe that needs to be a different trait.)

## Log levels

In the config, we'll allow user defined log levels that can be an aggregate of other levels.

At compile time add or remove log statements if they don't match the log level.

Exact syntax pending, this is the general idea though:

```
    // somewhere in your config

    // specify the logging init function, it must return a trait of type Logger, 
    // the logger class will do the actual writing out of the text that is resolved
    // logger.write(text) will not be called if the logging level doesn't line up
    
    // if no init function is defined, the compiler use the default logger
    // from internal which in turn uses a C function puts() or fwrite/fprintf(stderr) 
    
    // a customer logger is welcome to use other properties from the config
    // to control its behavior (like picking a log folder or an output format, or whatever)
    log_init_function: std::log::default_logger() // likely only in your top level config
     
    // this step might be unnecessary, but I can imagine it making it easier to 
    // switch from one setup to another 
    // list of valid log levels and the other log levels they include  
    log_levels: [ // likely only in your top level config
        none(),
        release(),
        debug_io(),
        debug_other(),
        debug(debug_io, debug_other, release)
    ]
    
    // specify a list of log levels that are active 
    active_log_levels: [ // likely only in your top level config
        debug // pulls in debug_io, debug_other, release
    ]
```

I think this makes a good argument for a `log` keyword, because it would change how the code compiles in a way you
couldn't do at compile time.

Without a keyword we could do:

```
    // this can be optimized by a keyword, but would work without one
    log(debug, 'this is a debug message.')
    
    // one could use a lambda even if we didn't have a keyword:
    log(debug_io, || -> {
        let x: int = some_calculation()
        return 'hear me: ' + x 
    })
```

With a keyword we could do:

```    
    // however maybe a syntax like this?
    log.debug_io {
        let x: int = some_calculation()
        return 'hear me: ' + x 
    }
    // and
    log.debug('this is a debug message.') 
```

I'll think on it a little more. The drawback to a keyword like `log` is largely an increased amount of knowledge a user
needs to master the language. It also makes it impossible to have a function named "log", which is what happens when you
make a keyword. However, I think that logger.write() or .append() is fine.

## Code blocks that only get compiled for specific configs

The log keyword thoughts led me to another thought. There are applications that need to initialize differently when they
are run on a local machine rather than on a server or a different environment.

If you can have blocks of code that run based on a config, it would allow you to specify it in a way that is idiomatic
and reduces the compiled executable's size or library requirements.

Syntax TBD:

```
fn my_fun() {

    // if local is set to true in the config:
    config(local) {
        // do code specific to the local config  
    }
    
    // I don't like this syntax, but some sort of conditional check on a config value:
    config(server_name == 'hal' && !local) {
        // do code specific to local  
    }
}
```

There's some risks associated with changing what is compiled, but if we treat variables declared within the scope as
local to that scope, then it shouldn't prevent any given config permutation from compiling.

## Embedded Modules

For Dog, normally a module is everything in the same directory and it gets its module name from the directory path
relative to the config definition.

However, you can declare that a file has a different module or you can define a part of a file to have a different
module. This is useful for embedding tests near the source code.

For example:

```
//... somewhere after the code in the current file...

test mod my_module_tests {
    // use statements
    // this module can see private members of parent module

    test fn my_test_function() {
        //... test code above.
    }

}

```

## Why is 'fail' a keyword?

Fail could just be a function, so why is it a keyword?

Currently, the syntax isn't very exotic. You simply do:

```
    fail('Any reason I want to print')
```

This could be a function in `std` that then calls into `internal` that in turn raises an exception. However, here's the
catch, how does it raise an exception? It needs the compiler to do it, because how exceptions are implemented are
specific to the compiler. So, the only way for it to raise the exception that acknowledges compiler magic is to have a
keyword, and so, here we are.

Sure, we could not make it a keyword and when we see a function named `fail()` we could instead do the same thing, but I
think that it is better to be apparent when we are doing compiler magic. In this way, anybody can easily trace back what
is happening in their code all the way to the internal modules to see exactly how the language works. When it is a
keyword, they know they won't find the logic in a module, but in the compiler itself, which they are free to look at and
now know where to start.

This sort of transparency may help other people who are writing their own programming languages understand how
functionality like this is implemented without being led on a wild goose chase through internal libraries only to see
some other obscure way of communicating with the compiler.

## Optional Commas
When I first looked at how Rust allowed trailing commas, I thought that it was lazy from a parsing perspective. You could
with a few more lines of code arrange it so that commas were only valid between elements in a list. Then it occurred to me
that it was about creating a visual effect that every line was the similar and adding or moving lines became trivial.

Commas are not necessary from a parsing perspective in the cases that I could think of and they simply add noise. 

With commas (even notice the trailing comma for the last element that some languages allow):
```
config MyConfig {
    prop1: "value1",
    prop2: 123,
    prop3: true,
}
```

Without commas:

```
config MyConfig {
    prop1: "value1"
    prop2: 123
    prop3: true
}
```

It's roughly 5% neater, right? I can see why people might like them, but they aren't strictly necessary.

Even if expressions are involved, I haven't thought of a situation where we _need_ a comma since Dog doesn't have any
operators that take in space delimited arguments. If we made parenthesis or curlies optional for structure 
initialization or function calls then we'd run into trouble. However, I'm arguing that parenthesis and curlies stay put.

This all works, but you may still like having commas for legibility in some cases:

```
// this looks good to me:
let x = MyStruct { 
    prop1: 1 + x
    prop2: x * y
    prop3: true
}

// these are not as appealing to me, but they would work fine
let y = call_func_1(a b c)          
let z = call_func_2(x true "hi")    
let u = call_func_3(                
        "this is a bunch of text" 
        "and this is another bunch of text"
        num1)

```

Myself, I think I would use commas for function parameters because it feels natural to me, but the fewer extra symbols
on the page at once, the less clutter there is in many cases. Sometimes, you need those symbols for legibility,
sometimes you don't need them for the sake of neatness. I would like the developer using the language to decide which is
better for their use case.

## Labeled and default function parameters

One of my hobby languages supports labeled and default parameters and while I didn't really see it fitting 
with Dog initially, I'm leaning toward supporting it. 

The argument against it is that it makes it harder to tell which function will actually be called when you are
reading somebody else's code, if the language supports overloading. The secondary argument is that, in some cases,
it makes compiling dramatically slower and the resulting executable dramatically bigger.

To work as a binary library with roughly the same amount of flexibility, you have to generate the extra 
permutations of method signatures that take in fewer parameters and then call the original method from the 
generated one, passing the defaults. For a function with 10 parameters that all have default values, the number of
method signatures to support permutations is rather large. If the function isn't exported as a library, you can 
avoid the extra method hop and the optimizer removes unused functions before creating the binary. I think one can 
avoid the compiler overhead of generating the extra permutations in the case that you aren't generating a library. 

```
// this function signature has a default value for every parameterf 
pub fn my_fun_1( a: int = 2, b: String = null, c: float = 1.2) {
}

// so does this one
pub fn my_fun_1(d: MyStruct = null) {
}

...

    //later on
    my_fun_1() // which of the above gets called? 
    
    // Ha! Trick question! Neither. This wouldn't compile because the signatures would overlap.
    
    // However, you can create situations that do compile and take a moment to figure out.
    // You can also be frustrated that you can't get your code to compile when you try to overload
    // a function's name and the compiler refuses to cooperate because of the conflicting permutations.
    
    // A modern IDE would make this simple, so I'm not sure that this is a big deal.
    
```

Example of calling with a label:
```
    
    // oh, and here is a call using a label
    my_fun_1( b: "hello world!")
    // if you don't specify all of the paramters or you don't want to specify them in the same order, 
    // then you have to label each paramter
```
## reflection

Most languages provide facilities to do reflection while some do not. Reflection is generally slow and ugly, and largely
discouraged. Because it isn't the primary concern of the language, most of the languages that do support reflection
aren't designed to make reflection fast, because making it faster would results in compromises, mostly around how 
much memory needs to be allocated around objects at runtime. In a system that doesn't have much memory or with programs
that would already be memory intensive, this is undesirable.

However, there are certain features that are very difficult to accommodate without reflection because, in the act of
making things easier for the user, the code has to understand the context of other code.

Let's start with some basics. 

NOTE: Keywords here will shift around as we get farther along.

Checking if a object is of a specific type -- if  language calls itself OO, it probably supports this idea:
```
    if x isa fn {
        // x "is a" function
    } 
    
    if x isa fn(int, float) {
        // x "is a" function that takes in an int and a float
    }
    
    if x isa struct {
        // x "is a" struct
    }

    if x isa MyStruct {
        // x "is a" struct named MyStruct
    }
    
    if x isa MyTrait {
        // x "has a" trait named MyTrait (yes, I considered a separate keyword 'hasa')
    }    
```

When we do early binding on member variables, we are directly referencing the variable offset within that known structure,
but there are times when you don't know which structure/class you have in a block of code and will need to do late
binding.

I'm still working through the details, but for now, I'm going to reserve the `$` as an operator for internal functions 
to lookup relationships like this.

It is better to do an 'isa' check then directly reference the methods after casting your struct to the appropriate
concrete type, but maybe you can't do that for some reason. We need a way to invoke methods and get properties.
```
    //NOTE: this exact code isn't right. 
    // I need to figure out implementation details, it's just conceptual at this point.
    if my_obj.$has('property') {
        let val: Object = my_obj.$get('property') // here is a situation where we need an 'object' type
        my_obj.$set('property', 1) // we need a generic object set here
        my_obj.$invoke('function', 'arg1', arg2, 8) // variadic invoke function. Would we do this?
    }
```

However, what if we didn't know if a function existed? We'd need to be able to ask for an array
of function objects:
```
  // maybe? This is a different concept than a lambda where we have a known signature. 
  // Here, we have no idea what the arguments or the return will be.
  let funcs: [Function] = my_obj.$functions()
```
We'd need to be able to invoke them. Maybe:
```
  // if we do this, will we run into parsing nightmares?
  let result = func[0](arg1, arg2, 42) 
  
  // if we do this, it is not as pretty, but parsing is easier
  let result = func[0].invoke(arg1, arg2, 42)
  
  // then we have to ask ourself what type is result... that brings us back to needing a
  // generic 'object' trait... that could hold a primitive... which is maybe where we
  // shove the $ functions.
  
  // we'd need to auto generate the object trait for all types, so the user doesn't have to
  // or can't specify object on it's own. 
```


What if we want to introspect on functions, structure types, traits, enums, or entry points?

```
  let my_traits: [Trait] = my_obj.$traits() //??
  let attrs = my_traits[0].attributes() // we don't need a $ here because we are on a first class object, the method name wouldn't collide
```


## Querying metadata
When working through the attribute design, I was pondering this:

String labels to look up an attribute:
```
    // done in two lines to show the type that is cast
    let y: Attribute = my_function.attribute("MyAttribute")
    let x: MyAttribute = y as MyAttribute 
```

vs a compile-time name:
```
    let x: MyAttribute = my_function.attribute(MyAttribute)
```

The first option doesn't require any special logic for the compiler to understand what we are doing, but there is
no compile-time check to ensure they typed the right attribute name we need a simple string lookup to return an
Attribute trait this isn't specific. 

The second option uses a generic to infer the return type and has compile-time checking to use attributes that are
known.

It seems obvious to me that we should allow the ability to pass an attribute by name rather than by string. 
This doesn't seem like anything special compared to other languages.

However, I began thinking: What if we went a step further?

This would work with what we've already talked about:
```
    if "interesting" == (myfun.attribute(MyAttribute).metadata otherwise null) {
        // do stuff
    }
    
    // or this:
    if "interesting" == myfun.attribute(MyAttribute).metadata {
        // do stuff
    } otherwise {
        // an exception occurred because the attribute didn't exist, but we can do something here with this info
    }
```

However, wouldn't this be easier to read?
```
    if "interesting" == myfun.attribute_value(MyAttribute::metadata1) {
    }
```

In a language like Java, you would check if an annotation exists, get it, cast it to the type you want, 
and then you'd access the property. With the Dog `otherwise` syntax, it is abbreviated, but by letting the compiler
know that a function, structure, or property can be passed around, it can compile-time validate names and 
reduce the effort to use the syntax.

## Distributed Parallelism using existing serverless and object storage technologies

Having the ability to flatmap/map/filter/fold/reduce/apply/etc is great to not only help with parallelism locally,
but lends itself to distributed computations. 

The traditional way to do distributed computing is to have your app running on multiple pieces of hardware and use
network protocols to coordinate the work. Ideally, the data is already available to each node so that you reduce
the overhead of moving data.

I know that I will seem a bit AWS focused below, but the gist is because AWS makes it easier to support any language and 
we have to start some place. I would love it if Azure and Google made it possible to support dog without doing
things that are ugly.

### Serverless support
There are very specific types of jobs that work well as serverless -- short running apps that deal with reasonably
small amounts of data. Could we break that mold?

We should be looking beyond EC2 instances and look to serverless technology. You could write `app`s that run as a
AWS lambdas to process data -- though we'd need to consider how we scale out in this case, but it is possible and it
would be cheaper in many cases than spinning up instances.

The less overhead it takes to start up a Dog application, the easier it is for it to be run as a Lambda, and the better
it positions us for serverless.

As long as starting a Dog application is quick, and isn't memory intensive, it should work well. The real challenge
with serverless is each instance is rather small and the caller must tell you what work to do -- because coordinating
work between instances can't be done real-time. Lambdas also are only designed to run for short periods of time (15 
minutes or less as of this writing.)

It is possible to have one lambda call another lambda, but there are complexities. For one, the developer would have
to deploy and configure multiple lambdas, each specialized to the work, which puts the onus of how to scale back on
the developer. For processing lots of data, this doesn't make Dog any more useful than any other language.

My thought is that it is possible to invoke lambdas asynchronously, and if you know which lambda you are, you could
invoke other copies of yourself but with instructions to only do a portion of the work. The exact design behind this
would require some effort. 

The primary node would take in the request, spin off worker lambdas, and then collate the result. There are complexities
in that a lambda can potentially run multiple times or fail without running at all.

Serverless feels like it is gaining traction and we should evaluate precisely how we accommodate it. The cost savings
are large for people who use it, but performing complicated functions over it is an even larger technological
challenge. We are in it for the challenge, aren't we?

To start with, our goal should be to support being a first class lambda language. Targeting AWS initially because
they support arbitrary languages out of the box. Azure and Google don't support arbitrary languages, so we'd either 
need to be made a first class language on their platforms, or put a NodeJS wrapper around our app that invokes us 
through HTTP or we'd need a transpiler -- yuck. There might be some other solution that I haven't seen.

I think that is is possible here to create an expansion of the serverless movement at this time by pushing the 
boundaries of what can be done.

### Traditional distributed support
Running on hardware instances that were spun up specifically for you is much easier than being serverless. It's always
easier to do what everybody else is already doing.

In the case of a short-running application, the primary instance is started and then the secondary instances could be 
started and given the IP of the primary node, or could be configured with an ip beforehand, or there could even be 
discovery of the primary through some sort of UDP exchange. Details would need to be worked out, or possibly multiple 
of these options would work. In the end, one or more secondary instances could connect to the primary to off-load 
parallel work when that work was done on files.

For programs that are of type `service` that handle adhoc requests indefinitely, maybe it is the same executable, and 
you simply deploy it to multiple clusters and you let them figure out which is the current master and which are the 
secondary. This would require UDP broadcasts for discovery and determining who is the current primary node, and would 
certainly increase the complexity, but it would give you a much more durable cluster. I don't think this type of 
complicated system lends itself to a short running batch process (those of type `app`), which if it fails, the code 
invoking the app would look at the error and determine if you simply start the primary app up again or if the error is 
reported as-is.
 
I don't know all of the details on exactly how other apps manage this, but it doesn't seem too hard to imagine. The 
primary node needs to coordinate the distribution of work -- and if the work is monoid in nature and the data is already 
available to the secondary instances, that distribution can be done quickly.

In the era of cloud object stores for data lakes, data doesn't ever start "near" compute, the way it did with old-school
on-prem hadoop clusters. However, that doesn't mean that ever processing instance needs to pull all of the data near
to it. Files are generally already partitioned -- or should be -- in reasonable chunks that each instance could pull
what it needs.

For a dataframe that is backed by a folder of files, doing this parallel work seems easy enough. For your average
local CSV file or even a large, in memory list... that's much trickier. The cost of distributing the data over the
network may be worse than simply doing a computation locally. However, if there will be repeated calculations? Maybe
there would be an advantage of distributing the read-only data once to secondary instances and then doing the calculations.

These secondary apps could be generated automatically if the configuration specifies that they need to exist. They 
would be very specialized to know how to connect to the primary instance, what types of requests could be done,
and how to do them, after all, their very existence would be defined as part of the codebase.

From an api perspective, it might look like this:
```
  let df: DataFrame = DataFrame::from('s3:\\some-bucket\folder\`)
  let distributed_df: DataFrame = df.distribute() // now the compiler somehow knows that if there are secondary nodes, they can help
  // do something here with the distributed dataframe that leverages all of the nodes  
```

This same thing could work for lists/maps/etc. Or, instead of being explicitly distributed like the above. It could
be when the config says that the app is distributed, then all collections are distributed automatically, but work
the primary worker would have to only ask the secondary nodes for help when it was likely it wouldn't be slower than
just doing the work without being distributed.

I like the option of saying in the config that the app is distributed more than declaring something is distributed in 
the code, since it makes it less flexible to switch between being stand-alone and distributed.
```
  // config is set that the code should be distributed in the prod config, so the below code is using a distributed
  // dataframe, however it is not using a distributed dataframe for local development. 
  let df: DataFrame = DataFrame::from('s3:\\some-bucket\folder\`)
```

Why not support HDFS or a similar distributed file system? Maybe we should, but the reason it isn't high priority in
my mind is that HDFS requires expensive semi-fixed infrastructure. You COULD run HDFS on EC2 and you CAN scale it up
and down, but the cost and complexity is not the same as simply pointing at an S3 bucket.

Files that are sized well (100 megabytes?) can be quickly pulled from s3 to an ec2 instance and you can autoscale that
ec2 cluster trivially to add compute. Scaling down with our model works fine as well, as long as when we lose connectivity
from a worker that we delegate the work to a new worker.

What about consistency? Object storage consistency has been steadily improving (https://aws.amazon.com/s3/consistency/)
and reducing the requirements of distributed file systems.

I think that Dog should be positioned to put the cloud first over on-prem, but even on-prem, you can mount network drives.
Yes, consistency is still an issue, and performance is still an issue, but simplicity is also important. I expect
the continued evolution and improvement of commodity storage. The whole advent of data processing was spun up by Google's
desire to use commodity hardware. Think about how we've moved away from monolithic compute with specialized hardware
to support massive amounts of IO on a single device? Then there was the pull to create clusters leveraging 
complicated-to-configure software to overcome the lack of support for massive amounts of data. However, now we have
object storage that can handle massive amounts of data, and yeah, it isn't perfect, but for most purposes, it works.

Being able to run on popular Operating Systems like Linux, Windows, and Mac, along with being able to support popular
cloud object storage, should make it easier to create, scale, and operate applications written in Dog. The more 
need to install and configure additional software makes it harder to adopt and use Dog. We are at the point where 
people know how to create EC2 instances easily, and they know how to scale them up. We should be able to leverage that
expertise without additional complexity. 


## Period vs Double Colon

There are some functions called like `my_trait.do_something()` while others look like `MyTrait::do_something()`, what's 
the difference. The double-colon signifies a reference to a constant. The function belongs in the namespace of a trait,
but isn't actually part of a given instance. A period however indicates that the reference is to something that is in
an instance and is aware of member variables.

The syntax difference is simply to ensure that the developer is aware of what they are doing, and that they are doing
what they are doing intentionally. You cannot override a constant function, but you could have a different trait with
the same function name. You also constant functions must be referenced with the namespace they are in, not an instance
of that namespace.

Given:
```
trait MyTrait {
  const fn do_something(): int {
    return 5
  }
}
```

This is invalid:
```
  let my_instance: MyTrait = ...
  
  let x: int = my_instance::do_something() // invalid: can't call a const function from an instance
```

This is valid:
```
  let x: int = MyTrait::do_something() // valid: can call a const function from trait
```