use std::collections::VecDeque;


struct MinStack {
    wrapper: Vec<i32>,
    min: VecDeque<i32>,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MinStack {

    fn new() -> Self { Self { wrapper: Vec::new(), min: VecDeque::new() } }
    
    fn push(&mut self, val: i32) {
        self.wrapper.push(val);
        if self.min.is_empty() || Some(&val) <= self.min.front() {
            self.min.push_front(val);
        }
    }
    
    fn pop(&mut self) {
        let val = self.wrapper.pop().unwrap();
        if Some(&val) == self.min.front() {
            self.min.pop_front();
        }
    }
    
    fn top(&self) -> i32 {
        self.wrapper.last().cloned().unwrap()
    }
    
    fn get_min(&self) -> i32 {
        self.min.front().cloned().unwrap()
    }
}

/**
 * Your MinStack object will be instantiated and called as such:
 * let obj = MinStack::new();
 * obj.push(val);
 * obj.pop();
 * let ret_3: i32 = obj.top();
 * let ret_4: i32 = obj.get_min();
 */
