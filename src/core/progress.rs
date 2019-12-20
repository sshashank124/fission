use std::io::{stdout, Write};
use std::time::Instant;

use super::*;

pub trait ProgressTracker {
    fn new(n: I) -> Self;
    fn update(&mut self) -> String;
    fn finish(&mut self) -> String;
}

#[must_use]
pub struct Progress<'a> {
    msg: &'a str,
    t:   Instant,
    p:   ProgressType,
}

impl<'a> Progress<'a> {
    pub fn new(msg: &'a str, n: Option<I>) -> Self {
        let mut p = Self { msg,
                           t: Instant::now(),
                           p: ProgressType::new(n.unwrap_or(1)) };
        p.update();
        p
    }

    pub fn update(&mut self) {
        print!("\r{} ... [{}]", self.msg, self.p.update());
        stdout().flush().unwrap();
    }
}

impl Drop for Progress<'_> {
    fn drop(&mut self) {
        println!("\r{} ... DONE ({:.2?}) {}",
                 self.msg,
                 self.t.elapsed(),
                 self.p.finish());
    }
}

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
    fn new(n: I) -> Self { Self { i: 0, w: 50, scale: 50. / n as F } }

    fn update(&mut self) -> String {
        let pf = self.i as F * self.scale;
        self.i += 1;
        let p = F::floor(pf) as usize;
        let dp = F::floor(pf.fract() * SUB_BOX.len() as F) as usize;
        format!("{}{:<cw$}", "█".repeat(p), SUB_BOX[dp], cw = self.w - p)
    }

    fn finish(&mut self) -> String { " ".repeat(self.w) }
}

pub struct CounterProgress {
    i: I,
    n: I,
    w: usize,
}

impl ProgressTracker for CounterProgress {
    fn new(n: I) -> Self {
        Self { i: 0, n, w: F::log10(n as F).ceili() as usize }
    }

    fn update(&mut self) -> String {
        self.i += 1;
        format!("{:width$}/{:width$}", self.i - 1, self.n, width = self.w)
    }

    fn finish(&mut self) -> String { " ".repeat(self.w) }
}

impl ProgressTracker for ProgressType {
    fn new(n: I) -> Self { BarProgress::new(n).into() }

    fn update(&mut self) -> String {
        match self {
            Self::Bar(p) => p.update(),
            Self::Counter(p) => p.update(),
        }
    }

    fn finish(&mut self) -> String {
        match self {
            Self::Bar(p) => p.finish(),
            Self::Counter(p) => p.finish(),
        }
    }
}

impl From<BarProgress> for ProgressType {
    fn from(p: BarProgress) -> Self { Self::Bar(p) }
}

impl From<CounterProgress> for ProgressType {
    fn from(p: CounterProgress) -> Self { Self::Counter(p) }
}
