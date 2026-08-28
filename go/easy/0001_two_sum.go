func twoSum(nums []int, target int) []int {
    m := make(map[int]int)
    for i, val := range nums {
        if j, ok := m[val]; ok {
            return []int{j, i}
        } else {
            m[target - val] = i
        }
    }
    return []int{}
}
