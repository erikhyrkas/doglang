# doglang

Dog language written in Rust. This is the "bootstrap" compiler.

A bootstrap compiler is the initial implementation of a language in a different language, before
it is capable of compiling itself. If the language gets far enough, eventually the bootstrap
compiler will not be maintained and only exist for posterity sake. It turns out, in this case, that 
the chicken comes first, then the egg, and then the new chicken.

## Setup Instructions

* Install git: https://git-scm.com/downloads
* Install LLVM [LLVM setup](llvm%20setup.md)
* Install Rust [Rust setup](rust%20setup.md)
* Install CLion [CLion setup](clion%20setup.md)
* Clone: https://github.com/erikhyrkas/doglang.git

## Read the Docs

[Dog docs](https://github.com/erikhyrkas/doglang/blob/main/docs/README.md)

## Helpful links
Rust:
* https://doc.rust-lang.org/rust-by-example/index.html
* https://doc.rust-lang.org/std/index.html
* https://github.com/rust-lang/rust
* https://doc.rust-lang.org/nomicon/index.html
* https://m-decoster.github.io//2017/01/16/fighting-borrowchk/ 
 
Utility:
* http://ellcc.org/demo/index.cgi
* https://mapping-high-level-constructs-to-llvm-ir.readthedocs.io/en/latest/README.html

Language comparison:
* https://www.programming-idioms.org/
  
LLVM more specifically:
* https://crates.io/crates/llvm-sys
* https://releases.llvm.org/2.7/docs/LangRef.html
* https://llvm.org/docs/tutorial/index.html
* http://llvm.org/doxygen/