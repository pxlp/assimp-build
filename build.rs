extern crate cmake;
extern crate pkg_config;

use cmake::Config;
use std::env;
use std::process::Command;
use std::path::Path;
use std::fs;
use std::io::ErrorKind;

fn main() {
    let target = env::var("TARGET").unwrap();
    let generator = env::var("GENERATOR");

    // Compile assimp from source
    // Disable unnecessary stuff, it takes long enough to compile already
    let mut cfg = Config::new("assimp");
    cfg.define("ASSIMP_BUILD_ASSIMP_TOOLS", "OFF")
        .define("ASSIMP_BUILD_TESTS", "OFF")
        .define("BUILD_SHARED_LIBS", "OFF")
        .define("CMAKE_BUILD_TYPE", "Release")
        .define("LIBRARY_SUFFIX", "");
    if let Ok(generator) = generator {
        cfg.generator(generator);
    }
    let dst = cfg.build();
    println!("cargo:rustc-link-search=native={}", dst.join("lib").display());

    println!("cargo:rustc-link-lib=static=assimp");

    // Link to libstdc++ on GNU
    if target.contains("gnu") {
        println!("cargo:rustc-link-lib=stdc++");
    }

    println!("cargo:rerun-if-changed=build.rs");
}
