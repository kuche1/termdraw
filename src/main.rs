use crossterm::{
    // cargo add crossterm
    ExecutableCommand,
    style::{Color, Print, ResetColor, SetBackgroundColor, SetForegroundColor},
};

const PIXEL: &str = "▀";

pub type Col = (u8, u8, u8);

pub struct TermDraw {
    stdout: std::io::Stdout,
    width: u32,
    height: u32,
}

impl TermDraw {
    fn uninitialized() -> Self {
        TermDraw {
            stdout: std::io::stdout(),
            width: 0,
            height: 0,
        }
    }

    pub fn new() -> Self {
        let mut data = TermDraw::uninitialized();
        data.recallibrate();
        data
    }

    pub fn recallibrate(&mut self) {
        let (cols, rows) = crossterm::terminal::size().unwrap();
        self.width = cols.into();
        let rows: u32 = rows.into();
        self.height = rows * 2;
    }

    fn dbg(&self) {
        println!("width={} height={}", self.width, self.height);
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

    drawer.dbg();
}
