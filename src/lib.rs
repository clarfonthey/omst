//! Reveals whomst thou art with a single character.
//!
//! This crate provides functions which ultimately are used to provide the functionality for the
//! `omst` binary.
#![warn(unsafe_op_in_unsafe_fn)]
use core::fmt;

cfg_if::cfg_if! {
    if #[cfg(windows)] {
        #[path = "winapi.rs"]
        mod r#impl;
    } else {
        #[path = "shadow.rs"]
        mod r#impl;
    }
}

pub use r#impl::{omst, Error};

/// Summary of a user's permissions.
///
/// This indicator is purely informational and should not be assumed to have any level of security.
#[derive(Copy, Clone, Eq, PartialEq, Hash, PartialOrd, Ord, Debug)]
#[repr(u8)]
pub enum Permissions {
    /// Restricted permissions.
    ///
    /// Usually, these users will be ephemeral and have their files deleted after logging out.
    ///
    /// # System-specific behavior
    ///
    /// On POSIX-based systems, this includes at least the `nobody` user,
    /// but may include other dedicated guest users.
    ///
    /// On Windows, this is specifically guest users.
    Guest = b'%',

    /// Ordinary user permissions.
    ///
    /// Users that represent a real person will have this permission level.
    User = b'$',

    /// System service permissions.
    ///
    /// These are users dedicated to running system services who may have elevated privileges, but
    /// do not have absolute system access.
    ///
    /// # System-specific behavior
    ///
    /// This is mostly only available on unix-family systems, for users with a UID below `UID_MIN`.
    /// In most cases, this means a UID below 1000, but some systems may start allocating ordinary
    /// users at UID 500.
    System = b'@',

    /// Absolute permissions.
    ///
    /// These users have full access to the system, to the extent that the OS allows.
    ///
    /// # System-specific behavior
    ///
    /// On unix-family systems and Android, this is the root user.
    ///
    /// On Windows, this refers to users with administrator privileges.
    Absolute = b'#',
}
impl Permissions {
    /// The permissions as a single ASCII character.
    ///
    /// In most cases, you want to use [`be`](Self::be) instead.
    #[inline]
    pub fn byte(self) -> u8 {
        self as u8
    }

    /// The permissions as a single character.
    ///
    /// Most often used as `omst().be()`.
    #[inline]
    pub fn be(self) -> char {
        self.byte() as char
    }
}
impl fmt::Display for Permissions {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.pad(match self {
            Permissions::Guest => "guest",
            Permissions::User => "user",
            Permissions::System => "system",
            Permissions::Absolute => "aboslute",
        })
    }
}

pub struct DisplayResult(Result<Permissions, Error>);
impl fmt::Display for DisplayResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.0 {
            Ok(ok) => fmt::Display::fmt(ok, f),
            Err(err) => fmt::Display::fmt(err, f),
        }
    }
}

/// Extension trait for return value of [`omst`].
pub trait ResultExt: Sized {
    /// The permissions as a single ASCII character.
    ///
    /// In most cases, you want to use [`be`](Self::be) instead.
    fn byte(self) -> u8;

    /// The permissions as a single character.
    ///
    /// Most often used as `omst().be()`.
    fn be(self) -> char;

    /// The permissions as a displayable value.
    ///
    /// Will fully explain errors.
    fn display(self) -> DisplayResult;
}
impl ResultExt for Result<Permissions, Error> {
    #[inline]
    fn byte(self) -> u8 {
        self.map_or(b'?', Permissions::byte)
    }
    #[inline]
    fn be(self) -> char {
        self.byte() as char
    }
    #[inline]
    fn display(self) -> DisplayResult {
        DisplayResult(self)
    }
}

#[test]
fn is_known() {
    assert!(omst().is_ok());
}
