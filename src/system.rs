use num::{One, Zero};
use std::{collections::BTreeMap, fmt::Debug, ops::AddAssign};

pub trait Config {
	type BlockNumber: Zero + AddAssign + One + Clone;
	type AccountId: ToString + Ord + Clone + Debug;
	type Nonce: Zero + AddAssign + One;
}

/**
 * The System Pallet is a "meta"-pallet which stores all the metadata needed for your blockchain
 * to function. For example, the current blocknumber or the nonce of users on your blockchain.
 * This pallet does not need to expose any functions to end users, but can still play an
 * important role in our overall state transition function. System Pallet handles low level
 * state needed for your blockchain.
 */

#[derive(Debug)]
pub struct Pallet<T: Config> {
	/**
	 * The current block number. TODO: Create a field `block_number` that stores a `u32`.
	 * A map from an account to their nonce. TODO: Create a field `nonce` that is a `BTreeMap`
	 * from `String` to `u32`.
	 */
	block_number: T::BlockNumber,
	nonce: BTreeMap<T::AccountId, T::Nonce>,
}

impl<T: Config> Pallet<T> {
	// Create a new instance of the System Pallet.
	pub fn new() -> Self {
		/* TODO: Return a new instance of the `Pallet` struct. */
		Self { block_number: T::BlockNumber::zero(), nonce: BTreeMap::new() }
	}

	// Get the current block number.
	pub fn block_number(&self) -> T::BlockNumber {
		/* TODO: Return the current block number. */
		self.block_number.clone()
	}

	// This function can be used to increment the block number. Increases the block number by one.
	pub fn inc_block_number(&mut self) {
		/* TODO: Increment the current block number by one. */
		self.block_number += T::BlockNumber::one();
	}

	// Increment the nonce of an account. This helps us keep track of how many transactions each
	// account has made.
	pub fn inc_nonce(&mut self, who: &T::AccountId) {
		/* TODO: Get the current nonce of `who`, and increment it by one. */
		*self.nonce.entry(who.clone()).or_insert(T::Nonce::zero()) += T::Nonce::one();
	}
}

#[cfg(test)]
mod test {

	#[test]
	fn init_system() {
		/* TODO: Create a test which checks the following:
			- Increment the current block number.
			- Increment the nonce of `alice`.

			- Check the block number is what we expect.
			- Check the nonce of `alice` is what we expect.
			- Check the nonce of `bob` is what we expect.
		*/
		struct TestConfig;
		impl super::Config for TestConfig {
			type AccountId = String;
			type BlockNumber = u32;
			type Nonce = u32;
		}
		let mut system = super::Pallet::<TestConfig>::new();
		system.inc_block_number();
		system.inc_nonce(&"Alice".to_string());
		assert_eq!(system.block_number(), 1);
		assert_eq!(system.nonce.get("Alice"), Some(&1));
		assert_eq!(system.nonce.get("Bob"), None);
	}
}
