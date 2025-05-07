use crossterm::{
    // cargo add crossterm
    ExecutableCommand,
    style::{Color, Print, ResetColor, SetBackgroundColor, SetForegroundColor},
};

const PIXEL: &str = "▀";

pub type Col = (u8, u8, u8);
type Density = u8;
type Pixel = (Col, Density);

pub struct TermDraw {
    stdout: std::io::Stdout,
    width: u32,
    height: u32,
    buf: Vec<Vec<Pixel>>,
}

impl TermDraw {
    fn uninitialized() -> Self {
        TermDraw {
            stdout: std::io::stdout(),
            width: 0,
            height: 0,
            buf: vec![vec![]],
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

        self.buf.clear();

        for _y in 0..self.height {
            self.buf
                .push(vec![((0, 0, 0), 0); self.width.try_into().unwrap()]);
        }
    }

    fn print_pixel(stdout: &mut std::io::Stdout, color_top: Col, color_bot: Col) {
        let (tr, tg, tb) = color_top;
        let (br, bg, bb) = color_bot;

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

    pub fn clear(&mut self) {
        for line in self.buf.iter_mut() {
            for pixel in line {
                *pixel = ((0, 0, 0), 0);
            }
        }
    }

    pub fn draw(&mut self) {
        for pair in self.buf.chunks(2) {
            let [line0, line1] = pair else {
                unreachable!();
            };

            for long_pixel in line0.into_iter().zip(line1) {
                let (pix0, pix1) = long_pixel;
                let (pix0, _) = *pix0;
                let (pix1, _) = *pix1;
                TermDraw::print_pixel(&mut self.stdout, pix0, pix1);
            }
        }
    }

    fn pixel_set(&mut self, x: usize, y: usize, col: Col) {
        self.buf[y][x] = (col, 0);
    }

    fn line0(&mut self) {
        for pos in 0..=10 {
            self.pixel_set(pos, pos, (255, 0, 0));
        }
    }

    fn line1(&mut self) {
        for pos in 10..=20 {
            self.pixel_set(pos, pos, (0, 255, 0));
        }
    }
}

fn main() {
    let mut canv = TermDraw::new();

    canv.line0();
    canv.draw();
    canv.clear();
    canv.line1();
    canv.draw();
    canv.clear();

    canv.line0();
    canv.line1();
    canv.draw();
}
