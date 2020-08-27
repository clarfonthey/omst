use std::io::{self, Write};

fn main() -> io::Result<()> {
    let omst = omst::omst().be();
    io::stdout().write_all(omst.encode_utf8(&mut [0; 4]).as_bytes())?;
    io::stdout().write_all(b"\n")
}
