use crate::pixel::Pixel;

pub fn calc_qoi_op_index(index: u8) -> u8 {
    0x00 | index
}

pub fn calc_qoi_op_run(run: u8) -> u8 {
    0x11 | (run - 1)
}

pub fn calc_qoi_rgb(pixel: Pixel) -> [u8; 4] {
    [0xfe, pixel.r, pixel.g, pixel.b]
}

pub fn calc_qoi_rgba(pixel: Pixel) -> [u8; 5] {
    [0xfe, pixel.r, pixel.g, pixel.b, pixel.a]
}

pub fn calc_qoi_op_diff(dr: u8, dg: u8, db: u8) -> u8 {
    0x40 | ((dr + 2) << 4) | ((dg + 2) << 2) | ((db + 2) << 0)
}

pub fn calc_qoi_op_luma(dr: u8, dg: u8, db: u8) -> [u8; 2] {
    [0x80 | (dg + 32), 0x00 | ((dr - dg + 8) << 4) | ((db - dg + 8) << 0)]
}