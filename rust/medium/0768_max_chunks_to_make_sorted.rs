impl Solution {
    pub fn max_chunks_to_sorted(arr: Vec<i32>) -> i32 {
        let mut chunks = 0;
        let (mut psum, mut pisum) = (0, 0);
        for i in 0..arr.len() {
            psum += arr[i];
            pisum += i as i32;
            if psum == pisum {
                chunks += 1;
            }
        } 
        chunks
    }
}
