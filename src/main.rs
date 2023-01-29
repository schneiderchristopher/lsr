extern crate lsr;
use lsr::path::paths::*;
use std::{
    env,
    fs::{self},
};

fn main() -> std::io::Result<()> {
    let path = env::current_dir()?;
    let contents = fs::read_dir(&path)?;
    let mut paths = Paths::default();
    for content in contents {
        let content_unwraped = content.unwrap();
        paths.paths.push(Path::new(content_unwraped));
    }
    paths.get_biggest_str_len();
    paths.identate_paths();
    for path in paths.paths.into_iter() {
        path.print();
    }
    Ok(())
}
