#![cfg_attr(not(test), no_main)]
extern crate alloc;

use alloc::vec::Vec;

use alloy_primitives::{Address, FixedBytes, U256};
use openzeppelin_stylus::{
    token::erc20::{
        self,
        extensions::{capped, Capped, Erc20Metadata, IErc20Burnable},
        Erc20, IErc20,
    },
    utils::{introspection::erc165::IErc165, pausable, Pausable},
};
use stylus_sdk::prelude::*;

const DECIMALS: u8 = 10;

#[derive(SolidityError, Debug)]
enum Error {
    Capped(capped::Error),
    Erc20(erc20::Error),
    Pausable(pausable::Error),
}

#[entrypoint]
#[storage]
struct Erc20Example {
    #[borrow]
    erc20: Erc20,
    #[borrow]
    metadata: Erc20Metadata,
    #[borrow]
    capped: Capped,
    #[borrow]
    pausable: Pausable,
}

#[public]
#[inherit(Erc20, Erc20Metadata, Capped, Pausable)]
impl Erc20Example {
    fn decimals(&self) -> u8 {
        DECIMALS
    }

    fn burn(&mut self, value: U256) -> Result<(), Error> {
        self.pausable.when_not_paused()?;
        self.erc20.burn(value).map_err(|e| e.into())
    }

    fn burn_from(
        &mut self,
        account: Address,
        value: U256,
    ) -> Result<(), Error> {
        self.pausable.when_not_paused()?;
        self.erc20.burn_from(account, value).map_err(|e| e.into())
    }

    fn mint(&mut self, account: Address, value: U256) -> Result<(), Error> {
        self.pausable.when_not_paused()?;
        let max_supply = self.capped.cap();

        let supply = self
            .erc20
            .total_supply()
            .checked_add(value)
            .expect("new supply should not exceed `U256::MAX`");

        if supply > max_supply {
            return Err(capped::Error::ExceededCap(
                capped::ERC20ExceededCap {
                    increased_supply: supply,
                    cap: max_supply,
                },
            ))?;
        }

        self.erc20._mint(account, value)?;
        Ok(())
    }

    fn transfer(&mut self, to: Address, value: U256) -> Result<bool, Error> {
        self.pausable.when_not_paused()?;
        self.erc20.transfer(to, value).map_err(|e| e.into())
    }

    fn transfer_from(
        &mut self,
        from: Address,
        to: Address,
        value: U256,
    ) -> Result<bool, Error> {
        self.pausable.when_not_paused()?;
        self.erc20.transfer_from(from, to, value).map_err(|e| e.into())
    }

    fn supports_interface(interface_id: FixedBytes<4>) -> bool {
        Erc20::supports_interface(interface_id)
            || Erc20Metadata::supports_interface(interface_id)
    }

    fn pause(&mut self) -> Result<(), Error> {
        self.pausable.pause().map_err(|e| e.into())
    }

    fn unpause(&mut self) -> Result<(), Error> {
        self.pausable.unpause().map_err(|e| e.into())
    }
}
