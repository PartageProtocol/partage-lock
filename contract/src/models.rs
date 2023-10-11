use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{env, AccountId, Timestamp, near_bindgen};
use near_sdk::serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, BorshDeserialize, BorshSerialize)]
#[serde(crate = "near_sdk::serde")]

pub struct Booking {
    id: i32,
  pub creator: AccountId,
    created_at: Timestamp,
    name: String,
    nbr_days: u128,
    // starting_date: DateTime,
    total_price: u128,
    description: String,
    pub passwords: String
}

impl Booking {
    pub fn new(
        id:i32, 
        name: String,
        nbr_days:u128, 
        /* starting_date: DateTime, */ 
        total_price:u128, 
        description: String, 
        passwords: String
    ) -> Self {   
        Booking {
            id,
            creator: env::signer_account_id(),
            created_at: env::block_timestamp(),
            name,
            nbr_days,
            // starting_date,
            total_price,
            description,
            passwords,
        }
    }
}