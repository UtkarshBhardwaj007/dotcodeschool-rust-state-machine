# Dot Code School Solutions

This repository contains my solutions for the [DotCodeSchool Courses](https://dotcodeschool.com/). It also contains some notes I made along the way.

## State Machine Concept:

A state machine is a computational model that can be in exactly one of a finite number of states at any given time
It transitions between these states based on specific rules or inputs
In computing, state machines are used to model systems with well-defined, predictable behavior

## State in Blockchain
### Global State: 
Represents the entire data of the blockchain at a specific point in time. This includes account balances, smart contract data, and more.
### State Transition Function: 
The rules that define how the blockchain moves from one state to another based on transactions.

## State Transitions:
Transactions cause state transitions
Each transaction is like an input that moves the blockchain from one valid state to another.
These transitions follow strict consensus rules.
Every node in the network applies the same state transition logic to maintain consistency

## Genesis State in Blockchains:

The Genesis State is the initial state of a blockchain. It represents the very first block, often referred to as "block 0" or the "genesis block." This block is unique because it is the only block that does not reference a previous block, as it is the starting point of the blockchain. It contains: Initial Accounts and Balances, Network Parameters, Smart Contracts and Code, and any Configuration Settings.

## Traits
Traits can contain within it two things:

* functions which must be implemented by the type
* associated types

## Pallet:
"Pallet" is a term specific to the Polkadot SDK, which refers to Rust modules which contain logic specific for your blockchain runtime.

## &'static str:
In Rust, `&'static str` is a type that represents a string slice with a `'static` lifetime. A `'static` lifetime means that the data is either embedded directly in the program's binary (like string literals) or is explicitly allocated to last for the entire program's execution.

## Blockchain Client:
The blockchain client is the software component responsible for managing the network interactions, consensus mechanism, peer-to-peer communication, transaction propagation, block production, and overall coordination of the blockchain nodes.

## State transition function
The state transition function defines how the blockchain's state changes in response to transactions and blocks. It encapsulates the business logic, rules, and operations that govern the blockchain's behavior. (If applicable) Manages the execution and state of smart contracts.

```mermaid
sequenceDiagram
    participant Client as Blockchain Client
    participant STF as State Transition Function
    participant DB as State Database
    participant Peers as Other Nodes
    
    %% Initialization
    Client->>Client: Initialize networking stack
    Client->>Client: Set up consensus mechanisms
    Client->>STF: Load STF (runtime) from Wasm
    
    %% Block Reception
    Peers->>Client: Receive new blocks
    Client->>Client: Verify block's validity based on consensus rules
    
    %% State Transition Execution
    Client->>STF: Invoke STF to process block's transactions
    STF->>STF: Read current state
    STF->>STF: Apply transactions and compute new state
    
    %% State Storage
    STF-->>Client: Return new state
    Client->>DB: Store new state in permanent state database (e.g., RocksDB)
    
    %% Propagation
    Client->>Peers: Propagate validated blocks to other nodes
    
    %% Runtime Upgrades
    Client->>Client: Detect runtime upgrade proposal (e.g., adding a new pallet)
    Client->>Peers: Update STF via on-chain governance
    Peers->>Client: Download new Wasm runtime
    Client->>STF: Load and use the new Wasm runtime for subsequent blocks
```

## DotCodeSchool Lecture flow:

### Balances Pallet:

* At the heart of a blockchain is a state machine.
* This Pallet will tell you: how much balance each user has, provide functions which allow users to transfer those balances, and even some low level functions to allow your blockchain system to manipulate those balances if needed.
* We can add state to our pallet by adding fields (like balances{BTreeMap}) into our Pallet struct.
* Add functions to our pallet to allow users to interact with the state.

### System Pallet
* The System Pallet is a "meta"-pallet which stores all the metadata needed for your blockchain to function. For example, the current blocknumber or the nonce of users on your blockchain.
* This pallet does not need to expose any functions to end users, but can still play an important role in our overall state transition function.
* System Pallet needs to expose functions which allow us to access and modify the **block number** and the **nonce**.
* **Block number**: Your blockchain's blocknumber stored in the System Pallet.
* **Nonce**: In this context, each user on your blockchain has a nonce which gives a unique value to each transaction the user submits to the blockchain. We keep track of 'nonce':'count_of_transactions' in a BTreeMap.

### Runtime Pallet
* You can think of the runtime as the accumulation of all logic which composes your state transition function. It will combine all of your pallets into a single object, and then expose that single object as the entry point for your users to interact with.
* The runtime contains the **System Pallet** and the **Balances Pallet**.

### Using Named And Generic Types and making them configurable:
* We use named types to clearly define what a type represents. We make these named types generic so that we can use them with different types.
* We implement a `Config` trait with associated types. Then we can use a single generic parameter `T` in our structs and have a trait bound of `Config` on `T`. This way, we can access datatypes from `T` like `T::AccountId` and `T::Balance` in our structs.
* We can also use **Trait Inheritance** to keep the repeatition to a minimum like - `pub trait Config: crate::system::Config {}`. However, we need to be aware of any **Tight Coupling**. In fact, with Substrate, all pallets are tightly coupled to the System Pallet, because the System Pallet provides all the meta-types for your blockchain system.

### Support Pallet
* The `support` module parallels something similar to the `frame_support` crate that you would find in the `Polkadot SDK`. The reason the `frame_support` crate exists, is to allow multiple other crates use common types and trait, while avoiding cyclic dependencies, which is not allowed in Rust.


