extern crate bindgen;

use std::env;
use std::path::PathBuf;
use cmake::Config;

fn make_builder<T: std::fmt::Display>(c_lib_path: &T, header: &str) -> bindgen::Builder {
    let include_args = vec!["-isystem".to_string(), format!("{}/include/", c_lib_path)].into_iter();
    bindgen::Builder::default()
        .clang_args(include_args)
        // The input header we would like to generate
        // bindings for.
        .header(format!("{}/include/{}", c_lib_path, header))
        // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
}

fn make_gnss_time_bindings<T: std::fmt::Display>(c_lib_path: &T) -> bindgen::Bindings {
    make_builder(c_lib_path, "swiftnav/gnss_time.h")
        .whitelist_type("gps_time_t")
        .whitelist_function("gpsdifftime")
        .whitelist_function("gps_time_valid")
        .whitelist_function("add_secs")
        .whitelist_var("FLOAT_EQUALITY_EPS")
        .whitelist_var("WEEK_SECS")
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings")
}

fn main() {
    let dst = Config::new("third-party/libswiftnav/")
        .build();

    // Tell cargo to tell rustc to link the system bzip2
    // shared library.
    //println!("cargo:rustc-link-lib=bz2");
    println!("cargo:rustc-link-search=native={}/lib/", dst.display());
    println!("cargo:rustc-link-lib=static=swiftnav");

    let gnss_time_bindings = make_gnss_time_bindings(&dst.display());

    // Write the bindings
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    gnss_time_bindings
        .write_to_file(out_path.join("gnss_time_bindings.rs"))
        .expect("Couldn't write bindings!");
}
