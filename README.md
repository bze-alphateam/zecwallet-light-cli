# Zeclite CLI - A command line ZecWallet light client. 

`zeclite-cli` is a command line zecwallet light client. To use it, download the latest binary from the releases page and run `./zeclite-cli`

This will launch the interactive prompt. Type `help` to get a list of commands

**zeclite-cli is currently in beta**


## Notes:
* The wallet is currently testnet only
* If you want to run your own server, please see [zeclite lightwalletd](https://github.com/adityapk00/lightwalletd), and then run `./zeclite-cli --server 127.0.0.1:9067`
* Support for reorgs is iffy. It your wallet gets into an inconsistent state, type `rescan` to reset the wallet. 
* The log file is in `~/.zcash/testnet3/zeclite.debug.log`

### Note Management
Zeclite does automatic note and utxo management, which means it doesn't allow you to manually select which address to send outgoing transactions from. It follows these priciples:
* Defaults to sending shielded transactions, even if you're sending to a transparent address
* Can select funds from multiple shielded addresses in the same transaction
* Will automatically shield your transparent funds at the first oppurtunity
    * When sending an outgoing transaction to a shielded address, Zeclite can decide to use the transaction to additionally shield your transparent funds (i.e., send your transparent funds to your own shielded address in the same transaction)

## Compiling from source

#### Pre-requisites
* Rust v1.37 or higher.
    * Run `rustup update` to get the latest version of Rust if you already have it installed

```
git clone https://github.com/adityapk00/lightwalletclient.git
cargo build --release
./target/release/zeclite-cli
```

## Options
CLI arguments you can pass to `zeclite-cli`

* `--server`: Connect to a custom zeclite lightwalletd server. 
    * Example: `./zeclite-cli --server 127.0.0.1:9067`
* `--seed`: Restore a wallet from a seed phrase. Note that this will fail if there is an existing wallet. Delete (or move) any existing wallet to restore from the 24-word seed phrase
    * Example: `./zeclite-cli --seed "twenty four words seed phrase"`
