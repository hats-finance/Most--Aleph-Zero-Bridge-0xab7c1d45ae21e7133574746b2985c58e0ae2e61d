#![cfg_attr(not(feature = "std"), no_std, no_main)]

pub use self::token::TokenRef;

#[ink::contract]
pub mod token {
    use ink::prelude::{string::String, vec::Vec};
    use ownable2step::*;
    use psp22::{PSP22Data, PSP22Error, PSP22Event, PSP22Metadata, PSP22};
    use psp22_traits::{Burnable, Mintable};

    #[ink(event)]
    #[derive(Debug)]
    #[cfg_attr(feature = "std", derive(Eq, PartialEq))]
    pub struct Approval {
        #[ink(topic)]
        pub owner: AccountId,
        #[ink(topic)]
        pub spender: AccountId,
        pub amount: u128,
    }

    #[ink(event)]
    #[derive(Debug)]
    #[cfg_attr(feature = "std", derive(Eq, PartialEq))]
    pub struct Transfer {
        #[ink(topic)]
        pub from: Option<AccountId>,
        #[ink(topic)]
        pub to: Option<AccountId>,
        pub value: u128,
    }

    #[ink(event)]
    #[derive(Debug)]
    #[cfg_attr(feature = "std", derive(Eq, PartialEq))]
    pub struct TransferOwnershipInitiated {
        pub new_owner: AccountId,
    }

    #[ink(event)]
    #[derive(Debug)]
    #[cfg_attr(feature = "std", derive(Eq, PartialEq))]
    pub struct TransferOwnershipAccepted {
        pub new_owner: AccountId,
    }

    #[ink(storage)]
    pub struct Token {
        data: PSP22Data,
        ownable_data: Ownable2StepData,
        name: Option<String>,
        symbol: Option<String>,
        decimals: u8,
        minter_burner: AccountId,
    }

    impl Token {
        #[ink(constructor)]
        pub fn new(
            total_supply: u128,
            name: Option<String>,
            symbol: Option<String>,
            decimals: u8,
            minter_burner: AccountId,
        ) -> Self {
            let caller = Self::env().caller();
            let data = PSP22Data::new(total_supply, caller);
            let ownable_data = Ownable2StepData::new(caller);

            Self {
                data,
                ownable_data,
                name,
                symbol,
                decimals,
                minter_burner,
            }
        }

        #[ink(message)]
        pub fn set_minter_burner(
            &mut self,
            new_minter_burner: AccountId,
        ) -> Result<(), PSP22Error> {
            self.ensure_owner()?;
            self.minter_burner = new_minter_burner;
            Ok(())
        }

        fn ensure_owner(&self) -> Result<(), PSP22Error> {
            <Self as Ownable2Step>::ensure_owner(self)
                .map_err(|_| PSP22Error::Custom(String::from("Caller has to be the owner.")))
        }

        fn ensure_minter(&self) -> Result<(), PSP22Error> {
            if self.env().caller() != self.minter() {
                Err(PSP22Error::Custom(String::from(
                    "Caller has to be the minter.",
                )))
            } else {
                Ok(())
            }
        }

        fn ensure_burner(&self) -> Result<(), PSP22Error> {
            if self.env().caller() != self.burner() {
                Err(PSP22Error::Custom(String::from(
                    "Caller has to be the burner.",
                )))
            } else {
                Ok(())
            }
        }

        fn emit_events(&self, events: Vec<PSP22Event>) {
            for event in events {
                match event {
                    PSP22Event::Transfer { from, to, value } => {
                        self.env().emit_event(Transfer { from, to, value })
                    }
                    PSP22Event::Approval {
                        owner,
                        spender,
                        amount,
                    } => self.env().emit_event(Approval {
                        owner,
                        spender,
                        amount,
                    }),
                }
            }
        }
    }

    impl PSP22Metadata for Token {
        #[ink(message)]
        fn token_name(&self) -> Option<String> {
            self.name.clone()
        }

        #[ink(message)]
        fn token_symbol(&self) -> Option<String> {
            self.symbol.clone()
        }

        #[ink(message)]
        fn token_decimals(&self) -> u8 {
            self.decimals
        }
    }

    impl PSP22 for Token {
        #[ink(message)]
        fn total_supply(&self) -> u128 {
            self.data.total_supply()
        }

        #[ink(message)]
        fn balance_of(&self, owner: AccountId) -> u128 {
            self.data.balance_of(owner)
        }

        #[ink(message)]
        fn allowance(&self, owner: AccountId, spender: AccountId) -> u128 {
            self.data.allowance(owner, spender)
        }

        #[ink(message)]
        fn transfer(
            &mut self,
            to: AccountId,
            value: u128,
            _data: Vec<u8>,
        ) -> Result<(), PSP22Error> {
            let events = self.data.transfer(self.env().caller(), to, value)?;
            self.emit_events(events);
            Ok(())
        }

        #[ink(message)]
        fn transfer_from(
            &mut self,
            from: AccountId,
            to: AccountId,
            value: u128,
            _data: Vec<u8>,
        ) -> Result<(), PSP22Error> {
            let events = self
                .data
                .transfer_from(self.env().caller(), from, to, value)?;
            self.emit_events(events);
            Ok(())
        }

        #[ink(message)]
        fn approve(&mut self, spender: AccountId, value: u128) -> Result<(), PSP22Error> {
            let events = self.data.approve(self.env().caller(), spender, value)?;
            self.emit_events(events);
            Ok(())
        }

        #[ink(message)]
        fn increase_allowance(
            &mut self,
            spender: AccountId,
            delta_value: u128,
        ) -> Result<(), PSP22Error> {
            let events = self
                .data
                .increase_allowance(self.env().caller(), spender, delta_value)?;
            self.emit_events(events);
            Ok(())
        }

        #[ink(message)]
        fn decrease_allowance(
            &mut self,
            spender: AccountId,
            delta_value: u128,
        ) -> Result<(), PSP22Error> {
            let events = self
                .data
                .decrease_allowance(self.env().caller(), spender, delta_value)?;
            self.emit_events(events);
            Ok(())
        }
    }

    impl Mintable for Token {
        #[ink(message)]
        fn mint(&mut self, to: AccountId, value: u128) -> Result<(), PSP22Error> {
            self.ensure_minter()?;
            let events = self.data.mint(to, value)?;
            self.emit_events(events);
            Ok(())
        }

        #[ink(message)]
        fn minter(&self) -> AccountId {
            self.minter_burner
        }
    }

    impl Burnable for Token {
        #[ink(message)]
        fn burn(&mut self, value: u128) -> Result<(), PSP22Error> {
            self.ensure_burner()?;
            let events = self.data.burn(self.env().caller(), value)?;
            self.emit_events(events);
            Ok(())
        }

        #[ink(message)]
        fn burner(&self) -> AccountId {
            self.minter_burner
        }
    }

    impl Ownable2Step for Token {
        #[ink(message)]
        fn get_owner(&self) -> Ownable2StepResult<AccountId> {
            self.ownable_data.get_owner()
        }

        #[ink(message)]
        fn get_pending_owner(&self) -> Ownable2StepResult<AccountId> {
            self.ownable_data.get_pending_owner()
        }

        #[ink(message)]
        fn transfer_ownership(&mut self, new_owner: AccountId) -> Ownable2StepResult<()> {
            self.ownable_data
                .transfer_ownership(self.env().caller(), new_owner)?;
            self.env()
                .emit_event(TransferOwnershipInitiated { new_owner });
            Ok(())
        }

        #[ink(message)]
        fn accept_ownership(&mut self) -> Ownable2StepResult<()> {
            let new_owner = self.env().caller();
            self.ownable_data.accept_ownership(new_owner)?;
            self.env()
                .emit_event(TransferOwnershipInitiated { new_owner });
            Ok(())
        }

        #[ink(message)]
        fn ensure_owner(&self) -> Ownable2StepResult<()> {
            self.ownable_data.ensure_owner(self.env().caller())
        }
    }

    #[cfg(test)]
    mod tests {
        use ink::env::{test::*, DefaultEnvironment as E};
        use ownable2step::Ownable2StepError;

        use super::*;

        const INIT_SUPPLY_TEST: u128 = 1_000_000;

        psp22::tests!(Token, crate::token::tests::init_contract);

        #[ink::test]
        fn setting_owner_works() {
            let mut token = init_contract(INIT_SUPPLY_TEST);
            let alice = default_accounts::<E>().alice;
            let bob = default_accounts::<E>().bob;

            set_caller::<E>(alice);
            assert_eq!(token.get_owner(), Ok(alice));
            assert!(token.transfer_ownership(bob).is_ok());
            set_caller::<E>(bob);
            assert!(token.accept_ownership().is_ok());
            assert_eq!(token.get_owner(), Ok(bob));
        }

        #[ink::test]
        fn non_owner_cannot_change_owner() {
            let mut token = init_contract(INIT_SUPPLY_TEST);
            let alice = default_accounts::<E>().alice;
            let bob = default_accounts::<E>().bob;

            set_caller::<E>(bob);
            assert_eq!(
                token.transfer_ownership(alice),
                Err(Ownable2StepError::CallerNotOwner(bob)),
            );
        }

        #[ink::test]
        fn owner_can_set_minter_burner() {
            let mut token = init_contract(INIT_SUPPLY_TEST);
            let alice = default_accounts::<E>().alice;
            let bob = default_accounts::<E>().bob;

            set_caller::<E>(alice);
            assert!(token.set_minter_burner(bob).is_ok());
            assert_eq!(token.minter(), bob);
        }

        #[ink::test]
        fn non_owner_cannot_set_minter_burner() {
            let mut token = init_contract(INIT_SUPPLY_TEST);
            let bob = default_accounts::<E>().bob;

            set_caller::<E>(bob);
            assert_eq!(
                token.set_minter_burner(bob),
                Err(PSP22Error::Custom(String::from(
                    "Caller has to be the owner.",
                )))
            );
        }

        #[ink::test]
        fn minter_can_mint() {
            let mut token = init_contract(INIT_SUPPLY_TEST);
            let bob = default_accounts::<E>().bob;
            let charlie = default_accounts::<E>().charlie;
            let bob_balance_before = token.balance_of(bob);

            set_caller::<E>(charlie);
            assert!(token.mint(bob, 100).is_ok());
            assert_eq!(token.balance_of(bob), bob_balance_before + 100);
        }

        #[ink::test]
        fn non_minter_cannot_mint() {
            let mut token = init_contract(INIT_SUPPLY_TEST);
            let alice = default_accounts::<E>().alice;
            let bob = default_accounts::<E>().bob;

            set_caller::<E>(bob);
            assert_eq!(
                token.mint(alice, 100),
                Err(PSP22Error::Custom(String::from(
                    "Caller has to be the minter."
                )))
            );
        }

        #[ink::test]
        fn burner_can_burn() {
            let mut token = init_contract(INIT_SUPPLY_TEST);
            let alice = default_accounts::<E>().alice;
            let charlie = default_accounts::<E>().charlie;

            set_caller::<E>(alice);
            token.transfer(charlie, 100, vec![]).unwrap();

            let charlie_balance_before = token.balance_of(charlie);
            set_caller::<E>(charlie);
            assert!(token.burn(100).is_ok());
            assert_eq!(token.balance_of(charlie), charlie_balance_before - 100);
        }

        #[ink::test]
        fn non_burner_cannot_burn() {
            let mut token = init_contract(INIT_SUPPLY_TEST);
            let bob = default_accounts::<E>().bob;

            set_caller::<E>(bob);
            assert_eq!(
                token.burn(100),
                Err(PSP22Error::Custom(String::from(
                    "Caller has to be the burner."
                )))
            );
        }

        fn init_contract(init_supply: u128) -> Token {
            set_caller::<E>(default_accounts::<E>().alice);
            Token::new(
                init_supply,
                Some(String::from("MOST wrapped Ether")),
                Some(String::from("mETH")),
                18,
                default_accounts::<E>().charlie,
            )
        }
    }
}
