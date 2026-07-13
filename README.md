# echoforge

Multi-client TCP chat server built using threads and mutex in Rust.

## Features

- Handles multiple simultaneous client connections
- Thread-per-client model with shared state guarded by mutex
- Broadcasts messages to all connected clients

## Build

```bash
cargo build --release
```

## Run

Start the server:

```bash
cargo run --bin server
```

Connect a client:

```bash
cargo run --bin client
```

## Usage

Run the server first, then connect with one or more clients in separate terminals. Messages sent from any client are broadcast to all others.
