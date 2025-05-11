use std::process::Command;


pub const FILE_NAME: &'static str = "scaler.js";

pub fn read_scaler_js_scale(d: String, scale: u32) -> String {
    let stdout_scaler = Command::new("node")
        .arg(FILE_NAME)
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