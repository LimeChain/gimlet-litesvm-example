# gimlet-litesvm-example

A minimal two-program Solana workspace for exercising and debugging sBPF programs via [LiteSVM](https://github.com/LiteSVM/litesvm) and Gimlet — available for both [VSCode](https://marketplace.visualstudio.com/items?itemName=Limechain.gimlet) and [RustRover](https://plugins.jetbrains.com/plugin/31739-gimlet).

`primary` makes a CPI into `cpi_target`. Both are written with [pinocchio](https://github.com/anza-xyz/pinocchio).

```
primary  ─── CPI ───▶  cpi_target
```

> ### ⚠️ This whole workflow hinges on one dependency
>
> The trace-emitting and debugger-attach behavior described below **only works** because this repo depends on:
>
> ```toml
> litesvm = { version = "0.13.0", features = ["sbpf-debugger"] }
> ```
>
> - `litesvm` `0.13.0` or later is required — these are the builds that honor `SBF_TRACE_DIR` and `SBF_DEBUG_PORT`.
> - The `sbpf-debugger` feature is what enables Gimlet integration. Without it, traces are not written and the VM will not pause for a debugger to attach.
>
> If you drop below `0.13.0` or remove the feature, the `Run the test` and `Debug a program` steps below will silently stop producing what Gimlet needs. See `programs/primary/Cargo.toml`.

## Prerequisites

- Rust (stable, with `cargo`)
- Solana CLI / Agave toolchain (provides `cargo build-sbf`)

Verify:

```sh
solana --version
cargo build-sbf --version
```

## Build

```sh
cargo-build-sbf --tools-version v1.54 --debug --arch v1
```

> **Why this?** `cargo build-sbf` step builds with SBPFv1 (`--arch v1`), debug symbols, and no optimizations — required for accurate coverage results.

> **Note:** At the time of writing, best coverage results are achieved with SBPFv1 (dynamic stack frames), which is why we use `--arch v1`. Only with dynamic stack frames can we safely disable optimizations (`opt-level = 0`) without hitting stack size limits. The `--tools-version` can be v1.51 or higher, and `--debug` is required for coverage to work. Starting with `cargo-build-sbf` 4.0.0, the `--debug` flag outputs artifacts to `target/deploy/debug` instead of `target/deploy`. If you are using an older version of `cargo-build-sbf`, replace `target/deploy/debug` with `target/deploy` in all the steps.

## Run the test

```sh
SBF_OUT_DIR=$(pwd)/target/deploy/debug \
SBF_TRACE_DIR=$(pwd)/target/sbf/trace \
cargo test -p primary --features no-entrypoint --test cpi -- --nocapture
```

- `SBF_OUT_DIR` tells the test where to find the built `.so` files.
- `SBF_TRACE_DIR` is where the sBPF VM writes execution traces consumed by Gimlet.

This runs the LiteSVM CPI test and prints the program logs. Expect to see `primary invoke [1]` → `cpi_target invoke [2]` → `Ping received by cpi_target!` and both programs returning `success`.

## Debug a program

```sh
SBF_OUT_DIR=$(pwd)/target/deploy/debug \
SBF_DEBUG_PORT=1212 \
SBF_TRACE_DIR=$(pwd)/target/sbf/trace \
cargo test -p primary --features no-entrypoint --test cpi -- --nocapture
```

`SBF_DEBUG_PORT=1212` makes the sBPF VM **open TCP port `1212` and block waiting for a debugger to attach** before stepping into the program.

> This blocking-on-port behavior is provided by the `sbpf-debugger` feature of `litesvm` `0.13.0` or later. On older versions or without that feature, `SBF_DEBUG_PORT` is a no-op and the test will run straight through.

## Clean

```sh
cargo clean
rm -rf programs/primary/target
```

Wipes `target/` and any stray nested `target/` left by prior runs, if any.

## Layout

```
.
├── Cargo.toml                                  # workspace
└── programs/
    ├── cpi_target/                             # callee program
    │   ├── Cargo.toml
    │   └── src/lib.rs
    └── primary/                                # caller program (does the CPI)
        ├── Cargo.toml
        ├── src/lib.rs
        └── tests/cpi.rs                        # litesvm test
```
