# Bitcoin Address Miner
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)  [![Crates.io](https://img.shields.io/crates/v/bitcoin_address_miner.svg)](https://crates.io/crates/bitcoin_address_miner)  
<br />
Mine Bitcoin [Vanity Addresses](https://en.bitcoin.it/wiki/Vanitygen) ⛏️🅰️🅱️🅾️  
<img src="images/btc-address.png" width="100" height="100">

## Overview

The Bitcoin Address Miner is a utility written in `Rust` that generates random [BIP-32](https://en.bitcoin.it/wiki/BIP_0032), *Base58 P2PKH* Bitcoin addresses and searches for addresses that start with a user-specified sequence of characters.  
It utilizes multiple asynchronous threads to mine addresses concurrently.  
- *Base58 P2PKH* refers to a specific encoding format used in Bitcoin addresses:  
1. **Base58** : This is a binary-to-text encoding scheme that is similar to Base64 but avoids using  
   easily confused characters  (like 0, O, I, and l) to make strings more legible.  
   Base58 is commonly used in Bitcoin addresses to represent data in a human-readable format.  
2. **P2PKH** : This stands for "Pay to Public Key Hash." It is a standard transaction script in Bitcoin that specifies how funds can be spent.  
   In a P2PKH transaction, the recipient's address is derived from the hash of their public key.  

So, when you combine Base58 encoding with a P2PKH address, you get a format that looks like a string of characters, such as `1A1zP1eP5QGefi2DMPTfTL5SLmv7DivfNa`.  
This is the well-known address associated with the first-ever Bitcoin transaction.


## Scope
The Bitcoin Address Miner is designed for individuals interested in exploring Bitcoin address generation and conducting experiments related to address matching.   Potential use cases include:

- *Educational Purposes*: Learn about Bitcoin address generation, key pairs, and the structure of Bitcoin addresses.

- *Security Research*: Test the robustness of address generation algorithms and study patterns in address creation.

- *Address Customization*: Find Bitcoin addresses that start with specific characters for custom address creation.

## Features
- Generates random Bitcoin addresses.
- Searches for addresses starting with a user-defined sequence of characters.
- Utilizes multiple asynchronous threads for concurrent mining.
- Prints matching addresses along with associated private and public keys.
- Calculates and displays the elapsed time for each successful address mining.
## Prerequisites 
- [Rust](https://www.rust-lang.org/) 


## How to Use 
1. Clone the repository:

```console
git clone https://github.com/r3drun3/bitcoin-address-miner
``` 
2. Navigate to the project directory:

```console
cd bitcoin-address-miner
``` 
3. Build and run the program:

```console
cargo build --release
cargo run
``` 
4. Enter the starting letters for the address when prompted. 
5. The program will run multiple threads to mine addresses concurrently.  
   Once a matching address is found, the program will display the address, private key, public key, and the elapsed time.  

<br/>

Example: download from [crates](https://crates.io/crates/bitcoin_address_miner) and run binary:  
```console
cargo install bitcoin_address_miner &&  bitcoin_address_miner  

Updating crates.io index
  Downloaded bitcoin_address_miner v0.1.2
  Downloaded 1 crate (25.3 KB) in 1.13s
  Installing bitcoin_address_miner v0.1.2
    Updating crates.io index
   Compiling libc v0.2.150
   Compiling cfg-if v1.0.0
   Compiling proc-macro2 v1.0.69
   Compiling bitcoin-internals v0.2.0
   Compiling unicode-ident v1.0.12
   Compiling autocfg v1.1.0
   Compiling parking_lot_core v0.9.9
   Compiling ppv-lite86 v0.2.17
   Compiling smallvec v1.11.2
   Compiling scopeguard v1.2.0
   Compiling hex-conservative v0.1.1
   Compiling lock_api v0.4.11
   Compiling hex_lit v0.1.1
   Compiling bitcoin v0.31.0
   Compiling bitcoin_hashes v0.13.0
   Compiling pin-project-lite v0.2.13
   Compiling bytes v1.5.0
   Compiling quote v1.0.33
   Compiling cc v1.0.83
   Compiling bech32 v0.10.0-beta
   Compiling syn v2.0.39
   Compiling getrandom v0.2.11
   Compiling rand_core v0.6.4
   Compiling mio v0.8.9
   Compiling rand_chacha v0.3.1
   Compiling parking_lot v0.12.1
   Compiling rand v0.8.5
   Compiling num_cpus v1.16.0
   Compiling signal-hook-registry v1.4.1
   Compiling socket2 v0.5.5
   Compiling secp256k1-sys v0.9.0
   Compiling hex v0.4.3
   Compiling tokio-macros v2.2.0
   Compiling tokio v1.34.0
   Compiling secp256k1 v0.28.0
   Compiling bitcoin_address_miner v0.1.2
    Finished release [optimized] target(s) in 31.17s
  Installing /Users/rago/.cargo/bin/bitcoin_address_miner
   Installed package `bitcoin_address_miner v0.1.2` (executable `bitcoin_address_miner`)

[BITCOIN KEY/ADDRESS MINING] 
 Enter the starting letters for the address: 
JoKe

 Starting mining...this may take many minutes (or even days) to complete
Found matching address: 1JoKesTr2vDhVaayJGwifva8hA9JaRKtFJ
Private Key: AF5E88FCC60EA189ADADCC694082CCC76066E096ADE33812B5A9914DA6C6FC73
Public Key: 023de08a1c546e14b87b62f26dee68f13894403c66b3878288eabed772abdda63d
Elapsed Time: 15.50 minutes
```  


## License

This utility is open-source and released under the [MIT License](https://github.com/R3DRUN3/bitcoin-address-miner/blob/main/LICENSE)