/*
    heap
    This question requires you to implement a binary heap function
*/
use std::cmp::Ord;
use std::default::Default;
// use std::fmt::Debug;
use std::process::id;

pub struct Heap<T>
where
    T: Default,
{
    count: usize,
    items: Vec<T>,
    comparator: fn(&T, &T) -> bool,
}

const HEAD_INDEX: usize = 1;

impl<T> Heap<T>
where
    T: Default,
{
    pub fn new(comparator: fn(&T, &T) -> bool) -> Self {
        Self {
            count: 0,
            items: vec![T::default()],
            comparator,
        }
    }

    pub fn len(&self) -> usize {
        self.count
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn add(&mut self, value: T) {
        //TODO
        self.items.push(value);
        self.count += 1;
        self.shift_up(self.count);
    }

    fn shift_up(&mut self, idx: usize) {
        // if idx == HEAD_INDEX {
        //     return;
        // }
        let mut curr = idx;
        while curr > HEAD_INDEX { 
            let p = self.parent_idx(curr);
            if (self.comparator)(&self.items[curr], &self.items[p]) {
                self.items.swap(curr, p);
                curr = p;
                // self.shift_up(p);
            }else {
                break;
            }
        }

        // let p = self.parent_idx(idx);

        // if (self.comparator)(&self.items.get(idx).unwrap(), &self.items.get(p).unwrap()) {
        //     self.items.swap(idx, p);
        //     self.shift_up(p);
        // }
    }

    fn parent_idx(&self, idx: usize) -> usize {
        idx / 2
    }

    // fn children_present(&self, idx: usize) -> bool {
    //     self.left_child_idx(idx) <= self.count
    // }

    fn left_child_idx(&self, idx: usize) -> usize {
        idx * 2
    }

    fn right_child_idx(&self, idx: usize) -> usize {
        self.left_child_idx(idx) + 1
    }

    fn smallest_child_idx(&self, idx: usize) -> usize {
        let lc_idx = self.left_child_idx(idx);
        if lc_idx > self.count {
            return 0;
        }
        let rc_idx = lc_idx + 1;
        if rc_idx > self.count || (self.comparator)(&self.items[lc_idx], &self.items[rc_idx]) {
            return lc_idx;
        }
        rc_idx
    }

    fn shift_down(&mut self, idx: usize) {

        let mut pidx = idx;

        loop {
            let sc_idx = self.smallest_child_idx(pidx);
            if sc_idx > 0 && (self.comparator)(&self.items[sc_idx], &self.items[pidx]) {
                self.items.swap(sc_idx, idx);
                pidx = sc_idx;
            }
            break;
        }
        
        // while sc_idx > 0 && (self.comparator)(&self.items[sc_idx], &self.items[pidx]) {
        //     self.items.swap(sc_idx, idx);
        //     pidx = sc_idx;
        //     sc_idx = self.smallest_child_idx(pidx);
        // }
    }
}

impl<T> Heap<T>
where
    T: Default + Ord,
{
    /// Create a new MinHeap
    pub fn new_min() -> Self {
        Self::new(|a, b| a < b)
    }

    /// Create a new MaxHeap
    pub fn new_max() -> Self {
        Self::new(|a, b| a > b)
    }
}

impl<T> Iterator for Heap<T>
where
    T: Default,
{
    type Item = T;

    fn next(&mut self) -> Option<T> {
        if self.is_empty() {
            return None;
        }

        self.items.swap(HEAD_INDEX, self.count);
        let old_head = self.items.pop();
        self.count -= 1;

        //至少2个，才有意义
        if self.count > 1 {
            self.shift_down(HEAD_INDEX);
        }

        return old_head;
    }
}

pub struct MinHeap;

impl MinHeap {
    #[allow(clippy::new_ret_no_self)]
    pub fn new<T>() -> Heap<T>
    where
        T: Default + Ord,
    {
        Heap::new(|a, b| a < b)
    }
}

pub struct MaxHeap;

impl MaxHeap {
    #[allow(clippy::new_ret_no_self)]
    pub fn new<T>() -> Heap<T>
    where
        T: Default + Ord,
    {
        Heap::new(|a, b| a > b)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_empty_heap() {
        let mut heap = MaxHeap::new::<i32>();
        assert_eq!(heap.next(), None);
    }

    #[test]
    fn test_min_heap() {
        let mut heap = MinHeap::new();
        heap.add(4);
        heap.add(2);
        heap.add(9);
        heap.add(11);
        assert_eq!(heap.len(), 4);
        assert_eq!(heap.next(), Some(2));
        assert_eq!(heap.next(), Some(4));
        assert_eq!(heap.next(), Some(9));
        heap.add(1);
        assert_eq!(heap.next(), Some(1));
    }

    #[test]
    fn test_max_heap() {
        let mut heap = MaxHeap::new();
        heap.add(4);
        heap.add(2);
        heap.add(9);
        heap.add(11);
        assert_eq!(heap.len(), 4);
        assert_eq!(heap.next(), Some(11));
        assert_eq!(heap.next(), Some(9));
        assert_eq!(heap.next(), Some(4));
        heap.add(1);
        assert_eq!(heap.next(), Some(2));
    }
}
