#![no_main]
extern crate alloc;

use alloy_sol_macro::sol;
use stylus_sdk::{
    alloy_primitives::{Address, U256},
    call::transfer_eth,
    contract, evm, msg,
    prelude::{public, sol_storage, SolidityError},
    stylus_proc::entrypoint,
};

sol_storage! {
    #[entrypoint]
    pub struct VisitorBook {
        uint256 fee;
        address[] visitors;
        mapping(address => bool) has_visited;
    }
}

sol! {
    event Visit(address indexed sender, string message);

    error InsufficientPayment(address visitor, uint256 payment);
    error TransferFailed(address recipient, uint256 amount);
    error AlreadyVisited();
    error IndexOutOfBounds();
}

#[derive(SolidityError)]
pub enum VisitorBookErrors {
    InsufficientPayment(InsufficientPayment),
    TransferFailed(TransferFailed),
    AlreadyVisited(AlreadyVisited),
    IndexOutOfBounds(IndexOutOfBounds),
}

#[public]
impl VisitorBook {
    pub fn initialize(&mut self) {
        // Initialize constants
        self.fee.set(U256::from(100));
    }
    // Function to record a new visitor
    pub fn sign_guestbook(&mut self, message: String) -> Result<(), VisitorBookErrors> {
        let visitor = msg::sender();
        let value = msg::value();

        // Require a payment of 100 wei
        if value < self.fee.get() {
            return Err(VisitorBookErrors::InsufficientPayment(
                InsufficientPayment {
                    visitor,
                    payment: value,
                },
            ));
        }

        if self.has_visited.get(visitor) {
            return Err(VisitorBookErrors::AlreadyVisited(AlreadyVisited {}));
        }

        // Add to visitors array
        self.visitors.push(visitor);
        // Mark as visited
        self.has_visited.setter(visitor).set(true);

        // Emit event
        evm::log(Visit {
            sender: visitor,
            message,
        });

        // Transfer reward
        if contract::balance() >= self.fee.get() {
            if let Err(_) = transfer_eth(visitor, self.fee.get()) {
                return Err(VisitorBookErrors::TransferFailed(TransferFailed {
                    recipient: visitor,
                    amount: self.fee.get(),
                }));
            }
        }

        Ok(())
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
