use clap::Parser;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// Display with Date Modified and File Size
    #[arg(short, long)]
    pub long: bool,
    /// Display all files including hidden ones
    #[arg(short, long)]
    pub all: bool,
    /// Display the files in a tree from the given directory
    #[arg(short, long, value_name = "DIR")]
    pub tree: Option<String>,
}
