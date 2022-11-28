//! This library has definitions for various color types and assorted utilities for manipulating and
//! working with the RGB8 color type. Colors are intended to be compatible with the
//! [smart-leds](https://github.com/smart-leds-rs/smart-leds) crate, which in turn is compatible
//! with the [rgb](https://github.com/kornelski/rust-rgb) crate.

use crate::utility::Progression;
use rgb::RGB8;
use smart_leds::colors::*;

pub fn color_lerp(
    factor: i32,
    in_min: i32,
    in_max: i32,
    start_color: RGB8,
    end_color: RGB8,
) -> RGB8 {
    let lerp = |start: u8, end: u8| {
        let start = start as i32;
        let end = end as i32;
        ((factor - in_min) * (end - start) / (in_max - in_min) + start) as u8
    };
    let mut mid_color = <RGB8>::new(0, 0, 0);
    mid_color.r = lerp(start_color.r, end_color.r);
    mid_color.g = lerp(start_color.g, end_color.g);
    mid_color.b = lerp(start_color.b, end_color.b);
    mid_color
}

pub trait ManipulatableColor<RgbType> {
    fn lerp_with(&self, to_color: RgbType, factor: Progression) -> RgbType;
    fn set_color(&mut self, c: RgbType);
}

impl ManipulatableColor<RGB8> for RGB8 {
    fn lerp_with(&self, to_color: RGB8, factor: Progression) -> RGB8 {
        color_lerp(
            factor.get_current() as i32,
            0,
            factor.total as i32,
            *self,
            to_color,
        )
    }

    fn set_color(&mut self, c: RGB8) {
        self.r = c.r;
        self.g = c.g;
        self.b = c.b;
    }
}

// Generic colors:
pub const DEEP_BLUE: RGB8 = RGB8 {
    r: 0,
    g: 127,
    b: 255,
};
pub const BLUE_PURPLE: RGB8 = RGB8 {
    r: 127,
    g: 0,
    b: 255,
};
pub const FUCHSIA: RGB8 = RGB8 {
    r: 255,
    g: 0,
    b: 255,
};
pub const DARK_PURPLE: RGB8 = RGB8 {
    r: 255,
    g: 0,
    b: 127,
};
pub const T_3000K: RGB8 = RGB8 {
    r: 255,
    g: 180,
    b: 107,
};
pub const T_3500K: RGB8 = RGB8 {
    r: 255,
    g: 196,
    b: 137,
};
pub const T_4000K: RGB8 = RGB8 {
    r: 255,
    g: 209,
    b: 163,
};
pub const T_5000K: RGB8 = RGB8 {
    r: 255,
    g: 228,
    b: 206,
};

// Use const generic rainbows to make iterable rainbows of various sizes. Rainbows contain a
// list of colors in order, which will be used by animations as a color rainbow.
pub type Rainbow<'a> = &'a [RGB8];

pub const R_BLACK: Rainbow = &[BLACK];
pub const R_WHITE: Rainbow = &[WHITE];
pub const R_RED: Rainbow = &[RED];
pub const R_ORANGE: Rainbow = &[ORANGE];
pub const R_YELLOW: Rainbow = &[YELLOW];
pub const R_CHARTREUSE: Rainbow = &[CHARTREUSE];
pub const R_LIME: Rainbow = &[LIME];
pub const R_SPRING_GREEN: Rainbow = &[SPRING_GREEN];
pub const R_CYAN: Rainbow = &[CYAN];
pub const R_DEEP_BLUE: Rainbow = &[DEEP_BLUE];
pub const R_BLUE: Rainbow = &[BLUE];
pub const R_BLUE_PURPLE: Rainbow = &[BLUE_PURPLE];
pub const R_FUCHSIA: Rainbow = &[FUCHSIA];
pub const R_DARK_PURPLE: Rainbow = &[DARK_PURPLE];
pub const R_ROYGBIV: Rainbow = &[RED, YELLOW, LIME, BLUE];
pub const R_RYB: Rainbow = &[RED, BLACK, YELLOW, BLACK, BLUE, BLACK];
pub const R_OGP: Rainbow = &[ORANGE, BLACK, LIME, BLACK, FUCHSIA, BLACK];
pub const R_RGB: Rainbow = &[RED, BLACK, LIME, BLACK, BLUE, BLACK];
pub const R_BY: Rainbow = &[BLUE, BLACK, YELLOW, BLACK];
pub const R_RB: Rainbow = &[RED, BLACK, CYAN, BLACK];
pub const R_OB: Rainbow = &[ORANGE, BLACK, DEEP_BLUE, BLACK];
pub const R_BW: Rainbow = &[BLUE, BLACK, WHITE, BLACK];
pub const R_RW: Rainbow = &[RED, BLACK, WHITE, BLACK];
pub const R_GW: Rainbow = &[LIME, BLACK, WHITE, BLACK];

pub const fn dark_pattern(base: RGB8) -> [RGB8; 6] {
    let mut colors = [BLACK; 6];
    let mut i = 0;
    while i < 3 {
        colors[i * 2] = RGB8 {
            r: base.r / 2,
            g: base.g / 2,
            b: base.b / 2,
        };
        colors[i * 2 + 1] = RGB8 {
            r: base.r / 4,
            g: base.g / 4,
            b: base.b / 4,
        };
        i += 1;
    }
    colors
}

pub const R_DARK_RED_PATTERN: Rainbow = &dark_pattern(RED);
pub const R_DARK_YELLOW_PATTERN: Rainbow = &dark_pattern(YELLOW);
pub const R_DARK_GREEN_PATTERN: Rainbow = &dark_pattern(LIME);
pub const R_DARK_SKY_BLUE_PATTERN: Rainbow = &dark_pattern(CYAN);
pub const R_DARK_BLUE_PATTERN: Rainbow = &dark_pattern(BLUE);
pub const R_DARK_PURPLE_PATTERN: Rainbow = &dark_pattern(FUCHSIA);
pub const R_WHITE_PATTERN: Rainbow = &dark_pattern(WHITE);
pub const R_VU_METER: Rainbow = &[
    LIME, LIME, LIME, LIME, LIME, LIME, LIME, YELLOW, YELLOW, RED,
];

pub const NUM_RAINBOWS: usize = 31;

/// This is an array of the rainbow consts above that can be used to cycle through rainbows in animations.
pub const RAINBOW_ARRAY: [&[RGB8]; NUM_RAINBOWS] = [
    R_BLACK,
    R_WHITE,
    R_RED,
    R_ORANGE,
    R_YELLOW,
    R_CHARTREUSE,
    R_LIME,
    R_SPRING_GREEN,
    R_CYAN,
    R_DEEP_BLUE,
    R_BLUE,
    R_BLUE_PURPLE,
    R_FUCHSIA,
    R_DARK_PURPLE,
    R_ROYGBIV,
    R_RYB,
    R_OGP,
    R_RGB,
    R_BY,
    R_RB,
    R_OB,
    R_BW,
    R_RW,
    R_GW,
    R_DARK_RED_PATTERN,
    R_DARK_YELLOW_PATTERN,
    R_DARK_GREEN_PATTERN,
    R_DARK_SKY_BLUE_PATTERN,
    R_DARK_BLUE_PATTERN,
    R_DARK_PURPLE_PATTERN,
    R_WHITE_PATTERN,
];
