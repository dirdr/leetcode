impl Solution {
    pub fn largest_square_area(bottom_left: Vec<Vec<i32>>, top_right: Vec<Vec<i32>>) -> i64 {
        let n = bottom_left.len();
        let mut max = 0;
        for i in 0..n {
            let (aleft, abottom) = (bottom_left[i][0], bottom_left[i][1]);
            let (aright, atop) = (top_right[i][0], top_right[i][1]);
            for j in i + 1..n {
                let (bleft, bbottom) = (bottom_left[j][0], bottom_left[j][1]);
            let (bright, btop) = (top_right[j][0], top_right[j][1]);
                let (ileft, iright, ibottom, itop) = (aleft.max(bleft), aright.min(bright), abottom.max(bbottom), (atop.min(btop)));
                if ileft < iright && ibottom < itop {
                    max = max.max((iright - ileft).min(itop - ibottom));
                }
            }
        }
        max as i64 * max as i64
    }
}
