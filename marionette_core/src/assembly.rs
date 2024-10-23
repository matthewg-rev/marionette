// Purpose: defines *code* regions for general purpose disassemblies.
// src\structure\region.rs

pub struct Range {
    start: u64,
    end: u64
}

impl Range {
    pub fn new(start: u64, end: u64) -> Range {
        Range { start, end }
    }

    pub fn size(&self) -> u64 {
        self.end - self.start
    }
}

pub trait Entropy {
    fn entropy(&self) -> f64;
}

pub trait Data {
    fn range(&self) -> Range;

    fn raw(&self) -> &[u8];
    fn text(&self) -> &str;
}

pub trait Region {
    fn range(&self) -> Range;
    fn contains(&self, addr: u64) -> bool;

    fn data(&self) -> &[Box<dyn Data>];
    fn raw(&self) -> &[u8];
    fn text(&self) -> &str;
}

pub trait Assembly {
    fn regions(&self) -> &[Box<dyn Region>];
    fn raw(&self) -> &[u8];
    fn text(&self) -> &str;
}