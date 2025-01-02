mod balances;
mod support;
mod system;

// These are the concrete types we will use in our simple state machine.
// Modules are configured for these types directly, and they satisfy all of our
// trait requirements.
mod types {
	pub type AccountID = String;
	pub type Balance = u128;
	pub type BlockNumber = u32;
	pub type Nonce = u32;
	pub type Extrinsic = crate::support::Extrinsic<AccountId, crate::RuntimeCall>;
	pub type Header = crate::support::Header<BlockNumber>;
	pub type Block = crate::support::Block<Header, Extrinsic>;
}

// These are all the calls which are exposed to the world.
// Note that it is just an accumulation of the calls exposed by each module.
pub enum RuntimeCall {
	// TODO: Not implemented yet.
}

// This is our main Runtime.
// It accumulates all of the different pallets we want to use.
#[derive(Debug)]
pub struct Runtime {
	/* TODO:
		- Create a field `system` which is of type `system::Pallet`.
		- Create a field `balances` which is of type `balances::Pallet`.
	*/
	system: system::Pallet<Self>,
	balances: balances::Pallet<Self>,
}

impl Runtime {
	// Create a new instance of the main Runtime, by creating a new instance of each pallet.
	fn new() -> Self {
		/* TODO: Create a new `Runtime` by creating new instances of `system` and `balances`. */
		Self { system: system::Pallet::<Self>::new(), balances: balances::Pallet::<Self>::new() }
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

fn main() {
	/* TODO: Create a mutable variable `runtime`, which is a new instance of `Runtime`. */
	let mut runtime = Runtime::new();
	/* TODO: Set the balance of `alice` to 100, allowing us to execute other transactions. */
	runtime.balances.set_balance(&"Alice".to_string(), 100);

	// start emulating a block
	runtime.system.inc_block_number();
	assert_eq!(runtime.system.block_number(), 1);

	// first transaction
	runtime.system.inc_nonce(&"Alice".to_string());
	/* TODO: Execute a transfer from `alice` to `bob` for 30 tokens.
		- The transfer _could_ return an error. We should use `map_err` to print
		  the error if there is one.
		- We should capture the result of the transfer in an unused variable like `_res`.
	*/

	let _res =
		runtime
			.balances
			.transfer("Alice".to_string(), "Bob".to_string(), 30)
			.map_err(|err| {
				println!("Transfer failed with error: {}", err);
			});

	// second transaction
	/* TODO: Increment the nonce of `alice` again. */
	runtime.system.inc_nonce(&"Alice".to_string());
	/* TODO: Execute another balance transfer, this time from `alice` to `charlie` for 20. */
	let _res = runtime
		.balances
		.transfer("Alice".to_string(), "Charlie".to_string(), 20)
		.map_err(|err| {
			println!("Transfer failed with error: {}", err);
		});
	// runtime.balances.print_balances();

	println!("{:#?}", runtime);
}
