func smallestIndex(nums []int) int {
    digit_sum := func(num int) int {
        s := 0
        for num > 0 {
            s += num % 10
            num /= 10
        }
        return s
    }
    for i, v := range nums {
        if digit_sum(v) == i {
            return i
        }
    }
    return -1
}
