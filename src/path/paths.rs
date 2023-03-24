use std::{
    ffi::OsString,
    fmt::{Debug, Display},
    fs::{DirEntry, FileType},
    io,
    os::unix::prelude::OsStrExt,
    time::SystemTime,
};

#[cfg(unix)]
use std::os::unix::fs::MetadataExt;

use owo_colors::OwoColorize;

use super::UnixPerms;

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
    pub(super) perms: bool,
    pub(super) created: bool,
    pub(super) modified: bool,
    pub(super) header: bool,

    #[cfg(unix)]
    pub(super) current_uid: u32,
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
            header: options.header,
            perms: options.perms,
            created: options.created,
            modified: options.modified,
            #[cfg(unix)]
            current_uid: users::get_current_uid(),
        }
    }
}

#[derive(Debug)]
pub struct CompositePath {
    pub(super) name: OsString,
    pub(super) dir_or_file: DirOrFile,
    pub(super) created: Option<SystemTime>,
    pub(super) modified: Option<SystemTime>,
    #[cfg(unix)]
    pub(super) permissions: UnixPerms,
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
        let created = metadata.created().ok();
        let modified = metadata.modified().ok();
        #[cfg(unix)]
        let permissions = UnixPerms {
            perms: metadata.mode(),
            owner_uid: metadata.uid(),
        };
        metadata.uid();
        Ok(CompositePath {
            name,
            dir_or_file,
            modified,
            created,
            permissions,
        })
    }
}

#[derive(Clone, Copy)]
pub(super) enum DirOrFile {
    Dir,
    File(u64),
}

pub(super) struct Icon<'a>(pub(super) &'a CompositePath);
pub(super) struct ColoredIcon<'a>(pub(super) &'a CompositePath);

impl CompositePath {
    pub(super) fn icon(&self) -> Icon {
        Icon(self)
    }
}
impl<'a> Icon<'a> {
    pub fn colored(self: Icon<'a>) -> ColoredIcon<'a> {
        ColoredIcon(self.0)
    }
}

#[derive(Clone, Copy)]
pub(super) struct ColoredDirOrFile(pub DirOrFile);

#[derive(Debug)]
pub struct PathOptions {
    pub(super) show_hidden: bool,
    pub(super) icons: bool,
    pub(super) show_size: bool,
    pub(super) perms: bool,
    pub(super) created: bool,
    pub(super) modified: bool,
    pub(super) si: bool,
    pub(super) header: bool,
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
    pub fn show_header(&mut self, show: bool) -> &mut Self {
        self.header = show;
        self
    }
    pub fn show_permissions(&mut self, show: bool) -> &mut Self {
        self.perms = show;
        self
    }
    pub fn show_modified(&mut self, show: bool) -> &mut Self {
        self.modified = show;
        self
    }
    pub fn show_created(&mut self, show: bool) -> &mut Self {
        self.created = show;
        self
    }
}

impl Default for PathOptions {
    fn default() -> Self {
        Self {
            show_hidden: false,
            show_size: true,
            perms: true,
            icons: false,
            header: false,
            created: false,
            modified: false,
            si: false,
        }
    }
}
