
#![feature(portable_simd)]
#[cfg(feature = "simd")]
use std::simd::i16x4;
use std::{
    io::{self, stdout},
    thread::sleep,
    time::Duration
};
use term_rain::*;

fn main() -> io::Result<()> {
    let color_info: ColorInfo = ColorInfo::new(i16x4::from([4, 255, 0, 0]), i16x4::from([27, 64, 27, 0]));
    let mut rng = rand::thread_rng();
    let mut stdout = stdout();
    let mut term_size = termion::terminal_size()?;
    let screen_info = ScreenInfo { color_info, term_size: &mut term_size };
    let mut char_rains = Vec::<CharRain>::new();
    let mut done_appending = false;
    loop {
        *screen_info.term_size = termion::terminal_size()?;
        if !done_appending {
            char_rains.push(CharRain::new_random(&mut rng, screen_info.term_size.0));
        }
        for rain in &mut char_rains {
            if let LineState::ReachedEnd = rain.draw(&mut stdout, &mut rng, &screen_info)? {
                if !done_appending {
                    done_appending = true;
                }
                rain.redrop_random(&mut rng, screen_info.term_size.0);
            }

        }
        sleep(Duration::from_millis(15));
        if false {
            break;
        }
    }

    Ok(())
}
