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
| `proto/proxelo/distrib/v1` | `DistributionService`: snapshot push, acknowledgements, proxy hello |
| `gen/go` | Generated Go code (committed) |
| `rust` | Rust crate, generated at build time |

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

[MIT](LICENSE)
