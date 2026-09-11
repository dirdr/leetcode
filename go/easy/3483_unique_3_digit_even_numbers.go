func totalNumbers(digits []int) int {
    n := len(digits)
    set := make(map[int]struct{})
    for i := 0; i < n; i++ {
        for j := 0; j < n; j++ {
            for k := 0; k < n; k++ {
                if i == j || i == k || j == k {
                    continue
                }
                num := 0
                num += digits[k]
                num += digits[j] * 10
                num += digits[i] * 100
                if num % 2 == 0 && num >= 100 {
                    set[num] = struct{}{}
                }
            }
        }
    }
    return len(set)
}
