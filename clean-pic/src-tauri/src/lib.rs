use image::guess_format;
use img_parts::{ImageEXIF, ImageICC};
use log::debug;
use rand::prelude::*;
use std::collections::HashMap;
// use std::fs::File;
use std::io::Error;
use std::path::PathBuf;
use tokio::fs::{File, OpenOptions};

/// image_stats provide stats on a particular image's cleanning process.
struct ImageReturn {
    img_data: Vec<u8>,
    segments_removed: u32,
    bits_removed: usize,
}

/// batch_stats is the combined stats of the entire process.
struct BatchStats {
    total_segments: u32,
    total_bits: usize,
    total_files: u32,
}
impl BatchStats {
    fn new() -> BatchStats {
        BatchStats {
            total_segments: 0,
            total_bits: 0,
            total_files: 0,
        }
    }

    fn add(&mut self, file_stats: &ImageReturn) {
        self.total_segments += file_stats.segments_removed;
        self.total_bits += file_stats.bits_removed;
        self.total_files += 1;
    }
}

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
                    match tokio::fs::try_exists(&path_buf).await {
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

async fn remove_metadata_jpeg(input_bytes: Vec<u8>) -> Result<ImageReturn, Error> {
    // Parse the JPEG structure from the byte array
    // This does NOT decode the actual image pixels, so it's fast and lossless
    let mut jpeg = img_parts::jpeg::Jpeg::from_bytes(input_bytes.into()).unwrap();

    jpeg.set_exif(None);
    jpeg.set_icc_profile(None);
    let mut segments_removed = 0;
    let mut bits_removed = 0;
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
            _ => {
                segments_removed += 1;
                bits_removed += segment.len();
                false
            }
        }
    });
    let mut img_data = Vec::new();
    jpeg.encoder().write_to(&mut img_data)?;

    let output = ImageReturn {
        img_data,
        segments_removed,
        bits_removed,
    };

    Ok(output)
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
                Ok(new_path)
            } else if e.kind() == std::io::ErrorKind::DirectoryNotEmpty {
                Ok(new_path)
            } else {
                Err(e)
            }
        }
    }
}

/// There is the 32! propbability of a collision, so if there is a collision detected we'll
/// regenerate a new filename.
async fn generate_filename(save_directory: &PathBuf) -> tokio::io::Result<(PathBuf, File)> {
    loop {
        let new_name = {
            let mut rng = rand::rng();
            let name = rng.random::<u32>();
            format!("{:#}", name)
        };
        let new_path = save_directory.join(format!("{}.jpg", new_name));

        let file_result = OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&new_path)
            .await;

        match file_result {
            // Success: We reserved the filename and have the handle
            Ok(file) => return Ok((new_path, file)),
            // Collision: File exists, loop again
            Err(ref e) if e.kind() == tokio::io::ErrorKind::AlreadyExists => continue,
            // Real Error: Permission denied, disk full, etc.
            Err(e) => return Err(e),
        }
    }
}

#[tauri::command]
async fn scrub_images(path: &str, save_directory: &str) -> Result<String, ()> {
    let path = PathBuf::from(path);
    // create a default if the client forgets to supply a directory.
    let save_directory = match save_directory {
        "" => "scrubbed_images",
        _ => save_directory,
    };

    let save_directory = make_write_dir(&path, save_directory).await.unwrap();
    println!("Saving scrubbed image to {:?}", save_directory);
    let mut stats = BatchStats::new();

    let files = gather_files(&path).await;
    for file in files {
        println!("Processing file: {}", &file);

        let data = std::fs::read(&file).unwrap();
        match guess_format(&data).unwrap() {
            image::ImageFormat::Jpeg => {
                let input_data = std::fs::read(file).unwrap();
                let output_data = remove_metadata_jpeg(input_data).await.unwrap();
                let new_path = generate_filename(&save_directory).await.unwrap();

                stats.add(&output_data);
                std::fs::write(new_path.0, output_data.img_data).unwrap();
            }
            // image::ImageFormat::Png => todo!("PNG"),
            // image::ImageFormat::Tiff => todo!("TIFF"),
            _ => println!("Unable to process file format. Try resaving the image."),
        };
    }
    Ok(save_directory.display().to_string())
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
        .expect("error While running tauri application");
}
