use std::collections::BinaryHeap;

#[derive(Eq, PartialEq, Debug)]
struct Node {
    letter: char,
    count: i32,
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.count.cmp(&other.count)
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Solution {
    pub fn longest_diverse_string(a: i32, b: i32, c: i32) -> String {
        let mut heap = BinaryHeap::new();

        if a > 0 {
            heap.push(Node {
                letter: 'a',
                count: a,
            });
        }
        if b > 0 {
            heap.push(Node {
                letter: 'b',
                count: b,
            });
        }
        if c > 0 {
            heap.push(Node {
                letter: 'c',
                count: c,
            });
        }

        let mut result = Vec::new();
        while let Some(mut node) = heap.pop() {
            if result.len() >= 2
                && result[result.len() - 1] == node.letter
                && result[result.len() - 2] == node.letter
            {
                if let Some(mut next_node) = heap.pop() {
                    result.push(next_node.letter);
                    next_node.count -= 1;
                    if next_node.count > 0 {
                        heap.push(next_node);
                    }
                    heap.push(node);
                } else {
                    break;
                }
            } else {
                result.push(node.letter);
                node.count -= 1;
                if node.count > 0 {
                    heap.push(node);
                }
            }
        }
        result.iter().collect()
    }
}
