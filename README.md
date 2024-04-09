## Install, Build, Deploy and Test

Let's run the test once to see what happens.

### Install `anchor`

First, make sure that `anchor` is installed:

Install `avm`:

```bash
$ cargo install --git https://github.com/coral-xyz/anchor avm --locked --force
...
```

Install latest `anchor` version:

```bash
$ avm install 0.27.0
...
$ avm use 0.27.0
...
```

> If you haven't installed `cargo`, please refer to this [doc](https://book.solmeet.dev/notes/solana-starter-kit#install-rust-and-solana-cli) for installation steps.

#### Extra Dependencies on Linux (Optional)

You may have to install some extra dependencies on Linux (ex. Ubuntu):

```bash
$ sudo apt-get update && sudo apt-get upgrade && sudo apt-get install -y pkg-config build-essential libudev-dev
...
```

#### Verify the Installation

Check if Anchor is successfully installed:

```bash
$ anchor --version
anchor-cli 0.27.0
```

### Install Dependencies

Next, install dependencies:

```
$ yarn
```

### Build

#### Update `program_id`

Get the public key of the deploy key. This keypair is generated automatically so a different key is exptected:

```bash
$ anchor keys list
barocasino: Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS
```

Replace the default value of `program_id` with this new value:

```toml
# Anchor.toml

[programs.localnet]
barocasino = "Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS"

...
```

```rust
// lib.rs

...

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

...
```

Build the program:

```
$ anchor build
```

### Deploy

Let's deploy the program. Notice that `barocasino` will be deployed on a [mainnet-fork](https://github.com/DappioWonderland/solana) test validator run by Dappio:

```
$ solana config set --url localhost
...
```

```
$ anchor deploy
...

Program Id: Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS

Deploy success
```

Finally, run the test:

```
$ anchor test --skip-deploy --skip-build --skip-local-validator
```
