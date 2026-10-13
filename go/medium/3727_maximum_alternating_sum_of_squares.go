func maxAlternatingSum(nums []int) int64 {
    n := len(nums)
    for i, v := range nums {
        nums[i] = v*v
    }
    slices.Sort(nums)
    var out int64
    for i := 0; i < n/2; i++ {
        out -= int64(nums[i])
        out += int64(nums[n-i-1])
    }
    if n%2 != 0 {
        out += int64(nums[n/2])
    }
    return out
}
