use std::{
    ffi::OsString,
    fmt::Display,
    fs::{DirEntry, FileType},
    io,
    os::unix::prelude::OsStrExt,
};

#[derive(Debug)]
pub struct Paths {
    pub(super) show_hidden: bool,
    pub(super) icons: bool,
    show_size: bool,
    pub(super) paths: Vec<CompositePath>,
}

impl From<PathOptions> for Paths {
    fn from(value: PathOptions) -> Self {
        Self {
            show_hidden: value.show_hidden,
            show_size: value.show_size,
            icons: value.icons,
            paths: Vec::new(),
        }
    }
}

impl Paths {
    pub fn new(options: PathOptions) -> Self {
        Self {
            show_hidden: options.show_hidden,
            show_size: options.show_size,
            icons: options.icons,
            paths: Vec::new(),
        }
    }

    pub fn push(&mut self, path: CompositePath) {
        self.paths.push(path)
    }

    pub fn push_entry(&mut self, entry: DirEntry) -> io::Result<()> {
        let path: CompositePath = entry.try_into()?;
        Ok(self.push(path))
    }

    pub fn from_iter<I>(options: PathOptions, iter: I) -> io::Result<Self>
    where
        I: Iterator<Item = std::io::Result<DirEntry>>,
    {
        let mut paths = Self::new(options);
        paths.fill_with_iter(iter)?;
        Ok(paths)
    }

    pub fn fill_with_iter<I>(&mut self, mut iter: I) -> io::Result<()>
    where
        I: Iterator<Item = std::io::Result<DirEntry>>,
    {
        if self.show_hidden {
            for entry in iter {
                let entry = entry?;
                let name = entry.file_name();
                if matches!(name.as_bytes().get(0), Some(b'.')) {
                    continue;
                }
                self.push_entry(entry)?;
            }
        } else {
            for entry in iter {
                let entry = entry?;
                let name = entry.file_name();
                if matches!(name.as_bytes().get(0), Some(b'.')) {
                    continue;
                }
                self.push_entry(entry)?;
            }
        };
        Ok(())
    }
}

#[derive(Debug)]
pub struct CompositePath {
    pub(super) name: OsString,
    pub(super) dir_or_file: DirOrFile,
}

impl TryFrom<DirEntry> for CompositePath {
    type Error = io::Error;

    fn try_from(value: DirEntry) -> Result<Self, Self::Error> {
        let metadata = value.metadata()?;
        let name = value.file_name();
        let dir_or_file = if metadata.is_dir() {
            DirOrFile::Dir
        } else {
            DirOrFile::File(metadata.len())
        };
        Ok(CompositePath { name, dir_or_file })
    }
}

#[derive(Debug, Clone, Copy)]
pub(super) enum DirOrFile {
    Dir,
    File(u64),
}

pub(super) struct ColoredDirOrFile(pub DirOrFile);

#[derive(Debug)]
pub struct PathOptions {
    pub(super) show_hidden: bool,
    pub(super) icons: bool,
    pub(super) show_size: bool,
}

impl PathOptions {
    pub fn new() -> PathOptions {
        Self::default()
    }
    pub fn show_hidden(&mut self, show: bool) -> &mut Self {
        self.show_hidden = show;
        self
    }
    pub fn show_icons(&mut self, show: bool) -> &mut Self {
        self.icons = show;
        self
    }
    pub fn show_size(&mut self, show: bool) -> &mut Self {
        self.show_size = show;
        self
    }
}

impl Default for PathOptions {
    fn default() -> Self {
        Self {
            show_hidden: false,
            show_size: true,
            icons: false,
        }
    }
}
