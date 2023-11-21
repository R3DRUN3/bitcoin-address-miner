# Bitcoin Address Miner
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)  [![Crates.io](https://img.shields.io/crates/v/bitcoin_address_miner.svg)](https://crates.io/crates/bitcoin_address_miner)  
<br />
Mine Bitcoin Addresses ⛏️🅰️🅱️🅾️  
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

Example:  
```console
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