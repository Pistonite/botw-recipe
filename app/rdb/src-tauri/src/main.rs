// Prevents additional console window on Windows in release
#![cfg_attr(not(feature = "devtools"), windows_subsystem = "windows")]

use std::collections::HashSet;
use std::ops::Deref;
use std::sync::{Arc, LazyLock, Mutex};

use enum_map::Enum;
use log::info;
use rdata::db::{Database, Filter, TempResult};
use rdata::Group;
use tauri::{AppHandle, Manager, RunEvent, State, WindowEvent};

mod config;
use config::Config;
mod cook;
mod events;
mod filter;
mod search;

use error::{Error, ResultInterop};



#[tauri::command]
fn load_override_localization_json(state: State<Global>) -> String {
    state.config.load_override_localization_json()
}

#[tauri::command]
fn get_result_limit(state: State<Global>) -> usize {
    state.config.result_limit
}


fn main() {
    env_logger::init();
    info!("configuring application");
    let executor = Arc::new(Executor::new(num_cpus::get()));
    let config = Config::load();
    let db = Arc::new(config::create_database());

    let app = tauri::Builder::default()
        .manage(Global {
            config,
            executor: Arc::clone(&executor),
            db: Arc::clone(&db),
            search_result: Arc::new(Mutex::new(None)),
            search_handles: Arc::new(Mutex::new(Vec::new())),
            filter_result: Arc::new(Mutex::new(None)),
            filter_handles: Arc::new(Mutex::new(Vec::new())),
            last_included: Arc::new(Mutex::new(HashSet::new())),
            cooking_handle: Arc::new(Mutex::new(None)),
        })
        .invoke_handler(tauri::generate_handler![
            set_title,
            initialize,
            load_override_localization_json,
            search,
            abort_search,
            filter,
            abort_filter,
            cook,
            get_result_limit
        ])
        .build(tauri::generate_context!())
        .expect("error while running tauri application");
    info!("starting application");

    app.run(move |app_handle, e| match e {
        RunEvent::WindowEvent {
            label,
            event: WindowEvent::CloseRequested { .. },
            ..
        } => {
            if label != "main" {
                return;
            }
            info!("closing application");
            let app = app_handle.clone();
            executor.pool().execute(move || {
                if let Some(window) = app.get_window("main") {
                    let _ = window.close();
                }
                info!("window closed");
            });
            let db = Arc::clone(&db);
            executor.pool().execute(move || {
                // tauri doesn't drop its state
                if let Ok(db) = db.as_ref().deref() {
                    db.close();
                }
                info!("database closed");
            });
        }
        RunEvent::ExitRequested { .. } => {
            info!("waiting for executor to finish");
            executor.join();
            info!("exiting application");
        }
        _ => {}
    });
}
