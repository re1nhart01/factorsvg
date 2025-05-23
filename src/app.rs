use std::fs;

use crate::{fontello_data::SvgData, fs_utils, js, json, utils, xml};


pub fn run_single_file(path: String, output: String) -> bool {
    let input = fs_utils::read_file_as_text(path.as_str()).expect("Can't read file");

    let (svg, _, __) = xml::filter_and_fix(input);

    if let Some(error) = fs_utils::create_file(String::from(output), svg) {
        println!("{}", error);
    }

    return false;
}

pub fn run_multiple_files(path: String, output: String, is_multithread: bool) -> bool {
    let mut handles = vec![];

    if let Ok(dir) = fs::read_dir(&path) {
        for file in dir.flatten() {
            let entry = file.path();

            if entry.is_file() {
                if let Some(ext) = entry.extension().and_then(|e| e.to_str()) {
                    if ext == "svg" {
                        let input_path = entry.clone();
                        let output_path = output.clone();

                        if is_multithread {
                            let handle = std::thread::spawn(move || {
                                run_single_file(
                                    input_path.to_string_lossy().to_string(),
                                    [
                                        output_path,
                                        String::from("fixed_"),
                                        entry.file_name().unwrap().to_string_lossy().to_string(),
                                    ]
                                    .join(""),
                                );
                            });
                            handles.push(handle);
                        } else {
                            run_single_file(
                                input_path.to_string_lossy().to_string(),
                                [
                                    output_path,
                                    String::from("fixed_"),
                                    entry.file_name().unwrap().to_string_lossy().to_string(),
                                ]
                                .join(""),
                            );
                        }
                    }
                }
            }
        }

        for handle in handles {
            handle.join().unwrap();
        }
    }

    true
}



pub fn run_auto_json(path_inputs: String, path_to_json: String, scaler_path: String) {
    let json_data = json::read_json_file_fontello(path_to_json).unwrap();

    let mut reloaded_config = json::reload_config(json_data, String::from("ASC"));

    // 3. Перебор SVG-файлов в папке
    if let Ok(entries) = fs::read_dir(&path_inputs) {
        for entry in entries.flatten() {
            let path = entry.path();
    
            if path.is_file() {
                if let Some(ext) = path.extension() {
                    if ext == "svg" {
                        if let Ok(file_content) = fs::read_to_string(&path) {
                            let (_, _d, _view_box) = xml::filter_and_fix(file_content.clone());
                            let file_name = path.file_stem().unwrap().to_string_lossy().to_string();

                            let view_box_w = utils::extract_viewbox_width(String::from(_view_box.clone())) as u32;
                            let view_box_h: u32 = utils::extract_viewbox_height(String::from(_view_box)) as u32;

                            let scale_x: u32 = 1000u32 / view_box_w;
                            let scale_y: u32 = 1000u32 / view_box_h;

                            let scale = scale_x.min(scale_y);

                            let rescaled_d = js::read_scaler_js_scale(_d, scale, scaler_path.clone());
                            reloaded_config = json::add_glyph(reloaded_config, file_name, SvgData{ path: rescaled_d, width: 1000 });
                        }    
                    }
                }
            }
        }
    }
    

    if let Ok(new_json) = serde_json::to_string_pretty(&reloaded_config) {
       if let Ok(()) = fs::write("output_config.json", new_json) {
            println!("json updated successfully");
       }
    }
}