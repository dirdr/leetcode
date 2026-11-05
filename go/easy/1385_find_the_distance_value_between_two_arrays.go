func findTheDistanceValue(arr1 []int, arr2 []int, d int) int {
    out := 0
    for _, v := range arr1 {
        flag := true
        for _, u := range arr2 {
            if max(v - u, u - v) <= d {
                flag = false
                break
            }
        }
        if flag {
            out++
        }
    }
    return out
}
