use clap::Parser;
use std::fs;

mod args;
mod fs_utils;
mod xml;

fn main() {
    let arg = args::Arguments::parse();

    let is_multifile = arg.multi;
    if (is_multifile) {
        let is_ok = run_multiple_files(arg.input.clone(), arg.output.clone(), arg.multithread);
        println!(
            "Status of work: {} with args input: {} output: {} multithread: {} mutliple: {}",
            is_ok, arg.input, arg.output, arg.multithread, arg.multi
        );
    } else {
        let is_ok = run_single_file(arg.input.clone(), arg.output.clone());
        println!(
            "Status of work: {} with args input: {} output: {} multithread: {} mutliple: {}",
            is_ok, arg.input, arg.output, false, false
        );
    }
}

fn run_single_file(path: String, output: String) -> bool {
    let input = fs_utils::read_file_as_text(path.as_str()).expect("Can't read file");

    let result = xml::filter_and_fix(input);

    if let Some(error) = fs_utils::create_file(String::from(output), result) {
        println!("{}", error);
    }

    return false;
}

fn run_multiple_files(path: String, output: String, is_multithread: bool) -> bool {
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

        // Ждем завершения всех потоков
        for handle in handles {
            handle.join().unwrap();
        }
    }

    true
}
