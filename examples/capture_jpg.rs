use minicap_client::read_from_stream;
use std::fs::File;
use std::io::{Result, Write};
use std::net::TcpStream;

fn main() -> Result<()> {
    let mut stream = TcpStream::connect("localhost:1717")?;
    let screen = read_from_stream(&mut stream)?;
    write_jpg(&screen.frame_body, "screenshot.jpg")?;

    Ok(())
}

fn write_jpg(buf: &[u8], filename: &str) -> Result<()> {
    assert!(
        buf[0] == 0xFF && buf[1] == 0xD8,
        "Frame body does not start with JPG header."
    );
    let mut file = File::create(filename)?;
    file.write_all(buf)?;
    Ok(())
}
