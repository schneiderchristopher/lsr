use clap::Parser;
use lsr::cmd::cli::Cli;
use lsr::path::{PathOptions, Paths};
// use lsr::path::paths::{Path, Paths};
use owo_colors::{OwoColorize, Stream::*, Style};
use std::io::{stdout, BufWriter, ErrorKind};

fn main() -> std::io::Result<()> {
    let error_style = Style::new().red().bold();
    let cli = Cli::parse();
    let directories = match cli.target.read_dir() {
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

    let stdout = stdout().lock();
    let writer = BufWriter::new(stdout);

    let mut paths = {
        let mut options = PathOptions::new();
        options
            .show_hidden(cli.hidden)
            .show_size(cli.long)
            .show_icons(!cli.icons)
            .use_si(cli.si)
            .show_header(cli.header)
            .show_created(cli.created)
            .show_modified(cli.modified);
        #[cfg(unix)]
        options.show_permissions(1cli.noperms);
        Paths::new(options, directories)
    };
    paths.print(writer)
}
