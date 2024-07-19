
#![feature(portable_simd)]
use rand::{rngs::ThreadRng, Rng};
use std::{
    cmp::min,
    io::{self, Stdout, Write}, ops::Range,
};
#[cfg(feature = "simd")]
use std::simd::i16x4;
use termion::{color, cursor};


// number of characters in each char-rains
const LENGTH: u8 = 40;
const LENGTH_U16: u16 = LENGTH as u16;

pub struct ColorInfo {
    #[cfg(not(feature = "simd"))]
    start: color::Rgb,
    #[cfg(not(feature = "simd"))]
    end: color::Rgb,
    #[cfg(not(feature = "simd"))]
    color_step: (i8, i8, i8),
    #[cfg(feature = "simd")]
    start: i16x4,
    #[cfg(feature = "simd")]
    color_step: i16x4,
}

impl ColorInfo {
    #[cfg(feature = "simd")]
    pub fn new(start: i16x4, end: i16x4) -> Self {
        Self {
            start,
            color_step: (end - start) / i16x4::splat(LENGTH as i16)
        }
    }
    #[cfg(not(feature = "simd"))]
    fn new(start: color::Rgb, end: color::Rgb) -> Self {
        let red_step = (end.0 - start.0) / LENGTH;
        let green_step = (end.1 - start.1) / LENGTH;
        let blue_step = (end.2 - start.2) / LENGTH;
        Self {
            start,
            end,
            red_step,
            green_step,
            blue_step,
        }
    }
}
pub struct ScreenInfo<'a> {
    pub color_info: ColorInfo,
    pub term_size: &'a mut (u16, u16),
}

pub struct CharRain {
    x: u16,
    /// position of bottom character
    y: u16,
}
pub enum LineState {
    Falling,
    ReachedEnd,
}

pub fn from_i16x4_to_rgb(input: i16x4) -> color::Rgb {
    color::Rgb(
        input[0] as u8,
        input[1] as u8,
        input[2] as u8
    )
}
impl CharRain {
    pub fn new(x: u16, y: u16) -> Self {
        CharRain { x, y }
    }
    pub fn new_random_x_range(rng: &mut ThreadRng, range: Range<u16>) -> Self {
        CharRain { x: rng.gen_range(range), y: 1 }
    }
    pub fn redrop_random_x_range(&mut self, rng: &mut ThreadRng, range: Range<u16>) {
        self.x = rng.gen_range(range);
        self.y = 1;
    }
    pub fn draw(
        &mut self,
        stdout: &mut Stdout,
        rng: &mut ThreadRng,
        screen_info: &ScreenInfo
    ) -> io::Result<LineState> {
        let &mut Self { x, y } = self;
        write!(stdout, "{}", cursor::Goto(x, y))?;
        let shown_length = min(y, LENGTH_U16);
        let ScreenInfo { color_info, term_size: &mut (.., height) } = screen_info;
        let (shown_length, mut current_color) = if y >= height {
            if y - LENGTH_U16 >= height {
                return Ok(LineState::ReachedEnd);
            }
            let gap = y - height;
            let color_start = color_info.start + color_info.color_step * i16x4::splat(gap as i16);
            write!(stdout, "{}", color::Fg(from_i16x4_to_rgb(color_start)))?;

            (
                shown_length - gap,
                color_start
            )
        } else {
            write!(stdout, "{}", color::Fg(color::White))?;
            (shown_length, color_info.start)
        };
        for count in 1..=shown_length {
            let color = from_i16x4_to_rgb(current_color);
            write!(
                stdout,
                "{}{}{}",
                rng.gen_range::<u8, _>(33..126) as char,
                color::Fg(color),
                cursor::Left(1),
            )?;
            if count != shown_length {
                write!(
                    stdout,
                    "{}",
                    cursor::Up(1),
                )?; 
            }
            current_color += color_info.color_step;
        }
        if y >= LENGTH_U16 {
            write!(stdout, " ")?;
        }

        self.y += 1;
        Ok(LineState::Falling)
    }
}
