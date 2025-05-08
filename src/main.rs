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
        let rows = rows.checked_add_signed(-1).unwrap(); // leave 1 like free for the new line
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

        println!("dbg: x={x} y={y}");

        self.buf[y][x] = ((nr, ng, nb), new_dens);
    }

    fn scale(&self, pos: Pos) -> PosRaw {
        let x_max: f32 = (self.width - 1) as f32;
        let y_max: f32 = (self.height - 1) as f32;

        let x = x_max * pos.0;
        let y = y_max * pos.1;

        (x as usize, y as usize)
    }

    pub fn dot(&mut self, pos: Pos, col: Col) {
        self.pixel_set(self.scale(pos), col);
    }

    pub fn line(&mut self, start: Pos, end: Pos, col: Col) {
        let (start_x, start_y) = self.scale(start);
        let (end_x, end_y) = self.scale(end);

        let x_len = start_x.abs_diff(end_x);
        let y_len = start_y.abs_diff(end_y);

        // this vvv sucks major balls, either use generators with ssize or do something about _len

        if (start_x <= end_x) && (start_y <= end_y) {
            if x_len <= y_len {
                // I hope this also works for when `x_len == y_len`
                let loop_end = end_y - start_y;
                for y_ofs in 0..=loop_end {
                    let x = start_x + x_len * y_ofs / loop_end;
                    let y = start_y + y_ofs;
                    self.pixel_set((x, y), col);
                }
            } else {
                let loop_end = end_x - start_x;
                for x_ofs in 0..=loop_end {
                    let x = start_x + x_ofs;
                    let y = start_y + y_len * x_ofs / loop_end;
                    self.pixel_set((x, y), col);
                }
            }
        } else if (start_x <= end_x) && (start_y > end_y) {
            if x_len <= y_len {
                // I hope this also works for when `x_len == y_len`
                let loop_end = start_y - end_y;
                for y_ofs in 0..=loop_end {
                    let x = end_x - x_len * y_ofs / loop_end; // TODO I want + here
                    let y = end_y + y_ofs;
                    self.pixel_set((x, y), col);
                }
            } else {
                let loop_end = end_x - start_x;
                for x_ofs in 0..=loop_end {
                    let x = start_x + x_ofs;
                    let y = start_y - y_len * x_ofs / loop_end; // TODO I want + here
                    self.pixel_set((x, y), col);
                }
            }
        } else if (start_x > end_x) && (start_y <= end_y) {
            if x_len <= y_len {
                // I hope this also works for when `x_len == y_len`
                let loop_end = end_y - start_y;
                for y_ofs in 0..=loop_end {
                    let x = start_x - x_len * y_ofs / loop_end; // TODO I want + here
                    let y = start_y + y_ofs;
                    self.pixel_set((x, y), col);
                }
            } else {
                let loop_end = start_x - end_x;
                for x_ofs in 0..=loop_end {
                    let x = start_x - x_ofs;
                    let y = start_y + y_len * x_ofs / loop_end;
                    self.pixel_set((x, y), col);
                }
            }
        } else if (start_x > end_x) && (start_y > end_y) {
            if x_len <= y_len {
                // I hope this also works for when `x_len == y_len`
                let loop_end = start_y - end_y;
                for y_ofs in 0..=loop_end {
                    let x = start_x - x_len * y_ofs / loop_end;
                    let y = start_y - y_ofs;
                    self.pixel_set((x, y), col);
                }
            } else {
                let loop_end = start_x - end_x;
                for x_ofs in 0..=loop_end {
                    let x = start_x - x_ofs;
                    let y = start_y - y_len * x_ofs / loop_end;
                    self.pixel_set((x, y), col);
                }
            }
        } else {
            //
            unreachable!();
        }
    }
}

fn main() {
    let mut canv = TermDraw::new();

    canv.dot((0.5, 0.5), (255, 0, 0));
    canv.dot((0.0, 0.5), (255, 0, 0));
    canv.dot((1.0, 0.5), (255, 0, 0));
    canv.dot((0.0, 0.0), (255, 0, 0));
    canv.dot((1.0, 1.0), (255, 0, 0));
    canv.draw();

    println!();

    canv.clear();
    canv.line((0.28, 0.1), (0.32, 0.9), (255, 0, 0));
    canv.line((0.1, 0.1), (0.6, 0.4), (0, 255, 0));
    canv.line((0.1, 0.7), (0.2, 0.4), (0, 0, 255));
    canv.line((0.2, 0.7), (0.6, 0.4), (255, 255, 0));
    canv.line((0.9, 0.4), (0.8, 0.75), (255, 0, 255));
    canv.line((0.9, 0.4), (0.45, 0.75), (0, 255, 255));
    canv.line((0.85, 0.65), (0.75, 0.28), (125, 0, 0));
    canv.line((0.85, 0.65), (0.45, 0.28), (0, 125, 0));
    canv.draw();
}
