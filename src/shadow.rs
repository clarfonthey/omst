use crate::Permissions;
use atoi::atoi;
use std::error::Error as StdError;
use std::fmt;
use std::fs::File;
use std::io::{self, BufRead, BufReader, ErrorKind};
use std::ops::RangeInclusive;

/// UID range from `/etc/login.defs`.
#[derive(Copy, Clone, Eq, PartialEq, Hash, PartialOrd, Ord, Debug)]
#[repr(u8)]
pub enum UidRange {
    /// Above `UID_MAX`.
    AboveMax = b'%',

    /// Inside `UID_MIN..=UID_MAX` range.
    InRange = b'$',

    /// Below `UID_MIN`.
    BelowMin = b'@',

    /// UID 0, root.
    Zero = b'#',
}
impl From<UidRange> for Permissions {
    #[inline]
    fn from(range: UidRange) -> Permissions {
        match range {
            UidRange::AboveMax => Permissions::Guest,
            UidRange::InRange => Permissions::User,
            UidRange::BelowMin => Permissions::System,
            UidRange::Zero => Permissions::Absolute,
        }
    }
}

/// Operation performed on `/etc/login.defs`.
#[derive(Debug)]
pub enum Operation {
    /// Opening the file.
    Open,

    /// Reading the file.
    Read,
}
impl fmt::Display for Operation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.pad(match self {
            Operation::Open => "open",
            Operation::Read => "read",
        })
    }
}

/// Definition in `/etc/login.defs`.
#[derive(Debug)]
pub enum Def {
    /// `UID_MIN`.
    Min,

    /// `UID_MAX`.
    Max,
}
impl fmt::Display for Def {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.pad(match self {
            Def::Min => "UID_MIN",
            Def::Max => "UID_MAX",
        })
    }
}

/// Problem with a definition in `/etc/login.defs`.
#[derive(Debug)]
pub enum Problem {
    /// Definition was missing.
    Missing,

    /// Definition was provided, but empty.
    Empty,

    /// Definition was not a valid UID.
    Invalid {
        /// Actual bytes of the UID.
        data: Vec<u8>,
    },
}
impl fmt::Display for Problem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Problem::Missing => write!(f, "was missing"),
            Problem::Empty => write!(f, "was empty"),
            Problem::Invalid { data } => write!(f, "was not a valid UID ({})", data.escape_ascii()),
        }
    }
}

/// Error that might occur when getting permissions. (`shadow-utils` implementation)
#[derive(Debug)]
pub enum Error {
    /// Error reading `/etc/login.defs`.
    LoginDefs {
        /// What operation caused the error.
        operation: Operation,

        /// The error.
        error: io::Error,
    },

    /// Invalid definition in `/etc/login.defs`.
    InvalidDef {
        /// Which definition was invalid.
        def: Def,

        /// What the problem was.
        problem: Problem,
    },
}
impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::LoginDefs { operation, error } => write!(
                f,
                "could not {operation} /etc/login.defs due to error: {error}"
            ),
            Error::InvalidDef { def, problem } => write!(f, "{def} in /etc/login.defs {problem}"),
        }
    }
}
impl StdError for Error {
    #[inline]
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        match self {
            Error::LoginDefs { error, .. } => Some(error),
            Error::InvalidDef { .. } => None,
        }
    }
}
impl From<Error> for io::Error {
    #[inline]
    fn from(err: Error) -> io::Error {
        io::Error::new(ErrorKind::InvalidData, err)
    }
}
impl Error {
    fn login_defs(operation: Operation) -> impl FnOnce(io::Error) -> Error {
        move |error| Error::LoginDefs { operation, error }
    }
}

/// Loads the `UID_MIN..=UID_MAX` range from `login.defs`.
#[inline]
fn login_defs_uid_range() -> Result<RangeInclusive<libc::uid_t>, Error> {
    let mut min = None;
    let mut max = None;

    let mut file =
        BufReader::new(File::open("/etc/login.defs").map_err(Error::login_defs(Operation::Open))?);

    let mut vec = Vec::new();
    loop {
        vec.clear();
        if file
            .read_until(b'\n', &mut vec)
            .map_err(Error::login_defs(Operation::Read))?
            == 0
        {
            let min = min.ok_or(Error::InvalidDef {
                def: Def::Min,
                problem: Problem::Empty,
            })?;
            let max = max.ok_or(Error::InvalidDef {
                def: Def::Max,
                problem: Problem::Empty,
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

        let def = match key {
            b"UID_MIN" => Def::Min,
            b"UID_MAX" => Def::Max,
            _ => continue,
        };

        let val_pos = buf.iter().position(|b| !b.is_ascii_whitespace());
        let buf = match val_pos {
            Some(pos) => &buf[pos..],
            None => {
                return Err(Error::InvalidDef {
                    def,
                    problem: Problem::Empty,
                })
            }
        };

        let end_pos = buf.iter().position(|b| b.is_ascii_whitespace());
        let val = match end_pos {
            Some(pos) => &buf[..pos],
            None => buf,
        };

        match atoi::<libc::uid_t>(val) {
            Some(id) => match def {
                Def::Min => min = Some(id),
                Def::Max => max = Some(id),
            },
            None => {
                return Err(Error::InvalidDef {
                    def,
                    problem: Problem::Invalid { data: val.to_vec() },
                })
            }
        }
    }
}

/// Determine [`UidRange`] based upon the user ID and the data from `shadow-utils`.
///
/// On all available systems, we special-case `uid == 0` as [`UidRange::Zero`], which corresponds
/// to [`Permissions::Absolute`](Permissions::Absolute). Beyond that, the `login.defs` file
/// provided by `shadow-utils` gives decent-enough information to help us guess the current user
/// permissions.
///
/// The `UID_MIN..=UID_MAX` range defined in `login.defs` determines the range of UIDs that are free
/// to allocate to "ordinary" users, and we assign [`UidRange::BelowMin`], [`UidRange::InRange`],
/// and [`UidRange::AboveMax`] depending on the UID's relation to the range. These are interpreted
/// as [`Permissions::System`], [`Permissions::User`], and [`Permissions::Guest`], respectively.
///
/// Although `login.defs` technically defines `SYS_UID_MIN..=SYS_UID_MAX` for system users and
/// `SUB_UID_MIN..=SUB_UID_MAX` for "subordinate users", these often don't tend to point to the
/// full ranges and aren't required to fill the rest of the UID range. Additionally, not all systems
/// will actually provide these ranges, so, we basically ignore them.
///
/// You can see more details in the man page for `login.defs(5)` on what exactly is defined by
/// `login.defs`, and additionally check your own systems to see how well this assumption maps to
/// your system's UIDs.
pub fn omst() -> Result<UidRange, Error> {
    let eff = unsafe { libc::geteuid() };
    if eff == 0 {
        Ok(UidRange::Zero)
    } else {
        login_defs_uid_range().map(|range| {
            if eff < *range.start() {
                UidRange::BelowMin
            } else if eff > *range.end() {
                UidRange::AboveMax
            } else {
                UidRange::InRange
            }
        })
    }
}
