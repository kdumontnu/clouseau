// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::sync::{Arc, Mutex};
use tauri::State;
use sysinfo::System;

struct SystemInfo(Arc<Mutex<System>>);

fn main() {
    tauri::Builder::default()
        .manage(SystemInfo(Arc::new(Mutex::new(System::new_all()))))
        .setup(|_app| {
            #[cfg(debug_assertions)] // only include this code on debug builds
            {
                let window = tauri::Manager::get_window(_app, "main").unwrap();
                window.open_devtools();
                window.close_devtools();
            }
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![cpu_info, cpu_usage])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

// TODO: use event & stream to avoid recreating the SYS object?
#[tauri::command]
fn cpu_info<'r>(arc_sys: State<'r, SystemInfo>) -> Vec<String> {
    let mut sys = arc_sys.0.lock().unwrap();
    sys.refresh_cpu(); // Refreshing CPU information.
    let mut v: Vec<String> = Vec::new();
    for cpu in sys.cpus() {
        v.push(cpu.name().to_string());
    }
    v
}

// TODO: use event & stream to avoid recreating the SYS object?
#[tauri::command]
fn cpu_usage<'r>(arc_sys: State<'r, SystemInfo>) -> Vec<f32> {
    let mut sys = arc_sys.0.lock().unwrap();
    sys.refresh_cpu(); // Refreshing CPU information.
    let mut v: Vec<f32> = Vec::new();
    for cpu in sys.cpus() {
        v.push(cpu.cpu_usage());
    }
    v
}
