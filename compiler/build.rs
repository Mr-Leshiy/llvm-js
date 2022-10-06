fn main() {
    let cmake = cmake::Config::new("..")
        .out_dir("..")
        .no_build_target(true)
        .build();
    println!("{}", cmake.display());
}
