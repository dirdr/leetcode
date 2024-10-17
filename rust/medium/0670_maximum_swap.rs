use std::{cmp::Ordering, collections::BinaryHeap};

#[derive(Debug, Eq, PartialEq)]
struct Node {
    idx: usize,
    val: i32,
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if let Ordering::Equal = self.val.cmp(&other.val) {
            return other.idx.cmp(&self.idx);
        }
        self.val.cmp(&other.val)
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(&other))
    }
}

impl Solution {
    pub fn maximum_swap(num: i32) -> i32 {
        let digit_cnt = (num as f64).log10() as usize + 1;
        if digit_cnt < 2 {
            return num;
        }
        let mut digits = Vec::with_capacity(digit_cnt);
        for nth in 0..digit_cnt {
            let digit = (num / 10i32.pow(nth as u32)) % 10;
            digits.push(digit);
        }
        digits.reverse();
        let mut heap: BinaryHeap<Node> = BinaryHeap::new();
        for (i, digit) in digits.iter().enumerate() {
            heap.push(Node {
                idx: i,
                val: *digit,
            });
        }
        let mut i = 0;
        loop {
            if heap.is_empty() {
                return num;
            }
            let Node { idx, val } = heap.peek().unwrap();
            if digits[i] == *val {
                heap.pop();
                i += 1;
            } else {
                break;
            }
        }
        let mut top = heap.pop().unwrap();
        loop {
            let see = heap.peek().unwrap();
            if top.val == see.val {
                top = heap.pop().unwrap();
            } else {
                break;
            }
        }
        digits.swap(i, top.idx);
        digits.iter().fold(0, |acc, &digit| acc * 10 + digit)
    }
}
