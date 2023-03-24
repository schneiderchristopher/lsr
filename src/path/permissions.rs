use std::{
    fmt::Display,
    io::{self, Write},
};

use owo_colors::OwoColorize;
use users::User;

#[derive(Debug, Clone, Copy)]
#[cfg(unix)]
pub struct UnixPerms {
    pub(super) perms: u32,
    pub(super) owner_uid: u32,
}
#[cfg(unix)]
impl UnixPerms {
    // Implementations that print username, I'll avoid them for now as they require complex
    // alignment

    // pub fn print_color<W: Write>(&self, mut w: &mut W, current_uid: u32) -> io::Result<()> {
    //     let Some(user) = self.owner_username() else {
    //     return Ok(())};
    //     let Some(name) = user.name().to_str() else {
    //     return Ok(())};
    //     if current_uid == self.owner_uid {
    //         write!(w, "{} ", name)
    //     } else {
    //         write!(w, "{} ", name)
    //     }
    // }
    //
    // pub fn print_color<W: Write>(&self, mut w: &mut W, current_uid: u32) -> io::Result<()> {
    //     let Some(user) = self.owner_username() else {
    //     return Ok(())};
    //     let Some(name) = user.name().to_str() else {
    //     return Ok(())};
    //     if current_uid == self.owner_uid {
    //         write!(w, "{} ", name.yellow().bold())
    //     } else {
    //         write!(w, "{} ", name)
    //     }
    // }

    pub fn print<W: Write>(&self, mut w: &mut W, current_uid: u32) -> io::Result<()> {
        let (user, group, other) = self.all();
        for perm in [user, group, other] {
            perm.print(w)?
        }
        Ok(())
    }

    pub fn print_color<W: Write>(&self, mut w: &mut W, current_uid: u32) -> io::Result<()> {
        let (user, group, other) = self.all();
        write!(w, "\x1b[1m");
        user.print_color(w)?;
        write!(w, "\x1b[0m");
        for perm in [group, other] {
            perm.print_color(w)?
        }
        Ok(())
    }
}

#[cfg(unix)]
impl UnixPerms {
    pub fn user(&self) -> UserPerms {
        let UnixPerms { perms, .. } = self;
        let user = (perms & 0o700 >> 6) as u8;
        UserPerms(user)
    }

    pub fn group(&self) -> UserPerms {
        let UnixPerms { perms, .. } = self;
        let group = (perms & 0o70 >> 3) as u8;
        UserPerms(group)
    }

    pub fn other(&self) -> UserPerms {
        let UnixPerms { perms, .. } = self;
        let other = (perms & 0o7) as u8;
        UserPerms(other)
    }

    /// Returns all user permissions, in the order of user -> group -> other
    pub fn all(&self) -> (UserPerms, UserPerms, UserPerms) {
        let UnixPerms { perms, .. } = self;
        let user = ((perms & 0o700) >> 6) as u8;
        let group = ((perms & 0o70) >> 3) as u8;
        let other = (perms & 0o7) as u8;
        (UserPerms(user), UserPerms(group), UserPerms(other))
    }

    pub fn owner_username(&self) -> Option<User> {
        users::get_user_by_uid(self.owner_uid)
    }
}

#[cfg(unix)]
#[derive(Debug)]
pub struct UserPerms(pub(super) u8);

#[cfg(unix)]
impl UserPerms {
    pub fn read(&self) -> bool {
        (self.0 & 0o4) != 0
    }

    pub fn write(&self) -> bool {
        (self.0 & 0o2) != 0
    }

    pub fn execute(&self) -> bool {
        (self.0 & 0o1) != 0
    }

    pub fn all(&self) -> (bool, bool, bool) {
        (self.read(), self.write(), self.execute())
    }

    const PLACEHOLDER: char = '-';
    pub fn print<W: Write>(&self, mut w: &mut W) -> io::Result<()> {
        write!(w, "{}", self.0);
        let read = if self.read() { 'r' } else { Self::PLACEHOLDER };
        let write = if self.write() { 'w' } else { Self::PLACEHOLDER };
        let execute = if self.execute() {
            'x'
        } else {
            Self::PLACEHOLDER
        };
        write!(w, "{read}{write}{execute}")
    }

    pub fn print_color<W: Write>(&self, mut w: &mut W) -> io::Result<()> {
        if self.read() {
            write!(w, "{}", 'r'.yellow())
        } else {
            write!(w, "{}", Self::PLACEHOLDER.fg_rgb::<128, 128, 128>())
        }?;
        if self.write() {
            write!(w, "{}", 'w'.red())
        } else {
            write!(w, "{}", Self::PLACEHOLDER.fg_rgb::<128, 128, 128>())
        }?;

        if self.execute() {
            write!(w, "{}", 'x'.green())
        } else {
            write!(w, "{}", Self::PLACEHOLDER.fg_rgb::<128, 128, 128>())
        }
    }
}
