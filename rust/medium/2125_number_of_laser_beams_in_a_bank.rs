impl Solution {
    pub fn number_of_beams(bank: Vec<String>) -> i32 {
        let mut prev = -1;
        bank.iter().fold(0, |acc, row| {
            let num = row.chars().filter(|&c| c == '1').count() as i32;
            let new = if num != 0 && prev != -1 {
                acc + (prev * num) as i32
            } else {
                acc
            };
            if num != 0 {
                prev = num;
            }
            new
        })
    }
}
