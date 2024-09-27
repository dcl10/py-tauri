// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::fs;

use pyo3::prelude::*;
use tauri::AppHandle;

#[tauri::command]
fn python_add(a: f32, b: f32, app_handle: AppHandle) -> Result<f32, String> {
    let py_mod_buf = app_handle
        .path_resolver()
        .resolve_resource("python/mod.py")
        .expect("Could not get python module");
    let py_path = py_mod_buf.to_str().unwrap();
    let py_mod = fs::read_to_string(py_path).expect("Couldn't read in python module");
    // let py_mod = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/python/mod.py"));
    let result = Python::with_gil(|py| -> Result<f32, String> {
        let module = PyModule::from_code_bound(py, &py_mod, "mod.py", "mod").unwrap();
        let add = module.getattr("add").unwrap();
        let results = add.call((a, b), None).unwrap();
        if let Ok(answer) = results.extract::<f32>() {
            Ok(answer)
        } else {
            Err("Unable to extract results from `mod.add`".to_string())
        }
    });
    result
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![python_add])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
