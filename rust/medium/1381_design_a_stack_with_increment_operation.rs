struct CustomStack {
    vec: Vec<i32>,
}

impl CustomStack {
    fn new(maxSize: i32) -> Self {
        CustomStack {
            vec: Vec::with_capacity(maxSize as usize),
        }
    }

    fn push(&mut self, x: i32) {
        if self.vec.len() + 1 <= self.vec.capacity() {
            self.vec.push(x);
        }
    }

    fn pop(&mut self) -> i32 {
        self.vec.pop().unwrap_or(-1)
    }

    fn increment(&mut self, k: i32, val: i32) {
        for num in self.vec.iter_mut().take(k as usize) {
            *num += val;
        }
    }
}
