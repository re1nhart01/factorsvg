use clap::Parser;
use std::{fs, process::exit};

mod args;
mod fs_utils;
mod xml;
mod utils;
mod json;
mod fontello_data;
mod app;
mod js;

fn main() {
    let arg = args::Arguments::parse();

    let is_multifile = arg.multi;

    let is_correct_input = utils::is_input_not_file(arg.input.clone());
    let is_correct_output = utils::is_input_not_file(arg.input.clone());
    
    if let Ok(existance) = fs::exists(arg.input.clone()) {
        if !existance {
            println!("Input path {} is not exists", arg.input.clone());
            exit(0);
        }
    }
    if !arg.json {
        if let Ok(existance) = fs::exists(arg.output.clone()) {
            if !existance {
                println!("Input path {} is not exists", arg.output.clone());
                exit(0);
            }
        }
    
        if is_multifile && is_correct_input && is_correct_output {
            let is_ok = app::run_multiple_files(arg.input.clone(), arg.output.clone(), arg.multithread);
            println!(
                "Status of work: {} with args input: {} output: {} multithread: {} mutliple: {}",
                is_ok, arg.input, arg.output, arg.multithread, arg.multi
            );
        } else {
            let is_ok = app::run_single_file(arg.input.clone(), arg.output.clone());
            println!(
                "Status of work: {} with args input: {} output: {} multithread: {} mutliple: {}",
                is_ok, arg.input, arg.output, false, false
            );
        }
    } else {
        if let Ok(existance) = fs::exists(arg.config.clone()) {
            if !existance {
                println!("Input json config path {} is not exists", arg.config.clone());
                exit(0);
            }
        }

        app::run_auto_json(arg.input, arg.config);
    }
}