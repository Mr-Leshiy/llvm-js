fn main() {
    cc::Build::new()
        .cpp(true)
        .file("../c/variable.cpp")
        .file("../c/logical.cpp")
        .file("../c/assertions.cpp")
        .out_dir("../c/out")
        .cargo_metadata(false)
        .compile("llvm-js")
}
