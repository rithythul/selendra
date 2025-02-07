# Selendra Network Operations Guide

## Table of Contents
1. [Network Monitoring](#network-monitoring)
2. [Performance Metrics](#performance-metrics)
3. [Troubleshooting Guide](#troubleshooting-guide)
4. [Backup Procedures](#backup-procedures)
5. [Emergency Response](#emergency-response)

## Network Monitoring

### 1. Monitoring Infrastructure
```yaml
# prometheus.yml
global:
  scrape_interval: 15s
  evaluation_interval: 15s

scrape_configs:
  - job_name: 'selendra_nodes'
    static_configs:
      - targets: ['localhost:9615']
    metrics_path: /metrics

  - job_name: 'node_exporter'
    static_configs:
      - targets: ['localhost:9100']

  - job_name: 'validator_metrics'
    static_configs:
      - targets: ['localhost:9616']
```

### 2. Alert Rules
```yaml
# alerts.yml
groups:
- name: selendra_alerts
  rules:
  - alert: HighBlockTime
    expr: selendra_block_time > 6
    for: 5m
    labels:
      severity: warning
    annotations:
      description: Block time is higher than expected

  - alert: LowValidatorCount
    expr: selendra_validator_count < 10
    for: 5m
    labels:
      severity: critical
    annotations:
      description: Number of validators is too low

  - alert: HighMemoryUsage
    expr: node_memory_MemAvailable_bytes / node_memory_MemTotal_bytes * 100 < 10
    for: 5m
    labels:
      severity: warning
    annotations:
      description: Less than 10% memory available
```

### 3. Grafana Dashboards
```json
{
  "dashboard": {
    "title": "Selendra Network Overview",
    "panels": [
      {
        "title": "Block Production",
        "type": "graph",
        "datasource": "Prometheus",
        "targets": [
          {
            "expr": "rate(selendra_block_height[5m])"
          }
        ]
      },
      {
        "title": "Network Peers",
        "type": "graph",
        "datasource": "Prometheus",
        "targets": [
          {
            "expr": "selendra_sub_libp2p_peers_count"
          }
        ]
      }
    ]
  }
}
```

## Performance Metrics

### 1. System Metrics
```rust
pub struct SystemMetrics {
    // Resource usage
    cpu_usage: f64,
    memory_usage: f64,
    disk_usage: f64,
    network_bandwidth: f64,
    
    // System load
    load_average: f64,
    process_count: u32,
    thread_count: u32,
    
    // IO metrics
    io_read_bytes: u64,
    io_write_bytes: u64,
    io_ops: u32,
}
```

### 2. Network Metrics
```rust
pub struct NetworkMetrics {
    // Block metrics
    block_time: Duration,
    block_size: usize,
    transactions_per_block: u32,
    
    // Peer metrics
    peer_count: u32,
    peer_bandwidth: HashMap<PeerId, Bandwidth>,
    peer_latency: HashMap<PeerId, Duration>,
    
    // Transaction metrics
    transaction_pool_size: u32,
    transaction_throughput: f64,
    transaction_latency: Duration,
}
```

### 3. Validator Metrics
```rust
pub struct ValidatorMetrics {
    // Performance metrics
    blocks_authored: u32,
    era_points: u32,
    session_uptime: f64,
    
    // Staking metrics
    total_stake: Balance,
    own_stake: Balance,
    nominators: u32,
    
    // Rewards metrics
    era_rewards: Balance,
    commission: Perbill,
    reward_points: u32,
}
```

## Troubleshooting Guide

### 1. Common Issues
```rust
pub enum CommonIssue {
    // Node issues
    NodeOutOfSync {
        current_block: BlockNumber,
        network_block: BlockNumber,
        peer_count: u32,
    },
    
    HighResourceUsage {
        cpu: f64,
        memory: f64,
        disk: f64,
    },
    
    NetworkConnectivity {
        peer_count: u32,
        bandwidth: f64,
        latency: Duration,
    },
}

pub struct TroubleshootingStep {
    issue: CommonIssue,
    diagnosis: String,
    solution: String,
    prevention: String,
}
```

### 2. Diagnostic Tools
```rust
pub struct Diagnostics {
    // Chain diagnostics
    pub fn check_chain_state() -> ChainStatus {
        // Check finality
        let finality_lag = Self::check_finality()?;
        
        // Check block production
        let block_production = Self::check_block_production()?;
        
        // Check transaction pool
        let tx_pool = Self::check_transaction_pool()?;
        
        Ok(ChainStatus {
            finality_lag,
            block_production,
            tx_pool,
        })
    }
    
    // Network diagnostics
    pub fn check_network_health() -> NetworkStatus {
        // Check peer connectivity
        let peer_status = Self::check_peers()?;
        
        // Check bandwidth usage
        let bandwidth = Self::check_bandwidth()?;
        
        // Check latency
        let latency = Self::check_latency()?;
        
        Ok(NetworkStatus {
            peer_status,
            bandwidth,
            latency,
        })
    }
}
```

### 3. Recovery Procedures
```rust
pub struct Recovery {
    // Node recovery
    pub fn recover_node(
        &self,
        issue: CommonIssue,
    ) -> Result<(), Error> {
        match issue {
            CommonIssue::NodeOutOfSync { .. } => {
                self.resync_node()?;
            },
            CommonIssue::HighResourceUsage { .. } => {
                self.optimize_resources()?;
            },
            CommonIssue::NetworkConnectivity { .. } => {
                self.restore_connectivity()?;
            },
        }
        Ok(())
    }
}
```

## Backup Procedures

### 1. Database Backup
```rust
pub struct DatabaseBackup {
    // Backup configuration
    pub struct BackupConfig {
        frequency: Duration,
        retention: Duration,
        compression: bool,
        encryption: bool,
    }
    
    // Backup operations
    pub fn create_backup(&self) -> Result<(), Error> {
        // Stop database writes
        self.pause_writes()?;
        
        // Create snapshot
        let snapshot = self.create_snapshot()?;
        
        // Compress and encrypt
        let backup = self.process_backup(snapshot)?;
        
        // Store backup
        self.store_backup(backup)?;
        
        // Resume database writes
        self.resume_writes()?;
        
        Ok(())
    }
}
```

### 2. State Backup
```rust
pub struct StateBackup {
    // State snapshot
    pub fn create_state_snapshot(
        &self,
        block_number: BlockNumber,
    ) -> Result<StateSnapshot, Error> {
        // Get state root
        let state_root = self.get_state_root(block_number)?;
        
        // Create merkle proof
        let proof = self.create_proof(state_root)?;
        
        // Verify snapshot
        self.verify_snapshot(proof)?;
        
        Ok(StateSnapshot {
            block_number,
            state_root,
            proof,
        })
    }
}
```

### 3. Key Backup
```rust
pub struct KeyBackup {
    // Key backup procedure
    pub fn backup_keys(
        &self,
        keys: Vec<NetworkKey>,
    ) -> Result<(), Error> {
        // Encrypt keys
        let encrypted = self.encrypt_keys(keys)?;
        
        // Split into shares
        let shares = self.create_shares(encrypted)?;
        
        // Distribute shares
        self.distribute_shares(shares)?;
        
        Ok(())
    }
}
```

## Emergency Response

### 1. Incident Response
```rust
pub struct IncidentResponse {
    // Response levels
    pub enum Severity {
        Low,
        Medium,
        High,
        Critical,
    }
    
    // Response procedure
    pub fn handle_incident(
        &self,
        incident: Incident,
    ) -> Result<(), Error> {
        // Assess severity
        let severity = self.assess_severity(incident)?;
        
        // Execute response plan
        match severity {
            Severity::Low => self.handle_low_severity(incident)?,
            Severity::Medium => self.handle_medium_severity(incident)?,
            Severity::High => self.handle_high_severity(incident)?,
            Severity::Critical => self.handle_critical_severity(incident)?,
        }
        
        Ok(())
    }
}
```

### 2. Communication Plan
```rust
pub struct CommunicationPlan {
    // Stakeholder communication
    pub fn notify_stakeholders(
        &self,
        incident: &Incident,
    ) -> Result<(), Error> {
        // Prepare message
        let message = self.prepare_message(incident)?;
        
        // Determine channels
        let channels = self.get_communication_channels(incident)?;
        
        // Send notifications
        self.send_notifications(message, channels)?;
        
        Ok(())
    }
}
```

### 3. Recovery Plan
```rust
pub struct RecoveryPlan {
    // Recovery steps
    pub fn execute_recovery(
        &self,
        incident: &Incident,
    ) -> Result<(), Error> {
        // Stop affected services
        self.stop_services()?;
        
        // Apply fixes
        self.apply_fixes(incident)?;
        
        // Verify fixes
        self.verify_fixes()?;
        
        // Restart services
        self.restart_services()?;
        
        Ok(())
    }
}
```
