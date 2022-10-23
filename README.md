# The JavaScript to LLVM IR Compiler

## Requirements
[rust](https://www.rust-lang.org)

[llvm-project](https://github.com/llvm/llvm-project.git)

[cmake > 3.5](https://cmake.org)

## Instal llvm from source (optional)
If you are using sophisticated OS which is not handled during build or for any other purposes you can build llvm from source.
Full guide you can find in the [llvm-project](https://github.com/llvm/llvm-project.git) repository.
Note, that you will need a `release/14.x` branch as a correct version.

After that you can easilty build compiler using 
```
cargo b
```

For the active contribution into this project it is most convinient and preferable way.

## Build
```
cargo b --config compiler/config.toml
```
