use crossterm::{
    // cargo add crossterm
    ExecutableCommand,
    style::{Color, Print, ResetColor, SetBackgroundColor, SetForegroundColor},
};
use std::io::stdout;

const PIXEL: &str = "▀";

type Col = (u8, u8, u8);

fn print_pixel(color_top: Col, color_bot: Col) {
    let (tr, tg, tb) = color_top;
    let (br, bg, bb) = color_bot;

    let mut stdout = stdout();

    stdout
        .execute(SetForegroundColor(Color::Rgb {
            r: tr,
            g: tg,
            b: tb,
        }))
        .unwrap();

    stdout
        .execute(SetBackgroundColor(Color::Rgb {
            r: br,
            g: bg,
            b: bb,
        }))
        .unwrap();

    stdout.execute(Print(PIXEL)).unwrap();

    stdout.execute(ResetColor).unwrap();
}

fn main() {
    print_pixel((255, 0, 0), (0, 255, 0));
    println!();
}
