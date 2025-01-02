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
	Balances(balances::Call<Runtime>),
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
			RuntimeCall::Balances(call) => {
				self.balances.dispatch(caller, call)?;
			},
		}
		Ok(())
	}
}

fn main() {
	// Create a new instance of the Runtime.
	// It will instantiate with it all the modules it uses.
	let mut runtime = Runtime::new();
	let alice = "alice".to_string();
	let bob = "bob".to_string();
	let charlie = "charlie".to_string();

	// Initialize the system with some initial balance.
	runtime.balances.set_balance(&alice, 100);

	// // start emulating a block
	// runtime.system.inc_block_number();
	// assert_eq!(runtime.system.block_number(), 1);

	// // first transaction
	// runtime.system.inc_nonce(&alice);
	// let _res = runtime
	// 	.balances
	// 	.transfer(alice.clone(), bob, 30)
	// 	.map_err(|e| eprintln!("{}", e));

	// // second transaction
	// runtime.system.inc_nonce(&alice);
	// let _res = runtime.balances.transfer(alice, charlie, 20).map_err(|e| eprintln!("{}", e));

	/*
		Replace the logic above with a new `Block`.
			- Set the block number to 1 in the `Header`.
			- Move your existing transactions into extrinsic format, using the
			  `Extrinsic` and `RuntimeCall`.
	*/
	let block_1 = types::Block {
		header: types::Header { block_number: 1 },
		extrinsics: vec![
			support::Extrinsic {
				caller: alice.clone(),
				call: RuntimeCall::Balances(balances::Call::Transfer { to: bob, amount: 30 }),
			},
			support::Extrinsic {
				caller: alice.clone(),
				call: RuntimeCall::Balances(balances::Call::Transfer { to: charlie, amount: 20 }),
			},
		],
	};

	/*
		Use your `runtime` to call the `execute_block` function with your new block.
		If the `execute_block` function returns an error, you should panic!
		We `expect` that all the blocks being executed must be valid.
	*/
	runtime.execute_block(block_1).expect("invalid block");
	// Simply print the debug format of our runtime state.
	println!("{:#?}", runtime);
}
