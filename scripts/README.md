# Local Network Setup

## Prerequisites
- Rust (nightly)
- Cargo
- substrate-cli

## Starting Local Network

### Single Node Development Mode
```bash
./start-local-network.sh
```

### Connecting to the Network
- WebSocket Endpoint: `ws://localhost:9945`
- RPC Endpoint: `http://localhost:9934`

## Testing Naming Service

1. Register a free name
```rust
// Rust example (to be implemented)
naming_service.register_name("myname")?;
```

2. Register additional names
```rust
// Each additional name will cost:
// 3rd name: 3 SEL
// 4th name: 6 SEL
// 5th name: 12 SEL
naming_service.register_name("secondname")?;
```

## Troubleshooting
- Ensure all dependencies are installed
- Check network connectivity
- Verify Rust and Substrate versions
