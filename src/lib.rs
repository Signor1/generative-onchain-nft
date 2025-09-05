#![cfg_attr(not(any(test, feature = "export-abi")), no_main)]
#![cfg_attr(not(any(test, feature = "export-abi")), no_std)]

mod base64;
mod generator;

#[macro_use]
extern crate alloc;

use std::collections::binary_heap;

use alloc::string::String;
use alloc::vec::Vec;

use alloy_sol_types::SolValue;
use openzeppelin_stylus::token::erc721::{self, extensions::consecutive, Erc721};

/// Import items from the SDK. The prelude contains common traits and macros.
use stylus_sdk::{
    alloy_primitives::{FixedBytes, U256},
    alloy_sol_types::sol,
    crypto::keccak,
    prelude::*,
};

// Define some persistent storage using the Solidity ABI.
sol_storage! {
    #[entrypoint]
    pub struct Squiggle {
        #[borrow]
        Erc721 erc721;

        uint256 mint_price;
        uint256 total_supply;
        address owner;
        mapping(uint256 => bytes32) seeds;
    }
}

sol! {
    error InsufficientPayment();
    error NotOwner();
}

#[derive(SolidityError)]
pub enum SquiggleError {
    InvalidOwner(erc721::ERC721InvalidOwner),
    NonexistentToken(erc721::ERC721NonexistentToken),
    IncorrectOwner(erc721::ERC721IncorrectOwner),
    InvalidSender(erc721::ERC721InvalidSender),
    InvalidReceiver(erc721::ERC721InvalidReceiver),
    InvalidReceiverWithReason(erc721::InvalidReceiverWithReason),
    InsufficientApproval(erc721::ERC721InsufficientApproval),
    InvalidApprover(erc721::ERC721InvalidApprover),
    InvalidOperator(erc721::ERC721InvalidOperator),
    InsufficientPayment(InsufficientPayment),
    NotOwner(NotOwner),
}

impl From<erc721::Error> for SquiggleError {
    fn from(value: erc721::Error) -> Self {
        match value {
            erc721::Error::IncorrectOwner(e) => SquiggleError::IncorrectOwner(e),
            erc721::Error::NonexistentToken(e) => SquiggleError::NonexistentToken(e),
            erc721::Error::InvalidOwner(e) => SquiggleError::InvalidOwner(e),
            erc721::Error::InvalidSender(e) => SquiggleError::InvalidSender(e),
            erc721::Error::InvalidReceiver(e) => SquiggleError::InvalidReceiver(e),
            erc721::Error::InvalidReceiverWithReason(e) => {
                SquiggleError::InvalidReceiverWithReason(e)
            }
            erc721::Error::InsufficientApproval(e) => SquiggleError::InsufficientApproval(e),
            erc721::Error::InvalidApprover(e) => SquiggleError::InvalidApprover(e),
            erc721::Error::InvalidOperator(e) => SquiggleError::InvalidOperator(e),
        }
    }
}

impl Squiggle {
    fn generate_seed(&self) -> FixedBytes<32> {
        let block_number = self.vm().block_number();
        let msg_sender = self.vm().msg_sender();
        let chain_id = self.vm().chain_id();

        let hash_data = (block_number, msg_sender, chain_id).abi_encode_sequence();

        keccak(&hash_data)
    }
}

#[public]
#[inherit(Erc721)]
impl Squiggle {
    #[constructor]
    fn constructor(&mut self, mint_price: U256) -> Result<(), SquiggleError> {
        let owner = self.vm().msg_sender();
        self.owner.set(owner);
        self.mint_price.set(mint_price);
        Ok(())
    }

    fn name(&self) -> String {
        String::from("Squiggle")
    }

    fn symbol(&self) -> String {
        String::from("SQGL")
    }

    #[selector(name = "tokenURI")]
    fn token_uri(&self, token_id: U256) -> Result<String, SquiggleError> {
        todo!()
    }

    #[payable]
    fn mint(&mut self) -> Result<(), SquiggleError> {
        let msg_value = self.vm().msg_value();
        let mint_price = self.mint_price.get();
        let minter = self.vm().msg_sender();

        if msg_value < mint_price {
            return Err(SquiggleError::InsufficientPayment(InsufficientPayment {}));
        }

        let seed = self.generate_seed();

        let token_id = self.total_supply.get();
        self.seeds.setter(token_id).set(seed);
        self.total_supply.set(token_id + U256::ONE);

        self.erc721._mint(minter, token_id)?;

        Ok(())
    }

    fn update_mint_price(&mut self, new_price: U256) -> Result<(), SquiggleError> {
        let fn_caller = self.vm().msg_sender();
        let owner = self.owner.get();

        if fn_caller != owner {
            return Err(SquiggleError::NotOwner(NotOwner {}));
        }
        self.mint_price.set(new_price);
        Ok(())
    }

    fn get_contract_balance(&self) -> Result<U256, SquiggleError> {
        if self.vm().msg_sender() != self.owner.get() {
            return Err(SquiggleError::NotOwner(NotOwner {}));
        }
        let contract = self.vm().contract_address();
        Ok(self.vm().balance(contract))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[no_mangle]
    pub unsafe extern "C" fn emit_log(_pointer: *const u8, _len: usize, _: usize) {}

    #[test]
    fn test_squiggle() {
        use stylus_sdk::testing::*;
        let vm = TestVM::default();
        let mut contract = Squiggle::from(&vm);

        todo!()
    }
}
