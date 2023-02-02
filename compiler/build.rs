use assembler::{CORE_LIB, FMT_LIB};
use std::{env, fs, path::Path};

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let out_dir = Path::new(out_dir.as_str());

    let core_lib = out_dir
        .join("build")
        .join("lib")
        .join(format!("lib{CORE_LIB}.a"));
    let fmt_lib = out_dir
        .join("build")
        .join("lib")
        .join(format!("lib{FMT_LIB}.a"));

    let dest = out_dir
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .parent()
        .unwrap();

    let cmake = cmake::Config::new("..").no_build_target(true).build();

    println!("{}", cmake.display());

    std::fs::File::create(dest.join(CORE_LIB)).unwrap();
    fs::copy(core_lib, dest.join(CORE_LIB)).unwrap();

    std::fs::File::create(dest.join(FMT_LIB)).unwrap();
    fs::copy(fmt_lib, dest.join(FMT_LIB)).unwrap();
}
