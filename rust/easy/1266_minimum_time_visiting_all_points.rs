impl Solution {
    pub fn min_time_to_visit_all_points(points: Vec<Vec<i32>>) -> i32 {
        fn normalize(x: i32) -> i32 {
            match x.cmp(&0) {
                std::cmp::Ordering::Less => -1,
                std::cmp::Ordering::Equal => 0,
                std::cmp::Ordering::Greater => 1,
            }
        }
        let (mut cx, mut cy) = (points[0][0], points[0][1]);
        let mut time = 0;
        for point in points.iter().skip(1) {
            let (x, y) = (point[0], point[1]);
            while (cx != x) || (cy != y) {
                let (dx, dy) = (normalize(x - cx), normalize(y - cy));
                cx += dx;
                cy += dy;
                time += 1;
            }
        }
        time
    }
}
