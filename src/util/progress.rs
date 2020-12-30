use std::io::{stdout, Write};
use std::time::Instant;

use super::*;

pub trait ProgressTracker {
    fn new(i: I, n: I) -> Self;
    fn update(&mut self) -> String;
    fn finish(&self) -> String;
}

#[must_use]
pub struct Progress<'a> {
    msg:  &'a str,
    t:    Instant,
    p:    ProgressType,
    done: bool,
}

impl<'a> Progress<'a> {
    pub fn new(msg: &'a str, i: I, n: I) -> Self {
        let mut p = Self { msg, t: Instant::now(), p: ProgressType::new(i, n),
                           done: false };
        p.update();
        p
    }

    pub fn indeterminate(msg: &'a str) -> Self { Self::new(msg, 0, 1) }

    pub fn update(&mut self) {
        print!("\r{} ... [{}]", self.msg, self.p.update());
        stdout().flush().unwrap();
    }

    fn print_result(&self, result: &str) {
        println!("\r{} ... {} ({:.2?}) {}",
                 self.msg, result, self.t.elapsed(), self.p.finish());
    }

    pub fn cancel(mut self)
    { self.print_result("CANCELLED"); self.done = true; }
}

impl Drop for Progress<'_>
{ fn drop(&mut self) { if !self.done { self.print_result("DONE"); } } }

pub enum ProgressType {
    Bar(BarProgress),
    Counter(CounterProgress),
}

pub struct BarProgress {
    i:     I,
    w:     usize,
    scale: F,
}
const SUB_BOX: [char; 8] = [' ', '▏', '▎', '▍', '▌', '▋', '▊', '▉'];

impl ProgressTracker for BarProgress {
    fn new(i: I, n: I) -> Self { Self { i, w: 50, scale: 50. / n as F } }

    fn update(&mut self) -> String {
        let pf = self.i as F * self.scale;
        self.i += 1;
        let p = F::floor(pf) as usize;
        let dp = F::floor(pf.fract() * SUB_BOX.len() as F) as usize;
        format!("{}{:<cw$}", "█".repeat(p), SUB_BOX[dp], cw = self.w - p)
    }

    fn finish(&self) -> String { " ".repeat(self.w) }
}

pub struct CounterProgress {
    i: I,
    n: I,
    w: usize,
}

impl ProgressTracker for CounterProgress {
    fn new(i: I, n: I) -> Self
    { Self { i, n, w: F::log10(n as F).ceili() as usize } }

    fn update(&mut self) -> String {
        self.i += 1;
        format!("{:width$}/{:width$}", self.i - 1, self.n, width = self.w)
    }

    fn finish(&self) -> String { " ".repeat(self.w) }
}

impl ProgressTracker for ProgressType {
    fn new(i: I, n: I) -> Self { BarProgress::new(i, n).into() }

    fn update(&mut self) -> String {
        match self {
            Self::Bar(p) => p.update(),
            Self::Counter(p) => p.update(),
        }
    }

    fn finish(&self) -> String {
        match self {
            Self::Bar(p) => p.finish(),
            Self::Counter(p) => p.finish(),
        }
    }
}

impl From<BarProgress> for ProgressType
{ fn from(p: BarProgress) -> Self { Self::Bar(p) } }

impl From<CounterProgress> for ProgressType
{ fn from(p: CounterProgress) -> Self { Self::Counter(p) } }
