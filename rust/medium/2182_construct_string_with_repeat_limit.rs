use std::collections::BinaryHeap;

impl Solution {
    pub fn repeat_limited_string(s: String, repeat_limit: i32) -> String {
        let mut heap = BinaryHeap::new();
        let mut freqs = vec![0; 26];
        for c in s.chars() {
            freqs[(c as u8 - b'a') as usize] += 1;
        }
        for (i, &f) in freqs.iter().enumerate() {
            if f != 0 {
                heap.push(((i as u8 + b'a') as char, f));
            }
        }
        let mut answer = vec![];
        while let Some((ch, freq)) = heap.pop() {
            if answer.last() == Some(&ch) {
                if let Some((sch, sfreq)) = heap.pop() {
                    answer.push(sch);
                    if sfreq > 1 {
                        heap.push((sch, sfreq - 1))
                    }
                    heap.push((ch, freq))
                } else {
                    break;
                }
            } else {
                let m = freq.min(repeat_limit);
                answer.extend(std::iter::repeat(ch).take(m as usize));
                if freq > m {
                    heap.push((ch, freq - m))
                }
            }
        }
        answer.into_iter().collect::<String>()
    }
}
