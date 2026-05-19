impl Solution {
    pub fn get_common(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        let (mut p1, mut p2) = (0, 0);
        while p1 < nums1.len() && p2 < nums2.len() {
            let n1 = nums1[p1];
            let n2 = nums2[p2];
            if n1 == n2 {
                return n1;
            } else if n1 > n2 {
                p2 += 1;
            } else {
                p1 += 1;
            }
        }
        -1
    }
}
