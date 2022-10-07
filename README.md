# The JavaScript to LLVM IR Compiler

## Requirements
[rust](https://www.rust-lang.org)

[cmake > 3.5](https://cmake.org)

## Build
### Macos/Linux
Install llvm library
```
cmake -B build -DLLVM=ON
cd build
make
make install
```

```
cargo b
```