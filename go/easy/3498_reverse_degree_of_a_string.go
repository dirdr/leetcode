func reverseDegree(s string) int {
    answer := 0
    for i, v := range s {
        answer += (26 - int(v - 'a')) * (i + 1)
    }
    return answer
}
