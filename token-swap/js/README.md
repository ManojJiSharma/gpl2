# Token-swap JavaScript API

The Token-swap JavaScript library comprises:

* A library to interact with the on-chain program
* A test client that exercises the program
* Scripts to facilitate building the program

## Getting Started

First fetch the npm dependencies, including `@gemachain/web3.js`, by running:
```sh
$ npm install
```

### Select a Network

The client connects to a local Gemachain cluster by default.

To enable on-chain program logs, set the `RUST_LOG` environment variable:

```bash
$ export RUST_LOG=gemachain_runtime::native_loader=trace,gemachain_runtime::system_instruction_processor=trace,gemachain_runtime::bank=debug,gemachain_bpf_loader=debug,gemachain_rbpf=debug
```

To start a local Gemachain cluster run:
```bash
$ npm run localnet:update
$ npm run localnet:up
```

Gemachain cluster logs are available with:
```bash
$ npm run localnet:logs
```

For more details on working with a local cluster, see the [full
instructions](https://github.com/gemachain-labs/gemachain-web3.js#local-network).

### Build the on-chain program

```bash
$ npm run build:program
```

### Run the test client

```sh
$ npm run start
```

## Pointing to a public Gemachain cluster

Gemachain maintains three public clusters:
- `devnet` - Development cluster with airdrops enabled
- `testnet` - Tour De gema test cluster without airdrops enabled
- `mainnet-beta` -  Main cluster

Use npm scripts to configure which cluster.

To point to `devnet`:
```bash
$ npm run cluster:devnet
```

To point back to the local cluster:
```bash
$ npm run cluster:localnet
```
