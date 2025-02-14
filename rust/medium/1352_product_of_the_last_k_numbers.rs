struct ProductOfNumbers {
    prefix: Vec<i32>,
}

/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl ProductOfNumbers {
    fn new() -> Self {
        Self {
            prefix: vec![],
        } 
    }
    
    fn add(&mut self, num: i32) {
        if num == 0 {
            self.prefix.clear();
            return;
        }
        let mut num = num;
        if self.prefix.len() > 0 {
            num *= self.prefix[self.prefix.len() - 1];
        }
        self.prefix.push(num);
    }
    
    fn get_product(&self, k: i32) -> i32 {
        let n = self.prefix.len();
        let k = k as usize;
        if k > n {
            return 0;
        }
        let mut result = self.prefix[n - 1];
        if k < n {
            result /= self.prefix[n - 1 - k as usize];
        }
        result
    }
}

/**
 * Your ProductOfNumbers object will be instantiated and called as such:
 * let obj = ProductOfNumbers::new();
 * obj.add(num);
 * let ret_2: i32 = obj.get_product(k);
 */
