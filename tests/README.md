# Functional testing

> [!NOTE]\
> This folder contains a set of functional tests.
It is written in typescript, using the [Moonwall](https://moonsong-labs.github.io/moonwall/) framework.

## Documentation Site:
https://moonsong-labs.github.io/moonwall/guide/intro/getting-started.html

## Test Categories

- `dev`: Tests that execute a single local dev node, using PolkadotJs / Ethers.js / Web3.js, to check the runtime and client in a relatively end-to-end manner.
- `zombie`: Tests that use the [ZombieNet](https://github.com/paritytech/zombienet) framework to verify zkVerify EVM parachains in the context of a parachain connected to a relay chain, and other topologies.

## Installation

> [!NOTE]\
> PNPM is the package manager of choice for this repo, due to its superior handling of heavily nested dependencies.
There are [various](https://pnpm.io/installation) ways to install it, but perhaps the easiest is `sudo npm -g i pnpm`

Before running tests always install and update the package dependencies:

```bash
cd tests
pnpm i 
```

## Usage Examples

Launch the CLI:

```bash
pnpm moonwall
```

Execute all dev tests:

```bash
pnpm moonwall test dev
```

Execute a single test:

```bash
pnpm moonwall test dev <test_case_id>
```

Execute a single test and keep node running:

```bash
pnpm moonwall run dev <test_case_id>
```

Spawn zombienet nodes and use CLI to run tests/interact with the environment:
```bash
pnpm moonwall run zombie
```

Running a particular zombienet test:

```bash
pnpm moonwall test zombie Z01002
```

Rename all prefixes for a suite (to keep them consistent)

```bash
pnpm moonwall derive <suite_root_dir> 
```

> [!NOTE]\
> For a full list of test environments and suites available, inspect the `moonwall.config.json` file.
Alternatively, use the CLI to browse networks and tests available.
