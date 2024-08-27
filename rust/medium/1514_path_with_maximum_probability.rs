use std::cmp::Ordering;
use std::collections::BinaryHeap;

#[derive(PartialEq)]
struct NonNan(f64);

impl PartialOrd for NonNan {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.0.partial_cmp(&other.0)
    }
}

impl Ord for NonNan {
    fn cmp(&self, other: &NonNan) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}
impl Eq for NonNan {}

impl Solution {
    pub fn max_probability(
        n: i32,
        edges: Vec<Vec<i32>>,
        succ_prob: Vec<f64>,
        start: i32,
        end: i32,
    ) -> f64 {
        let mut graph: Vec<Vec<(i32, f64)>> = vec![vec![]; n as usize];
        let mut queue: BinaryHeap<(NonNan, i32)> = BinaryHeap::new();
        for i in 0..edges.len() {
            graph[edges[i][0] as usize].push((edges[i][1], succ_prob[i]));
            graph[edges[i][1] as usize].push((edges[i][0], succ_prob[i]));
        }
        let mut max_res = vec![0.0; n as usize];
        max_res[start as usize] = 1.0;
        queue.push((NonNan(1.0), start));
        while let Some((NonNan(prob), cur)) = queue.pop() {
            if cur == end {
                return prob;
            }
            for adj in &graph[cur as usize] {
                if prob * adj.1 > max_res[adj.0 as usize] {
                    max_res[adj.0 as usize] = prob * adj.1;
                    queue.push((NonNan(max_res[adj.0 as usize]), adj.0));
                }
            }
        }
        return 0.0;
    }
}
