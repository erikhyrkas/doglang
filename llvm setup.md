Install LLVM

* Install Python 3

https://www.python.org/downloads/release/python-390/

* Install CMake

https://cmake.org/download/

* Install Visual Studio 2019 with C++
* Create LLVM Visual Studio Project

At command line:

```
cd \
mkdir llvmsrc
cd llvmsrc
git clone https://github.com/llvm/llvm-project.git
cd llvm-project
mkdir build
cd build
cmake -Thost=x64 -DCMAKE_BUILD_TYPE=Release -DLLVM_ENABLE_PROJECTS=lld -G "Visual Studio 16 2019" ../llvm -B /llvm
```

* Open LLVM.sln with Visual Studio
* Click Build

Expect this to take 20-30 minutes.

* IMPORTANT: Add `c:\llvm\debug\bin` to `Path` environment variable