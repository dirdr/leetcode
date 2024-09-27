use std::collections::BTreeMap;

struct MyCalendarTwo {
    bookings: BTreeMap<i32, i32>,
}

impl MyCalendarTwo {
    fn new() -> Self {
        MyCalendarTwo {
            bookings: BTreeMap::new(),
        }
    }
    
    fn book(&mut self, start: i32, end: i32) -> bool {
        *self.bookings.entry(start).or_insert(0) += 1;
        *self.bookings.entry(end).or_insert(0) -= 1;
        let mut active_bookings = 0;
        for &count in self.bookings.values() {
            active_bookings += count;
            if active_bookings > 2 {
                *self.bookings.entry(start).or_insert(0) -= 1;
                *self.bookings.entry(end).or_insert(0) += 1;
                return false;
            }
        } 
        true
    }
