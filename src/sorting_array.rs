use super::*;
use std::{
    cmp::Ordering,
    sync::{Arc, Mutex},
};

/// Each kind of sorting operation.
#[derive(Clone, Copy, Debug)]
pub enum Operation {
    Init,
    Write { idx: usize },
    Read { idx: usize },
    Swap { a: usize, b: usize },
    Compare { a: usize, b: usize, res: bool },
}

/// The sortable array.
#[derive(Debug)]
pub struct SortArray {
    /// The array of data.
    arr: Vec<usize>,

    /// The most recent sorting operation.
    last_op: Operation,
    /// A buffer of previous sorting operations.
    op_buffer: Vec<Operation>,
}

impl SortArray {
    /// Creates a new sorting array with `len` elements.
    pub fn with_size(len: usize) -> Self {
        Self {
            arr: (0..len).collect(),
            op_buffer: Vec::with_capacity(100),
            last_op: Operation::Init,
        }
    }

    /// Writes `value` to position `idx`. Will panic if `idx > `[`SortArray::len()`].
    pub fn write(&mut self, idx: usize, value: usize) {
        self.add_op(Operation::Write { idx });
        self.arr[idx] = value;
    }

    /// Returns the value as position `idx`. Will panic if `idx > `[`SortArray::len()`].
    pub fn read(&mut self, idx: usize) -> usize {
        self.add_op(Operation::Read { idx });
        self.arr[idx]
    }

    /// Swaps the elements at positions `a` and `b`. Will panic if either index is
    /// greater than [`SortArray::len()`].
    pub fn swap(&mut self, a: usize, b: usize) {
        self.add_op(Operation::Swap { a, b });
        self.arr.swap(a, b);
    }

    /// Compares the elements at positions `a` and `b` to match `ord`. Will panic if
    /// either index is greater than [`SortArray::len()`].
    pub fn cmp(&mut self, a: usize, b: usize, ord: Ordering) -> bool {
        let cmp = self.arr[a].cmp(&self.arr[b]);
        let res = matches!(cmp, ord);

        self.add_op(Operation::Compare { a, b, res });

        res
    }

    /// Returns the last sorting operation, if it exists.
    pub const fn last_operation(&self) -> Operation {
        self.last_op
    }

    /// Returns a slice of previous operations.
    pub fn op_buffer(&self) -> &[Operation] {
        &self.op_buffer
    }

    /// Clears the operation buffer.
    pub fn clear_op_buffer(&mut self) {
        if let Some(op) = self.op_buffer().last().copied() {
            self.last_op = op;
        }

        self.op_buffer.clear();
    }

    /// Resizes the sorting array.
    pub fn resize(&mut self, new_size: usize) {
        self.arr.resize(new_size, 0);
        self.arr.iter_mut().enumerate().for_each(|(i, x)| *x = i);
    }

    /// Forces the array to be sorted with `std::sort_unstable()`.
    pub fn force_sort(&mut self) {
        self.arr.sort_unstable();
    }

    /// The length of the sorting array.
    pub fn len(&self) -> usize {
        self.arr.len()
    }

    /// Returns the sorting array as a slice.
    pub fn as_slice(&self) -> &[usize] {
        &self.arr
    }

    fn add_op(&mut self, op: Operation) {
        self.last_op = op;
        self.op_buffer.push(op);
    }
}
