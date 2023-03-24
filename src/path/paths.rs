use std::{
    ffi::OsString,
    fmt::{Debug, Display},
    fs::{DirEntry, FileType},
    io,
    os::unix::prelude::OsStrExt,
};

use once_cell::sync::Lazy;
use owo_colors::OwoColorize;

#[derive(Debug)]
pub struct Paths<I>
where
    I: Iterator<Item = io::Result<DirEntry>>,
{
    pub(super) show_hidden: bool,
    pub(super) icons: bool,
    pub(super) show_size: bool,
    pub(super) si: bool,
    pub(super) paths: I,
}

pub enum EitherIter<AIterType, BIterType> {
    A(AIterType),
    B(BIterType),
}

impl<I: std::iter::Iterator<Item = io::Result<DirEntry>>> Paths<I> {
    pub fn new(options: PathOptions, iter: I) -> Self {
        Self::from_iter(options, iter)
    }

    pub fn process_entry(entry: DirEntry) -> io::Result<CompositePath> {
        entry.try_into()
    }

    pub fn from_iter(options: PathOptions, iter: I) -> Paths<I> {
        Self {
            show_hidden: options.show_hidden,
            icons: options.icons,
            show_size: options.show_size,
            si: options.si,
            paths: iter,
        }
    }
}

#[derive(Debug)]
pub struct CompositePath {
    pub(super) name: OsString,
    pub(super) dir_or_file: DirOrFile,
}

pub(super) struct ColoredCompositePath<'a>(pub &'a CompositePath);

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

#[derive(Clone, Copy)]
pub(super) enum DirOrFile {
    Dir,
    File(u64),
}

impl DirOrFile {
    pub(super) fn icon(&self) -> &'static str {
        match self {
            Self::Dir => "",
            Self::File(_) => "",
        }
    }
}

pub(super) struct ColoredDirOrFile(pub DirOrFile);

#[derive(Debug)]
pub struct PathOptions {
    pub(super) show_hidden: bool,
    pub(super) icons: bool,
    pub(super) show_size: bool,
    pub(super) si: bool,
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
    pub fn use_si(&mut self, si: bool) -> &mut Self {
        self.si = si;
        self
    }
}

impl Default for PathOptions {
    fn default() -> Self {
        Self {
            show_hidden: false,
            show_size: true,
            icons: false,
            si: false,
        }
    }
}
