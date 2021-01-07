use std::io::{stdout, Write};
use std::time::Instant;

#[allow(clippy::wildcard_imports)]
use graphite::*;

pub trait Status {
    fn new(i: I, n: I) -> Self;
    fn update(&mut self) -> String;
    fn finish(&self) -> String;
}

#[must_use]
pub struct Progress<'a> {
    msg:  &'a str,
    t:    Instant,
    p:    Type,
    done: bool,
}

impl<'a> Progress<'a> {
    pub fn new(msg: &'a str, i: I, n: I) -> Self {
        let mut p = Self { msg, t: Instant::now(), p: Type::new(i, n),
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

enum Type {
    Bar(Bar),
    Counter(Counter),
}

struct Bar {
    i:     I,
    w:     usize,
    scale: F,
}
const SUB_BOX: [char; 8] = [' ', '▏', '▎', '▍', '▌', '▋', '▊', '▉'];

impl Status for Bar {
    fn new(i: I, n: I) -> Self { Self { i, w: 50, scale: 50. / F::of(n) } }

    fn update(&mut self) -> String {
        let pf = F::of(self.i) * self.scale;
        self.i += 1;
        let p = usize::of(F::floor(pf));
        let dp = usize::of(F::floor(pf.fract() * F::of(SUB_BOX.len())));
        format!("{}{:<cw$}", "█".repeat(p), SUB_BOX[dp], cw = self.w - p)
    }

    fn finish(&self) -> String { " ".repeat(self.w) }
}

struct Counter {
    i: I,
    n: I,
    w: usize,
}

impl Status for Counter {
    fn new(i: I, n: I) -> Self
    { Self { i, n, w: usize::of(F::log10(F::of(n)).ceili()) } }

    fn update(&mut self) -> String {
        self.i += 1;
        format!("{:width$}/{:width$}", self.i - 1, self.n, width = self.w)
    }

    fn finish(&self) -> String { " ".repeat(self.w) }
}

impl Status for Type {
    fn new(i: I, n: I) -> Self { Bar::new(i, n).into() }

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

impl From<Bar> for Type
{ fn from(p: Bar) -> Self { Self::Bar(p) } }

impl From<Counter> for Type
{ fn from(p: Counter) -> Self { Self::Counter(p) } }
