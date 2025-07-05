func findLucky(arr []int) int {
    freq := make(map[int]int)

    for _, el := range arr {
        freq[el]++
    }

    res := -1
    for k, v := range freq {
        if k == v && k > res {
            res = k
        }
    }

    return res
}
