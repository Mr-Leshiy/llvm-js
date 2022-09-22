fn main() {
    cc::Build::new()
        .cpp(true)
        .flag("-std=c++17")
        .file("../c/variable.cpp")
        .file("../c/logical.cpp")
        .file("../c/assertions.cpp")
        .file("../c/arithmetic.cpp")
        .out_dir("../c/out")
        .cargo_metadata(false)
        .compile("llvm-js")
}
