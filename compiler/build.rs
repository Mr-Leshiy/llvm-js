fn main() {
    cc::Build::new()
        .file("../c/test.c")
        // .out_dir("../c/out")
        .compile("foo")
}
