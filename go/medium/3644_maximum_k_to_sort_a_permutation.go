func sortPermutation(nums []int) int {
    nip := make([]int, 0)
    for i, v := range nums {
        if i != v {
            nip = append(nip, v)
        }
    }
    if len(nip) == 0 {
        return 0
    }
    // invarient, for sure two elements
    out := nip[0] & nip[1]
    for i := 2; i < len(nip); i++ {
        out &= nip[i]
    }
    return out
}
