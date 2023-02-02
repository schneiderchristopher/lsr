use chrono::{DateTime, Local, NaiveDateTime, Offset, TimeZone};
use colored::Colorize;
use std::{
    fmt::Error,
    fs::DirEntry,
    time::{SystemTime, UNIX_EPOCH},
};

const KYLOBYTE: u64 = 1000;
const MEGABYTE: u64 = 1000 * KYLOBYTE;
const GIGABYTE: u64 = 1000 * MEGABYTE;
const TERABYTE: u64 = 1000 * GIGABYTE;

#[derive(Debug, PartialEq, Clone)]
pub struct Path {
    file_name: String,
    is_dir: bool,
    size: String,
    time: String,
    print_string: String,
}

impl Path {
    pub fn print(&self) {
        println!("{}", self.print_string);
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
            time: Path::set_time(metadata.modified().unwrap()).unwrap(),
            print_string: String::new(),
        }
    }

    fn size_string_formatter(size: u64) -> String {
        if size == 0 {
            "-".to_string()
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

    fn set_time(sys_time: SystemTime) -> Result<String, Error> {
        if let Ok(duration) = sys_time.duration_since(UNIX_EPOCH) {
            if let Some(time) = NaiveDateTime::from_timestamp_millis(duration.as_millis() as i64) {
                let local_offset = Local.timestamp_nanos(time.timestamp_nanos()).offset().fix();
                let local_date_time = DateTime::<Local>::from_utc(time, local_offset);
                Ok(local_date_time.format("%e %b %R").to_string())
            } else {
                panic!("Could not get time from milliseconds")
            }
        } else {
            panic!("Time must gone backwards!")
        }
    }
}

#[derive(Debug, Default)]
pub struct Paths {
    pub paths: Vec<Path>,
    pub long: bool,
    pub all: bool,
    pub tree: (bool, String),
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
        self.print_constructor();
        for path in self.paths.into_iter() {
            path.print();
        }
    }

    fn print_constructor(&mut self) {
        if !self.all {
            self.paths
                .retain_mut(|path| !path.file_name.starts_with('.'));
        }

        if self.long && !self.tree.0 {
            self.paths.iter_mut().for_each(|path| {
                let mut size_color = path.size.white();
                let file_name_color: colored::ColoredString = if path.is_dir {
                    path.file_name.blue()
                } else {
                    size_color = path.size.yellow();
                    path.file_name.white()
                };
                path.print_string = format!(
                    "{} {} {}",
                    size_color,
                    path.time.bright_cyan(),
                    file_name_color
                );
            });
        } else if self.long && self.tree.0 {
            todo!()
        } else if !self.long && self.tree.0 {
            todo!()
        } else {
            self.paths.iter_mut().for_each(|path| {
                let file_name_color: colored::ColoredString = if path.is_dir {
                    path.file_name.blue()
                } else {
                    path.file_name.white()
                };
                path.print_string = format!("{file_name_color}");
            });
        }
    }

    pub fn setup_args(&mut self, args: (bool, bool, Option<String>)) {
        let (all, long, tree) = args;
        self.all = all;
        self.long = long;
        if let Some(tree) = tree {
            self.tree = (true, tree);
        }
    }
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

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
            print_string: "print".to_owned(),
        };
        let path2 = Path {
            file_name: "test_test".to_owned(),
            is_dir: false,
            size: "1kb".to_owned(),
            time: "test".to_owned(),
            print_string: "print".to_owned(),
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
            print_string: "print".to_owned(),
        };
        let path2 = Path {
            file_name: "test_test".to_owned(),
            is_dir: false,
            size: "1kb".to_owned(),
            time: "test".to_owned(),
            print_string: "print".to_owned(),
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

    #[test]
    fn date_format_should_be_corret() {
        // Mon Jan 30 2023 20:37:54 UTC+0
        let time = SystemTime::UNIX_EPOCH + Duration::from_millis(1675111074521);
        let time_formatted = Path::set_time(time).unwrap();

        assert_eq!(time_formatted, "30 Jan 20:37")
    }

    #[test]
    fn setup_args_should_setup() {
        let mut paths = Paths::default();
        let all = true;
        let long = true;
        let tree = Some("dir".to_owned());

        paths.setup_args((all, long, tree));

        assert_eq!(paths.all, all);
        assert_eq!(paths.long, long);
        assert_eq!(paths.tree, (true, "dir".to_owned()));
    }

    #[test]
    fn no_all_argument_should_not_print_dot_files() {
        let mut paths = Paths::default();
        let all = false;
        let long = false;
        let tree = None;
        paths.setup_args((all, long, tree));

        let mut path1 = Path {
            file_name: ".test".to_owned(),
            is_dir: false,
            size: "1kb".to_owned(),
            time: "test".to_owned(),
            print_string: "print".to_owned(),
        };

        paths.paths.push(path1.clone());
        paths.print_constructor();
        // This is need as this is the result of any string using the Colored white fn
        path1.print_string = "\u{1b}[37m.test\u{1b}[0m".to_owned();

        assert_eq!(paths.paths.contains(&path1), false);
    }

    #[test]
    fn all_argument_should_print_dot_files() {
        let mut paths = Paths::default();
        let all = true;
        let long = false;
        let tree = None;
        paths.setup_args((all, long, tree));

        let mut path1 = Path {
            file_name: ".test".to_owned(),
            is_dir: false,
            size: "1kb".to_owned(),
            time: "test".to_owned(),
            print_string: "print".to_owned(),
        };

        paths.paths.push(path1.clone());
        paths.print_constructor();
        // This is need as this is the result of any string using the Colored white fn
        path1.print_string = "\u{1b}[37m.test\u{1b}[0m".to_owned();

        assert_eq!(paths.paths.contains(&path1), true);
    }
}
