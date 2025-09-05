#![cfg_attr(not(any(test, feature = "export-abi")), no_main)]
#![cfg_attr(not(any(test, feature = "export-abi")), no_std)]

#[macro_use]
extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

use alloy_sol_types::SolValue;
use openzeppelin_stylus::token::erc721::{self, Erc721};

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
        mapping(uint256 => bytes32) seeds;
    }
}

sol! {
    error InsufficientPayment();
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
