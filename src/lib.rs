use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{near_bindgen, env};

near_sdk::setup_alloc!();

const PUZZLE_NUMBER: u8 = 1;

#[near_bindgen]
#[derive(Default, BorshDeserialize, BorshSerialize)]
pub struct Contract {
    crossword_solution: String
}

#[near_bindgen]
impl Contract {
    pub fn get_puzzle_number(&self) -> u8 {
        PUZZLE_NUMBER
    }

    pub fn set_solution(&mut self, solution: String) {
        self.crossword_solution = solution;
    }

    pub fn guess_solution(&mut self, solution: String) {
        if solution == self.crossword_solution {
            env::log(b"You guessed right!")
        } else {
            env::log(b"Try again.")
        }
    }
}

/*
 * the rest of this file sets up unit tests
 * to run these, the command will be:
 * cargo test --package rust-template -- --nocapture
 * Note: 'rust-template' comes from cargo.toml's 'name' key
 */

// use the attribute below for unit tests
#[cfg(test)]
mod tests {
    use super::*;
    use near_sdk::{MockedBlockchain, testing_env};
    use near_sdk::test_utils::VMContextBuilder;
    use near_sdk::json_types::ValidAccountId;

    // part of writing unit tests is setting up a mock context
    // provide a `predecessor` here, it'll modify the default context
    fn get_context(predecessor: ValidAccountId) -> VMContextBuilder {
        let mut builder = VMContextBuilder::new();
        builder.predecessor_account_id(predecessor);
        builder
    }

    // TESTS HERE
}
