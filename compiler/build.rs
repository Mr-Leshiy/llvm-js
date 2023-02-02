use assembler::{CORE_LIB, FMT_LIB};
use std::{env, fs, path::Path};

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let out_dir = Path::new(out_dir.as_str());

    let core_lib_name = format!("lib{CORE_LIB}.a");
    let core_lib = out_dir
        .join("build")
        .join("lib")
        .join(core_lib_name.as_str());

    let fmt_lib_name = format!("lib{FMT_LIB}.a");
    let fmt_lib = out_dir
        .join("build")
        .join("lib")
        .join(fmt_lib_name.as_str());

    let dest = out_dir
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .parent()
        .unwrap();

    let cmake = cmake::Config::new("..").no_build_target(true).build();

    println!("{}", cmake.display());

    std::fs::File::create(dest.join(core_lib_name.as_str())).unwrap();
    fs::copy(core_lib, dest.join(core_lib_name.as_str())).unwrap();

    std::fs::File::create(dest.join(fmt_lib_name.as_str())).unwrap();
    fs::copy(fmt_lib, dest.join(fmt_lib_name.as_str())).unwrap();
}
