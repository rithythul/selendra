# Selendra Network

## 🙋‍♀️ Introduction 
Selendra is a Substrate-based smart contract network supporting Wasm and EVM, based in Cambodia, tailored for developers and enterprise adoption.

### Mission
Our mission is to build simple and easy to use tools for next generation of blockchain builders and users, especially those from developing countries. So they can help accelerate mass adoption and reach all corners of the world.

### Goal
Open opportunities for individuals, businesses, and communities by making access to emerging blockchain products and services a reality, for all members of society. Bringing the blockchain evolution a step closer to the masses and helping accelerate the advancement and strengthen the industry as a whole, together with other industry builders and players.

## Features

- **Hybrid Runtime**: Supports both EVM and Wasm smart contracts for maximum flexibility
- **Dynamic Fee System**: Automatically adjusts transaction fees based on network usage to ensure accessibility
- **Cross-chain Bridge**: Enables seamless asset and data transfer between chains
- **On-chain Governance**: Democratic decision-making process for network upgrades
- **Proof of Stake**: Energy-efficient consensus with validator staking
- **Selendra Naming Service**: Provides human-readable names for complex hex addresses

## Selendra Naming Service

### Key Capabilities
- **Memorable Addresses**: Replace complex hex addresses with human-readable names
- **Single Network Naming**: Unified identity within the Selendra ecosystem
- **Secure Registration**: Name validation and ownership tracking
- **Low-Cost Registration**: Affordable name reservation

### Name Format
- Minimum length: 3 characters
- Maximum length: 32 characters
- Allowed characters: Lowercase letters and numbers
- Example: `johndoe`, `wallet123`

### Benefits
- Simplified user experience
- Enhanced address readability
- Built-in name ownership management
- Flexible name transfer capabilities

### Selendra Naming Service Roadmap

#### Phase 1: Core Implementation (Q3 2025)
- [x] Basic name registration mechanism
- [x] Two free names per account
- [x] Tiered pricing structure
- [x] Governance-controlled pricing
- [ ] Integration with core runtime
- [ ] Basic UI/UX for name management

#### Phase 2: Enhanced Naming (Q4 2025)
- [ ] Name marketplace development
- [ ] Secondary name trading platform
- [ ] Advanced name features
- [ ] User reputation system
- [ ] Expanded name management tools

#### Phase 3: Cross-Chain Preparation (Q1 2026)
- [ ] Name resolution standards research
- [ ] Multi-network compatibility design
- [ ] Cross-chain name mapping protocols
- [ ] Initial cross-chain name resolution prototype
- [ ] Community and developer feedback integration

### Naming Service Pricing
- First 2 names: Free
- 3rd name: 3 SEL
- 4th name: 6 SEL
- 5th name: 12 SEL
- 6+ names: 24 SEL

**Economic Model**:
- Exponential pricing prevents name squatting
- Generates network revenue
- Maintains user accessibility
- Flexible governance-controlled pricing

## Architecture

The network consists of several key components:

### Core Components

1. **Consensus Pallet**
   - BABE for block production
   - GRANDPA for finality
   - Validator selection and rotation

2. **Fees Pallet**
   - Dynamic fee adjustment
   - Fee distribution to validators
   - Network usage monitoring

3. **Bridge Pallet**
   - Cross-chain message passing
   - Asset transfer protocol
   - Multi-signature security

4. **Governance Pallet**
   - Proposal submission and voting
   - Automated execution
   - Emergency procedures

5. **Validator Pallet**
   - Staking mechanism
   - Performance tracking
   - Reward distribution

## Getting Started

### Prerequisites

- Rust and Cargo
- Substrate development environment
- Node.js (for frontend development)

### Installation

1. Clone the repository:
   ```bash
   git clone https://github.com/selendra/selendra.git
   cd selendra
   ```

2. Install dependencies:
   ```bash
   cargo build --release
   ```

3. Run the node:
   ```bash
   ./target/release/selendra
   ```

## Development

### Building

```bash
cd selendra
cargo build --release
```

### Testing

```bash
cargo test --all
```

### Running a Development Chain

```bash
./target/release/selendra --dev
```

## Contributing

We welcome contributions! Please follow these steps:

1. Fork the repository
2. Create a feature branch
3. Commit your changes
4. Push to your fork
5. Submit a pull request

## Documentation

Comprehensive documentation is available in the `/docs` directory:

- Technical Documentation
- API Reference
- Integration Guides
- Security Guidelines

## Acknowledgement

Selendra project is inspired by the excellent work of many growing projects in the Polkadot and Ethereum ecosystem and many other blockchain developers around the world. Our progress in the past, the present and the future is only possible thanks to the open sources software community, framework, and tools. Thank you!

## License

Selendra is licensed under the Apache License 2.0 - see the [LICENSE](LICENSE) file for details.

## Contact

- Website: https://selendra.org
- Email: info@selendra.org
- Twitter: @SelendraNetwork
- Telegram: @Selendra
