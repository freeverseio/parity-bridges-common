## LAOS Bridge Common

This directory contains common crates that are used for establishing bridge between LAOS ownership parachain and LAOS evolution chains.

### Relay

`laos-relay` is used to establish bridge between LAOS chains. It currently supports one-way bridge from LAOS evolution chain to LAOS ownership parachain. It syncs latest finalized header, submits finality proof to a light client installed in ownership parachain. It is a modified, customized version of [substrate-relay](https://github.com/paritytech/parity-bridges-common/tree/master/relays/bin-substrate).

*Supported Bridges*:

- `evochain-to-ownership-parachain` - one-way bridge from LAOS evolution chain to LAOS ownership parachain.

*Commands*:

- `init-bridge` - initializes a bridge between two chains by registering current finalized header of source chain in target chain.
- `relay-headers` - syncs latest finalized header, submits finality proof to a light client installed in target chain.

### Client crates

Crates with `client-{chain}` prefix are used to define types used in establishing a bridge with that specific `chain`. It contains higher level types than primitives, such as `HeaderId`, `SyncHeader`, etc. and also creates a type that represents the `chain` in light-client implementations.

## Build

To build `laos-relay` binary, run:

```bash
cargo build -p laos-relay
```

## Run

To run the bridge locally, you will need an instance of ownerhip parachain and evolution chain running. 

First, initialize a bridge between two chains by running:

```bash
RUST_LOG=bridge=debug \
./target/debug/laos-relay init-bridge evochain-to-ownership-parachain \
--source-host localhost \
--source-port {EVOCHAIN_WS_PORT} \
--target-host localhost \
--target-port {OWNERSHIP_PARACHAIN_WS_PORT} \
--target-signer //Alice \
--source-version-mode Bundle \
--target-version-mode Bundle
```

This will initialize a bridge between evolution chain and ownership parachain by registering current finalized header of evolution chain in ownership parachain.

Then, run the bridge by running:

```bash
RUST_LOG=bridge=debug \
./target/debug/laos-relay relay-headers evochain-to-ownership-parachain \
--source-host localhost \
--source-port {EVOCHAIN_WS_PORT} \
--target-host localhost \
--target-port {OWNERSHIP_PARACHAIN_WS_PORT} \
--target-signer //Alice \
--source-version-mode Bundle \
--target-version-mode Bundle
```

You should see logs similar to:

```bash
2023-08-24 14:48:07 +03 INFO bridge Connecting to Evochain node at ws://localhost:9944
2023-08-24 14:48:07 +03 INFO bridge Connecting to OwnershipParachain node at ws://localhost:9999
2023-08-24 14:48:07 +03 INFO bridge Exposed substrate_relay_build_info metric: version=1.0.1 commit=184d0f4-dirty
2023-08-24 14:48:07 +03 INFO bridge Starting Evochain -> OwnershipParachain finality proof relay
[Evochain_to_OwnershipParachain_Sync] 2023-08-24 14:48:07 +03 WARN bridge Evochain finality proofs stream is being started / restarted
[Evochain_to_OwnershipParachain_Sync] 2023-08-24 14:48:07 +03 INFO bridge Synced 1 of 9 headers
[Evochain_to_OwnershipParachain_Sync] 2023-08-24 14:48:17 +03 INFO bridge Synced 1 of 10 headers
[Evochain_to_OwnershipParachain_Sync] 2023-08-24 14:48:17 +03 DEBUG bridge Going to submit finality proof of Evochain header #10 to OwnershipParachain
```
