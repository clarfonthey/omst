//! Determine the user permissions based upon the user ID and the data from `shadow-utils`.
//!
//! On all available systems, we assume that `uid == 0` is `root`, which we will always mark as
//! [`Permissions::Absolute`]. Beyond that, the `login.defs` file provided by `shadow-utils` gives
//! decent-enough information to help us guess the current user permissions.
//!
//! The `UID_MIN..=UID_MAX` range defined in `login.defs` determines the range of UIDs that are free
//! to allocate to "ordinary" users, which we mark as [`Permissions::User`]. UIDs below `UID_MIN`
//! are assumed to be system accounts, and UIDs above `UID_MAX` are assumed to be the `nobody` user
//! and/or any guest users.
//!
//! Although `login.defs` technically defines `SYS_UID_MIN..=SYS_UID_MAX` for system users and
//! `SUB_UID_MIN..=SUB_UID_MAX` for "subordinate users", these often don't tend to point to the
//! full ranges and aren't required to fill the rest of the UID range.
//!
//! You can see more details in the man page for `login.defs(5)` on what exactly is defined by
//! `login.defs`, and additionally check your own systems to see how well this assumption maps to
//! your system's UIDs.

use crate::Permissions;
use atoi::atoi;
use std::error::Error as StdError;
use std::fmt;
use std::fs::File;
use std::io::{BufRead, BufReader, Error as IoError, ErrorKind};
use std::ops::RangeInclusive;

#[derive(Debug)]
struct InvalidUid(Vec<u8>);
impl fmt::Display for InvalidUid {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} was not a valid UID", self.0.escape_ascii())
    }
}
impl StdError for InvalidUid {}

/// Loads the `UID_MIN..=UID_MAX` range from `login.defs`.
#[inline]
fn login_defs_uid_range() -> Result<RangeInclusive<libc::uid_t>, IoError> {
    let mut min = None;
    let mut max = None;

    let mut file = BufReader::new(File::open("/etc/login.defs")?);

    let mut vec = Vec::new();
    loop {
        vec.clear();
        if file.read_until(b'\n', &mut vec)? == 0 {
            let min = min.ok_or_else(|| {
                IoError::new(ErrorKind::NotFound, "UID_MIN not defined in login.defs")
            })?;
            let max = max.ok_or_else(|| {
                IoError::new(ErrorKind::NotFound, "UID_MAX not defined in login.defs")
            })?;
            return Ok(min..=max);
        }
        let buf = &vec[..];

        let comment_pos = buf.iter().rposition(|b| *b == b'#');
        let buf = match comment_pos {
            Some(pos) => &buf[..pos],
            None => buf,
        };
        let key_pos = buf.iter().position(|b| !b.is_ascii_whitespace());
        let buf = match key_pos {
            Some(pos) => &buf[pos..],
            None => continue,
        };
        let space_pos = buf.iter().position(|b| b.is_ascii_whitespace());
        let (key, buf) = match space_pos {
            Some(pos) => buf.split_at(pos),
            None => (buf, &b""[..]),
        };

        let is_min = match key {
            b"UID_MIN" => true,
            b"UID_MAX" => false,
            _ => continue,
        };

        let val_pos = buf.iter().position(|b| !b.is_ascii_whitespace());
        let buf = match val_pos {
            Some(pos) => &buf[pos..],
            None => {
                return Err(IoError::new(
                    ErrorKind::InvalidData,
                    if is_min {
                        "UID_MIN defined in login.defs without a value"
                    } else {
                        "UID_MAX defined in login.defs without a value"
                    },
                ))
            }
        };

        let end_pos = buf.iter().position(|b| b.is_ascii_whitespace());
        let val = match end_pos {
            Some(pos) => &buf[..pos],
            None => buf,
        };

        match atoi::<libc::uid_t>(val) {
            Some(id) => {
                if is_min {
                    min = Some(id);
                } else {
                    max = Some(id);
                }
            }
            None => {
                return Err(IoError::new(
                    ErrorKind::InvalidData,
                    InvalidUid(val.to_vec()),
                ))
            }
        }
    }
}

pub fn omst() -> Permissions {
    let eff = unsafe { libc::geteuid() };
    if eff == 0 {
        Permissions::Absolute
    } else {
        match login_defs_uid_range() {
            Ok(range) => {
                if eff < *range.start() {
                    Permissions::System
                } else if eff > *range.end() {
                    Permissions::Guest
                } else {
                    Permissions::User
                }
            }
            Err(err) => {
                eprintln!("{}", err);
                Permissions::Unknown
            }
        }
    }
}
