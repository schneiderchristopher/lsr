use colored::Colorize;
use std::fs::DirEntry;

const KYLOBYTE: u64 = 1000;
const MEGABYTE: u64 = 1000 * KYLOBYTE;
const GIGABYTE: u64 = 1000 * MEGABYTE;
const TERABYTE: u64 = 1000 * GIGABYTE;

#[derive(Debug)]
pub struct Path {
    file_name: String,
    is_dir: bool,
    size: String,
}

impl Path {
    pub fn print(&self) {
        if self.is_dir {
            println!("{} -", self.file_name.cyan())
        } else {
            println!("{} {}", self.file_name.white(), self.size.yellow())
        }
    }

    pub fn new(paths: DirEntry) -> Self {
        let file_name = paths.file_name().into_string().unwrap();
        let is_dir = paths.metadata().unwrap().is_dir();
        let size = paths.metadata().unwrap().len();
        Path {
            file_name,
            is_dir,
            size: Path::size_string_formatter(size),
        }
    }

    fn size_string_formatter(size: u64) -> String {
        if size < KYLOBYTE {
            format!("{size}B")
        } else if size < MEGABYTE {
            format!("{}KB", size / KYLOBYTE)
        } else if size < GIGABYTE {
            format!("{}MB", size / MEGABYTE)
        } else if size < TERABYTE {
            format!("{}GB", size / GIGABYTE)
        } else {
            format!("{}TB", size / TERABYTE)
        }
    }
}

#[derive(Debug, Default)]
pub struct Paths {
    pub paths: Vec<Path>,
    biggest_str_len: usize,
}

impl Paths {
    pub fn get_biggest_str_len(&mut self) {
        let mut start_len: usize = 0;
        for path in self.paths.iter_mut() {
            if path.file_name.len() > start_len {
                start_len = path.file_name.len();
            }
        }
        self.biggest_str_len = start_len;
    }

    pub fn indentate_paths(&mut self) {
        for path in self.paths.iter_mut() {
            let spaces_to_add = self.biggest_str_len - path.file_name.len();
            for _ in 0..spaces_to_add + 1 {
                path.file_name.push(' ');
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn size_string_formatter_less_than_1_kb() {
        assert_eq!(Path::size_string_formatter(495), "495B");
    }

    #[test]
    fn size_string_formatter_exactly_1_kb() {
        assert_eq!(Path::size_string_formatter(1000), "1KB");
    }

    #[test]
    fn size_string_formatter_less_than_1_tb() {
        assert_eq!(Path::size_string_formatter(299392942), "299MB");
    }

    #[test]
    fn size_string_formatter_exactly_1_tb() {
        assert_eq!(Path::size_string_formatter(1000000000000), "1TB");
    }

    #[test]
    fn size_string_formatter_more_than_1_tb() {
        assert_eq!(Path::size_string_formatter(293380504804052), "293TB");
    }
}
