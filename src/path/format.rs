use std::{
    fmt::{self, Alignment, Debug, Display},
    fs::DirEntry,
    io::{self, Write},
    os::unix::prelude::OsStrExt,
    time::SystemTime,
};

use chrono::{DateTime, Datelike, Local};
use owo_colors::{DynColor, OwoColorize, Style};

use crate::{
    size::{IntoSize, MAX_SIZE_LEN},
    ZERO_SIZE,
};

use super::{
    ColoredCompositePath, ColoredDirOrFile, ColoredIcon, CompositePath, DirOrFile, Icon, Paths,
};

impl<I: std::iter::Iterator<Item = io::Result<DirEntry>>> Paths<I> {
    pub fn print<W: Write>(&mut self, mut w: W) -> io::Result<()> {
        // Check if the current terminal supports color
        let color = supports_color::on_cached(supports_color::Stream::Stdout);
        // Set up functions to be used for printing
        #[cfg(unix)]
        let print_perms: fn(u32, &CompositePath, &mut W) -> Result<(), io::Error> = 'perms: {
            if !self.perms {
                break 'perms |_, _, _| Ok(());
            };
            if color.is_some() {
                |current_uid, path, w| path.permissions.print_color(w, current_uid)
            } else {
                |current_uid, path, w| path.permissions.print(w, current_uid)
            }
        };

        let print_size: fn(&CompositePath, &mut W) -> Result<(), io::Error> = 'size: {
            if !self.show_size {
                break 'size |_, _| Ok(());
            };
            match (color.is_some(), self.si) {
                (true, false) => |path, w| write!(w, "{} ", ColoredDirOrFile(path.dir_or_file)),
                (true, true) => |path, w| write!(w, "{:?} ", ColoredDirOrFile(path.dir_or_file)),
                (false, false) => |path, w| write!(w, "{} ", path.dir_or_file),
                (false, true) => |path, w| write!(w, "{:?} ", path.dir_or_file),
            }
        };
        let current_time = chrono::Local::now();
        let print_created: fn(&DateTime<Local>, &CompositePath, &mut W) -> Result<(), io::Error> = 'created: {
            if !self.created {
                break 'created |_, _, _| Ok(());
            }
            if color.is_some() {
                |current_time, path, w| {
                    let Some(datetime): Option<DateTime<Local>> = path.created.map(|time|time.into()) else {return write!(w, "             ")};
                    let format = if matches!(current_time.years_since(datetime), Some(years) if years >= 1)
                    {
                        "%e %b  %Y"
                    } else {
                        "%e %b %R"
                    };
                    let formatted = datetime.format(format);
                    write!(w, "{} ", formatted.blue())
                }
            } else {
                |current_time, path, w| {
                    let Some(datetime): Option<DateTime<Local>> = path.created.map(|time|time.into()) else {return write!(w, "             ")};
                    let format = if matches!(current_time.years_since(datetime), Some(years) if years >= 1)
                    {
                        "%e %b  %Y"
                    } else {
                        "%e %b %R"
                    };
                    let formatted = datetime.format(format);
                    write!(w, "{} ", formatted)
                }
            }
        };
        let print_modified: fn(&DateTime<Local>, &CompositePath, &mut W) -> Result<(), io::Error> = 'modified: {
            if !self.modified {
                break 'modified |_, _, _| Ok(());
            }
            if color.is_some() {
                |current_time, path, w| {
                    let Some(datetime): Option<DateTime<Local>> = path.modified.map(|time|time.into()) else {return write!(w, "             ")};
                    let format = if matches!(current_time.years_since(datetime), Some(years) if years >= 1)
                    {
                        "%e %b  %Y"
                    } else {
                        "%e %b %R"
                    };
                    let formatted = datetime.format(format);
                    write!(w, "{} ", formatted.purple())
                }
            } else {
                |current_time, path, w| {
                    let Some(datetime): Option<DateTime<Local>> = path.modified.map(|time|time.into()) else {return write!(w, "             ")};
                    let format = if matches!(current_time.years_since(datetime), Some(years) if years >= 1)
                    {
                        "%e %b  %Y"
                    } else {
                        "%e %b %R"
                    };
                    let formatted = datetime.format(format);
                    write!(w, "{} ", formatted)
                }
            }
        };
        let print_icon: fn(&CompositePath, &mut W) -> Result<(), io::Error> = 'icons: {
            if !self.icons {
                break 'icons |_, _| Ok(());
            };
            if color.is_some() {
                |path, w| write!(w, "{} ", path.icon().colored())
            } else {
                |path, w| write!(w, "{} ", path.icon())
            }
        };
        let print_name: fn(&CompositePath, &mut W) -> Result<(), io::Error> = 'size: {
            if color.is_some() {
                |path, w| match path.dir_or_file {
                    DirOrFile::Dir => {
                        write!(w, "{} ", path.blue().bold())
                    }
                    DirOrFile::File(_) => {
                        write!(w, "{} ", ColoredCompositePath(path))
                    }
                }
            } else {
                |path, w| write!(w, "{} ", path)
            }
        };
        // Print the header if it's enabled
        if self.header {
            if self.perms {
                write!(w, "Perms    ")?;
            }
            write!(w, "   Size ")?;
            if self.icons {
                write!(w, "  ")?;
            }
            if self.created {
                write!(w, "   Created  ")?;
            }
            if self.modified {
                write!(w, "  Modified  ")?;
            }
            write!(w, "Filename")?;
            writeln!(w)?;
        }
        if self.show_hidden {
            self.paths.try_for_each(|entry| -> io::Result<()> {
                let entry = entry?;
                let path: CompositePath = entry.try_into()?;
                #[cfg(unix)]
                print_perms(self.current_uid, &path, &mut w)?;
                print_size(&path, &mut w)?;
                print_created(&current_time, &path, &mut w)?;
                print_modified(&current_time, &path, &mut w)?;
                print_icon(&path, &mut w)?;
                print_name(&path, &mut w)?;
                writeln!(w)
            })
        } else {
            self.paths.try_for_each(|entry| -> io::Result<()> {
                let entry = entry?;
                let path: CompositePath = entry.try_into()?;

                if path.name.as_bytes().get(0).map(|&b| b) != Some(b'.') {
                    // If the name doesn't start with a .
                    #[cfg(unix)]
                    print_perms(self.current_uid, &path, &mut w)?;
                    print_size(&path, &mut w)?;
                    print_created(&current_time, &path, &mut w)?;
                    print_modified(&current_time, &path, &mut w)?;
                    print_icon(&path, &mut w)?;
                    print_name(&path, &mut w)?;
                    writeln!(w)
                } else {
                    Ok(())
                }
            })
        }
    }
}

impl Display for CompositePath {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name.to_string_lossy())
    }
}

impl Display for ColoredCompositePath<'_> {
    #[cfg(unix)]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let perms = self.0.permissions.user();
        let name = self.0.name.to_string_lossy();
        match self.0.dir_or_file {
            DirOrFile::Dir => {
                if perms.read() {
                    write!(f, "{}", "".blue().bold())
                } else {
                    write!(f, "{}", "".yellow().bold())
                }
            }
            DirOrFile::File(_) => match (perms.read(), perms.execute()) {
                (false, false) => write!(f, "{}", name.fg_rgb::<128, 128, 128>()),
                (false, true) => write!(f, "{}", name.bold().fg_rgb::<128, 128, 128>()),
                (true, true) => write!(f, "{}", name.bold().green()),
                (true, false) => write!(f, "{}", name),
            },
        }
    }

    #[cfg(not(unix))]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.0.dir_or_file {
            DirOrFile::Dir => write!(f, "{}", "".blue().bold()),
            DirOrFile::File(_) => write!(f, "{}", ""),
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

impl Display for Icon<'_> {
    #[cfg(unix)]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.0.dir_or_file {
            DirOrFile::Dir => write!(f, ""),
            DirOrFile::File(_) => {
                if self.0.permissions.user().execute() {
                    write!(f, "")
                } else {
                    write!(f, "")
                }
            }
        }
    }
    #[cfg(not(unix))]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.0.dir_or_file {
            DirOrFile::Dir => write!(f, ""),
            DirOrFile::File(_) => write!(f, ""),
        }
    }
}

impl Display for ColoredIcon<'_> {
    #[cfg(unix)]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let perms = self.0.permissions.user();
        match self.0.dir_or_file {
            DirOrFile::Dir => {
                if perms.read() {
                    write!(f, "{}", "".blue().bold())
                } else {
                    write!(f, "{}", "".yellow().bold())
                }
            }
            DirOrFile::File(_) => match (perms.read(), perms.execute()) {
                (false, false) => write!(f, "{}", "".fg_rgb::<128, 128, 128>()),
                (false, true) => write!(f, "{}", "".fg_rgb::<128, 128, 128>()),
                (true, true) => write!(f, "{}", "".yellow()),
                (true, false) => write!(f, "{}", ""),
            },
        }
    }

    #[cfg(not(unix))]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.0.dir_or_file {
            DirOrFile::Dir => write!(f, "{}", "".blue().bold()),
            DirOrFile::File(_) => write!(f, "{}", ""),
        }
    }
}
