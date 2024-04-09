mpl Solution {
    pub fn average_value(nums: Vec<i32>) -> i32 {
        let (mut count, mut total) = (0, 0);
        nums.iter().for_each(|el| {
            if *el % 3 == 0 && *el % 2 == 0 {
                 count += 1;
                 total += el;
            };
        });
        if (count == 0) {return 0};
        total / count
    }
}
