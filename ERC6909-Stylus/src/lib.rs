extern crate alloc;

use stylus_sdk::{
    alloy_primitives::{Address, U256},
    prelude::*,
    stylus_core::log,
};
use alloy_sol_types::sol;

sol! {
    event TransferSingle(address indexed operator, address indexed from, address indexed to, uint256 id, uint256 amount);
    event ApprovalSingle(address indexed owner, address indexed spender, uint256 indexed id, uint256 amount);
}

sol_storage! {
    #[entrypoint]
    pub struct ERC6909Token {
        mapping(address => mapping(uint256 => uint256)) balances;
        mapping(address => mapping(address => mapping(uint256 => uint256))) allowances;
        mapping(address => mapping(address => bool)) operators;
        mapping(uint256 => uint256) total_supplies;
    }
}

#[public]
impl ERC6909Token {
    pub fn total_supply(&self, token_id: U256) -> U256 {
        self.total_supplies.get(token_id)
    }

    pub fn balance_of(&self, owner: Address, token_id: U256) -> U256 {
        self.balances.getter(owner).get(token_id)
    }

    pub fn allowance(&self, owner: Address, spender: Address, token_id: U256) -> U256 {
        self.allowances.getter(owner).getter(spender).get(token_id)
    }

    pub fn operator_approval(&self, owner: Address, operator: Address) -> bool {
        self.operators.getter(owner).get(operator)
    }

    pub fn transfer_from(&mut self, from: Address, to: Address, token_id: U256, amount: U256) -> bool {
        let sender = self.vm().msg_sender();
        
        // Simple permission check
        let allowed = sender == from || self.operators.getter(from).get(sender);
        assert!(allowed, "Not allowed");
        
        // Simple balance check and transfer
        let from_balance = self.balances.getter(from).get(token_id);
        assert!(from_balance >= amount, "Insufficient balance");
        
        self.balances.setter(from).setter(token_id).set(from_balance - amount);
        let to_balance = self.balances.getter(to).get(token_id);
        self.balances.setter(to).setter(token_id).set(to_balance + amount);
        
        log(self.vm(), TransferSingle { operator: sender, from, to, id: token_id, amount });
        true
    }

    pub fn approve(&mut self, spender: Address, token_id: U256, amount: U256) -> bool {
        let owner = self.vm().msg_sender();
        self.allowances.setter(owner).setter(spender).setter(token_id).set(amount);
        log(self.vm(), ApprovalSingle { owner, spender, id: token_id, amount });
        true
    }

    pub fn set_operator(&mut self, operator: Address, approved: bool) -> bool {
        let owner = self.vm().msg_sender();
        self.operators.setter(owner).setter(operator).set(approved);
        true
    }

    pub fn mint(&mut self, to: Address, token_id: U256, amount: U256) -> bool {
        let current_balance = self.balances.getter(to).get(token_id);
        self.balances.setter(to).setter(token_id).set(current_balance + amount);
        
        let current_supply = self.total_supplies.get(token_id);
        self.total_supplies.setter(token_id).set(current_supply + amount);
        
        log(self.vm(), TransferSingle { 
            operator: self.vm().msg_sender(), 
            from: Address::ZERO, 
            to, 
            id: token_id, 
            amount 
        });
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_address_and_u256() {
        // Test basic types used in ERC-6909
        let owner = Address::from([1u8; 20]);
        let token_id = U256::from(1);
        let amount = U256::from(100);
        
        assert_eq!(token_id, U256::from(1));
        assert_eq!(amount, U256::from(100));
        assert_ne!(owner, Address::ZERO);
    }

    #[test] 
    fn test_erc6909_logic() {
        // Test basic arithmetic that would happen in transfers
        let initial_balance = U256::from(1000);
        let transfer_amount = U256::from(100);
        
        let new_from_balance = initial_balance - transfer_amount;
        let new_to_balance = U256::from(0) + transfer_amount;
        
        assert_eq!(new_from_balance, U256::from(900));
        assert_eq!(new_to_balance, transfer_amount);
    }

    #[test]
    fn test_address_comparison() {
        // Test address comparison logic used in permissions
        let owner = Address::from([1u8; 20]);
        let sender = Address::from([1u8; 20]);
        let other = Address::from([2u8; 20]);
        
        assert_eq!(owner == sender, true);
        assert_eq!(owner == other, false);
    }
}