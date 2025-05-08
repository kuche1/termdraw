use crossterm::{
    // cargo add crossterm
    ExecutableCommand,
    style::{Print, ResetColor, SetBackgroundColor, SetForegroundColor},
};

const PIXEL: &str = "▀";

pub type Col = (u8, u8, u8);
type Density = u8;
type Pixel = (Col, Density);
type Pos = (f32, f32);
type PosRaw = (usize, usize);

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
        // TODO8 I think there is a 2nd buffer in the terminal that we can use, instead of printing char by char

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

    fn pixel_set(&mut self, pos: PosRaw, col: Col) {
        // there are 2 way to do this
        // 1 - calculate color on each pixel set (current)
        // 2 - accumilate r,g,b and calculate color on draw

        let (x, y) = pos;

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

    fn scale(&self, pos: Pos) -> PosRaw {
        let x_max: f32 = (self.buf[0].len() - 1) as f32;
        let y_max: f32 = (self.buf.len() - 1) as f32;

        let x = x_max * pos.0;
        let y = y_max * pos.1;

        (x as usize, y as usize)
    }

    pub fn dot(&mut self, pos: Pos, col: Col) {
        self.pixel_set(self.scale(pos), col);
    }

    // fn line_basic_x(&mut self, x_start: usize, x_end: usize, y: usize, col: Col) {
    //     let (x_start, x_end) = {
    //         if x_start <= x_end {
    //             (x_start, x_end)
    //         } else {
    //             (x_end, x_start)
    //         }
    //     };

    //     for x in x_start..=x_end {
    //         println!("x={x} y={y}");
    //         self.pixel_set(x, y, col);
    //     }
    // }

    // // fn line_basic_y(&mut self, x: usize, y_start: usize, y_end: usize, col: Col) {
    // //     for y in y_start..=y_end {
    // //         self.pixel_set(x, y, col);
    // //     }
    // // }

    // pub fn line(&mut self, x_start: f32, x_end: f32, y_start: f32, y_end: f32, col: Col) {
    //     let x_max: f32 = (self.buf[0].len() - 1) as f32;
    //     let y_max: f32 = (self.buf.len() - 1) as f32;

    //     let x_start = x_start * x_max;
    //     let y_start = y_start * y_max;

    //     let x_end = x_end * x_max;
    //     let y_end = y_end * y_max;

    //     if (x_start <= x_end) && (y_start <= y_end) {
    //         let x_len = x_end - x_start;
    //         let y_len = y_end - y_start;

    //         let x_step = x_len / y_len;

    //         let mut x: f32 = x_start;

    //         for y in (y_start as usize)..=(y_end as usize) {
    //             // not 100% sure if this is correct, but it does seem to work
    //             let end = x + x_step;
    //             self.line_basic_x(x as usize, end as usize - 1, y, col);
    //             x = end;
    //         }
    //     } else {
    //         todo!();
    //     }
    // }
}

fn main() {
    let mut canv = TermDraw::new();

    // canv.line(0.2, 0.8, 0.2, 0.4, (255, 0, 0));
    // canv.line(0.4, 0.8, 0.2, 0.4, (0, 255, 0));
    // canv.line(0.0, 1.0, 0.0, 1.0, (0, 0, 255));
    // canv.draw();

    canv.dot((0.5, 0.5), (255, 0, 0));
    canv.draw();
}
