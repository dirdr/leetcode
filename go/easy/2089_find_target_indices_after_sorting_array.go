func targetIndices(nums []int, target int) []int {
    slices.Sort(nums)
    out := make([]int, 0)
    for i, v := range nums {
        if v == target {
            out = append(out, i)
        }
    }
    return out
}
