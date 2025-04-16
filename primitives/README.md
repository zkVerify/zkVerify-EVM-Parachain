This folder contains some crates used for implementing some of the tracing related rpc requests from go-ethereum [debug namespace](https://geth.ethereum.org/docs/interacting-with-geth/rpc/ns-debug) (i.e. `debug_traceTransaction`, `debug_traceBlockByNumber` and `debug_traceBlockByHash`).

The code was forked from [`Moonbeam v0.36.0`](https://github.com/moonbeam-foundation/moonbeam/tree/v0.36.0/).

- ext: Defines the interface of the host functions used to send tracing events back to the node.
- rpc/debug: Defines the interface of runtime APIs that provide the tracing methods.
- evm-tracing-events: Defines the listener mechanism for sending the tracing events to the client rpc listeners.



