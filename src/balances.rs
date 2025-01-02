use num::{CheckedAdd, CheckedSub, Zero};
use std::{collections::BTreeMap, fmt::Debug};

pub trait Config: crate::system::Config {
	type Balance: Zero + CheckedAdd + CheckedSub + Debug + Copy;
}

/**
 * "Pallet" is a term specific to the Polkadot SDK, which refers to Rust modules which contain
 * logic specific for your blockchain runtime. We are going to start using this term here
 * because what we build here will closely mirror what you will see with the Polkadot SDK.
 */
#[derive(Debug)]
pub struct Pallet<T: Config> {
	/**
	 * In the Polkadot SDK, there is a separate storage layer which manages a proper key-value
	 * database which holds all the information (past and present) of our blockchain system.
	 * There are abstractions which look and behave just like a BTreeMap in the Polkadot SDK,
	 * but the underlying logic which maintains that data is much more complex.
	 */
	balances: BTreeMap<T::AccountID, T::Balance>,
}

impl<T: Config> Pallet<T> {
	pub fn new() -> Self {
		Self { balances: BTreeMap::new() }
	}

	pub fn set_balance(&mut self, who: &T::AccountID, amount: T::Balance) {
		self.balances.insert(who.clone(), amount);
	}

	/**
	 * Get the balance of an account `who`. If the account has no stored balance, we return
	 * zero.
	 */
	pub fn balance(&self, who: &T::AccountID) -> T::Balance {
		*self.balances.get(who).unwrap_or(&T::Balance::zero())
	}

	/**
	 * Transfer `amount` from one account to another. This function verifies that `from` has at
	 * least `amount` balance to transfer, and that no mathematical overflows occur.
	 */

	pub fn transfer(
		&mut self,
		caller: T::AccountID,
		to: T::AccountID,
		amount: T::Balance,
	) -> crate::support::DispatchResult {
		let caller_balance = self.balance(&caller);
		let to_balance = self.balance(&to);
		let new_caller_balance = caller_balance.checked_sub(&amount).ok_or("Not enough funds")?;
		let new_to_balance = to_balance.checked_add(&amount).ok_or("Integer Overflow")?;
		self.set_balance(&caller, new_caller_balance);
		self.set_balance(&to, new_to_balance);

		Ok(())
	}
}

// A public enum which describes the calls we want to expose to the dispatcher.
// We should expect that the caller of each call will be provided by the dispatcher,
// and not included as a parameter of the call.
pub enum Call<T: Config> {
	Transfer { to: T::AccountID, amount: T::Balance },
}

/// Implementation of the dispatch logic, mapping from `BalancesCall` to the appropriate underlying
/// function we want to execute.
impl<T: Config> crate::support::Dispatch for Pallet<T> {
	type Caller = T::AccountID;
	type Call = Call<T>;

	fn dispatch(
		&mut self,
		caller: Self::Caller,
		call: Self::Call,
	) -> crate::support::DispatchResult {
		match call {
			Call::Transfer { to, amount } => {
				self.transfer(caller, to, amount)?;
			},
		}
		Ok(())
	}
}

#[cfg(test)]
mod tests {
	struct TestConfig;
	impl crate::system::Config for TestConfig {
		type AccountID = String;
		type BlockNumber = u32;
		type Nonce = u32;
	}
	impl super::Config for TestConfig {
		type Balance = u128;
	}

	#[test]
	fn init_balances() {
		/* TODO: Create a mutable variable `balances`, which is a new instance of `Pallet`. */
		let mut balances = super::Pallet::<TestConfig>::new();
		/* TODO: Assert that the balance of `alice` starts at zero. */
		assert_eq!(balances.balance(&"alice".to_string()), 0);
		/* TODO: Set the balance of `alice` to 100. */
		balances.set_balance(&"alice".to_string(), 100);
		/* TODO: Assert the balance of `alice` is now 100. */
		assert_eq!(balances.balance(&"alice".to_string()), 100);
		/* TODO: Assert the balance of `bob` has not changed and is 0. */
		assert_eq!(balances.balance(&"bob".to_string()), 0);
	}

	#[test]
	fn transfer_balance() {
		/* TODO: Create a test that checks the following:
			- That `alice` cannot transfer funds she does not have.
			- That `alice` can successfully transfer funds to `bob`.
			- That the balance of `alice` and `bob` is correctly updated.
		*/
		let mut balances = super::Pallet::<TestConfig>::new();
		balances.set_balance(&"Alice".to_string(), 100);
		balances.set_balance(&"Bob".to_string(), 50);

		let result = balances.transfer("Alice".to_string(), "Bob".to_string(), 150);
		assert!(result.is_err());
		assert_eq!(result.unwrap_err(), "Not enough funds");
		assert_eq!(balances.balance(&"Alice".to_string()), 100);
		assert_eq!(balances.balance(&"Bob".to_string()), 50);

		let result = balances.transfer("Alice".to_string(), "Bob".to_string(), 30);
		assert!(result.is_ok());
		assert_eq!(balances.balance(&"Alice".to_string()), 70);
		assert_eq!(balances.balance(&"Bob".to_string()), 80);
	}
}
