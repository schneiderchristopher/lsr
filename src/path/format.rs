use std::fmt::{self, Alignment, Display, Write};

use owo_colors::{DynColor, OwoColorize, Style};

use crate::{
    size::{IntoSize, MAX_SIZE_LEN},
    ZERO_SIZE,
};

use super::{ColoredCompositePath, ColoredDirOrFile, CompositePath, DirOrFile, Paths};

impl Display for Paths {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let color = supports_color::on_cached(supports_color::Stream::Stdout);
        match (self.show_size, self.icons, color.is_some()) {
            // This is a horrible approach, I will **have** to rewrite this.
            (true, true, true) => self.paths.iter().try_for_each(|path| -> fmt::Result {
                writeln!(
                    f,
                    "{} {} {}",
                    ColoredDirOrFile(path.dir_or_file),
                    path.dir_or_file.icon(),
                    ColoredCompositePath(path),
                )
            }),
            (false, true, true) => self.paths.iter().try_for_each(|path| -> fmt::Result {
                writeln!(
                    f,
                    "{} {}",
                    path.dir_or_file.icon(),
                    ColoredCompositePath(path),
                )
            }),
            (true, false, true) => self.paths.iter().try_for_each(|path| -> fmt::Result {
                writeln!(
                    f,
                    "{} {}",
                    ColoredDirOrFile(path.dir_or_file),
                    ColoredCompositePath(path),
                )
            }),
            (true, true, false) => self.paths.iter().try_for_each(|path| -> fmt::Result {
                writeln!(
                    f,
                    "{} {} {}",
                    path.dir_or_file,
                    path.dir_or_file.icon(),
                    path,
                )
            }),
            (false, false, false) => self
                .paths
                .iter()
                .try_for_each(|path| -> fmt::Result { writeln!(f, "{}", path,) }),
            (false, false, true) => self.paths.iter().try_for_each(|path| -> fmt::Result {
                writeln!(f, "{}", ColoredCompositePath(path),)
            }),
            (true, false, false) => self.paths.iter().try_for_each(|path| -> fmt::Result {
                writeln!(f, "{} {}", ColoredDirOrFile(path.dir_or_file), path,)
            }),
            (false, true, false) => self.paths.iter().try_for_each(|path| -> fmt::Result {
                writeln!(f, "{} {}", path.dir_or_file.icon(), path,)
            }),
        }
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
