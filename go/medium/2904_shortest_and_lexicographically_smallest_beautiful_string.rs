func shortestBeautifulSubstring(s string, k int) string {
    beautiful := func(sub string, k int) bool {
        count := 0
        for _, b := range []byte(sub) {
            if b == '1' {
                count++
            }
        }
        return count == k
    }
    smallest := ""
    for i := 0; i < len(s); i++ {
        for j := i; j < len(s); j++ {
            candidate := s[i : j + 1]
            if beautiful(candidate, k) && (smallest == "" || (len(candidate) < len(smallest)) || (len(candidate) == len(smallest) && candidate < smallest)) {
                smallest = candidate
            }
        }
    }
    return smallest
}
