#[derive(Debug, Clone)]
pub struct Hex {
    hex: String,
}

impl Hex {
    pub fn from_hex(hex: String) -> Hex {
        let body = hex[1..7].to_ascii_uppercase();
        let upper_cased_hex = ["#", &body].concat();
        return Hex {
            hex: upper_cased_hex,
        };
    }
    pub fn to_rgb(&self) -> RGB {
        let r = i32::from_str_radix(&self.hex[1..3], 16).expect("Cannot convert to hex.");
        let g = i32::from_str_radix(&self.hex[3..5], 16).expect("Cannot convert to hex.");
        let b = i32::from_str_radix(&self.hex[5..7], 16).expect("Cannot convert to hex.");

        return RGB::from_rgb((r, g, b));
    }
}

#[derive(Debug, Clone)]
pub struct RGB {
    pub rgb: (i32, i32, i32),
}

impl RGB {
    pub fn from_rgb(rgb: (i32, i32, i32)) -> RGB {
        return RGB { rgb };
    }
    pub fn to_hex(&self) -> Hex {
        let (r, g, b) = &self.rgb;
        let r_hex = format!("{:02X}", r);
        let g_hex = format!("{:02X}", g);
        let b_hex = format!("{:02X}", b);

        let value = ["#", &r_hex, &g_hex, &b_hex].concat();

        return Hex::from_hex(value);
    }
    pub fn to_hsv(&self) -> HSV {
        let h = self.hue();
        let s = self.saturation();
        let v = self.value();

        return HSV::from_value(h, s, v);
    }

    fn max(&self) -> i32 {
        let (r, g, b) = self.rgb;
        return [r, g, b].iter().max().expect("No max").clone();
    }
    fn min(&self) -> i32 {
        let (r, g, b) = self.rgb;
        return [r, g, b].iter().min().expect("No max").clone();
    }

    fn hue(&self) -> i32 {
        let max = self.max();
        let min = self.min();
        let (r, g, b) = self.rgb;

        if max == 0 {
            return 0;
        }

        let delta = max - min;

        if delta == 0 {
            return 0;
        }

        let h = if max == r {
            60 * (b - g) / delta
        } else if g == max {
            60 * (2 + (r - b) / delta)
        } else {
            60 * (4 + (g - r) / delta)
        };

        if h < 0 {
            return h + 360;
        }

        return h;
    }
    fn saturation(&self) -> i32 {
        let max = self.max();

        if max == 0 {
            return 0;
        }

        return max;
    }
    fn value(&self) -> i32 {
        let max = self.max();
        let min = self.min();

        if max == 0 {
            return 0;
        }

        return 255 * (max - min) / max;
    }
}

#[derive(Debug, Clone)]
pub struct HSV {
    hsv: (i32, i32, i32),
}

impl HSV {
    pub fn from_value(h: i32, s: i32, v: i32) -> HSV {
        return HSV { hsv: (h, s, v) };
    }
    pub fn to_rgb(&self) -> RGB {
        let (h, s, v) = &self.hsv;

        if *s == 0 {
            return RGB::from_rgb((0, 0, 0));
        }

        let h_float = *h as f32;
        let s_float = *s as f32;
        let v_float = *v as f32;

        let hi = (h_float) / 60.0;
        let hi_floored = hi.floor();
        let f = hi - hi_floored;

        let m = v_float * (1.0 - s_float / 255.0);
        let n = v_float * (1.0 - s_float / 255.0 * f);
        let k = v_float * (1.0 - s_float / 255.0 * (1.0 - f));

        println!("{}: {} {} {} {}", h, v, m, n, k);
        let (r_float, g_float, b_float) = match hi_floored {
            0.0 => (v_float, k, m),
            1.0 => (n, v_float, m),
            2.0 => (m, v_float, k),
            3.0 => (m, n, v_float),
            4.0 => (k, m, v_float),
            5.0 => (v_float, m, n),
            _ => (v_float, k, m),
        };

        let r = r_float as i32;
        let g = g_float as i32;
        let b = b_float as i32;

        return RGB::from_rgb((r, g, b));
    }
}
