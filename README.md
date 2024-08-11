# Casascius Bitcoin Mini-key Cracker

## Educational Project: Use Responsibly

This Rust-based tool is designed for educational purposes to demonstrate cryptographic principles related to Bitcoin. It should not be used to attempt access to wallets you don't own.

## Project Overview

This program generates and validates mini-keys against a list of Bitcoin addresses. It's important to note that this approach is computationally infeasible for finding actual private keys due to the vast keyspace of Bitcoin :(

## Key Features

- Random mini-key generation
- Conversion of mini-keys to Wallet Import Format (WIF)
- Validation of WIF keys against Bitcoin addresses
- CSV-based address input
- Telegram notifications for successful matches

## What are Casascius Bitcoins?

Casascius Bitcoins, created by Mike Caldwell in 2011, were physical coins containing embedded Bitcoin private keys. Each coin had a tamper-evident hologram sticker concealing the private key. The private key was encoded in "mini private key format", a 22 or 30 character string.

These physical tokens were designed to make Bitcoin tangible and easier to understand for newcomers. However, production ceased in 2013 due to regulatory concerns. Casascius Bitcoins are now collectors' items, with any unredeemed coins still holding their Bitcoin value.


## Setup and Dependencies

1. Ensure Rust and Cargo are installed
2. Clone this repository
3. Add the following to your `Cargo.toml`:
   ```toml
   [dependencies]
   sha2 = "0.9.8"
   base58 = "0.2.0"
   reqwest = { version = "0.11", features = ["json"] }
   serde = { version = "1.0", features = ["derive"] }
   bitcoin = "0.27.1"
   rand = "0.8.5"
   csv = "1.1"
   tokio = { version = "1", features = ["full"] }
