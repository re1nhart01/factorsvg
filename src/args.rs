use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "factorsvg")]
pub struct Arguments {
    #[arg(short, long)]
    pub input: String, //path where to read many files, if multi == false, then just path to file

    #[arg(short, long)]
    pub output: String, // analogic with path, if multi = false, then just path to file

    #[arg(long)]
    pub multithread: bool, //multithreading with std::threads

    #[arg(long)]
    pub multi: bool, // is multi files
}
