use std::path::PathBuf;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// Display with Date Modified and File Size
    #[arg(short, long)]
    pub long: bool,
    /// Display all files including hidden ones
    #[arg(short = 'H', long)]
    pub hidden: bool,
    /// Use SI units instead of true powers of two
    #[arg(short = 'S', long)]
    pub si: bool,
    /// Don't show file sizes
    #[arg(short = 's', long)]
    pub nosize: bool,
    /// Display the files in a tree
    #[arg(short, long)]
    pub tree: bool,
    /// The folder to list files in
    #[arg(default_value = ".", value_name = "DIR")]
    pub target: PathBuf,
}
