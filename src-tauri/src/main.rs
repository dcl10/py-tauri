// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use core::str;
use std::process::Command;

mod python;

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
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
