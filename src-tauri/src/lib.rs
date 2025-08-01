// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
use base64::{engine::general_purpose, Engine as _};
use opencv::{
    core::{no_array, Vector},
    imgcodecs::{imencode, IMWRITE_JPEG_QUALITY},
    objdetect::QRCodeDetector,
    prelude::*,
    videoio::{VideoCapture, CAP_ANY},
};

use tauri::{AppHandle, Emitter};
use printers::{common::base::job::PrinterJobOptions, get_default_printer, get_printer_by_name, get_printers};



#[tauri::command]
fn greet(name: &str) -> String {
    println!("greet");
    format!("Hello, {}! You've been greeted from Rust!", name)
}


#[tauri::command]
async fn start_camera(app: AppHandle) -> Result<(), String> {
    std::thread::spawn(move || {
        let mut cam = VideoCapture::new(0, CAP_ANY).map_err(|e| e.to_string()).unwrap();
        if !cam.is_opened().unwrap() {
            return Err("Failed to open camera".to_string()).unwrap();
        }

        let mut frame = Mat::default();
        let mut output = Mat::default();
        let mut points = Mat::default();
        let mut qr_detector = QRCodeDetector::default().unwrap();

        loop {
            cam.read(&mut frame).unwrap();

            if frame.size().unwrap().width == 0 {
                continue;
            }

            // Decode QR
            let decoded = qr_detector.detect_and_decode(&frame, &mut points, &mut output);
            match decoded {
                Ok(decoded) => {
                    if !decoded.is_empty() {
                        let text = String::from_utf8_lossy(&decoded).to_string();
                        println!("QR code detected: {:?}", text);
                        app.emit("qrcode_detected", text).unwrap();
                    }
                }
                Err(e) => {
                    println!("Error decoding QR code: {:?}", e);
                }
            }

            // Encode frame to JPEG
            let mut buf = Vector::new();
            imencode(".jpg", &frame, &mut buf, &Vector::from(vec![IMWRITE_JPEG_QUALITY, 70])).unwrap();
            let b64 = general_purpose::STANDARD.encode(&buf);

            let _ = app.emit("camera_frame", b64);
            // println!("Camera frame: {:?}", b64);

            // sleep for a bit to avoid hogging CPU
            std::thread::sleep(std::time::Duration::from_millis(100));
        }
    });

    Ok(())
}


fn print_file(file_path: &str) -> Result<u64, String> {
    // Iterate all available printers
    for printer in get_printers() {
        println!("{:?}", printer);
    }

    // Get a printer by the name
    let my_printer = get_printer_by_name("PDF");
    if my_printer.is_some() {
        let job_id_result = my_printer.unwrap().print_file(file_path, PrinterJobOptions::none());
        match job_id_result {
            Ok(job_id) => {
                println!("Job ID: {:?}", job_id);
                return Ok(job_id);
            }
            Err(e) => {
                return Err(format!("PDF printer error: {}", e));
            }
        }
    }

    // Use the default printer
    let default_printer = get_default_printer()
        .ok_or("No default printer found")?;
    
    let job_id_result = default_printer.print_file(&file_path, PrinterJobOptions {
        name: None,
        // options are currently UNIX-only. see https://www.cups.org/doc/options.html
        raw_properties: &[
            ("document-format", "application/vnd.cups-raw"),
            ("copies", "1"),
        ],
    });

    match job_id_result {
        Ok(job_id) => Ok(job_id),
        Err(e) => Err(format!("Default printer error: {}", e)),
    }
}

#[tauri::command]
fn print_badge(app: AppHandle, image: String) -> Result<(), String> {
    let decoded = general_purpose::STANDARD.decode(&image)
        .map_err(|e| format!("Failed to decode base64: {}", e))?;
    
    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();
    

    let filename = format!("/tmp/badge_{}.jpg", timestamp);
    std::fs::write(&filename, decoded)
        .map_err(|e| format!("Failed to write file: {}", e))?;
    
    println!("Image saved to {}", &filename);

    match print_file(&filename) {
        Ok(job_id) => {
            println!("Print job started with ID: {}", job_id);
            app.emit("badge_printed", job_id).unwrap();
            
            // Delete the file after printing
            if let Err(e) = std::fs::remove_file(&filename) {
                println!("Warning: Failed to delete temporary file {}: {}", &filename, e);
            } else {
                println!("Temporary file deleted: {}", &filename);
            }
        }
        Err(e) => {
            println!("Print error: {}", e);
            app.emit("badge_print_error", e).unwrap();
            
            // Delete the file even if printing failed
            if let Err(e) = std::fs::remove_file(&filename) {
                println!("Warning: Failed to delete temporary file {}: {}", filename, e);
            }
        }
    }
    
    Ok(())
}


#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet, start_camera, print_badge])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
