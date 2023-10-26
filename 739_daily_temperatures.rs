impl Solution {
    pub fn daily_temperatures(temperatures: Vec<i32>) -> Vec<i32> {
        let mut stack: Vec<(usize, i32)> = vec![];
        let mut result: Vec<i32> = vec![0; temperatures.len()];
        for (i, &el) in temperatures.iter().enumerate() {
            while !stack.is_empty() && el > stack.last().unwrap().1 {
                let stel = stack.pop().unwrap();
                result[stel.0] = (i - stel.0) as i32;
            }
            stack.push((i, el));
        }
        result
    }
}
