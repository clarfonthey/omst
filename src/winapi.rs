//! Determine the user permissions based upon the Windows API `NetUserGetInfo` function.
//!
//! The Windows API has several different ways of getting user permissions, but the way this
//! library does so is by obtaining a `USER_INFO_1` struct and checking the `ussri1_priv` field;
//! the value of this field is either `USER_PRIV_GUEST`, `USER_PRIV_USER`, or `USER_PRIV_ADMIN`
//! depending on the permission level of the user, and these are mapped to [`Permissions::Guest`],
//! [`Permissions::User`], and [`Permissions::Absolute`] respectively.
//!
//! To actually call the `NetUserGetInfo` function, we first call `GetUserNameW` to get the current
//! user name, then pass this to `NetUserGetInfo` to obtain a `USER_INFO_1` struct with the data we
//! need.
//!
//! The implementation was derived from
//! [this answer on Stack Overflow](https://stackoverflow.com/a/45125995).
use crate::Permissions;
use std::error::Error as StdError;
use std::fmt;
use std::io::Error as IoError;
use std::mem::size_of;
use std::process::abort;
use std::ptr;
use winapi::ctypes::c_void;
use winapi::shared::lmcons::UNLEN;
use winapi::shared::minwindef::{BYTE, DWORD};
use winapi::um::lmaccess::{
    NetUserGetInfo, USER_INFO_1, USER_PRIV_ADMIN, USER_PRIV_GUEST, USER_PRIV_USER,
};
use winapi::um::lmapibuf::NetApiBufferFree;
use winapi::um::winbase::GetUserNameW;
use winapi::um::winnt::WCHAR;

/// Operation done when getting user privileges.
#[derive(Debug)]
pub enum Operation {
    /// `GetUserNameW`.
    GetUserName,

    /// `NetNetUserGetInfo`.
    NetUserGetInfo,
}
impl fmt::Display for Operation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.pad(match self {
            Operation::GetUserName => "get username",
            Operation::NetUserGetInfo => "get user info",
        })
    }
}

/// Error that can occur when getting permissions.
#[derive(Debug)]
pub enum Error {
    /// Error getting privileges.
    GetPriv {
        /// Operation that failed.
        operation: Operation,

        /// Error that occurred.
        error: IoError,
    },

    /// Invalid user privileges.
    InvalidPriv { data: DWORD },
}
impl StdError for Error {}
impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::GetPriv { operation, error } => {
                write!(f, "could not {operation} due to error: {error}")
            }
            Error::InvalidPriv { data } => {
                write!(f, "user privileges had invalid value ({data:#x})")
            }
        }
    }
}

#[repr(transparent)]
struct UserInfoPtr(*mut USER_INFO_1);
impl Drop for UserInfoPtr {
    fn drop(&mut self) {
        if !self.0.is_null() {
            // shouldn't be needed, but we're gonna do it anyway
            let ptr = self.0 as *mut c_void;
            self.0 = ptr::null_mut();

            let err = unsafe { NetApiBufferFree(ptr) };
            if err != 0 {
                abort();
            }
        }
    }
}

pub fn omst() -> Result<Permissions, Error> {
    let mut uname = [WCHAR::default(); UNLEN as usize];
    let mut ulen = size_of::<[WCHAR; UNLEN as usize]>() as DWORD;
    let mut uinfo = UserInfoPtr(ptr::null_mut());
    let uinfo_ptr = ptr::NonNull::from(&mut uinfo);

    let err = unsafe { GetUserNameW(uname.as_mut_ptr(), &mut ulen) };
    if err == 0 {
        return Err(Error::GetPriv {
            operation: Operation::GetUserName,
            error: IoError::last_os_error(),
        });
    }

    let err = unsafe {
        NetUserGetInfo(
            ptr::null(),
            uname.as_mut_ptr(),
            1,
            uinfo_ptr.cast::<*mut BYTE>().as_ptr(),
        )
    };
    if err != 0 {
        return Err(Error::GetPriv {
            operation: Operation::NetUserGetInfo,
            error: IoError::from_raw_os_error(err as i32),
        });
    }

    let privs = unsafe { *uinfo.0 }.usri1_priv;
    Ok(match privs {
        USER_PRIV_ADMIN => Permissions::Absolute,
        USER_PRIV_GUEST => Permissions::Guest,
        USER_PRIV_USER => Permissions::User,
        _ => return Err(Error::InvalidPriv { data: privs }),
    })
}
