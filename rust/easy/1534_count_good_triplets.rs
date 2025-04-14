impl Solution {
    pub fn count_good_triplets(arr: Vec<i32>, a: i32, b: i32, c: i32) -> i32 {
        let n = arr.len();
        let mut answer = 0;
        for i in 0..n - 2 {
            for j in i + 1..n - 1 {
                if (arr[j] - arr[i]).abs() > a {
                    continue
                }
                for k in j + 1..n {
                    if (arr[k] - arr[j]).abs() > b {
                        continue;
                    }
                    if (arr[i] - arr[k]).abs() > c {
                        continue;
                    }
                    answer += 1;
                }
            }
        }
        return answer
    }
}
