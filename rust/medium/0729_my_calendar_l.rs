struct Range {
    start: i32,
    end: i32
}

impl Range {
    fn new(start: i32, end: i32) -> Self {
        Range { start, end }
    }

    fn collide(&self, other: &Range) -> bool {
       (other.start > self.start && other.start < self.end) || (other.end > self.start && other.start < self.end)
    }
}

struct MyCalendar {
    books: Vec<Range>,
}


impl MyCalendar {

    fn new() -> Self {
        Self {
            books: vec![],
        }
    }
    
    fn book(&mut self, start: i32, end: i32) -> bool {
        let new_range = Range::new(start, end);
        if self.books.iter().any(|b| b.collide(&new_range)) {
            return false;
        }
        self.books.push(new_range);
        true
    }
}

/**
 * Your MyCalendar object will be instantiated and called as such:
 * let obj = MyCalendar::new();
 * let ret_1: bool = obj.book(start, end);
 */
