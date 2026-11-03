func minMoves(nums []int) int {
    if len(nums) < 2 {
        return 0;
    }
    maxVal := 0
    for _, v := range nums {
        maxVal = max(maxVal, v)
    }
    out := 0;
    for _, v := range nums {
        out += maxVal - v;
    }
    return out
}
