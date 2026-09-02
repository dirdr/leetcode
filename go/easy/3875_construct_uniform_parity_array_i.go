func uniformArray(nums1 []int) bool {
    check := func(nums []int, predicate func(int) bool) bool {
        for i, u := range nums {
            if predicate(u) {
                continue
            }
            p := false
            for j, v := range nums {
                if i == j {
                    continue
                }
                if predicate(u + u + v) {
                    p = true
                    break
                }
            }
            if !p {
                return false
            }
        }
        return true
    }
    even := check(nums1, func(n int) bool {
        return n % 2 == 0
    })
    odd := check(nums1, func(n int) bool {
        return n % 2 != 0
    })
    return odd || even  
}
