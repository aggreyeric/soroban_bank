# soroban_bank

## Project Structure

1.  Build Contract
```sh

stellar contract deploy --wasm  hello_w
orld.optimized.wasm --source eric --network testnet 
CC7FPIPJNE7NMZGL5IWLSYGHQMXV25SDR6QR23GN2GR5EQYKVHGJSIQZ


```

This repository uses the recommended structure for a Soroban project:
```text
.
├── contracts
│   └── hello_world
│       ├── src
│       │   ├── lib.rs
│       │   └── test.rs
│       └── Cargo.toml
├── Cargo.toml
└── README.md
```

- New Soroban contracts can be put in `contracts`, each in their own directory. There is already a `hello_world` contract in there to get you started.
- If you initialized this project with any other example contracts via `--with-example`, those contracts will be in the `contracts` directory as well.
- Contracts should have their own `Cargo.toml` files that rely on the top-level `Cargo.toml` workspace for their dependencies.
- Frontend libraries can be added to the top-level directory as well. If you initialized this project with a frontend template via `--frontend-template` you will have those files already included.# 
