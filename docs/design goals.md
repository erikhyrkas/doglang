#Design goals of Dog

See more details here: [Design Decisions](design%20decisions.md)

##Goal statement
Process and move data efficiently and with little effort.

##Secondary goals
* Safe
* Easy
* Fast
* Flexible  
* Cross platform

##Process Data
First-class SQL support, dataframes, and a collections API all let you operate on data using a familiar syntax.

##Moving Data
Support for common file formats (csv, parquet, json, etc.), cloud object-store support (s3/adfs), REST/HTTP API support, 
and ODBC database connectivity all allow data to be extracted from a source, transformed, and loaded into a new 
destination.

##Easy
Simple and repeatable syntax patterns that are familiar without burdening the user with a deep understanding of memory, 
threads, sockets, registers, or serialization. 

Legible syntax that prefers keywords over symbols.

Simple approach to structures and traits that avoids inheritance pitfalls and encourages delegation.

No need to configure garbage collector or the heap.

##Safe
Garbage collection, exception handling, resource management, and constructs to encourage safe concurrency.

##Fast
Produces natively compiled executables optimized to avoid unnecessary register clears, expensive heap allocations and 
de-allocations, and maximize concurrency. 

Support for GPUs is a stretch goal.

## Cross Platform
Support for common and popular operating systems and cpu architectures. The initial goal for OS support is for 
Windows, Macos, and Linux. The initial cpu architecture goal is x86 and x64, with ARM as a stretch goal.

