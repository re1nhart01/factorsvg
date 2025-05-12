use std::{env, path::PathBuf, process::Command};


pub const FILE_NAME: &'static str = "scaler.js";

pub fn read_scaler_js_scale(d: String, scale: u32, mut scaler_path: String) -> String {
    let exe_path: PathBuf = env::current_exe()
    .map(|p| p.parent().unwrap().join(FILE_NAME))
    .unwrap_or_else(|_| PathBuf::from(FILE_NAME));

    println!("Execution scaler.js path: {}", exe_path.to_string_lossy().clone());

    if scaler_path == String::from("default") {
        scaler_path = String::from(exe_path.to_string_lossy());
    }

    let stdout_scaler = Command::new("node")
        .arg(scaler_path)
        .arg(d)
        .arg(scale.to_string())
        .output();

        let output = match stdout_scaler {
            Ok(output) if output.status.success() => {
                String::from_utf8(output.stdout).unwrap_or_else(|_| String::from("Invalid UTF-8"))
            }
            Ok(output) => {
                let err = String::from_utf8_lossy(&output.stderr);
                format!("JS error: {}", err.trim())
            }
            Err(e) => format!("Failed to run node: {}", e),
        };
    
        output.trim().to_string()
}