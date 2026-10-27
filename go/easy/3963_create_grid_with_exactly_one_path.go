func createGrid(m int, n int) []string {
    out := make([]string, m)
    first := strings.Repeat(".", n)
    out[0] = first
    for i := 1; i < m; i++ {
        curr := strings.Repeat("#", n - 1)
        curr += "."
        out[i] = curr
    }
    return out
}
