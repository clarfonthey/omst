use omst::{omst, ResultExt};
use std::io::{self, Write};
use std::process::ExitCode;

fn main() -> io::Result<ExitCode> {
    let omst = omst();
    let is_error = omst.is_err();
    let omst = omst.be();
    io::stdout().write_all(omst.encode_utf8(&mut [0; 4]).as_bytes())?;
    io::stdout().write_all(b"\n")?;
    Ok(if is_error {
        ExitCode::FAILURE
    } else {
        ExitCode::SUCCESS
    })
}
