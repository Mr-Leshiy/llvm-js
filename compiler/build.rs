fn main() {
    cc::Build::new()
        .file("../c/variable.c")
        .out_dir("../c/out")
        .cargo_metadata(false)
        .compile("llvm-js-lib")
}
