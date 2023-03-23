use clap::Parser;
use lsr::cmd::cli::Cli;
use lsr::path::paths::{Path, Paths};
use lsr::size::{ExactSize, IntoSize, Size};
use std::{env, fs};

fn main() -> std::io::Result<()> {
    // let cli = Cli::parse();
    // let mut paths = Paths::default();
    // paths.setup_args((cli.all, cli.long, cli.tree));
    //
    // let path = env::current_dir()?;
    // let contents = fs::read_dir(&path)?;
    //
    // for content in contents {
    //     paths.paths.push(Path::new(content?));
    // }
    //
    // paths.print();
    let sizes = vec![1u64, 1024, 2048, 1024 * 1024, 1024 * 1024 * 1024];
    for size in sizes {
        let size = size.into_size();
        println!("{size}");
    }
    Ok(())
}
