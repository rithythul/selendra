# Selendra Network Codebase Overview

## Introduction

This document provides an in-depth technical overview of the Selendra Network codebase, designed to help our internal team understand the architecture, components, and key design principles.

## Architectural Philosophy

Selendra is built on the Substrate framework with a focus on:
- Flexibility
- Scalability
- Sustainability
- Developer-friendly design

## Core Components

### 1. Primitives (`/primitives`)
Defines core types and traits used across the entire network:
- Basic blockchain types (AccountId, Balance, BlockNumber)
- Cross-chain message structures
- Validator and governance-related enums
- Fee calculation structures

**Key Features:**
- Type-safe blockchain primitives
- Supports both Wasm and EVM runtimes
- Flexible type definitions for future extensibility

### 2. Runtime (`/runtime`)
The core logic of the Selendra blockchain:
- Integrates all custom pallets
- Defines runtime configuration
- Implements core blockchain logic
- Supports modular runtime upgrades

**Key Features:**
- Hybrid runtime supporting Wasm and EVM
- Configurable system parameters
- Flexible pallet integration
- Runtime API implementations

### 3. Custom Pallets (`/pallets`)

#### a. Consensus Pallet
- Implements block production and finality mechanisms
- Manages validator selection and rotation
- Tracks validator performance
- Supports BABE (Blind Assignment for Blockchain Extension) and GRANDPA finality

#### b. Fees Pallet
- Dynamic transaction fee adjustment
- Network usage monitoring
- Automatic fee distribution to validators
- Ensures network accessibility and sustainability

#### c. Bridge Pallet
- Cross-chain message passing
- Secure asset transfers between chains
- Multi-signature security
- Message validation and confirmation protocols

#### d. Governance Pallet
- On-chain proposal and voting system
- Automated proposal execution
- Emergency action capabilities
- Democratic network upgrade mechanism

#### e. Validator Pallet
- Staking and validator management
- Performance tracking
- Reward distribution
- Slashing mechanisms for network security

#### f. Naming Service Pallet
- Human-readable name registration
- Address mapping and resolution
- Secure name ownership
- Low-cost name reservation
- Supports up to 10 names per account
- Flexible name transfer capabilities

### 4. Node Implementation (`/node`)
- CLI interface for network interaction
- Chain specification management
- Network service configuration
- RPC endpoint implementations

## Key Design Principles

1. **Modularity**: Each component is designed to be independently upgradable
2. **Security**: Multi-layered security approach with formal verification support
3. **Flexibility**: Supports both Wasm and EVM smart contracts
4. **Sustainability**: Dynamic fee mechanisms and energy-efficient consensus

## Technical Highlights

- Substrate-based blockchain framework
- Hybrid runtime (Wasm + EVM)
- Dynamic fee adjustment
- Cross-chain bridging
- On-chain governance
- Proof of Stake consensus

## Development Workflow

1. **Building**: 
   ```bash
   cargo build --release
   ```

2. **Testing**:
   ```bash
   cargo test --all
   ```

3. **Running Development Chain**:
   ```bash
   ./target/release/selendra --dev
   ```

## Future Roadmap

- Enhance cross-chain interoperability
- Implement more advanced governance mechanisms
- Improve performance and scalability
- Expand developer tooling and documentation

## Naming Service Roadmap

### Phase 1: Core Implementation (Q3 2025)
- [x] Develop core naming pallet
- [x] Implement two free names mechanism
- [x] Create tiered pricing structure
- [x] Add governance pricing controls
- [ ] Full runtime integration
- [ ] Comprehensive test coverage

### Phase 2: Enhanced Naming (Q4 2025)
- [ ] Develop name marketplace
- [ ] Implement secondary name trading
- [ ] Create advanced name management features
- [ ] Design user reputation system
- [ ] Expand naming service SDK

### Phase 3: Cross-Chain Preparation (Q1 2026)
- [ ] Research cross-chain name resolution
- [ ] Design multi-network name mapping
- [ ] Develop cross-chain name resolution protocols
- [ ] Create initial cross-chain name prototype
- [ ] Gather community feedback

### Technical Considerations

1. **Naming Pallet Design**
   - Substrate-native implementation
   - Secure name ownership tracking
   - Flexible pricing model
   - Governance-controlled parameters

2. **Economic Model**
   - Two free names per account
   - Exponential pricing for additional names
   - Revenue generation mechanism
   - Prevents name squatting

3. **Future Expansion**
   - Potential for cross-chain name resolution
   - Integration with identity systems
   - Advanced name management features

### Naming Service Pricing Structure
- First 2 names: Free
- 3rd name: 3 SEL
- 4th name: 6 SEL
- 5th name: 12 SEL
- 6+ names: 24 SEL

### Economic Considerations
- **Pricing Mechanism**: Exponential increase
- **Revenue Generation**: Significant network income
- **Anti-Squatting**: High-cost additional names
- **User Accessibility**: Two free names maintained

## Contribution Guidelines

1. Follow Rust best practices
2. Write comprehensive tests
3. Maintain code modularity
4. Update documentation with code changes

## Contact

For more information, contact the Selendra development team:
- Email: dev@selendra.org
- Blockchain Development: Lay Nath
- Design: Rithy Thul

---

**Note**: This is a living document. Please update it as the codebase evolves.
