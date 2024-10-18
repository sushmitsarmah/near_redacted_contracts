// Find all our documentation at https://docs.near.org
// use near_sdk::{log, near};
use near_sdk::{near};
use near_sdk::{AccountId, PanicOnDefault};
use near_sdk::json_types::U64;
use near_sdk::store::IterableMap;

#[near(serializers = [json, borsh])]
#[derive(Clone)]
pub struct Question {
    question_id: U64,
    coin_id: String,
    target_price: U64,
    answer: bool,
}

#[near(serializers = [json, borsh])]
#[derive(Clone)]
pub struct UserAnswer {
    address: String,
    question_id: U64,
    answer: bool,
    stake: U64,
}

#[near(serializers = [json, borsh])]
#[derive(Clone)]
pub struct UserStake {
    pub amount: U64,
}

// Define the contract structure
// #[near(contract_state)]
#[near(contract_state)]
#[derive(PanicOnDefault)]
pub struct MoraqRound {
    round_id: u128,
    start_time: U64,
    end_time: U64,
    questions: Vec<Question>,
    user_answers: Vec<UserAnswer>,
    user_stakes: IterableMap<AccountId, UserStake>,
    total_staked: U64,
    participants: Vec<AccountId>,
    winners: Vec<AccountId>
}

// Define the default, which automatically initializes the contract
// impl Default for MoraqRound {
//     fn default() -> Self {
//         Self {
//             greeting: "Hello".to_string(),
//         }
//     }
// }

// Implement the contract structure
// #[near]
// impl MoraqRound {
//     // Public method - returns the greeting saved, defaulting to DEFAULT_GREETING
//     pub fn get_greeting(&self) -> String {
//         self.greeting.clone()
//     }

//     // Public method - accepts a greeting, such as "howdy", and records it
//     pub fn set_greeting(&mut self, greeting: String) {
//         log!("Saving greeting: {greeting}");
//         self.greeting = greeting;
//     }
// }

/*
 * The rest of this file holds the inline tests for the code above
 * Learn more about Rust tests: https://doc.rust-lang.org/book/ch11-01-writing-tests.html
 */
// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn get_default_greeting() {
//         let contract = Contract::default();
//         // this test did not call set_greeting so should return the default "Hello" greeting
//         assert_eq!(contract.get_greeting(), "Hello");
//     }

//     #[test]
//     fn set_then_get_greeting() {
//         let mut contract = Contract::default();
//         contract.set_greeting("howdy".to_string());
//         assert_eq!(contract.get_greeting(), "howdy");
//     }
// }
