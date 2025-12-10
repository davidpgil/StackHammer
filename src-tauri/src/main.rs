#![cfg_attr(all(not(debug_assertions), target_os = "windows"), windows_subsystem = "windows")]

// This is the main file for the backend of StackHammer, a tool that helps turn game screenshots into organized sets of image tiles for creating games. 
// The code here is written in a programming language called Rust, which is used for the heavy work of processing images.

use image::{GenericImageView, ImageBuffer, Rgba}; // Tools for working with images
use std::collections::HashSet; // A way to store unique items
use seahash::hash; // A method to create a unique identifier for data
use base64::{engine::general_purpose::STANDARD, Engine as _}; // Converts data into a text format that can be sent easily
use once_cell::sync::Lazy; // Helps store data that is created only once
use std::sync::Mutex; // Ensures that only one part of the program can change certain data at a time
use tauri_plugin_dialog::DialogExt; // Allows showing file save dialogs to the user

static LAST_TILESET: Lazy<Mutex<Option<Vec<u8>>>> = Lazy::new(|| Mutex::new(None));
// This keeps track of the last processed set of tiles so it can be saved later. It's stored in a way that prevents conflicts if multiple actions happen at once.

#[tauri::command]
async fn extract_tileset(data_url: String) -> Result<String, String> {
// This function takes an image sent from the user interface as a special text format (data URL).
// It processes the image to break it into small square pieces called tiles (16x16 pixels each).
// It finds unique tiles, arranges them into a new image, and sends this new image back to the user interface.
    let base64_data = data_url.split(',').nth(1).ok_or("Invalid data URL")?;
    let img_data = STANDARD.decode(base64_data).map_err(|e| e.to_string())?;
    let img = image::load_from_memory(&img_data).map_err(|e| e.to_string())?;

    let (w, h) = img.dimensions();
    let tile_size = 16u32;
    let mut unique_tiles = Vec::new();
    let mut seen = HashSet::new();

    for y in (0..h).step_by(tile_size as usize) {
        for x in (0..w).step_by(tile_size as usize) {
            if x + tile_size > w || y + tile_size > h { continue; }
            let tile = img.view(x, y, tile_size, tile_size).to_image();
            let tile_bytes = tile.into_raw();  // Raw bytes for hashing
            let tile_hash = hash(&tile_bytes);

            if seen.insert(tile_hash) {
                unique_tiles.push(tile_bytes);
            }
        }
    }

    let cols = (unique_tiles.len() as f32).sqrt().ceil() as u32;
    let rows = (unique_tiles.len() + cols as usize - 1) / cols as usize;
    let mut tileset = ImageBuffer::<Rgba<u8>, Vec<u8>>::new(cols * tile_size, rows as u32 * tile_size);

    for (i, tile_bytes) in unique_tiles.iter().enumerate() {
        let tx = (i as u32 % cols) * tile_size;
        let ty = (i as u32 / cols) * tile_size;
        let temp_tile = ImageBuffer::<Rgba<u8>, Vec<u8>>::from_raw(tile_size, tile_size, tile_bytes.clone()).unwrap();
        image::imageops::replace(&mut tileset, &temp_tile, tx as i64, ty as i64);
    }

    let mut png_bytes = Vec::new();
    tileset.write_to(&mut std::io::Cursor::new(&mut png_bytes), image::ImageFormat::Png)
        .map_err(|e| e.to_string())?;

    *LAST_TILESET.lock().unwrap() = Some(png_bytes.clone());

    Ok(format!("data:image/png;base64,{}", STANDARD.encode(&png_bytes)))
}

#[tauri::command]
async fn save_tileset(app: tauri::AppHandle) -> Result<(), String> {
// This function opens a window for the user to choose where to save the processed set of tiles.
// It takes the last processed image stored earlier and saves it as a file on the user's computer.
    let path = app.dialog()
        .file()
        .set_file_name("stackhammer_tileset.png")
        .blocking_save_file();

    if let Some(path) = path {
        if let Some(bytes) = LAST_TILESET.lock().unwrap().clone() {
            std::fs::write(path.to_string(), &bytes).map_err(|e| e.to_string())?;
        }
    }
    Ok(())
}

fn main() {
// This is the starting point of the backend part of StackHammer.
// It sets up the application to listen for requests from the user interface, like processing images or saving files.
// It connects the functions defined above (extract_tileset and save_tileset) so they can be called from the front part of the app.
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![extract_tileset, save_tileset])
        .run(tauri::generate_context!())
        .expect("StackHammer v0.1 failed to start");
}
