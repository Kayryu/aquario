# Aquario node

Aquario is a stable coin developed based on substrate inspired by MakerDAO.

## Build

Rust version > 1.41

Install Rust:

```bash
curl https://sh.rustup.rs -sSf | sh
```

Initialize your Wasm Build environment:

```bash
./scripts/init.sh
```

Build Wasm and native code:

```bash
cargo build --release
```

## Run

### Single node development chain


```bash
./target/release/node-template purge-chain --dev
```

Start a development chain with:

```bash
./target/release/node-template --dev
```

## Feature

MVP features：

- [ ] mortgage
- [ ] auction
- [ ] liquidation
- [ ] oracle
- [ ] exchange