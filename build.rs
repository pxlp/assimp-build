use std::env;

fn main() {
    let module_path = env::var("CARGO_MANIFEST_DIR").unwrap();
    println!("cargo:rustc-link-search=native={}", module_path);
}
