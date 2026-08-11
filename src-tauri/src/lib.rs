mod database;
mod features;

use database::connection::create_pool;
use features::students::commands::{create_student, get_students};
use tauri::Manager;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            let app_data_dir = app
                .path()
                .app_data_dir()
                .expect("No se pudo obtener app_data_dir");

            std::fs::create_dir_all(&app_data_dir)?;

            let db_path = app_data_dir.join("countly.db");

            print!("\nSQite path: {:?}\n", db_path);

            let pool = tauri::async_runtime::block_on(create_pool(db_path))?;

            app.manage(pool);

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![create_student, get_students]) // habilitados
        // los comando
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
