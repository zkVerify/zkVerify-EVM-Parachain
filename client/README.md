This folder contains some crates used for implementing some of the tracing related rpc requests from go-ethereum [debug namespace](https://geth.ethereum.org/docs/interacting-with-geth/rpc/ns-debug) (i.e. `debug_traceTransaction`, `debug_traceBlockByNumber` and `debug_traceBlockByHash`).

The code was forked from [`Moonbeam v0.36.0`](https://github.com/moonbeam-foundation/moonbeam/tree/v0.36.0/).

- evm-tracing: Defines the different tracers supported by the debug rpc methods.
- rpc/debug: Implements the RPC server that handles the debug rpc requests.
- rpc-core: Defines debug RPC interface



