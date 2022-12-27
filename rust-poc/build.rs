extern crate bindgen;

use cmake::Config;
use glob::glob;

fn main() {
    const LIB_PATH: &str = "../simple-lib";
    const LIB_NAME: &str = "SimpleLib";

    let cmake_directory = Config::new(LIB_PATH)
        .always_configure(true)
        .build_target(LIB_NAME)
        .build();

    let lib_build_directory = cmake_directory.as_path().join("build");
    println!(
        "cargo:rustc-link-search={}",
        lib_build_directory
            .to_str()
            .expect("Unable to convert lib build directory to str")
    );

    // Tell cargo to tell rustc to link lib
    println!("cargo:rustc-link-lib={}", LIB_NAME);

    // Tell cargo to invalidate the built crate whenever the lib changes
    println!("cargo:rerun-if-changed={}", LIB_PATH);

    // get all headers
    let header_pattern = LIB_PATH.to_owned() + "/include/*.h";
    let header_paths = glob(&header_pattern)
        .map(|paths| {
            paths
                .filter(|path| path.is_ok())
                .map(|path| path.unwrap().as_path().to_owned())
        })
        .expect("There was a problem collecting the header files")
        .filter_map(|path| path.to_owned().to_str().map(|temp_str| temp_str.to_owned()));

    // generate bindings
    let mut bindings_builder = bindgen::Builder::default();
    for header in header_paths {
        bindings_builder = bindings_builder.header(header);
    }
    bindings_builder = bindings_builder.parse_callbacks(Box::new(bindgen::CargoCallbacks));
    let bindings = bindings_builder
        .generate()
        .expect("Could not generate bindings correctly");

    bindings
        .write_to_file("src/bindings.rs")
        .expect("Could not write bindings");
}
