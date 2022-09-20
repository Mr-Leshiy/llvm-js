fn main() {
    cc::Build::new()
        .cpp(true)
        .flag("-std=c++17")
        .file("../c/variable.cpp")
        .file("../c/logical.cpp")
        .file("../c/assertions.cpp")
        .out_dir("../c/out")
        .cargo_metadata(false)
        .compile("llvm-js")
}
