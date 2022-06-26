/// Import `borsh` from `near_sdk` crate 
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
/// Import `serde` from `near_sdk` crate 
use near_sdk::serde::{Serialize, Deserialize};
use near_sdk::env;

/// Implements both `serde` and `borsh` serialization.
/// `serde` is typically useful when returning a struct in JSON format for a frontend.
// This Struct should act as a template definition to all my products
#[derive(Serialize, Deserialize, BorshDeserialize, BorshSerialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Order {
    pub order_id: String,
    pub order_details: String,
    pub customer_name: String,
    pub quantity: u8,
    pub order_date: u8,
    pub product_id: String,    
}

impl Order {
    pub fn generate_invoice(&self) {
        let msg = format!(
            "Order id: {} 
            \nOrder details: {}
            \nCustomer name: {}
            \nQuantity: {}\n
            Order date: {}\n
            Product id: {}\n
            ", 
            self.order_id,
            self.order_details,
            self.customer_name,
            self.quantity,
            self.order_date,
            self.product_id
    );
        env::log(msg.as_bytes());
    }

    // pub fn update_order(&mut self, order: Order) {
    //     self.customer_name = order.customer_name;
    //     self.order_date = order.order_date;
    //     self.order_details = order.order_details;
    //     self.order_id = order.order_id;
    //     self.product_id = order.product_id;
    //     self.quantity = order.quantity;
    // }

    pub fn calculate_total(&self) -> u8 {
        self.quantity
    }
}