use omst::{omst, ResultExt};
use std::{
    io::{self, Write},
    process::ExitCode,
};

fn main() -> io::Result<ExitCode> {
    let omst = omst();
    let is_error = omst.is_err();
    let omst = omst.display();
    io::stdout().write_fmt(format_args!("{}\n", omst))?;
    Ok(if is_error {
        ExitCode::FAILURE
    } else {
        ExitCode::SUCCESS
    })
}
