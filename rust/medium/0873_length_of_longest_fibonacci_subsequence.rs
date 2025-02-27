use std::collections::HashSet;

impl Solution {
    pub fn len_longest_fib_subseq(arr: Vec<i32>) -> i32 {
        let set = HashSet::<i32>::from_iter(arr.iter().cloned());
        let mut max = 0;
        for i in 0..arr.len() {
            for j in i + 1..arr.len() {
                let mut prev = arr[j];
                let mut current = prev + arr[i];
                let mut size = 2;
                while set.contains(&current) {
                    let temp = prev;
                    prev = current;
                    current = current + temp;
                    size += 1;
                }
                max = max.max(size);
            }
        }
        if max >= 3 { max } else { 0 }
    }
}
