impl Solution {
    pub fn largest_triangle_area(points: Vec<Vec<i32>>) -> f64 {
        let mut max = 0.0f64;
        for a in 0..points.len() - 2 {
            for b in a + 1..points.len() - 1 {
                for c in b + 1..points.len() {
                    let (x1, x2, x3) = (points[a][0] as f64, points[b][0] as f64, points[c][0] as f64);
                    let (y1, y2, y3) = (points[a][1] as f64, points[b][1] as f64, points[c][1] as f64);
                    // (1/2) |x1(y2 − y3) + x2(y3 − y1) + x3(y1 − y2)|
                    max = max.max(0.5 * (x1 * (y2 - y3) + x2 * (y3 - y1) + x3 * (y1 - y2)).abs());
                }
            }
        }
        max
    }
}
