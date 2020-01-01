# cargo-notify

Cargo notify is a cargo plugin to run cargo build-like commands(`check`, `build`) and notify users of build completion and number of errors found (if any). Pairs nicely with [carg-watch](https://github.com/passcod/cargo-watch).

# Installation
```bash
cargo install --git https://github.com/nikhilmitrax/cargo-notify
```

# But why?
The usual development workflow to notice errors in builds is to run (RLS/rust-analyzer) in the IDE, and potentially use [carg-watch](https://github.com/passcod/cargo-watch) to run check/build on file change. This works fairly well for small or medium sized projects, but can get a bit cumbersome for larger projects, where the compile time may be longer than a few seconds, at which point, getting a notification is useful.

# Interface
`cargo notify <cmd>`.

For example
```bash
cargo notify check
cargo watch -c -x 'notify check'
```
