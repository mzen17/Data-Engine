// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod render;

fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![fetch_files])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}

// Returns a vector with a list of Base64 encoded versions of the images from a file path
#[tauri::command]
fn fetch_files(dir: &str, extension: &str) -> Result<Vec<String>, String> {
    let mut files = Vec::new();
    println!("{}", dir);

    let paths = std::fs::read_dir(dir)
        .map_err(|e| format!("Error reading directory: {}", e))?;

    for path in paths {
        let path = path.map_err(|e| format!("Error reading path: {}", e))?.path();
        if let Some(path_str) = path.to_str() {
            if path_str.ends_with(extension) {
                files.push(path_str.to_string());
            }
        }
    }
    Ok(files)
}
