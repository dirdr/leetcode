impl Solution {
    pub fn minimum_effort(mut tasks: Vec<Vec<i32>>) -> i32 {
        let mut total = 0;
        let mut curr = 0;
        // here, we define the efficiency of a task to be to 'most reward', eg the less consumed but the more required.
        // e = minimum - actual.
        // we sort e descending. which greedly make us optimize the amount of energy spent.
        tasks.sort_unstable_by(|a, b| (b[1] - b[0]).cmp(&(a[1] - a[0])).then(a[0].cmp(&b[0])));
        for task in &tasks {
            let delta = task[1] - curr;
            if curr < task[1] {
                total += delta;
                curr += delta;
            }
            curr -= task[0];
        }
        total
    }
}
