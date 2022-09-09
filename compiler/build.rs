fn main() {
    cc::Build::new()
        .file("../c/variable.c")
        // .out_dir("../c/out")
        .compile("foo")
}
