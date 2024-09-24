// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use core::str;
use std::process::Command;

use pyo3::prelude::*;

mod python;

#[tauri::command]
fn python_add(a: f32, b: f32) -> Result<f32, String> {
    let py_mod = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/python/mod.py"));
    let result = Python::with_gil(|py| -> Result<f32, String> {
        let module = PyModule::from_code_bound(py, py_mod, "mod", "mod").unwrap();
        let add = module.getattr("add").unwrap();
        let results = add.call((a, b), None).unwrap();
        if let Ok(answer) = results.extract::<f32>() {
            return Ok(answer);
        } else {
            Err("Unable to extract results from `mod.add`".to_string())
        }
    });
    result
}

fn main() {
    tauri::Builder::default()
        .setup(|_app| {
            use crate::python::{check_python_version, PYTHON_SUFFIXES};
            for py in PYTHON_SUFFIXES.into_iter() {
                let exe = format!("python{}", py);
                match Command::new(&exe).args(&["--version"]).output() {
                    Ok(output) => {
                        if let Ok(version) = str::from_utf8(&output.stdout) {
                            let v = version
                                .split(" ")
                                .last()
                                .expect("Couldn't get python version");
                            if check_python_version(v) {
                                println!("Got a good version of Python");
                            } else {
                                println!("Wrong or no python installed");
                            }
                        }
                    }
                    Err(e) => println!("{:?}", e),
                }
            }
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![python_add])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
