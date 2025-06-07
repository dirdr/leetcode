use std::collections::BinaryHeap;

#[derive(Eq, PartialEq, Debug, Clone)]
struct Node {
    ch: char,
    pos: usize,
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other
            .ch
            .cmp(&self.ch)
            .then_with(|| self.pos.cmp(&other.pos))
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Solution {
    pub fn clear_stars(s: String) -> String {
        let chars: Vec<char> = s.chars().collect();
        let len = chars.len();

        let mut banned = vec![false; len];
        let mut heap = BinaryHeap::<Node>::with_capacity(len);

        for (pos, &ch) in chars.iter().enumerate() {
            if ch == '*' {
                if let Some(node) = heap.pop() {
                    banned[node.pos] = true;
                    banned[pos] = true;
                }
            } else {
                heap.push(Node { ch, pos });
            }
        }

        let mut result = String::with_capacity(len);
        for (i, &ch) in chars.iter().enumerate() {
            if !banned[i] {
                result.push(ch);
            }
        }

        result
    }
}
