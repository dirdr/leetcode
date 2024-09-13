impl Solution {
    pub fn xor_queries(arr: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let mut answer = Vec::with_capacity(queries.len());
        for query in queries.iter() {
            answer.push(
                (arr[query[0] as usize..=query[1] as usize])
                    .iter()
                    .fold(0, |acc, &x| acc ^ x),
            );
        }
        answer
    }
}
