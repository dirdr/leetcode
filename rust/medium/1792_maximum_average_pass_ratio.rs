use std::cmp::Ordering;
use std::collections::BinaryHeap;

#[derive(Debug, Clone)]
struct Node {
    index: usize,
    value: f64,
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl Eq for Node {}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.value.partial_cmp(&other.value)
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        self.value.partial_cmp(&other.value).unwrap()
    }
}

impl Solution {
    pub fn max_average_ratio(classes: Vec<Vec<i32>>, extra_students: i32) -> f64 {
        let mut classes = classes;
        let mut heap: BinaryHeap<Node> = BinaryHeap::new();
        for (i, class) in classes.iter().enumerate() {
            let initial = class[0] as f64 / class[1] as f64;
            let with_student = (class[0] as f64 + 1.0) / (class[1] as f64 + 1.0);
            heap.push(Node {
                value: with_student - initial,
                index: i,
            });
        }
        for _ in 0..extra_students as usize {
            let highest = heap.pop().unwrap();
            classes[highest.index][0] += 1;
            classes[highest.index][1] += 1;
            let initial = classes[highest.index][0] as f64 / classes[highest.index][1] as f64;
            let with_student =
                (classes[highest.index][0] as f64 + 1.0) / (classes[highest.index][1] as f64 + 1.0);
            heap.push(Node {
                value: with_student - initial,
                index: highest.index,
            });
        }
        classes
            .iter()
            .map(|e| e[0] as f64 / e[1] as f64)
            .sum::<f64>()
            / classes.len() as f64
    }
}
