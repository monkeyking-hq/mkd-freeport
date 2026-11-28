# Rust Library API

Add `mkd-freeport` to your `Cargo.toml`:

```toml
[dependencies]
mkd-freeport = "0.4.1"
```

## Basic Discovery (`examples/basic_discovery.rs`)

```rust
{{#include ../../examples/basic_discovery.rs}}
```

Run it with:

```bash
cargo run --release --example basic_discovery
```

## Custom Range & Ascending Order (`examples/custom_range.rs`)

```rust
{{#include ../../examples/custom_range.rs}}
```

Run it with:

```bash
cargo run --release --example custom_range
```

## Owned UDP Reservations (`examples/reserve_udp.rs`)

```rust
{{#include ../../examples/reserve_udp.rs}}
```

Run it with:

```bash
cargo run --release --example reserve_udp
```
