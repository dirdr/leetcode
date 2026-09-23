func minOperations(nums []int, x int) int {
    n, sum := len(nums), 0
    for _, v := range nums {
        sum += v
    }
    target, curr, l, mx := sum - x, 0, 0, -1
    for r := 0; r < n; r++ {
        curr += nums[r]
        for curr > target && l <= r {
            curr -= nums[l]
            l += 1
        }
        if curr == target {
            mx = max(mx, r - l + 1)
        }
    }
    if mx == -1 {
        return -1
    }
    return n - mx
}
