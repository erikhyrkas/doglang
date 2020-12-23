# New Project

# Single file program

A dog program could be a single file with the extension of `.dog`, however
this isn't useful for anything but the simplest use cases. When you compile
a single file, a default configuration is used for all settings.

```
> doglang compile simple.dog
```

This will produce the executables and libraries defined by the file's entry
points. Yes, a single file can produce multiple outputs. Read more in the 
[Entry Points](#Entry-Points) section.

# Project Structure

If you have a more complicated application or library, you will will need multiple
files and possibly multiple folders.

The easiest way to create a new project is by typing:
```
> doglang generate my-project-name app
```
This will creat a new folder with a config file and a `.dog` file that holds the
entry point.
```
project-folder
+-- config.dog
+-- main.dog
```
The config file holds any project-level configuration that the compiler would need
to know about to compile and link it, including which external libraries to include.

Variables defined in the config are global and can be used in any file in the project.

A project may have multiple config files. If a configuration is not specified, then 
all configurations are built.

Each non-config file belongs so to a module in this project based upon the folder it is
in or through explicitly setting the module membership using a line like 
`mod my-module-name::my-sub-module-name`. See the [Modules](#modules) section.

## Entry Points

### Console Application
keyword: app

Creates a standard console application that can take in parameters.

```
app MyApp(args:[string]) {
}
```

Args is optional, so it can be written like:

```
app MyApp() {
}
```

### Library
keyword: lib

Creates a dog library that can be used with other dog programs.

### Service
keyword: service

Creates a service that can take in socket connections concurrently.

### UI
keyword: ui

Creates an application with a graphical user interface.

TODO: this is lower priority and may not be implemented for some time.


# Modules

A module belongs to a project and is a collection of files, usually in the same folder, that represent
a single feature or feature category.

All files in the same module can see the public members of that module without an explict `use` statement.

Files in the top most folder are in the `default` module. If this project is a library, that module is 
represented as the project's name.

## mod keyword

A file's module can be specified like this on the first non-comment line:
```
mod overide-module-name
```

In general, the use cases for overriding the default module name are limited and it should be avoided.

## use keyword

At the top of a file, after the optional `mod` statement, you can specify `use` statements that tell the compiler explicitly where a function, 
structure, trait, etc. are from and even create an alias name to resolve conflicts.

This will make all public members of the module std::collections visible in this file. 
```
use std::collections
```

If you only want a single trait available, you can be more explict:
```
use std::collections:List
```

Or, if you want multiple traits from the same module:
```
use std::collections:{List, Map}
```

## default use
Even if you specify no use statements, the compiler assumes there are a few you would want:
```
use std
```

TODO: are there others? `std` should definitely be a default, should `std::collections` be a default?

## Config

```
config Release {
    version: string = "1.0.0"
    my_prop: int 12
}

// pull all properties from Release and overwrite any values specified here
// you cannot declare any custom properties that weren't already declared
// built in properties exist whether you declare them or not, so you could
// overwrite the default without explicitly setting it in the main build.
config Debug : Release { 
    // version is pulled from Release
    my_prop: int = 42 // it is not 12 in the debug build, it is definitely 42
}
```

In that example `my_prop` is a custom property that the code can use, where version is a built-in
property that exists even if you don't specify it, but you'd get a default value of "0.0.1".

