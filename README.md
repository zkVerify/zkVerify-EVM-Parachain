# zkVerify EVM Parachain

The **zkVerify EVM Parachain** is the next generation EVM chain specifically tailored to building zero-knowledge (ZK) dApps, designed to be closely connected to the zkVerify chain.

## 🚀 Getting Started

### 🦀 Rust Setup

Make sure you have Rust installed along with everything that is needed to compile a Substrate node. More details [here](./docs/rust-setup.md).

### 🔧 Build

1. Clone the zkVerify EVM Parachain repository:

```sh
git clone https://github.com/zkVerify/zkVerify-EVM-Parachain
```

2. Check following dependencies are installed:
```sh
sudo apt-get install cmake clang clang-format ninja-build libstdc++-12-dev
```

3. Ensure cmake version is >= 3.24 (cmake --version). If not, update it.<br>
On Ubuntu you can do it with:
```sh
sudo apt remove cmake -y
sudo pip install cmake --upgrade
```

4. Ensure gcc version is >= 13 (gcc --version). If not, update it.<br>
On Ubuntu you can do it with:
```sh
sudo apt install software-properties-common
sudo add-apt-repository ppa:ubuntu-toolchain-r/test
sudo apt install gcc-13 g++-13
sudo update-alternatives --install /usr/bin/gcc gcc /usr/bin/gcc-13 100 --slave /usr/bin/g++ g++ /usr/bin/g++-13
```

5. Ensure rust is updated to latest version:
```sh
rustup update
```

6. Use `cargo` to build the parachain node without launching it:

```sh
cargo build --release
```

### 🕸️ Run a local network
 You will need a compatible release of [zkVerify](https://github.com/zkVerify/zkVerify/) to run a local network. You may also want to use [Zombienet](https://github.com/paritytech/zombienet/releases) (available for Linux and MacOS),  for spinning up a full fledged relay chain - parachain environment.
 You can find more information about running a local test network [HERE](./zombienet-config/README.md)

 Another option is to run nodes in a docker environment: you can find an example and tutorial [HERE](./docker/README.md).


👉 Learn more about parachains [here](https://wiki.polkadot.network/docs/learn-parachains), and parathreads [here](https://wiki.polkadot.network/docs/learn-parathreads).

