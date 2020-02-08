use std::io::{Read, Result};

pub fn read_from_stream<R: Read>(stream: &mut R) -> Result<MiniCapScreen> {
    let mut banner = [0; 24];
    stream.read_exact(&mut banner)?;
    let banner = parse_banner(&banner);

    let mut header_buf = [0; 4];
    stream.read_exact(&mut header_buf)?;
    let body_length = encode_u32(&header_buf);

    let mut frame_body = vec![0; body_length as usize];
    stream.read_exact(&mut frame_body)?;

    assert!(
        frame_body[0] == 0xFF && frame_body[1] == 0xD8,
        "Frame body does not start with JPG header."
    );

    Ok(MiniCapScreen { frame_body, banner })
}

#[derive(Debug, Clone)]
pub struct MiniCapScreen {
    pub frame_body: Vec<u8>,
    pub banner: Banner,
}

#[derive(Debug, Clone)]
pub struct Banner {
    pub version: u8,
    pub pid: u32,
    pub real_width: u32,
    pub real_height: u32,
    pub virtual_width: u32,
    pub virtual_height: u32,
    pub orientation: u8,
    pub quirks: u8,
}

fn encode_u32(buf: &[u8]) -> u32 {
    assert_eq!(buf.len(), 4);
    let mut result = 0;
    for i in 0..4 {
        result += (buf[i] as u32) << (i as u32 * 8);
    }
    result
}

fn parse_banner(buf: &[u8]) -> Banner {
    assert_eq!(buf.len(), 24);
    let version = buf[0];
    let banner_length = buf[1];
    assert_eq!(banner_length, 24);
    let pid = encode_u32(&buf[2..6]);
    let real_width = encode_u32(&buf[6..10]);
    let real_height = encode_u32(&buf[10..14]);
    let virtual_width = encode_u32(&buf[14..18]);
    let virtual_height = encode_u32(&buf[18..22]);
    let orientation = buf[22];
    let quirks = buf[23];
    Banner {
        version,
        pid,
        real_width,
        real_height,
        virtual_width,
        virtual_height,
        orientation,
        quirks,
    }
}
