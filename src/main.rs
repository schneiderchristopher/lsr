use clap::Parser;
use lsr::cmd::cli::Cli;
use lsr::path::{PathOptions, Paths};
// use lsr::path::paths::{Path, Paths};
use lsr::size::{IntoSize, LongSize, Size};
use owo_colors::{OwoColorize, Stream::*, Style};
use std::io::{self, stdout, ErrorKind, Write};
use std::os::unix::prelude::OsStrExt;
use std::{env, fs};

fn main() -> std::io::Result<()> {
    let error_style = Style::new().red().bold();
    let cli = Cli::parse();
    let mut directories = match cli.target.read_dir() {
        Ok(read_dir) => read_dir,
        Err(error) if matches!(error.kind(), ErrorKind::NotFound) => {
            eprintln!(
                "Target directory {} doesn't exist",
                cli.target
                    .display()
                    .if_supports_color(Stderr, |text| text.style(error_style))
            );
            std::process::exit(74)
        }
        Err(err) => {
            eprintln!("Encountered error: {err}");
            std::process::exit(64)
        }
    };

    let mut stdout = stdout().lock();

    let mut paths = {
        let mut options = PathOptions::new();
        options.show_hidden(cli.hidden);
        options.show_size(!cli.nosize);
        options.show_icons(!cli.noicons);
        options.use_si(cli.si);
        Paths::new(options, directories)
    };
    paths.print(stdout)?;

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
    // let sizes = vec![1u64, 1024, 2048, 1024 * 1024, 1024 * 1024 * 1024];
    // for size in sizes {
    //     let size = size.into_decimalsize();
    //     println!("{size}");
    // }
    Ok(())
}
