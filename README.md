# Ghost Matrix Bot

This is my first matrix bot written in Rust (and also working).  
In this repository, I'm experimenting with Rust and the matrix-rust-sdk.

Currently, this bot is telling me which self-hosted service is currently not available.

### Overview

1. [x] Synapse (Matrix)
2. [x] Forgejo (Gitea)
3. [x] Nextcloud
4. [x] Keycloak
5. [x] Portainer
6. [x] Bitwarden
7. [x] WordPress

## How to run this program

### Build from source

Run `cargo run -- MATRIX_URL MATRIX_ID MATRIX_PASSWORD` to simple run it.  
Run `cargo build --release && ./target/release/ghost MATRIX_URL MATRIX_ID MATRIX_PASSWORD` to run the optimized version.
