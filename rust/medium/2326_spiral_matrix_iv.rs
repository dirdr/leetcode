// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//   pub val: i32,
//   pub next: Option<Box<ListNode>>
// }
// 
// impl ListNode {
//   #[inline]
//   fn new(val: i32) -> Self {
//     ListNode {
//       next: None,
//       val
//     }
//   }
// }
use std::collections::HashSet;

impl Solution {
    pub fn spiral_matrix(m: i32, n: i32, head: Option<Box<ListNode>>) -> Vec<Vec<i32>> {
        let directions = vec![(0, 1), (1, 0), (0, -1), (-1, 0)];
        let mut curr_dir = 0;
        let (mut r, mut c) = (0, 0);
        let mut answer = vec![vec![-1; n as usize]; m as usize];
        let mut curr = head.as_ref();
        let mut visited = HashSet::new();
        while let Some(node) = curr {
            if r >= 0 && r < m && c >= 0 && c < n && !visited.contains(&(r, c)) {
                answer[r as usize][c as usize] = node.val;
                visited.insert((r, c));
                curr = node.next.as_ref();
            }
            let (dr, dc) = directions[curr_dir];
            let (next_r, next_c) = (r + dr, c + dc);
            if next_r < 0 || next_r >= m || next_c < 0 || next_c >= n || visited.contains(&(next_r, next_c)) {
                curr_dir = (curr_dir + 1) % 4;
            } else {
                r = next_r;
                c = next_c;
            }
            if visited.len() == (m * n) as usize {
                break;
            }
        }
        answer
    }
}
