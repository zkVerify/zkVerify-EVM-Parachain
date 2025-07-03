# VFlow

The **VFlow Parachain** is an EVM Parachain built on top of the [zkVerify Relay Chain](https://github.com/zkVerify/zkVerify), specifically designed to act as a gateway, for zkVerify, to the EVM world.

👉 Learn more about parachains [here](https://wiki.polkadot.network/docs/learn-parachains), and parathreads [here](https://wiki.polkadot.network/docs/learn-parathreads).

## What is VFlow

The primary purpose of VFlow is to enable bridging back and forth, from zkVerify to any EVM Chain, the VFY Token, leveraging Layer Zero.
As such, VFlow is a *permissioned* EVM where only a *technical committee* is entitled to deploy contracts.
Any other user interaction with such contracts, and with the chain in general, is allowed instead.

Future plans to fully open up the chain are under evaluation. Make sure to follow [zkVerify.io](https://zkverify.io/) for updates in this regard.

## EVM Compatibility

VFlow was built starting from the [OpenZeppelin EVM Template](https://github.com/OpenZeppelin/polkadot-runtime-templates/tree/main/evm-template) and [Moonbeam](https://moonbeam.network/) [fork](https://github.com/moonbeam-foundation/frontier) of [Frontier](https://github.com/polkadot-evm/frontier).

Frontier provides an EVM compatibility layer for Substrate, with full support to all the Ethereum RPC APIs allowing to develop Dapps and interact with them leveraging the usual EVM developer tools (Metamask, Foundry, Hardhat, ReMix, etc).  


## 🚀 Getting Started

### 🦀 Rust Setup

Make sure you have Rust installed along with everything that is needed to compile a Substrate node. More details [here](./docs/rust-setup.md).

### 🔧 Build

1. Clone the zkVerify EVM Parachain repository:

```sh
git clone https://github.com/zkVerify/zkVerify-EVM-Parachain
```

2. Ensure rust is updated to latest version:
```sh
rustup update
```

3. Use `cargo` to build the parachain node without launching it:

```sh
cargo build --release
```

## 🕸️ Run a local network

In order to run a local network, you'll need to spin up a full fledged relay chain - parachain environment.

### 🐋 Docker

If you don't want to bother about dependencies, local installation and configuration, VFlow includes some Docker files for building the client and running one or more nodes locally.
For more information, see [docker/README.md](docker/README.md).

### 👽 Zombienet
Otherwise, you can leverage the Zombienet testing environment to quickly spin up a zkVerify relay chain and a VFlow Parachain instances automatically connected to it.
For more information, see [zombienet-config/README.md](zombienet-config/README.md).