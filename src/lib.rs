use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{log, near_bindgen};

const DEFAULT_MESSAGE: &str = "Hello world";

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Contract {
    message: String,
}


impl Default for Contract{

    fn default() -> Self{
        Self{message: DEFAULT_MESSAGE.to_string()}
    }

}

#[near_bindgen]
impl Contract {
    
    pub fn get_message(&self) -> String {
        
        return self.message.clone();
    }

    pub fn set_message(&mut self, message: String) {
        // Use env::log to record logs permanently to the blockchain!
        log!("Saving greeting {}", message);
        self.message = message;
    }
}
