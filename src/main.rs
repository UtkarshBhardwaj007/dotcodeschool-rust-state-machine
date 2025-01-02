mod balances;
mod support;
mod system;

use crate::support::Dispatch;

// These are the concrete types we will use in our simple state machine.
// Modules are configured for these types directly, and they satisfy all of our
// trait requirements.
mod types {
	pub type AccountID = String;
	pub type Balance = u128;
	pub type BlockNumber = u32;
	pub type Nonce = u32;
	pub type Extrinsic = crate::support::Extrinsic<AccountID, crate::RuntimeCall>;
	pub type Header = crate::support::Header<BlockNumber>;
	pub type Block = crate::support::Block<Header, Extrinsic>;
}

// These are all the calls which are exposed to the world.
// Note that it is just an accumulation of the calls exposed by each module.
pub enum RuntimeCall {
	BalancesTransfer { to: types::AccountID, amount: types::Balance },
}

// This is our main Runtime.
// It accumulates all of the different pallets we want to use.
#[derive(Debug)]
pub struct Runtime {
	system: system::Pallet<Self>,
	balances: balances::Pallet<Self>,
}

impl Runtime {
	// Create a new instance of the main Runtime, by creating a new instance of each pallet.
	fn new() -> Self {
		Self { system: system::Pallet::<Self>::new(), balances: balances::Pallet::<Self>::new() }
	}

	// Execute a block of extrinsics. Increments the block number.
	fn execute_block(&mut self, block: types::Block) -> support::DispatchResult {
		// Increment the system's block number.
		self.system.inc_block_number();

		// Check that the block number of the incoming block matches the current block number,
		// or return an error.
		if block.header.block_number != self.system.block_number() {
			return Err("block number does not match what is expected");
		}

		// Iterate over the extrinsics in the block
		for (i, support::Extrinsic { caller, call }) in block.extrinsics.into_iter().enumerate() {
			// Increment the nonce of the caller.
			self.system.inc_nonce(&caller);
			// Dispatch the call.
			let _res = self.dispatch(caller, call).map_err(|e| {
				eprintln!(
					"Extrinsic Error\n\tBlock Number: {}\n\tExtrinsic Number: {}\n\tError: {}",
					block.header.block_number, i, e
				)
			});
		}
		Ok(())
	}
}

impl system::Config for Runtime {
	type BlockNumber = types::BlockNumber;
	type AccountID = types::AccountID;
	type Nonce = types::Nonce;
}

impl balances::Config for Runtime {
	type Balance = types::Balance;
}

impl support::Dispatch for Runtime {
	type Caller = types::AccountID;
	type Call = RuntimeCall;
	// Dispatch a call on behalf of a caller. Increments the caller's nonce.
	//
	// Dispatch allows us to identify which underlying module call we want to execute.
	// Note that we extract the `caller` from the extrinsic, and use that information
	// to determine who we are executing the call on behalf of.
	fn dispatch(
		&mut self,
		caller: Self::Caller,
		runtime_call: Self::Call,
	) -> support::DispatchResult {
		match runtime_call {
			RuntimeCall::BalancesTransfer { to, amount } => {
				self.balances.transfer(caller, to, amount)?;
			},
		}
		Ok(())
	}
}

fn main() {
	// Create a mutable variable `runtime`, which is a new instance of `Runtime`.
	let mut runtime = Runtime::new();
	// Set the balance of `alice` to 100, allowing us to execute other transactions.
	runtime.balances.set_balance(&"Alice".to_string(), 100);

	// start emulating a block
	runtime.system.inc_block_number();
	assert_eq!(runtime.system.block_number(), 1);

	// first transaction
	runtime.system.inc_nonce(&"Alice".to_string());

	let _res =
		runtime
			.balances
			.transfer("Alice".to_string(), "Bob".to_string(), 30)
			.map_err(|err| {
				println!("Transfer failed with error: {}", err);
			});

	// second transaction
	runtime.system.inc_nonce(&"Alice".to_string());
	let _res = runtime
		.balances
		.transfer("Alice".to_string(), "Charlie".to_string(), 20)
		.map_err(|err| {
			println!("Transfer failed with error: {}", err);
		});

	println!("{:#?}", runtime);
}
