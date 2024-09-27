use std::env;

fn main() {
    let python_path = env::var("CUSTOM_PYTHON")
        .expect("Please specify CUSTOM_PYTHON to static Python lib directory");
    println!("cargo:rustc-link-lib=static=python3.11");
    println!("cargo:rustc-link-search=native={}", python_path);
    tauri_build::build()
}
