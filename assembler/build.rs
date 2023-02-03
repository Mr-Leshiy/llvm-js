use std::{env, fs, path::Path};

const CORE_LIB: &str = "core";
const FMT_LIB: &str = "fmtd";

fn prepare_lib_file(lib_name: &str) {
    let out_dir = env::var("OUT_DIR").unwrap();
    let out_dir = Path::new(out_dir.as_str());

    #[cfg(any(target_os = "linux", target_os = "macos"))]
    let lib_name = format!("lib{lib_name}.a");
    #[cfg(any(target_os = "windows"))]
    let lib_name = format!("{lib_name}.lib");

    let lib = out_dir.join("build").join("lib");
    #[cfg(any(target_os = "windows"))]
    let lib = match env::var("PROFILE").unwrap().as_str() {
        "release" => lib.join("Release"),
        "debug" => lib.join("Debug"),
        _ => panic!(),
    };
    let lib = lib.join(lib_name.as_str());

    let dest = out_dir
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .parent()
        .unwrap();

    std::fs::File::create(dest.join(lib_name.as_str())).unwrap();
    fs::copy(lib, dest.join(lib_name.as_str())).unwrap();
}

fn main() {
    cmake::Config::new("..").no_build_target(true).build();

    prepare_lib_file(CORE_LIB);
    prepare_lib_file(FMT_LIB);
}
