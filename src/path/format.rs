use std::fmt::{self, Alignment, Display, Write};

use owo_colors::{DynColor, OwoColorize, Style};

use crate::{
    size::{IntoSize, MAX_SIZE_LEN},
    ZERO_SIZE,
};

use super::{ColoredDirOrFile, DirOrFile, Paths};

impl Display for Paths {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let color = supports_color::on_cached(supports_color::Stream::Stdout);
        if color.is_some() {
            self.paths.iter().try_for_each(|path| -> fmt::Result {
                // Gets executed for every path
                writeln!(f, "{}", ColoredDirOrFile(path.dir_or_file))
            })
        } else {
            self.paths.iter().try_for_each(|path| -> fmt::Result {
                // Gets executed for every path
                writeln!(f, "{}", path.dir_or_file)
            })
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
