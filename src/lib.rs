use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{env, near_bindgen};

// Smart contract
#[near_bindgen]
#[derive(Default, BorshDeserialize, BorshSerialize)]
pub struct Contract {
    crosswordSolution: String,
}

#[near_bindgen]
impl Contract {
    #[init]
    pub fn new(solution: String) -> Self {
        Self {
            crosswordSolution: solution,
        }
    }


    pub fn guessSolution(&mut self, solution: String) -> bool {
        let hashed_input = env::sha256(solution.as_bytes());
        let hashed_input_hex = hex::encode(&hashed_input);

        if hashed_input_hex == self.crosswordSolution {
            env::log_str("It's True!");
            true
        } else {
            env::log_str("False. Let's try do it");
            false
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use near_sdk::test_utils::{get_logs, VMContextBuilder};
    use near_sdk::{testing_env, AccountId};

    #[test]
    fn debug_get_hash() {  
        testing_env!(VMContextBuilder::new().build());
    
        let debug_solution = "protocol near contract rust";
        let debug_hash_bytes = env::sha256(debug_solution.as_bytes());
        let debug_hash_string = hex::encode(debug_hash_bytes);
        println!("Let's debug: {:?}", debug_hash_string);
    }

    fn get_context(predecessor: AccountId) -> VMContextBuilder {
        let mut builder = VMContextBuilder::new();
        builder.predecessor_account_id(predecessor);
        builder
    }
    
    #[test]
    fn check_guess_solution() {
        let thanh = AccountId::new_unchecked("thanh.testnet".to_string());
        let context = get_context(thanh);
        testing_env!(context.build());
    

        let mut contract = Contract::new(
            "31431d5ac4f7d9a4e28aae69cd614458a2f1c5dda5386ebd7bd66b75232fdb1f".to_string(),
        );


        let mut guess_result = contract.guessSolution("near smart thanh".to_string());
        assert!(!guess_result, "Guess crossWord is wrong!");
        assert_eq!(get_logs(), ["Let try it!"], "Expected a failure log.");
        guess_result = contract.guessSolution("protocol near contract rust".to_string());
        assert!(guess_result, "Guess crossWord is true!");
        assert_eq!(
            get_logs(),
            ["When try guess it again", "You guessed right!"],
            "Expected a successful log after the previous failed log."
        );
    }
}