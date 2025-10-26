struct Bank {
    balance: Vec<i64>,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Bank {

    fn new(balance: Vec<i64>) -> Self {
        Self {
            balance
        }
    }
    
    
    fn transfer(&mut self, account1: i32, account2: i32, money: i64) -> bool {
        let (a, b) = (account1 as usize - 1, account2 as usize - 1);
        if a < 0 || a >= self.balance.len() {
            return false;
        }
        if b < 0 || b >= self.balance.len() {
            return false;
        }
        if self.balance[a] < money {
            false
        } else {
            self.balance[a] -= money;
            self.balance[b] += money;
            true
        }
    }
    
    fn deposit(&mut self, account: i32, money: i64) -> bool {
        let a = account as usize - 1;
        if a < 0 || a >= self.balance.len() {
            false
        } else {
            self.balance[a] += money;
            true
        }
    }
    
    fn withdraw(&mut self, account: i32, money: i64) -> bool {
        let a = account as usize - 1;
        if a < 0 || a >= self.balance.len() || self.balance[a] < money {
            false
        } else {
            self.balance[a] -= money;
            true
        }
    }
}

/**
 * Your Bank object will be instantiated and called as such:
 * let obj = Bank::new(balance);
 * let ret_1: bool = obj.transfer(account1, account2, money);
 * let ret_2: bool = obj.deposit(account, money);
 * let ret_3: bool = obj.withdraw(account, money);
 */
