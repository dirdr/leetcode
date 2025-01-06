impl Solution {
    pub fn min_operations(boxes: String) -> Vec<i32> {
        let boxes = boxes.chars().collect::<Vec<_>>();
        let mut answer = vec![0; boxes.len()];
        let (mut balls, mut moves) = (0, 0);
        for i in 0..boxes.len() {
            answer[i] = balls + moves;
            moves += balls;
            balls += boxes[i].to_digit(10).unwrap() as i32;
        }
        let (mut balls, mut moves) = (0, 0);
        for i in (0..boxes.len()).rev() {
            answer[i] += balls + moves;
            moves += balls;
            balls += boxes[i].to_digit(10).unwrap() as i32;
        }
        answer
    }
}
