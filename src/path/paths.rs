use chrono::NaiveDateTime;
use colored::Colorize;
use std::{
    fmt::Error,
    fs::{DirEntry, Metadata},
    time::UNIX_EPOCH,
};

const KYLOBYTE: u64 = 1000;
const MEGABYTE: u64 = 1000 * KYLOBYTE;
const GIGABYTE: u64 = 1000 * MEGABYTE;
const TERABYTE: u64 = 1000 * GIGABYTE;

#[derive(Debug)]
pub struct Path {
    file_name: String,
    is_dir: bool,
    size: String,
    time: String,
}

impl Path {
    pub fn print(&self) {
        if !self.is_dir {
            self.size.yellow();
        }
        println!(
            "{} {} {}",
            self.file_name.white(),
            self.size,
            self.time.blue()
        )
    }

    pub fn new(paths: DirEntry) -> Self {
        let metadata = paths.metadata().unwrap();
        let file_name = paths.file_name().into_string().unwrap();
        let is_dir = metadata.is_dir();
        let size = metadata.len();
        Path {
            file_name,
            is_dir,
            size: Path::size_string_formatter(size),
            time: Path::set_time(metadata).unwrap(),
        }
    }

    fn size_string_formatter(size: u64) -> String {
        if size == 0 {
            format!("-")
        } else if size < KYLOBYTE {
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

    fn set_time(metadata: Metadata) -> Result<String, Error> {
        if let Ok(sys_time) = metadata.modified() {
            if let Ok(duration) = sys_time.duration_since(UNIX_EPOCH) {
                if let Some(time) =
                    NaiveDateTime::from_timestamp_millis(duration.as_millis() as i64)
                {
                    Ok(time.format("%e %b %R").to_string())
                } else {
                    panic!("Could not get time from milliseconds")
                }
            } else {
                panic!("Time must gone backwards!")
            }
        } else {
            panic!("Not implement in this platform")
        }
    }
}

#[derive(Debug, Default)]
pub struct Paths {
    pub paths: Vec<Path>,
}

impl Paths {
    fn get_biggest_str_len(&mut self) -> (usize, usize) {
        let (mut start_len_name, mut start_size_len) = (0, 0);
        for path in self.paths.iter_mut() {
            if path.file_name.len() > start_len_name {
                start_len_name = path.file_name.len();
            }
            if path.size.len() > start_size_len {
                start_size_len = path.size.len();
            }
        }
        (start_len_name, start_size_len)
    }

    fn indentate_paths(&mut self) {
        let (biggest_name_len, biggest_size_len) = self.get_biggest_str_len();
        for path in self.paths.iter_mut() {
            let spaces_to_add = biggest_name_len - path.file_name.len();
            for _ in 0..spaces_to_add + 1 {
                path.file_name.push(' ');
            }
            let spaces_to_add = biggest_size_len - path.size.len();
            for _ in 0..spaces_to_add + 1 {
                path.size.push(' ');
            }
        }
    }

    pub fn print(mut self) {
        self.indentate_paths();
        for path in self.paths.into_iter() {
            path.print();
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

    #[test]
    fn names_should_have_same_length() {
        let path1 = Path {
            file_name: "test".to_owned(),
            is_dir: false,
            size: "1kb".to_owned(),
            time: "test".to_owned(),
        };
        let path2 = Path {
            file_name: "test_test".to_owned(),
            is_dir: false,
            size: "1kb".to_owned(),
            time: "test".to_owned(),
        };
        let mut paths = Paths::default();
        paths.paths.push(path1);
        paths.paths.push(path2);

        paths.indentate_paths();

        assert_eq!(
            paths.paths.get(0).unwrap().file_name.len(),
            paths.paths.get(1).unwrap().file_name.len(),
        );
    }

    #[test]
    fn size_should_have_same_length() {
        let path1 = Path {
            file_name: "test".to_owned(),
            is_dir: false,
            size: "1kb".to_owned(),
            time: "test".to_owned(),
        };
        let path2 = Path {
            file_name: "test_test".to_owned(),
            is_dir: false,
            size: "1kb".to_owned(),
            time: "test".to_owned(),
        };
        let mut paths = Paths::default();
        paths.paths.push(path1);
        paths.paths.push(path2);

        paths.indentate_paths();

        assert_eq!(
            paths.paths.get(0).unwrap().size.len(),
            paths.paths.get(1).unwrap().size.len(),
        );
    }
}
