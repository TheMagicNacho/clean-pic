use image::guess_format;
use img_parts::{ImageEXIF, ImageICC};
use log::debug;
use rand::prelude::*;
use std::collections::HashMap;
use std::fs::File;
use std::io::Error;
use std::path::PathBuf;

fn is_image(path: PathBuf) -> bool {
    let img_extensions: HashMap<&str, bool> = [
        ("jpg", true),
        ("jpeg", true),
        // ("png", true),
        // ("gif", true),
        // ("tiff", true),
        // ("webp", true),
    ]
    .iter()
    .cloned()
    .collect();

    if let Some(extension) = path.extension() {
        if let Some(ext_str) = extension.to_str() {
            return img_extensions.contains_key(&ext_str.to_lowercase()[..]);
        }
    }
    false
}

async fn gather_files(path: &PathBuf) -> Vec<String> {
    let mut files: Vec<String> = Vec::new();
    match tokio::fs::read_dir(path).await {
        Ok(mut dir) => {
            while let Ok(Some(entry)) = dir.next_entry().await {
                let path_buf = entry.path();
                if is_image(path_buf.clone()) {
                    match File::open(&path_buf) {
                        Ok(_) => files.push(path_buf.to_str().unwrap().to_string()),
                        Err(e) => debug!("Error opening file: {}", e),
                    }
                }
            }
        }
        Err(e) => debug!("Error reading directory: {}", e),
    }
    files
}

async fn remove_metadata_jpeg(input_bytes: Vec<u8>) -> Result<Vec<u8>, Error> {
    // Parse the JPEG structure from the byte array
    // This does NOT decode the actual image pixels, so it's fast and lossless
    let mut jpeg = img_parts::jpeg::Jpeg::from_bytes(input_bytes.into()).unwrap();

    jpeg.set_exif(None);
    jpeg.set_icc_profile(None);

    jpeg.segments_mut().retain(|segment| {
        match segment.marker() {
            // --- CRITICAL FOR DECODING ---
            // SOF0: Baseline DCT (Standard JPEG)
            img_parts::jpeg::markers::SOF0 => true,
            // SOF2: Progressive DCT (Progressive JPEG)
            img_parts::jpeg::markers::SOF2 => true,
            // DQT: Define Quantization Table (Required to decompress)
            img_parts::jpeg::markers::DQT => true,
            // DHT: Define Huffman Table (Required to decompress)
            img_parts::jpeg::markers::DHT => true,
            // DRI: Define Restart Interval (Required for error recovery/sync)
            img_parts::jpeg::markers::DRI => true,
            // SOS: Start Of Scan (This contains the actual compressed image data)
            img_parts::jpeg::markers::SOS => true,
            // APP0: JFIF Header.
            img_parts::jpeg::markers::APP0 => true,
            // APP14: Adobe Header. Critical for color correctness.
            img_parts::jpeg::markers::APP14 => true,

            // --- EVERYTHING ELSE IS REMOVED ---
            // APP1 (Exif), APP2 (ICC), COM (Comments), etc.
            _ => false,
        }
    });
    let mut output_bytes = Vec::new();
    jpeg.encoder().write_to(&mut output_bytes)?;

    Ok(output_bytes)
}

// creates a directory to store new files within the given path
async fn make_write_dir(path: &PathBuf, new_dir: &str) -> Result<PathBuf, Error> {
    let mut new_path = path.clone();

    new_path.push(new_dir);
    println!("Creating new directory at {:?}", new_path);
    match tokio::fs::create_dir(&new_path).await {
        Ok(_) => Ok(new_path),
        Err(e) => {
            if e.kind() == std::io::ErrorKind::AlreadyExists {
                tokio::fs::remove_dir(&new_path).await?;
                tokio::fs::create_dir(&new_path).await?;
                Ok(new_path)
            } else {
                Err(e)
            }
        }
    }
}

async fn generate_filename() -> String {
    let mut rng = rand::rng();

    let name = rng.random::<u32>();

    format!("{:#}", name)
}

#[tauri::command]
async fn scrub_images(path: &str) -> Result<String, ()> {
    let path = PathBuf::from(path);
    // TODO: Make scrubbed_images configurable
    let save_directory = make_write_dir(&path, "scrubbed_images").await.unwrap();
    println!("Saving scrubbed image to {:?}", save_directory);
    let files = gather_files(&path).await;
    for file in files {
        println!("Processing file: {}", &file);

        let data = std::fs::read(&file).unwrap();
        match guess_format(&data).unwrap() {
            image::ImageFormat::Jpeg => {
                let input_data = std::fs::read(file).unwrap();
                let output_data = remove_metadata_jpeg(input_data).await.unwrap();
                let new_name = generate_filename().await;
                let new_path = format!("{}/{}.jpg", save_directory.display(), new_name);

                std::fs::write(new_path, output_data).unwrap();
            }
            image::ImageFormat::Png => todo!("PNG"),
            image::ImageFormat::Tiff => todo!("TIFF"),
            _ => todo!("Other formats"),
        };
    }

    Ok("Scrubbing complete".to_string())
}

#[tauri::command]
fn count_images(path: &str) -> u64 {
    println!("Counting image files in {}", path);
    let mut counter: u64 = 0;
    match std::fs::read_dir(path) {
        Ok(entries) => {
            for entry in entries.flatten() {
                let path_buf = entry.path();
                if is_image(path_buf) {
                    counter += 1;
                }
            }
            debug!("Found {} image files in {}", counter, path);
            counter
        }
        Err(e) => {
            debug!("Error reading directory: {}", e);
            0
        }
    }
}
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![count_images, scrub_images])
        .run(tauri::generate_context!())
        .expect("error Ehile running tauri application");
}
