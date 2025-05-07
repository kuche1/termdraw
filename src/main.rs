use crossterm::{
    // cargo add crossterm
    ExecutableCommand,
    style::{Print, ResetColor, SetBackgroundColor, SetForegroundColor},
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

        // TODO1 crash if width or height is 0 (easier array access)

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
            .execute(SetForegroundColor(crossterm::style::Color::Rgb {
                r: tr,
                g: tg,
                b: tb,
            }))
            .unwrap();

        stdout
            .execute(SetBackgroundColor(crossterm::style::Color::Rgb {
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
        // there are 2 way to do this
        // 1 - calculate color on each pixel set (current)
        // 2 - accumilate r,g,b and calculate color on draw

        let (cr, cg, cb) = col;
        let cr: u16 = cr.into();
        let cg: u16 = cg.into();
        let cb: u16 = cb.into();

        let (old_col, old_dens) = self.buf[y][x];
        let (or, og, ob) = old_col;
        let or: u16 = or.into();
        let og: u16 = og.into();
        let ob: u16 = ob.into();

        let old_dens: u16 = old_dens.into();
        let new_dens = old_dens + 1;

        let nr = (or * old_dens + cr) / new_dens;
        let ng = (og * old_dens + cg) / new_dens;
        let nb = (ob * old_dens + cb) / new_dens;

        let nr: u8 = nr.try_into().unwrap();
        let ng: u8 = ng.try_into().unwrap();
        let nb: u8 = nb.try_into().unwrap();
        let new_dens: u8 = new_dens.try_into().unwrap();

        self.buf[y][x] = ((nr, ng, nb), new_dens);
    }

    fn line0(&mut self) {
        // TODO1 delete
        for pos in 0..=10 {
            self.pixel_set(pos, pos, (255, 0, 0));
        }
    }

    fn line1(&mut self) {
        // TODO1 delete
        for pos in 10..=20 {
            self.pixel_set(pos, pos, (0, 255, 0));
        }
    }

    fn line2(&mut self, col: Col) {
        let h: f32 = 1.0;
        let w: f32 = 0.8;

        let y_start: usize = 0;
        let y_end: f32 = ((self.buf.len() - 1) as f32) * h;
        let y_end: usize = y_end as usize;
        let y_len = y_end - y_start; // TODO0 and what if bad pos?

        let x_start: usize = 0;
        let x_end: f32 = ((self.buf[0].len() - 1) as f32) * w;
        let x_end: usize = x_end as usize;
        let x_len = x_end - x_start; // TODO0 and what if bad pos?

        for x in 0..=x_end {
            self.pixel_set(x, 5, col);
        }
    }
}

fn main() {
    let mut canv = TermDraw::new();

    // canv.line0();
    // canv.draw();
    // canv.clear();
    // canv.line1();
    // canv.draw();
    // canv.clear();

    // canv.line0();
    // canv.line1();
    // canv.draw();
    // canv.clear();

    canv.line2((0, 0, 255));
    canv.line2((255, 0, 0));
    canv.draw();
}
