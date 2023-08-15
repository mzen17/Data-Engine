// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod render;

fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![fetch_files, split_img_render, copyfiles_to_location])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}

// Returns a list of files from searching the directory
#[tauri::command]
fn fetch_files(dir: &str, extension: &str) -> Result<Vec<String>, String> {
    let mut files = Vec::new();

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


#[tauri::command]
fn copyfiles_to_location(files: Vec<String>, location: String) {
    // Check if dir exists. If not, create it
    if !std::path::Path::new(&location).exists() {
        std::fs::create_dir(&location).expect("Error creating directory");
    }
    // Copy the files to the location
    for file in files {
        let file_name = file.split('/').last().unwrap();
        let new_file_path = format!("{}/{}", location, file_name);
        std::fs::copy(file, new_file_path).expect("Error copying file");
    }
    
}


#[tauri::command]
fn split_img_render(img_path: &str, lines: Vec<Vec<f32>>, id: i32) -> Result<Vec<String>, String> {
    // See the number of lines
    let imgpth = img_path.to_string();
    println!("lines: {}", &imgpth);
    let test_img = get_image(img_path);
    if test_img.is_err() {
        return Err("Error reading image".to_string());
    }
    let img = test_img.unwrap();

    let width = img.width();
    let height = img.height();

    let mut new_lines = lines.clone();
    // Add boundries based on the image size
    new_lines.push(vec![0.0, 0.0, width as f32, 0.0]);
    new_lines.push(vec![0.0, 0.0, 0.0, height as f32]);
    new_lines.push(vec![width as f32, 0.0, width as f32, height as f32]);
    new_lines.push(vec![0.0, height as f32, width as f32, height as f32]);


    // Call the function to split the image
    let result_images = render::img_split::split_image(img.clone(), new_lines);
    let img_extension = img_path.split('.').last().unwrap();


    // Define a path to save the generated images
    let output_dir = "/tmp/sxde_output/";
    std::fs::create_dir_all(output_dir).expect("Failed to create output directory");
    let output_dir2 = format!("/tmp/sxde_output/{}", id);
    std::fs::create_dir_all(&output_dir2).expect("Failed to create output directory");


    // Iterate through the generated images and save them
    let mut output_paths: Vec<String> = Vec::new();
    for (index, img) in result_images.iter().enumerate() {
        let output_path = format!("{}/output_{}.{}", output_dir2, index, img_extension);
        println!("out: {}" , output_path);

        img.save(&output_path).expect("Failed to save image");
        output_paths.push(output_path); // Push the output path into the vector
    }

    println!("outs {:?}", output_paths);
    Ok(output_paths)
}

fn get_image(image_path: &str) -> Result<image::DynamicImage, image::error::ImageError> {
    let img = image::io::Reader::open(image_path)?
    .with_guessed_format()?
    .decode()?;
    Ok(img)
}