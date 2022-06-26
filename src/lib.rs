mod product;
mod order;

use product::Product;
use order::Order;

/// Import `borsh` from `near_sdk` crate 
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{env, near_bindgen, PanicOnDefault};
use near_sdk::collections::UnorderedMap;
// use std::collections::BTreeMap;

/// Main contract structure serialized with Borsh
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Hardware {
    pub product_list: UnorderedMap<String, Product>,
    pub order_list: UnorderedMap<String, Order>,
}

#[near_bindgen]
impl Hardware {

    #[init]
    pub fn new() -> Hardware {
        let product_list: UnorderedMap<String, Product> = UnorderedMap::new(b"s".to_vec());
        let order_list: UnorderedMap<String, Order> = UnorderedMap::new(b"s".to_vec());
        Hardware {
            product_list,
            order_list,
        }
    }

    pub fn new_order(&mut self, order: Order) {
        let neworder = Order {
            customer_name: order.customer_name,
            order_date: order.order_date,
            order_details: order.order_details,
            order_id: order.order_id,
            product_id: order.product_id,
            quantity: order.quantity,
        };

        let id = &neworder.order_id;
        self.order_list.insert(id, &neworder);

    }


    pub fn update_order(&mut self, order: Order) {
        let updatedorder = Order {
            customer_name: order.customer_name,
            order_date: order.order_date,
            order_details: order.order_details,
            order_id: order.order_id,
            product_id: order.product_id,
            quantity: order.quantity,
        };

        let id = &updatedorder.order_id;
        self.order_list.insert(id, &updatedorder);

    }

    pub fn update_stock(&mut self, product: Product) {

        let newproduct = Product {
            product_id: product.product_id,
            product_name: product.product_name,
            stock_quantity: product.stock_quantity,
            price: product.price,
        };
        let id = &newproduct.product_id;
        self.product_list.insert(&id, &newproduct);
        
        
    }
    pub fn get_product(&self, id: String) -> Product {
        let product = match self.product_list.get(&id) {
            Some(product) => product,
            None => {
                let msg = format!("There is no such product");
                env::panic(msg.as_bytes());
            } 
        };
        product
    }

    pub fn get_order(&self, id: String) -> Order {
        let order = match self.order_list.get(&id) {
            Some(order) => order,
            None => {
                let msg = format!("There is no such product");
                env::panic(msg.as_bytes());
            } 
        };
        order
    }

    pub fn generate_invoice(&self, id: String) {
        self.get_order(id).generate_invoice();
    }

    pub fn check_stock(&self, id: String) -> u8 {
        self.get_product(id).check_stock()
    }
}

// Tests
#[cfg(test)]
mod tests {
    use super::*;
    use near_sdk::MockedBlockchain;
    use near_sdk::{testing_env, VMContext};

    fn get_context(input: Vec<u8>, is_view: bool) -> VMContext {
        VMContext {
            current_account_id: "alice.testnet".to_string(),
            signer_account_id: "robert.testnet".to_string(),
            signer_account_pk: vec![0, 1, 2],
            predecessor_account_id: "jane.testnet".to_string(),
            input,
            block_index: 0,
            block_timestamp: 0,
            account_balance: 1_000_000_000_000_000_000_000_000,
            account_locked_balance: 0,
            storage_usage: 0,
            attached_deposit: 0,
            prepaid_gas: 10u64.pow(18),
            random_seed: vec![0, 1, 2],
            is_view,
            output_data_receivers: vec![],
            epoch_height: 19,
        }
    }
    
    #[test] 
    fn new_order() {
        let context = get_context(vec![], false);
        testing_env!(context);

        let mut hardware = Hardware::new();
        let order = Order{
            order_id: "1".to_string(),
            order_details: "lambistic".to_string(),
            customer_name: "kinuthia".to_string(),
            quantity: 20,
            order_date: 10,
            product_id: "1".to_string(),
        };

        hardware.new_order(order);
        assert_eq!("kinuthia", hardware.get_order("1".to_string()).customer_name);
    }

    #[test]
    fn update_stock() {
        let context = get_context(vec![], false);
        testing_env!(context);

        let mut hardware = Hardware::new();
        let product = Product{
            product_id: "1".to_string(),
            product_name: "kinuthia".to_string(),
            stock_quantity: 10,
            price: 10.00,
        };

        hardware.update_stock(product);
        assert_eq!("kinuthia", hardware.get_product("1".to_string()).product_name)
    }
    #[test]
    fn update_order() {
        let context = get_context(vec![], false);
        testing_env!(context);

        let mut hardware = Hardware::new();
        let order = Order{
            order_id: "1".to_string(),
            order_details: "abra".to_string(),
            customer_name: "kinuthia".to_string(),
            quantity: 20,
            order_date: 10,
            product_id: "1".to_string(),
        };

        hardware.update_order(order);
        assert_eq!("abra", hardware.get_order("1".to_string()).order_details);
    }

    #[test]
    fn check_stock() {
        let context = get_context(vec![], false);
        testing_env!(context);

        let mut hardware = Hardware::new();
        let product = Product{
            product_id: "1".to_string(),
            product_name: "kinuthia".to_string(),
            stock_quantity: 10,
            price: 10.00,
        };

        hardware.update_stock(product);
        assert_eq!(10, hardware.get_product("1".to_string()).check_stock());
    }
}