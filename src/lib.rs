use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{env, near_bindgen};

#[near_bindgen]
#[derive(Default, BorshDeserialize, BorshSerialize)]
pub struct Contract {
    crossword_solution: String,
}

#[near_bindgen]
impl Contract {
    #[init]
    pub fn new(solution: String) -> Self {
        Self {
            crossword_solution: solution,
        }
    }

    pub fn get_solution(&self) -> String {
        self.crossword_solution.clone()
    }

    pub fn guess_solution(&mut self, solution: String) -> bool {
        if solution == self.crossword_solution {
            env::log_str("You guessed right!");
            true
        } else {
            env::log_str("Try again.");
            false
        }
    }
}

/*
 * the rest of this file sets up unit tests
 * to run these, the command will be:
 * cargo test --package rust-template -- --nocapture
 * Note: 'rust-template' comes from Cargo.toml's 'name' key
 */

// use the attribute below for unit tests
#[cfg(test)]
mod tests {
    use super::*;
    use near_sdk::test_utils::{get_logs, VMContextBuilder};
    use near_sdk::{testing_env, AccountId};

    // part of writing unit tests is setting up a mock context
    // provide a `predecessor` here, it'll modify the default context
    fn get_context(predecessor: AccountId) -> VMContextBuilder {
        let mut builder = VMContextBuilder::new();
        builder.predecessor_account_id(predecessor);
        builder
    }

    #[test]
    fn debug_get_hash() {
        // Basic set up for a unit test
        testing_env!(VMContextBuilder::new().build());

        // Using a unit test to rapidly debug and iterate
        let debug_solution = "near nomicon ref finance";
        let debug_hash_bytes = env::sha256(debug_solution.as_bytes());
        let debug_byte_array: Vec<String> = debug_hash_bytes
            .iter()
            .map(|b| format!("{:02X}", b))
            .collect();
        let debug_hash_string = debug_byte_array.join("");
        println!("Let's debug: {:02X?}", debug_hash_string);
    }

    #[test]
    fn check_guess_solution() {
        // Get Alice as an account ID
        let alice = AccountId::new_unchecked("alice.testnet".to_string());
        // Set up the testing context and unit test environment
        let context = get_context(alice);
        testing_env!(context.build());

        // Set up contract object and call methods
        let mut contract = Contract::default();
        // Set the solution to be the hash we determined from the previous, helper unit test
        contract.set_solution(
            "69C2FEB084439956193F4C21936025F14A5A5A78979D67AE34762E18A7206A0F".to_string(),
        );
        contract.guess_solution("wrong answer here".to_string());
        assert_eq!(get_logs(), ["Try again."], "Expected a failure log.");
        contract.guess_solution(
            "69C2FEB084439956193F4C21936025F14A5A5A78979D67AE34762E18A7206A0F".to_string(),
        );
        assert_eq!(
            get_logs(),
            ["Try again.", "You guessed right!"],
            "Expected a successful log after the previous failed log."
        );
    }
}
