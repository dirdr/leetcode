import "math"

func missingMultiple(nums []int, k int) int {
    set := make(map[int] struct{})
    for _, n := range nums {
        set[n] = struct{}{}
    }

    for i := 1; i <= math.MaxInt32; i++ {
        if _, ok := set[i]; !ok && i % k == 0 {
            return i
        }
    }

    return -1
}
