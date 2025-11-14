impl Solution {
    pub fn range_add_queries(n: i32, queries: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let n = n as usize;
        let mut delta = vec![vec![0i32; n]; n];
        for q in queries {
            for i in (q[0] as usize)..=(q[2] as usize) {
                delta[i][q[1] as usize] += 1;
                if q[3] as usize + 1 < n {
                    delta[i][q[3] as usize + 1] -= 1;
                }
            }
        }
        let mut answer = vec![vec![0; n]; n];
        for i in 0..n {
            let mut current = 0;
            for j in 0..n {
                current += delta[i][j];
                answer[i][j] = current;
            }
        }
        answer
    }
}
