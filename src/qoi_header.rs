pub struct QoiHeader {
    magic_bytes: String,
    width: u32,
    height: u32,
    channels: u8,
    colorspace: u8,
}

impl QoiHeader {
    pub fn init(width: u32, height: u32, channels: u8, colorspace: u8) -> QoiHeader {
        QoiHeader {
            magic_bytes: String::from("qoif"),
            width,
            height,
            channels,
            colorspace,
        }
    }
}
