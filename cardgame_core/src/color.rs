#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

impl Color {
    pub const fn new(r: f32, g: f32, b: f32, a: f32) -> Self {
        Self { r, g, b, a }
    }

    pub const fn rgb(r: f32, g: f32, b: f32) -> Self {
        Self { r, g, b, a: 1.0 }
    }

    pub const fn rgba(r: f32, g: f32, b: f32, a: f32) -> Self {
        Self { r, g, b, a }
    }

    pub const WHITE: Self = Self::rgb(1.0, 1.0, 1.0);
    pub const BLACK: Self = Self::rgb(0.0, 0.0, 0.0);
    pub const RED:   Self = Self::rgb(1.0, 0.0, 0.0);
    pub const GREEN: Self = Self::rgb(0.0, 0.6, 0.0);
    pub const BLUE:  Self = Self::rgb(0.0, 0.0, 1.0);
    pub const YELLOW:  Self = Self::rgb(0.87, 0.87, 1.0);
    pub const CLEAR: Self = Self::rgba(0.0, 0.0, 0.0, 0.0);

    #[inline]
    pub fn clamp01(self) -> Self {
        Self {
            r: self.r.clamp(0.0, 1.0),
            g: self.g.clamp(0.0, 1.0),
            b: self.b.clamp(0.0, 1.0),
            a: self.a.clamp(0.0, 1.0),
        }
    }

    #[inline]
    pub fn with_alpha(self, a: f32) -> Self {
        Self { a, ..self }
    }

    #[inline]
    pub fn premultiplied(self) -> Self {
        Self {
            r: self.r * self.a,
            g: self.g * self.a,
            b: self.b * self.a,
            a: self.a,
        }
    }

    #[inline]
    pub fn lerp(self, other: Self, t: f32) -> Self {
        let t = t.clamp(0.0, 1.0);
        Self {
            r: self.r + (other.r - self.r) * t,
            g: self.g + (other.g - self.g) * t,
            b: self.b + (other.b - self.b) * t,
            a: self.a + (other.a - self.a) * t,
        }
    }

    /// Convert to [u8; 4] RGBA (0..255)
    #[inline]
    pub fn to_rgba8(self) -> [u8; 4] {
        let c = self.clamp01();
        [
            (c.r * 255.0).round() as u8,
            (c.g * 255.0).round() as u8,
            (c.b * 255.0).round() as u8,
            (c.a * 255.0).round() as u8,
        ]
    }

    /// Convert from [u8; 4] RGBA (0..255)
    #[inline]
    pub fn from_rgba8(rgba: [u8; 4]) -> Self {
        Self {
            r: rgba[0] as f32 / 255.0,
            g: rgba[1] as f32 / 255.0,
            b: rgba[2] as f32 / 255.0,
            a: rgba[3] as f32 / 255.0,
        }
    }

    /// 0xRRGGBBAA
    #[inline]
    pub fn to_u32_rgba(self) -> u32 {
        let [r, g, b, a] = self.to_rgba8();
        ((r as u32) << 24) | ((g as u32) << 16) | ((b as u32) << 8) | (a as u32)
    }

    /// 0xRRGGBBAA
    #[inline]
    pub fn from_u32_rgba(v: u32) -> Self {
        let r = ((v >> 24) & 0xFF) as u8;
        let g = ((v >> 16) & 0xFF) as u8;
        let b = ((v >> 8) & 0xFF) as u8;
        let a = (v & 0xFF) as u8;
        Self::from_rgba8([r, g, b, a])
    }

    /// 0xRRGGBB (alpha assumed 1.0)
    #[inline]
    pub fn from_u32_rgb(v: u32) -> Self {
        let r = ((v >> 16) & 0xFF) as u8;
        let g = ((v >> 8) & 0xFF) as u8;
        let b = (v & 0xFF) as u8;
        Self::from_rgba8([r, g, b, 255])
    }

    /// Parse "#RRGGBB" or "#RRGGBBAA" (also accepts without '#')
    pub fn from_hex(mut s: &str) -> Option<Self> {
        if let Some(rest) = s.strip_prefix('#') {
            s = rest;
        }

        let bytes = match s.len() {
            6 => {
                let v = u32::from_str_radix(s, 16).ok()?;
                let r = ((v >> 16) & 0xFF) as u8;
                let g = ((v >> 8) & 0xFF) as u8;
                let b = (v & 0xFF) as u8;
                [r, g, b, 255]
            }
            8 => {
                let v = u32::from_str_radix(s, 16).ok()?;
                let r = ((v >> 24) & 0xFF) as u8;
                let g = ((v >> 16) & 0xFF) as u8;
                let b = ((v >> 8) & 0xFF) as u8;
                let a = (v & 0xFF) as u8;
                [r, g, b, a]
            }
            _ => return None,
        };

        Some(Self::from_rgba8(bytes))
    }

    /// Returns "#RRGGBBAA"
    pub fn to_hex_rgba(self) -> String {
        let [r, g, b, a] = self.to_rgba8();
        format!("#{:02X}{:02X}{:02X}{:02X}", r, g, b, a)
    }

    /// Returns "#RRGGBB" (alpha ignored)
    pub fn to_hex_rgb(self) -> String {
        let [r, g, b, _] = self.to_rgba8();
        format!("#{:02X}{:02X}{:02X}", r, g, b)
    }
}

impl Default for Color {
    fn default() -> Self {
        Self::WHITE
    }
}

impl From<[f32; 4]> for Color {
    fn from(v: [f32; 4]) -> Self {
        Self::rgba(v[0], v[1], v[2], v[3])
    }
}

impl From<Color> for [f32; 4] {
    fn from(c: Color) -> Self {
        [c.r, c.g, c.b, c.a]
    }
}

impl From<[u8; 4]> for Color {
    fn from(v: [u8; 4]) -> Self {
        Self::from_rgba8(v)
    }
}

impl From<Color> for [u8; 4] {
    fn from(c: Color) -> Self {
        c.to_rgba8()
    }
}
