impl Solution {
    pub fn divide_players(skill: Vec<i32>) -> i64 {
        let mut skill = skill;
        skill.sort_unstable();
        let n = skill.len();
        let target = skill.first().unwrap() + skill.last().unwrap();
        let chemistry = skill
            .iter()
            .zip(skill.iter().rev())
            .take((n + 1) / 2)
            .try_fold(0i64, |acc, (&a, &b)| {
                if a + b != target {
                    None
                } else {
                    Some(acc + (a as i64) * (b as i64))
                }
            });

        chemistry.unwrap_or(-1)
    }
}
