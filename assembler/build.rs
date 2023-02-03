use std::{env, fs, path::Path};

const CORE_LIB: &str = "core";
const FMT_LIB: &str = "fmtd";

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let out_dir = Path::new(out_dir.as_str());

    let profile = env::var("PROFILE").unwrap();

    #[cfg(any(target_os = "linux", target_os = "macos"))]
    let core_lib_name = format!("lib{CORE_LIB}.a");
    #[cfg(any(target_os = "windows"))]
    let core_lib_name = format!("{CORE_LIB}.lib");
    let core_lib = out_dir.join("build").join("lib");
    #[cfg(any(target_os = "windows"))]
    let core_lib = match profile.as_str() {
        "release" => core_lib.join("Release"),
        "debug" => core_lib.join("Debug"),
        _ => panic!(),
    };
    let core_lib = core_lib.join(core_lib_name.as_str());

    #[cfg(any(target_os = "linux", target_os = "macos"))]
    let fmt_lib_name = format!("lib{FMT_LIB}.a");
    #[cfg(any(target_os = "windows"))]
    let fmt_lib_name = format!("{FMT_LIB}.lib");
    let fmt_lib = out_dir.join("build").join("lib");
    #[cfg(any(target_os = "windows"))]
    let fmt_lib = match profile.as_str() {
        "release" => fmt_lib.join("Release"),
        "debug" => fmt_lib.join("Debug"),
        _ => panic!(),
    };
    let fmt_lib = fmt_lib.join(fmt_lib_name.as_str());

    let dest = out_dir
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .parent()
        .unwrap();

    cmake::Config::new("..").no_build_target(true).build();

    std::fs::File::create(dest.join(core_lib_name.as_str())).unwrap();
    fs::copy(core_lib, dest.join(core_lib_name.as_str())).unwrap();

    std::fs::File::create(dest.join(fmt_lib_name.as_str())).unwrap();
    fs::copy(fmt_lib, dest.join(fmt_lib_name.as_str())).unwrap();
}
