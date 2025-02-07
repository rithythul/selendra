# Selendra Consensus Technical Specification
Version 1.0 - February 2025

## Overview

This document details the technical implementation of Selendra's optimized consensus mechanism, achieving ~3 second finality while maintaining high throughput and security.

## 1. BABE Configuration

### 1.1 Slot Time Optimization
```rust
pub struct BABEConfig {
    // Reduced from 6000ms to 1500ms
    pub slot_duration: u64 = 1500,
    // Increased for faster block propagation
    pub block_proposal_timeout: u64 = 750,
    // Optimized for 1.5s slots
    pub max_block_proposal_delay: u64 = 500,
}
```

### 1.2 Block Production Parameters
```rust
pub struct BlockProductionConfig {
    // Maximum block size (optimized for network conditions)
    pub max_block_size: u32 = 4_194_304,  // 4MB
    // Maximum block weight
    pub max_block_weight: u64 = 2_000_000_000_000,
    // Targeted block fullness
    pub target_block_fullness: Percent = 80,
}
```

### 1.3 Network Propagation
```rust
pub struct PropagationConfig {
    // Enhanced block announcement
    pub announce_block_prefix: bool = true,
    // Parallel block verification
    pub parallel_verification: bool = true,
    // Block parts transmission
    pub max_parallel_block_chunks: u32 = 4,
}
```

## 2. GRANDPA Optimization

### 2.1 Vote Processing
```rust
pub struct GrandpaConfig {
    // Parallel vote processing
    pub parallel_vote_processing: bool = true,
    // Vote aggregation threshold
    pub vote_threshold: Threshold = Threshold::TwoThirds,
    // Optimized timeout for 3s finality
    pub voting_period: Duration = Duration::from_millis(1000),
    // Maximum pending votes to process in parallel
    pub max_pending_votes: u32 = 1000,
}
```

### 2.2 Single-Block Finality
```rust
pub struct FinalityConfig {
    // Prioritize single block finality
    pub prioritize_single_block: bool = true,
    // Maximum finality lag
    pub max_finality_lag: u32 = 2,
    // Vote persistence duration
    pub vote_persistence: Duration = Duration::from_millis(750),
}
```

### 2.3 Network Message Optimization
```rust
pub struct NetworkConfig {
    // Prioritized finality messages
    pub finality_message_priority: Priority = Priority::High,
    // Optimized gossip parameters
    pub gossip_duration: Duration = Duration::from_millis(500),
    // Enhanced peer discovery
    pub min_peers_for_vote_broadcast: u32 = 4,
}
```

## 3. Performance Optimizations

### 3.1 Block Verification
```rust
pub struct VerificationConfig {
    // Parallel signature verification
    pub parallel_signature_verification: bool = true,
    // Pre-validation of block headers
    pub pre_validate_headers: bool = true,
    // Maximum parallel verifications
    pub max_parallel_verifications: u32 = 8,
}
```

### 3.2 State Management
```rust
pub struct StateConfig {
    // State pruning strategy
    pub state_pruning: PruningMode = PruningMode::Archived(1000),
    // State caching
    pub state_cache_size: u32 = 25_165_824,  // 24MB
    // Parallel state updates
    pub parallel_state_updates: bool = true,
}
```

## 4. Implementation Details

### 4.1 Block Production Flow
```rust
impl BlockProduction {
    pub async fn produce_block(&mut self) -> Result<Block> {
        // 1. Slot assignment check (BABE)
        let slot_assignment = self.check_slot_assignment()?;
        
        // 2. Parallel transaction gathering
        let transactions = self.gather_transactions_parallel().await?;
        
        // 3. Block building with optimized weight calculation
        let block = self.build_block_optimized(transactions).await?;
        
        // 4. Quick validation
        self.validate_block_parallel(&block).await?;
        
        // 5. Propagate to network with enhanced announcement
        self.propagate_block_enhanced(block).await?;
        
        Ok(block)
    }
}
```

### 4.2 Finality Processing
```rust
impl FinalityGadget {
    pub async fn process_finality(&mut self) -> Result<()> {
        // 1. Collect votes in parallel
        let votes = self.collect_votes_parallel().await?;
        
        // 2. Process votes with optimized aggregation
        let finality_proof = self.aggregate_votes_optimized(votes).await?;
        
        // 3. Quick finality check
        if self.can_finalize_quickly(&finality_proof) {
            // 4. Immediate finalization
            self.finalize_block_immediate(finality_proof).await?;
        }
        
        Ok(())
    }
}
```

## 5. Network Requirements

### 5.1 Validator Requirements
- Minimum bandwidth: 100 Mbps
- Maximum latency: 100ms
- CPU: 4+ cores
- RAM: 16+ GB
- Storage: NVMe SSD

### 5.2 Network Topology
- Minimum peers: 8
- Optimal peers: 16-24
- Maximum peers: 50
- Geographic distribution: At least 3 continents

## 6. Monitoring and Metrics

### 6.1 Key Performance Indicators
```rust
pub struct ConsensusMetrics {
    // Block production metrics
    pub block_production_time: Histogram,
    pub block_propagation_time: Histogram,
    
    // Finality metrics
    pub time_to_finality: Histogram,
    pub votes_processed_per_second: Counter,
    
    // Network metrics
    pub network_messages_per_second: Counter,
    pub peer_connection_stability: Gauge,
}
```

## 7. Security Considerations

### 7.1 Attack Prevention
- Equivocation detection and slashing
- Vote signature aggregation
- Network message validation
- Peer reputation tracking

### 7.2 Recovery Mechanisms
- Automatic recovery from network partitions
- Vote recovery for missed rounds
- Block recovery optimization
- State sync optimization

## 8. Future Optimizations

### 8.1 Planned Improvements
- Dynamic slot time adjustment
- Enhanced vote aggregation
- Improved network topology
- Adaptive state pruning

### 8.2 Research Areas
- Zero-knowledge finality proofs
- Enhanced fork choice rules
- Network layer optimizations
- State commitment optimizations
