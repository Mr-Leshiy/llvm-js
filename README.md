# The JavaScript to LLVM IR Compiler

## Requirements
[rust](https://www.rust-lang.org)

[llvm-project](https://github.com/llvm/llvm-project.git)

[cmake > 3.5](https://cmake.org)

For the active contribution into this project it is most convinient and preferable way.

## Build
### Linux/Macos
Build C/C++ dependencies
```
cmake . -B build -DLLVM=ON
cmake --build build --parallel 2
cmake --install build
```
Run build 
```
cargo b
```
Run tests
```
cargo test
```
Run
```
jsc --input=test_scripts/basic.js --binary-name=run --clean
```
