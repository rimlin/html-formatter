### Rust based HTML formatter

Run command:
```
RUST_LOG=trace cargo run -- ./examples/**/*.html
```

Test command:
```
cargo test
```

## Roadmap

- Multithreaded files formatting
- Widening config. Support more settings:
  - ignore files
  - max length of row
  - bracket Line
