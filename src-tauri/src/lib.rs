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


#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet, start_camera])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
