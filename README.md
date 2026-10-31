# SigmaOS mintlocale

This crate is currently a `no_std` identity scaffold. Its only public type
constructs a `Component` with the fixed name `SigmaOS Component`. It does not
manage locale settings, translate messages, or install language packs.

The inline unit test checks only the fixed scaffold name. No locale behavior is
implemented or exercised.

## Build

```sh
cargo check --lib
```

## License

Apache-2.0. See [`Cargo.toml`](Cargo.toml).
