use std::{
    fmt::{self, Alignment, Debug, Display},
    fs::DirEntry,
    io::{self, Write},
    os::unix::prelude::OsStrExt,
};

use owo_colors::{DynColor, OwoColorize, Style};

use crate::{
    size::{IntoSize, MAX_SIZE_LEN},
    ZERO_SIZE,
};

use super::{ColoredCompositePath, ColoredDirOrFile, CompositePath, DirOrFile, Paths};

impl<I: std::iter::Iterator<Item = io::Result<DirEntry>>> Paths<I> {
    pub fn print<W: Write>(&mut self, mut w: W) -> io::Result<()> {
        let color = supports_color::on_cached(supports_color::Stream::Stdout);
        let print_size: fn(&CompositePath, &mut W) -> Result<(), io::Error> = 'size: {
            if !self.show_size {
                break 'size |_, _| Ok(());
            };
            match (color.is_some(), self.si) {
                (true, false) => |path, mut w| write!(w, "{}", ColoredDirOrFile(path.dir_or_file)),
                (true, true) => |path, mut w| write!(w, "{:?}", ColoredDirOrFile(path.dir_or_file)),
                (false, false) => |path, mut w| write!(w, "{}", path.dir_or_file),
                (false, true) => |path, mut w| write!(w, "{:?}", path.dir_or_file),
            }
            // if color.is_some() {
            //
            // } else {
            //     |path, mut w| w.write_all(b"Doesn't support color")
            // }
        };
        if self.show_hidden {
            self.paths.try_for_each(|entry| -> io::Result<()> {
                let entry = entry?;
                let path: CompositePath = entry.try_into()?;
                if path.name.as_bytes().get(0).map(|&b| b) != Some(b'.') {
                    // If the name doesn't start with a .
                    return print_size(&path, &mut w);
                }
                Ok(())
            });
        } else {
            self.paths.try_for_each(|entry| -> io::Result<()> {
                let entry = entry?;
                let path: CompositePath = entry.try_into()?;
                return print_size(&path, &mut w);
            });
        };
        return Ok(());

        // match (self.show_size, self.icons, color.is_some()) {
        //     // This is a horrible approach, I will **have** to rewrite this.
        //     (true, true, true) => self.paths.iter().try_for_each(|path| -> fmt::Result {
        //         writeln!(
        //             f,
        //             "{} {} {}",
        //             ColoredDirOrFile(path.dir_or_file),
        //             path.dir_or_file.icon(),
        //             ColoredCompositePath(path),
        //         )
        //     }),
        //     (false, true, true) => self.paths.iter().try_for_each(|path| -> fmt::Result {
        //         writeln!(
        //             f,
        //             "{} {}",
        //             path.dir_or_file.icon(),
        //             ColoredCompositePath(path),
        //         )
        //     }),
        //     (true, false, true) => self.paths.iter().try_for_each(|path| -> fmt::Result {
        //         writeln!(
        //             f,
        //             "{} {}",
        //             ColoredDirOrFile(path.dir_or_file),
        //             ColoredCompositePath(path),
        //         )
        //     }),
        //     (true, true, false) => self.paths.iter().try_for_each(|path| -> fmt::Result {
        //         writeln!(
        //             f,
        //             "{} {} {}",
        //             path.dir_or_file,
        //             path.dir_or_file.icon(),
        //             path,
        //         )
        //     }),
        //     (false, false, false) => self
        //         .paths
        //         .iter()
        //         .try_for_each(|path| -> fmt::Result { writeln!(f, "{}", path,) }),
        //     (false, false, true) => self.paths.iter().try_for_each(|path| -> fmt::Result {
        //         writeln!(f, "{}", ColoredCompositePath(path),)
        //     }),
        //     (true, false, false) => self.paths.iter().try_for_each(|path| -> fmt::Result {
        //         writeln!(f, "{} {}", ColoredDirOrFile(path.dir_or_file), path,)
        //     }),
        //     (false, true, false) => self.paths.iter().try_for_each(|path| -> fmt::Result {
        //         writeln!(f, "{} {}", path.dir_or_file.icon(), path,)
        //     }),
        // }
    }
}

impl Display for CompositePath {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name.to_string_lossy())
    }
}

impl Display for ColoredCompositePath<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.0.dir_or_file {
            DirOrFile::Dir => write!(f, "{}", self.0.name.to_string_lossy().blue().bold()),
            DirOrFile::File(_) => write!(f, "{}", self.0.name.to_string_lossy()),
        }
    }
}

impl Display for ColoredDirOrFile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use DirOrFile::*;
        let ColoredDirOrFile(dir_or_file) = self;
        match dir_or_file {
            Dir => write!(
                f,
                "{:>width$}",
                ZERO_SIZE.fg_rgb::<128, 128, 128>(),
                width = MAX_SIZE_LEN
            ),
            File(size) => {
                write!(
                    f,
                    "{:>width$}",
                    size.into_decimalsize().green().bold(),
                    width = MAX_SIZE_LEN - 1
                )
            }
        }
    }
}

impl Debug for ColoredDirOrFile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use DirOrFile::*;
        let ColoredDirOrFile(dir_or_file) = self;
        match dir_or_file {
            Dir => write!(
                f,
                "{:>width$}",
                ZERO_SIZE.fg_rgb::<128, 128, 128>(),
                width = MAX_SIZE_LEN
            ),
            File(size) => {
                write!(
                    f,
                    "{:>width$?}",
                    size.into_decimalsize().green().bold(),
                    width = MAX_SIZE_LEN - 1
                )
            }
        }
    }
}

impl Display for DirOrFile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use DirOrFile::*;
        match self {
            Dir => write!(f, "{:>width$}", ZERO_SIZE, width = MAX_SIZE_LEN),
            File(size) => {
                write!(
                    f,
                    "{:>width$}",
                    size.into_decimalsize(),
                    width = MAX_SIZE_LEN - 1
                )
            }
        }
    }
}

impl Debug for DirOrFile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use DirOrFile::*;
        match self {
            Dir => write!(f, "{:>width$}", ZERO_SIZE, width = MAX_SIZE_LEN),
            File(size) => {
                write!(
                    f,
                    "{:>width$?}",
                    size.into_decimalsize(),
                    width = MAX_SIZE_LEN - 1
                )
            }
        }
    }
}
