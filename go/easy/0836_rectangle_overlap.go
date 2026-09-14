func isRectangleOverlap(rec1 []int, rec2 []int) bool {
    xa1, ya1, xb1, yb1 := rec1[0], rec1[1], rec1[2], rec1[3]
    xa2, ya2, xb2, yb2 := rec2[0], rec2[1], rec2[2], rec2[3]
    if xb1 <= xa2 {
        //fmt.Println("First tr is strictly left to other")
        return false
    }
    if xa1 >= xb2 {
        //fmt.Println("First tr is strictly right to other")
        return false
    }
    if yb1 <= ya2 {
        //fmt.Println("First tr is strictly under to other")
        return false
    }
    if ya1 >= yb2 {
        //fmt.Println("First tr is strictly above to other")
        return false
    }
    return true
}
