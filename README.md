# TREX

TREX is a project to develop a protocol and network to support permissionless & trustless 
timed-release encryption in Web3. :rocket:

### Using Nix

Install [nix](https://nixos.org/) and optionally [direnv](https://github.com/direnv/direnv) and
[lorri](https://github.com/nix-community/lorri) for a fully plug and play experience for setting up
the development environment. To get all the correct dependencies activate direnv `direnv allow` and
lorri `lorri shell`.

### Setup

First, complete the [basic Rust setup instructions](./docs/rust-setup.md).

Make sure that Google [Protocol Buffers](https://developers.google.com/protocol-buffers) were installed in the machine. 

### Run

Use Rust's native `cargo` command to build and launch the template node:

```sh
cargo run --release -- --dev
```

### Build

Note: The current code is built OK on Ubuntu 22.04 with current [rust toolchain](./rust-toolchain.toml).
It has errors when linking with "rust-lld" on macOS 13.0.1.

The `cargo run` command will perform an initial build. Use the following command to build the node
without launching it:

- If you can perform Remote Attestation:
  ```
  cargo build --release
  ```
- If you can **NOT** perform Remote Attestation:
  ```
  cargo build --release --features "skip-ias-check"
  ```

### Embedded Docs

Once the project has been built, the following command can be used to explore all parameters and
subcommands:

```sh
./target/release/trex -h
```
## Unit Test
All core functions are covered by unit tests. To test the core functions, simply run
```shell
cargo test
```
If using a container to run the unit test and develop inside a container, you may pull or build a prebuilt image 
as [run in docker section](#run-in-docker).
Once you have the image, assume the name of the image is `trex-node:prebuild`. You may run unittest inside the container.
```shell
docker run -it trexnode.azurecr.io/trex-node:prebuild
cd trex && cargo test
```

## Run

The provided `cargo run` command will launch a temporary node and its state will be discarded after
you terminate the process. After the project has been built, there are other ways to launch the
node.

### Single-Node Development Chain

This command will start the single-node development chain with non-persistent state:

```bash
./target/release/trex --dev
```

Purge the development chain's state:

```bash
./target/release/trex purge-chain --dev
```

Start the development chain with detailed logging:

```bash
RUST_BACKTRACE=1 ./target/release/trex -ldebug --dev
```

> Development chain means that the state of our chain will be in a tmp folder while the nodes are
> running. Also, **alice** account will be authority and sudo account as declared in the
> [genesis state](https://github.com/substrate-developer-hub/substrate-node-template/blob/main/node/src/chain_spec.rs#L49).
> At the same time the following accounts will be pre-funded:
> - Alice
> - Bob
> - Alice//stash
> - Bob//stash

In case of being interested in maintaining the chain' state between runs a base path must be added
so the db can be stored in the provided folder instead of a temporal one. We could use this folder
to store different chain databases, as a different folder will be created per different chain that
is ran. The following commands shows how to use a newly created folder as our db base path.

```bash
// Create a folder to use as the db base path
$ mkdir my-chain-state

// Use of that folder to store the chain state
$ ./target/release/trex --dev --base-path ./my-chain-state/

// Check the folder structure created inside the base path after running the chain
$ ls ./my-chain-state
chains
$ ls ./my-chain-state/chains/
dev
$ ls ./my-chain-state/chains/dev
db keystore network
```


### Connect with Polkadot-JS Apps Front-end

Once the TREX node is running locally, you can connect it with **Polkadot-JS Apps** front-end
to interact with your chain. [Click
here](https://polkadot.js.org/apps/#/explorer?rpc=ws://localhost:9944) connecting the Apps to your
local TREX node.

### Multi-Node Local Testnet

If you want to see the multi-node consensus algorithm in action, refer to our
[Simulate a network tutorial](https://docs.substrate.io/tutorials/get-started/simulate-network/).

## TREX Node Project Structure

The TREX project consists of a number of components that are spread across a few
directories.

### Node

A TREX node is an application that allows users to participate in a blockchain network.
The TREX blockchain nodes expose a number of capabilities:

- Networking: TREX nodes use the [`libp2p`](https://libp2p.io/) networking stack to allow the
  nodes in the network to communicate with one another.
- Consensus: The TREX network uses [Aura](https://docs.substrate.io/reference/glossary/#authority-round-aura) algorithm 
for authoring blocks and the [GRANDPA](https://paritytech.github.io/substrate/master/sc_finality_grandpa/index.html) 
algorithm for block finalization that have been built on top of
  [Web3 Foundation research](https://research.web3.foundation/en/latest/polkadot/NPoS/index.html).
- RPC Server: A remote procedure call (RPC) server is used to interact with TREX nodes.
The frontend applications will also use RPC calls to access key-holder data for threshold encryption.

There are several files in the `node` directory - take special note of the following:

- [`chain_spec.rs`](./node/src/chain_spec.rs): A
  [chain specification](https://docs.substrate.io/main-docs/build/chain-spec/) is a
  source code file that defines a TREX blockchain's initial (genesis) state. Chain specifications
  are useful for development and testing, and critical when architecting the launch of a
  production chain. Take note of the `development_config` and `testnet_genesis` functions, which
  are used to define the genesis state for the local development chain configuration. These
  functions identify some
  [well-known accounts](https://docs.substrate.io/reference/command-line-tools/subkey/)
  and use them to configure the blockchain's initial state.
- [`service.rs`](./node/src/service.rs): This file defines the node implementation. Take note of
  the libraries that this file imports and the names of the functions it invokes. In particular,
  there are references to consensus-related topics, such as the
  [block finalization and forks](https://docs.substrate.io/main-docs/fundamentals/consensus/#finalization-and-forks)
  and other [consensus mechanisms](https://docs.substrate.io/main-docs/fundamentals/consensus/#default-consensus-models)
  such as Aura for block authoring and GRANDPA for finality.

After the node has been [built](#build), refer to the embedded documentation to learn more about the
capabilities and configuration parameters that it exposes:

```shell
./target/release/trex --help
```

### Runtime

In TREX network, the terms "runtime" and "state transition function"
are analogous - they refer to the core logic of the blockchain that is responsible for validating
blocks and executing the state changes they define. The TREX project in this repository uses
[FRAME](https://docs.substrate.io/main-docs/fundamentals/runtime-intro/#frame) to construct a
blockchain runtime. FRAME allows runtime developers to declare domain-specific logic in modules
called "pallets". At the heart of FRAME is a helpful
[macro language](https://docs.substrate.io/reference/frame-macros/) that makes it easy to
create pallets and flexibly compose them to create blockchains that can address
[a variety of needs](https://substrate.io/ecosystem/projects/).

Review the [FRAME runtime implementation](./runtime/src/lib.rs) included in this project and note
the following:

- This file configures several pallets to include in the runtime. Each pallet configuration is
  defined by a code block that begins with `impl $PALLET_NAME::Config for Runtime`.
- The pallets are composed into a single runtime by way of the
  [`construct_runtime!`](https://crates.parity.io/frame_support/macro.construct_runtime.html)
  macro, which is part of the core
  FRAME Support [system](https://docs.substrate.io/reference/frame-pallets/#system-pallets) library.

### Pallets

The runtime in this project is constructed using many FRAME pallets that ship with the
[core Substrate repository](https://github.com/paritytech/substrate/tree/master/frame)

We also create two specific pallets for the TREX network and its off-chain workers.
#### TEE Pallet
This pallet is responsible for verification of remote attestation from off-chain workers, which are also called as 
key-holders in the TREX network.

- Storage: The on-chain storage keeps the verification status of each off-chain worker and its RSA public key for shielding
 the data communication between nodes and off-chain workers.
- Dispatchables: Runtime APIs to register, unregister an off-chain worker with Intel SGX enclaves, and query the status 
of off-chain workers.
- Events: A event is emitted when an off-chain worker is registered or unregistered.
- Errors: When registration of a new key-holder fails, it returns an error.

#### TREX Pallet
This pallet supports the core functionality of the TREX network - decentralized timed-release encryption.

- Dispatchables: Runtime APIs to send encrypted data with shielded key pieces and decrypt data with key pieces released 
from off-chain workers.
- Events: A event is emitted when encrypted data were sent to the blockchain and the key-holder will handle the event 
and hold corresponding keys in the enclave.
- Errors: When sending TREX data fails, it returns an error.

### Build Docker Image
First, install [Docker](https://docs.docker.com/get-docker/) and
[Docker Compose](https://docs.docker.com/compose/install/).
You may build docker images for deployments and test.
Run the following command to build a docker image for deployment.
```bash
docker build -t trex-node:latest .
```

For supporting integration tests in non-sgx environment with keyholder nodes, you may use the following 
command to build the test image. The corresponding keyholder nodes may run in software-simulated 
SGX mode, thus, **the IAS check need to be skipped**. This image is only for integration test alongside the keyholder
but not unit tests.
```shell
docker build --build-arg FEATURES="skip-ias-check" -t trex-node:test .
```

For unit tests inside a container, we have another image tagged as "prebuild".
```shell
docker build -f docker/prebuild.Dockerfile -t trex-node:prebuild .
```

### Run in Docker

You may pull the latest image from a public container registry instead of building it by yourself.
Use the following command to pull a pre-built image.

For deployment:
```shell
docker pull trexnode.azurecr.io/trex-node:latest
```
For integration test:
```shell
docker pull trexnode.azurecr.io/trex-node:test
```
For unit test:
```shell
docker pull trexnode.azurecr.io/trex-node:prebuild
```
Remember to sign out with `docker logout` from your existing login credentials if you have errors.

Then run the following command to start a single node development chain.
```bash
./scripts/docker_run.sh
```

This command will use the pre-compiled executable in the image, and then start a local development network. 
By default, it supports external WebSocket connection (for key-holder connections) and external RPC methods 
(for user queries). 

You can also replace the default command
(`--dev --ws-external --rpc-cors all --rpc-methods=unsafe --rpc-external`)
by appending your own. Based on your application and deployment environments, you may turn off WebSocket or RPC supports 
for enhanced safety.

A few useful ones are as follows.

```bash
# Run TREX node without RPC supports
./scripts/docker_run.sh --dev --ws-external

# Purge the local dev chain
./scripts/docker_run.sh purge-chain --dev
```
