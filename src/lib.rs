#![no_main]
extern crate alloc;

use stylus_sdk::{
    alloy_primitives::{Address, U256},
    msg,
    prelude::{public, sol_storage},
    stylus_proc::entrypoint,
};

sol_storage! {
    #[entrypoint]
    pub struct VisitorBook {
        address[] visitors;
        mapping(address => bool) has_visited;
    }
}

#[public]
impl VisitorBook {
    // Function to record a new visitor
    pub fn sign_guestbook(&mut self) {
        let visitor = msg::sender();

        // Check if the address has already visited
        if !self.has_visited.get(visitor) {
            // Add to visitors array
            self.visitors.push(visitor);
            // Mark as visited
            self.has_visited.setter(visitor).set(true);
        }
    }

    // Get total number of unique visitors
    pub fn get_total_visitors(&self) -> U256 {
        U256::from(self.visitors.len())
    }

    // Get visitor at specific index
    pub fn get_visitor_at_index(&self, index: U256) -> Address {
        self.visitors.get(index).unwrap()
    }

    // Check if an address has visited
    pub fn has_address_visited(&self, address: Address) -> bool {
        self.has_visited.get(address)
    }
}
