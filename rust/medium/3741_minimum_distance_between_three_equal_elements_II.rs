impl Solution {
    pub fn minimum_distance(mut nums: Vec<i32>) -> i32 {
        if nums.len() < 3 {
            return -1;
        }
        let mut tupled = vec![(0, 0); nums.len()];
        for i in 0..nums.len() {
            tupled[i] = (nums[i], i);
        }
        let mut min = usize::MAX;
        tupled.sort_unstable();
        for i in 0..tupled.len() - 2 {
            if tupled[i].0 == tupled[i + 1].0 && tupled[i].0 == tupled[i + 2].0 && tupled[i + 1].0 == tupled[i + 2].0 {
                let fd = (tupled[i].1 as isize - tupled[i + 1].1 as isize).abs() as usize;
                let sd = (tupled[i + 1].1 as isize - tupled[i + 2].1 as isize).abs() as usize;
                let td = (tupled[i + 2].1 as isize - tupled[i].1  as isize).abs() as usize;
                min = min.min(fd + sd + td);
            }
        }
        min as i32
    }
}
