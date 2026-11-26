# proxelo-api

Protocol Buffers definitions shared by the Proxelo control plane
([`proxelo`](https://github.com/null-variable/proxelo)) and the proxy
([`proxelo-proxy`](https://github.com/null-variable/proxelo-proxy)).

The two processes exchange nothing but the messages defined here. The control
plane compiles user configuration into a `Snapshot` and pushes it over the
`DistributionService` stream. The proxy loads it, swaps it in atomically and
acknowledges the revision.

## Layout

| Path | Contents |
|---|---|
| `proto/proxelo/snapshot/v1` | `Snapshot`: the resolved configuration a proxy serves |
| `proto/proxelo/distrib/v1` | `DistributionService`: snapshot push, ACME challenges, acknowledgements, traffic counters |
| `proto/proxelo/sim/v1` | Framing of the `proxelo-proxy sim` helper the control plane uses for dry-loads and request simulation |
| `proto/proxelo/expr/v1` | `Request`: the variable route CEL expressions see, type-checked by the control plane and evaluated by the proxy |
| `gen/go` | Generated Go code (committed) |
| `rust` | Rust crate, generated at build time |

## How the messages are used

**Snapshot.** The control plane resolves everything a proxy would otherwise
work out per request: references between resources, policy inheritance,
defaults, and the order in which sites and routes are matched. Objects refer
to each other by index into the snapshot's lists, never by name. A proxy
validates a snapshot completely and builds its runtime form before it swaps
it in, so a snapshot is accepted or rejected as a whole.

**Distribution stream.** The proxy dials the control plane over a Unix socket
and opens `Sync`:

1. Proxy sends `Hello` with its node ID, version and the revision it serves.
2. Control plane sends the newest snapshot as `snapshot_data` (serialized
   bytes, so the proxy can check the SHA-256 hash recorded in them and store
   exactly those bytes as its last-known-good snapshot), then a new one after
   every apply or rollback.
3. Proxy answers each snapshot with `Ack` (applied, or rejected with a
   reason) and sends `Stats` every few seconds: counters since start, from
   which the control plane derives error rates while a revision awaits
   confirmation.
4. ACME challenges arrive as `Challenges`, replacing the previous set. They
   are not part of a snapshot and are never persisted.

When the stream ends the proxy keeps serving and dials again with backoff.

**Simulator.** `proxelo-proxy sim` reads length-delimited `SimRequest`
messages on stdin and writes `SimResponse` messages on stdout. It runs the
serving engine without sockets, so validation and the request simulator in
the control plane give the same answers as a live proxy.

## Usage

Go:

```sh
go get github.com/null-variable/proxelo-api@latest
```

```go
import snapshotv1 "github.com/null-variable/proxelo-api/gen/go/proxelo/snapshot/v1"
```

Rust (`Cargo.toml`):

```toml
[dependencies]
proxelo-api = { git = "https://github.com/null-variable/proxelo-api", tag = "v0.0.1" }
```

```rust
use proxelo_api::snapshot::v1::Snapshot;
```

## Development

Requires [buf](https://buf.build) and a Rust toolchain.

```sh
buf lint
buf generate          # regenerates gen/go
cd rust && cargo build
```

## Compatibility

- Releases follow semantic versioning. Tags are shared by the Go module and the Rust crate.
- Adding fields or messages is a minor release. Removing or renumbering fields is not allowed within a package; incompatible changes go into a new package version (`v2`).
- CI runs `buf breaking` against the default branch on every pull request.

## License

Licensed under the [Apache License, Version 2.0](LICENSE). See [NOTICE](NOTICE).
