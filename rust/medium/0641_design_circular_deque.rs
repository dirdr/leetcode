pub struct MyCircularDeque {
    data: Vec<i32>,
    head: usize,
    tail: usize,
    size: usize,
    capacity: usize,
}

impl MyCircularDeque {
    pub fn new(k: i32) -> Self {
        let capacity = k as usize;
        MyCircularDeque {
            data: vec![0; capacity],
            head: 0,
            tail: k as usize - 1,
            size: 0,
            capacity,
        }
    }

    pub fn insert_front(&mut self, value: i32) -> bool {
        if self.is_full() {
            return false;
        }
        self.head = (self.head + self.capacity - 1) % self.capacity;
        self.data[self.head] = value;
        self.size += 1;
        true
    }

    pub fn insert_last(&mut self, value: i32) -> bool {
        if self.is_full() {
            return false;
        }
        self.tail = (self.tail + 1) % self.capacity;
        self.data[self.tail] = value;
        self.size += 1;
        true
    }

    pub fn delete_front(&mut self) -> bool {
        if self.is_empty() {
            return false;
        }
        self.head = (self.head + 1) % self.capacity;
        self.size -= 1;
        true
    }

    pub fn delete_last(&mut self) -> bool {
        if self.is_empty() {
            return false;
        }
        self.tail = (self.tail + self.capacity - 1) % self.capacity;
        self.size -= 1;
        true
    }

    pub fn get_front(&self) -> i32 {
        if self.is_empty() {
            return -1;
        }
        self.data[self.head]
    }

    pub fn get_rear(&self) -> i32 {
        if self.is_empty() {
            return -1;
        }
        self.data[self.tail]
    }

    pub fn is_empty(&self) -> bool {
        self.size == 0
    }

    pub fn is_full(&self) -> bool {
        self.size == self.capacity
    }
}
