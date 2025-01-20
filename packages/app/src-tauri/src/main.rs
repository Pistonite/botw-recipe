// Prevents additional console window on Windows in release
#![cfg_attr(not(feature = "devtools"), windows_subsystem = "windows")]

use std::ops::Deref;
use std::sync::Arc;

use log::info;
use tauri::{Manager, RunEvent, WindowEvent};

use botw_recipe_searcher_tauri::{commands, Global};

fn main() {
    env_logger::init();
    info!("configuring application");

    let global = Global::default();
    let executor = global.get_executor();
    let db = global.get_db_handle();

    let app = tauri::Builder::default()
        .manage(global)
        .invoke_handler(tauri::generate_handler![
            commands::set_title,
            commands::initialize,
            commands::search,
            commands::abort_search,
            commands::filter,
            commands::abort_filter,
            commands::cook,
            commands::get_result_limit
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
