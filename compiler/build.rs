fn main() {
    cc::Build::new()
        .file("../c/variable.c")
        .file("../c/logical.c")
        .file("../c/assertions.c")
        .out_dir("../c/out")
        .cargo_metadata(false)
        .compile("llvm-js-lib")
}
