use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "factorsvg")]
struct Arguments {
    input: String, //path where to read many files, if multi == false, then just path to file
    output: String, // analogic with path, if multi = false, then just path to file

    multithread: bool, //multithreading with std::threads
    multi: bool, // is multi files
}