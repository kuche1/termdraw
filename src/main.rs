use crossterm::{
    // cargo add crossterm
    ExecutableCommand,
    style::{Color, Print, ResetColor, SetBackgroundColor, SetForegroundColor},
};

const PIXEL: &str = "▀";

pub type Col = (u8, u8, u8);

pub struct TermDraw {
    stdout: std::io::Stdout,
}

impl TermDraw {
    pub fn new() -> Self {
        TermDraw {
            stdout: std::io::stdout(),
        }
    }

    pub fn print_pixel(&mut self, color_top: Col, color_bot: Col) {
        let (tr, tg, tb) = color_top;
        let (br, bg, bb) = color_bot;

        self.stdout
            .execute(SetForegroundColor(Color::Rgb {
                r: tr,
                g: tg,
                b: tb,
            }))
            .unwrap();

        self.stdout
            .execute(SetBackgroundColor(Color::Rgb {
                r: br,
                g: bg,
                b: bb,
            }))
            .unwrap();

        self.stdout.execute(Print(PIXEL)).unwrap();

        self.stdout.execute(ResetColor).unwrap();
    }
}

fn main() {
    let mut drawer = TermDraw::new();

    drawer.print_pixel((255, 0, 0), (0, 255, 0));

    println!();

    let (cols, rows) = crossterm::terminal::size().unwrap();
    println!("cols={cols} rows={rows}");
}
