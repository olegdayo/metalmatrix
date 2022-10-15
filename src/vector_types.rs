use std::fmt::{Debug, Formatter};
use std::ops::Index;

pub trait Vector: Index<usize, Output=f64> {
    fn len(&self) -> usize;
}

pub(crate) struct FullVector {
    pub(crate) row: Vec<f64>,
    pub(crate) index: usize,
    pub(crate) len: usize,
}

pub(crate) struct CompressedVector {
    pub(crate) val: f64,
    pub(crate) index: usize,
    pub(crate) len: usize,
}

impl Vector for FullVector {
    fn len(&self) -> usize {
        self.len
    }
}

impl Vector for CompressedVector {
    fn len(&self) -> usize {
        self.len
    }
}

impl Debug for FullVector {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl Debug for CompressedVector {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}


impl PartialEq for FullVector {
    fn eq(&self, other: &Self) -> bool {
        if self.index != other.index {
            return false;
        }

        if self.row.len() != other.row.len() {
            return false;
        }

        for i in 0..self.row.len() {
            if self.row[i] != other.row[i] {
                return false;
            }
        }

        true
    }
}

impl PartialEq for CompressedVector {
    fn eq(&self, other: &Self) -> bool {
        self.index == other.index && self.val == other.val
    }
}
