func maxSum(nums []int, k int) int {
    const MOD = 1_000_000_007
    n := len(nums)
    bitReserve := make([]int, 31)

    for _, v := range nums {
        for b := 0; b < 31; b++ {
            if v>>b&1 == 1 {
                bitReserve[b]++
            }
        }
    }

    afterGravity := make([]int64, n)
    for b := 0; b < 31; b++ {
        count := bitReserve[b]
        for j := 0; j < count; j++ {
            afterGravity[j] |= 1 << b
        }
    }
    var out int64
    for i, v := range afterGravity {
        if i == k {
            break
        }
        out = (out + v*v) % MOD
    }
    return int(out)
}
