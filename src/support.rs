/**
 * The reason the frame_support crate exists, is to allow multiple other crates use common types
 * and trait, while avoiding cyclic dependencies, which is not allowed in Rust.
 */

/// The most primitive representation of a Blockchain block.
pub struct Block<Header, Extrinsic> {
	/**
	 * Block is generic over Header and Extrinsic type. These are not teh same as the Header
	 * and Extrinsic structs defined below.
	 * */
	/// The block header contains metadata about the block.
	pub header: Header,
	/**
	 * Extrinsic literally means "coming from the outside. In the context of a blockchain, it
	 * denotes any input that comes from outside the core state of the blockchain.
	 * */
	/// The extrinsics represent the state transitions to be executed in this block.
	pub extrinsics: Vec<Extrinsic>,
}

/// We are using an extremely simplified header which only contains the current block number.
/// On a real blockchain, you would expect to also find:
/// - parent block hash
/// - state root
/// - extrinsics root
/// - etc...
pub struct Header<BlockNumber> {
	pub block_number: BlockNumber,
}

/// This is an "extrinsic": literally an external message from outside of the blockchain.
/// This simplified version of an extrinsic tells us who is making the call, and which call they are
/// making.
pub struct Extrinsic<Caller, Call> {
	pub caller: Caller,
	pub call: Call,
}

/// The Result type for our runtime. When everything completes successfully, we return `Ok(())`,
/// otherwise return a static error message.
pub type DispatchResult = Result<(), &'static str>;

/// A trait which allows us to dispatch an incoming extrinsic to the appropriate state transition
/// function call.
pub trait Dispatch {
	/// The type used to identify the caller of the function.
	type Caller;
	/// The state transition function call the caller is trying to access.
	type Call;

	/// This function takes a `caller` and the `call` they want to make, and returns a `Result`
	/// based on the outcome of that function call.
	fn dispatch(&mut self, caller: Self::Caller, call: Self::Call) -> DispatchResult;
}
