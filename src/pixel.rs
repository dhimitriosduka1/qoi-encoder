pub struct Pixel {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Pixel {
    pub fn init(r: u8, g: u8, b: u8, a: u8) -> Pixel {
        Pixel {
            r,
            g,
            b,
            a,
        }
    }

    pub fn hash(self) -> u8 {
        ((self.r as u16 * 3 + self.g as u16 * 5 + self.b as u16 * 7 + self.a as u16 * 11) % 64) as u8
    }
}

impl PartialEq<Self> for Pixel {
    fn eq(&self, other: &Self) -> bool {
        self.r == other.r && self.g == other.g && self.b == other.b && self.a == other.a
    }
}

