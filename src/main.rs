use lsr::path::paths::{Path, Paths};
use std::{env, fs};

fn main() -> std::io::Result<()> {
    let path = env::current_dir()?;
    let contents = fs::read_dir(&path)?;
    let mut paths = Paths::default();
    for content in contents {
        paths.paths.push(Path::new(content?));
    }
    paths.indentate_paths();
    for path in paths.paths.into_iter() {
        path.print();
    }
    Ok(())
}
