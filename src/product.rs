/// Import `borsh` from `near_sdk` crate 
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
/// Import `serde` from `near_sdk` crate 
use near_sdk::serde::{Serialize, Deserialize};
//use near_sdk::env;

/// Implements both `serde` and `borsh` serialization.
/// `serde` is typically useful when returning a struct in JSON format for a frontend.
// This Struct should act as a template definition to all my products
#[derive(Serialize, Deserialize, BorshDeserialize, BorshSerialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Product {
    pub product_id: String,
    pub product_name: String,
    pub stock_quantity: u8,
    pub price: u32,
}

impl Product{
    pub fn check_stock(&self) -> u8 {
        self.stock_quantity
    }
}