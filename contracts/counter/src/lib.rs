// Find all our documentation at https://docs.near.org
use near_sdk::{log, near};

// Define the contract structure
#[near(contract_state)]
#[derive(Default)]
pub struct Counter {
    val: i8,
}

// Implement the contract structure
#[near]
impl Counter {
    pub fn get_num(&self) -> i8 {
        return self.val;
    }

    pub fn increment(&mut self) {
        self.val += 1;
        log!("Increased number to {}", self.val);
    }

    pub fn decrement(&mut self) {
        self.val += -1;
        log!("Decreased number to {}", self.val);
    }

    pub fn reset(&mut self) {
        self.val = 0;
        log!("Reset number to {}", self.val);
    }
}

/*
 * The rest of this file holds the inline tests for the code above
 * Learn more about Rust tests: https://doc.rust-lang.org/book/ch11-01-writing-tests.html
 */
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_num() {
        let contract = Counter::default();
        // this test did not call set_greeting so should return the default "Hello" greeting
        assert_eq!(contract.get_num(), 0);
    }

    #[test]
    fn incr_counter() {
        let mut contract = Counter::default();
        contract.increment();
        assert_eq!(contract.get_num(), 1);
    }

    #[test]
    fn decr_counter() {
        let mut contract = Counter::default();
        contract.decrement();
        assert_eq!(contract.get_num(), -1); 
    }

    #[test]
    fn reset_counter() {
        let mut contract = Counter::default();
        contract.reset();
        assert_eq!(contract.get_num(), 0); 
    }
}