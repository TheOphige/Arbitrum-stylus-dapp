// // tests/erc20_tests.rs

// #![cfg(test)]

// extern crate alloc;

// use alloc::string::ToString;
// use erc20_stylus::erc20::*;
// use erc20_stylus::e;
// use stylus_sdk::testing::*;
// use stylus_sdk::alloy_primitives::{Address, U256};

// // Helper function to create a dummy address
// fn addr(n: u8) -> Address {
//     let mut a = [0u8; 20];
//     a[19] = n;
//     Address::from(a)
// }

// #[test]
// fn test_mint() {
//     let vm = TestVm::default();
//     let mut token = StylusToken::deploy(&vm);

//     let user = addr(1);
//     vm.set_sender(user); // Simulate msg.sender
//     let amount = U256::from(1000);

//     assert!(token.mint(amount).is_ok());
//     assert_eq!(token.erc20.balance_of(user), amount);
//     assert_eq!(token.erc20.total_supply(), amount);
// }

// #[test]
// fn test_burn() {
//     let vm = TestVm::default();
//     let mut token = StylusToken::deploy(&vm);

//     let user = addr(1);
//     vm.set_sender(user);

//     let amount = U256::from(1000);
//     token.mint(amount).unwrap();
//     assert!(token.erc20.burn(user, U256::from(400)).is_ok());

//     assert_eq!(token.erc20.balance_of(user), U256::from(600));
//     assert_eq!(token.erc20.total_supply(), U256::from(600));
// }

// #[test]
// fn test_transfer() {
//     let vm = TestVm::default();
//     let mut token = StylusToken::deploy(&vm);

//     let alice = addr(1);
//     let bob = addr(2);

//     vm.set_sender(alice);
//     token.mint(U256::from(500)).unwrap();

//     // Transfer 200 from Alice to Bob
//     assert!(token.erc20.transfer(bob, U256::from(200)).is_ok());
//     assert_eq!(token.erc20.balance_of(alice), U256::from(300));
//     assert_eq!(token.erc20.balance_of(bob), U256::from(200));
// }

// #[test]
// fn test_approve_and_transfer_from() {
//     let vm = TestVm::default();
//     let mut token = StylusToken::deploy(&vm);

//     let alice = addr(1);
//     let bob = addr(2);
//     let carol = addr(3);

//     // Alice mints 500 tokens
//     vm.set_sender(alice);
//     token.mint(U256::from(500)).unwrap();

//     // Alice approves Bob to spend 200
//     assert!(token.erc20.approve(bob, U256::from(200)).is_ok());
//     assert_eq!(token.erc20.allowance(alice, bob), U256::from(200));

//     // Bob transfers 150 from Alice to Carol
//     vm.set_sender(bob);
//     assert!(token
//         .erc20
//         .transfer_from(alice, carol, U256::from(150))
//         .is_ok());

//     // Check balances
//     assert_eq!(token.erc20.balance_of(alice), U256::from(350));
//     assert_eq!(token.erc20.balance_of(carol), U256::from(150));
//     assert_eq!(token.erc20.allowance(alice, bob), U256::from(50));
// }

// #[test]
// fn test_insufficient_balance() {
//     let vm = TestVm::default();
//     let mut token = StylusToken::deploy(&vm);

//     let alice = addr(1);
//     let bob = addr(2);

//     vm.set_sender(alice);
//     token.mint(U256::from(100)).unwrap();

//     let res = token.erc20.transfer(bob, U256::from(200));
//     assert!(matches!(res, Err(Erc20Error::InsufficientBalance(_))));
// }

// #[test]
// fn test_insufficient_allowance() {
//     let vm = TestVm::default();
//     let mut token = StylusToken::deploy(&vm);

//     let alice = addr(1);
//     let bob = addr(2);
//     let carol = addr(3);

//     vm.set_sender(alice);
//     token.mint(U256::from(100)).unwrap();

//     vm.set_sender(bob);
//     let res = token.erc20.transfer_from(alice, carol, U256::from(50));
//     assert!(matches!(res, Err(Erc20Error::InsufficientAllowance(_))));
// }

// #[test]
// fn test_double_approve_overwrites() {
//     let vm = TestVm::default();
//     let mut token = StylusToken::deploy(&vm);

//     let alice = addr(1);
//     let bob = addr(2);

//     vm.set_sender(alice);
//     token.mint(U256::from(100)).unwrap();

//     assert!(token.erc20.approve(bob, U256::from(40)).is_ok());
//     assert_eq!(token.erc20.allowance(alice, bob), U256::from(40));

//     // Overwrite allowance
//     assert!(token.erc20.approve(bob, U256::from(70)).is_ok());
//     assert_eq!(token.erc20.allowance(alice, bob), U256::from(70));
// }

// #[test]
// fn test_burn_more_than_balance() {
//     let vm = TestVm::default();
//     let mut token = StylusToken::deploy(&vm);

//     let user = addr(1);
//     vm.set_sender(user);
//     token.mint(U256::from(50)).unwrap();

//     let res = token.erc20.burn(user, U256::from(100));
//     assert!(matches!(res, Err(Erc20Error::InsufficientBalance(_))));
// }

// #[test]
// fn test_transfer_zero_tokens() {
//     let vm = TestVm::default();
//     let mut token = StylusToken::deploy(&vm);

//     let alice = addr(1);
//     let bob = addr(2);

//     vm.set_sender(alice);
//     token.mint(U256::from(100)).unwrap();

//     // Transfer zero tokens should succeed and not change balances
//     assert!(token.erc20.transfer(bob, U256::from(0)).is_ok());
//     assert_eq!(token.erc20.balance_of(alice), U256::from(100));
//     assert_eq!(token.erc20.balance_of(bob), U256::from(0));
// }

// #[test]
// fn test_approve_self_and_transfer_from() {
//     let vm = TestVm::default();
//     let mut token = StylusToken::deploy(&vm);

//     let alice = addr(1);

//     vm.set_sender(alice);
//     token.mint(U256::from(100)).unwrap();

//     // Alice approves herself
//     assert!(token.erc20.approve(alice, U256::from(100)).is_ok());
//     assert_eq!(token.erc20.allowance(alice, alice), U256::from(100));

//     // Alice transfers from herself to herself
//     assert!(token.erc20.transfer_from(alice, alice, U256::from(50)).is_ok());
//     assert_eq!(token.erc20.balance_of(alice), U256::from(100));
//     assert_eq!(token.erc20.allowance(alice, alice), U256::from(50));
// }

// #[test]
// fn test_total_supply_after_multiple_mints_and_burns() {
//     let vm = TestVm::default();
//     let mut token = StylusToken::deploy(&vm);

//     let alice = addr(1);
//     let bob = addr(2);

//     vm.set_sender(alice);
//     token.mint(U256::from(100)).unwrap();

//     vm.set_sender(bob);
//     token.mint(U256::from(200)).unwrap();

//     assert_eq!(token.erc20.total_supply(), U256::from(300));

//     token.erc20.burn(alice, U256::from(40)).unwrap();
//     token.erc20.burn(bob, U256::from(100)).unwrap();

//     assert_eq!(token.erc20.total_supply(), U256::from(160));
//     assert_eq!(token.erc20.balance_of(alice), U256::from(60));
//     assert_eq!(token.erc20.balance_of(bob), U256::from(100));
// }

// #[test]
// fn test_transfer_to_self() {
//     let vm = TestVm::default();
//     let mut token = StylusToken::deploy(&vm);

//     let alice = addr(1);

//     vm.set_sender(alice);
//     token.mint(U256::from(100)).unwrap();

//     // Transfer to self should succeed and not change balance
//     assert!(token.erc20.transfer(alice, U256::from(50)).is_ok());
//     assert_eq!(token.erc20.balance_of(alice), U256::from(100));
// }

// #[test]
// fn test_approve_zero_and_nonzero() {
//     let vm = TestVm::default();
//     let mut token = StylusToken::deploy(&vm);

//     let alice = addr(1);
//     let bob = addr(2);

//     vm.set_sender(alice);
//     token.mint(U256::from(100)).unwrap();

//     // Approve zero
//     assert!(token.erc20.approve(bob, U256::from(0)).is_ok());
//     assert_eq!(token.erc20.allowance(alice, bob), U256::from(0));

//     // Approve nonzero
//     assert!(token.erc20.approve(bob, U256::from(30)).is_ok());
//     assert_eq!(token.erc20.allowance(alice, bob), U256::from(30));
// }

// #[test]
// fn test_allowance_does_not_increase_on_transfer() {
//     let vm = TestVm::default();
//     let mut token = StylusToken::deploy(&vm);

//     let alice = addr(1);
//     let bob = addr(2);

//     vm.set_sender(alice);
//     token.mint(U256::from(100)).unwrap();
//     token.erc20.approve(bob, U256::from(50)).unwrap();

//     vm.set_sender(alice);
//     token.erc20.transfer(bob, U256::from(10)).unwrap();

//     // Allowance should not change on direct transfer
//     assert_eq!(token.erc20.allowance(alice, bob), U256::from(50));
// }

// #[test]
// fn test_transfer_from_exact_allowance() {
//     let vm = TestVm::default();
//     let mut token = StylusToken::deploy(&vm);

//     let alice = addr(1);
//     let bob = addr(2);
//     let carol = addr(3);

//     vm.set_sender(alice);
//     token.mint(U256::from(100)).unwrap();
//     token.erc20.approve(bob, U256::from(25)).unwrap();

//     vm.set_sender(bob);
//     assert!(token.erc20.transfer_from(alice, carol, U256::from(25)).is_ok());
//     assert_eq!(token.erc20.allowance(alice, bob), U256::from(0));
//     assert_eq!(token.erc20.balance_of(carol), U256::from(25));
//     assert_eq!(token.erc20.balance_of(alice), U256::from(75));
// }

// #[test]
// fn test_cannot_burn_from_other_account() {
//     let vm = TestVm::default();
//     let mut token = StylusToken::deploy(&vm);

//     let alice = addr(1);
//     let bob = addr(2);

//     vm.set_sender(alice);
//     token.mint(U256::from(100)).unwrap();

//     // Bob tries to burn Alice's tokens
//     vm.set_sender(bob);
//     let res = token.erc20.burn(alice, U256::from(10));
//     assert!(matches!(res, Err(Erc20Error::InsufficientBalance(_))));
// }

// #[test]
// fn test_multiple_approvals_and_transfers() {
//     let vm = TestVm::default();
//     let mut token = StylusToken::deploy(&vm);

//     let alice = addr(1);
//     let bob = addr(2);
//     let carol = addr(3);

//     vm.set_sender(alice);
//     token.mint(U256::from(100)).unwrap();
//     token.erc20.approve(bob, U256::from(40)).unwrap();
//     token.erc20.approve(carol, U256::from(30)).unwrap();

//     vm.set_sender(bob);
//     token.erc20.transfer_from(alice, bob, U256::from(20)).unwrap();

//     vm.set_sender(carol);
//     token.erc20.transfer_from(alice, carol, U256::from(10)).unwrap();

//     assert_eq!(token.erc20.balance_of(alice), U256::from(70));
//     assert_eq!(token.erc20.balance_of(bob), U256::from(20));
//     assert_eq!(token.erc20.balance_of(carol), U256::from(10));
//     assert_eq!(token.erc20.allowance(alice, bob), U256::from(20));
//     assert_eq!(token.erc20.allowance(alice, carol), U256::from(20));
// }