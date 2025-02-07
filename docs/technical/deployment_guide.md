# Selendra Network Deployment Guide

## 1. Automated Deployment Scripts

### 1.1 Node Setup Script
```bash
#!/bin/bash
# setup_node.sh

# Configuration
NODE_TYPE=$1  # validator, rpc, archive, seed
NODE_NAME=$2
CHAIN_SPEC=$3

# System update and dependencies
apt-get update && apt-get upgrade -y
apt-get install -y build-essential git clang curl libssl-dev protobuf-compiler

# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
source $HOME/.cargo/env
rustup default stable
rustup update nightly
rustup target add wasm32-unknown-unknown --toolchain nightly

# Install monitoring tools
apt-get install -y prometheus grafana

# Configure firewall
ufw allow ssh
ufw allow 30333/tcp  # p2p
ufw allow 9933/tcp   # rpc
ufw allow 9944/tcp   # ws
ufw enable

# Setup node specific requirements
case $NODE_TYPE in
  "validator")
    # Validator specific setup
    apt-get install -y fail2ban
    # Configure secure networking
    ;;
  "rpc")
    # RPC node specific setup
    apt-get install -y nginx
    # Configure load balancer
    ;;
  "archive")
    # Archive node specific setup
    # Configure storage
    ;;
  "seed")
    # Seed node specific setup
    # Configure peer discovery
    ;;
esac

# Clone and build Selendra
git clone https://github.com/selendra/selendra
cd selendra
cargo build --release

# Setup systemd service
cat > /etc/systemd/system/selendra.service << EOF
[Unit]
Description=Selendra Node
After=network.target

[Service]
Type=simple
User=selendra
ExecStart=/usr/local/bin/selendra \
    --name "$NODE_NAME" \
    --chain "$CHAIN_SPEC" \
    --port 30333 \
    --rpc-port 9933 \
    --ws-port 9944 \
    --prometheus-port 9615 \
    --validator
Restart=always
RestartSec=3
LimitNOFILE=65535

[Install]
WantedBy=multi-user.target
EOF

systemctl enable selendra
systemctl start selendra
```

### 1.2 Monitoring Setup Script
```bash
#!/bin/bash
# setup_monitoring.sh

# Prometheus configuration
cat > /etc/prometheus/prometheus.yml << EOF
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
EOF

# Grafana dashboard
cat > /etc/grafana/provisioning/dashboards/selendra.json << EOF
{
  "dashboard": {
    "title": "Selendra Node Metrics",
    "panels": [
      {
        "title": "Block Height",
        "type": "graph",
        "datasource": "Prometheus",
        "targets": [
          {
            "expr": "selendra_block_height"
          }
        ]
      },
      {
        "title": "Peers Count",
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
EOF
```

### 1.3 Backup Script
```bash
#!/bin/bash
# backup_node.sh

BACKUP_DIR="/var/backups/selendra"
CHAIN_DATA="/var/lib/selendra"
DATE=$(date +%Y%m%d_%H%M%S)

# Create backup directory
mkdir -p $BACKUP_DIR

# Stop service
systemctl stop selendra

# Create backup
tar -czf $BACKUP_DIR/selendra_backup_$DATE.tar.gz $CHAIN_DATA

# Start service
systemctl start selendra

# Cleanup old backups (keep last 7 days)
find $BACKUP_DIR -type f -mtime +7 -name '*.tar.gz' -delete
```

## 2. Deployment Procedures

### 2.1 Initial Network Setup

1. **Prepare Infrastructure**
```bash
# For each region (Cambodia, Vietnam, Thailand)
for region in cambodia vietnam thailand; do
  # Deploy validators
  ./setup_node.sh validator "validator-$region-1" mainnet
  ./setup_node.sh validator "validator-$region-2" mainnet
  
  # Deploy RPC node
  ./setup_node.sh rpc "rpc-$region-1" mainnet
  
  # Deploy Archive node (only in primary region)
  if [ "$region" == "cambodia" ]; then
    ./setup_node.sh archive "archive-$region-1" mainnet
  fi
done
```

2. **Configure Load Balancers**
```bash
# Install HAProxy
apt-get install -y haproxy

# Configure HAProxy
cat > /etc/haproxy/haproxy.cfg << EOF
frontend selendra_rpc
    bind *:9933
    mode tcp
    default_backend rpc_nodes

backend rpc_nodes
    mode tcp
    balance roundrobin
    server rpc1 rpc-cambodia-1:9933 check
    server rpc2 rpc-vietnam-1:9933 check
    server rpc3 rpc-thailand-1:9933 check
EOF
```

3. **Setup Monitoring**
```bash
# Deploy monitoring for all nodes
for node in $(cat nodes.txt); do
  ssh $node "bash -s" < setup_monitoring.sh
done

# Setup central monitoring
./setup_central_monitoring.sh
```

### 2.2 Validator Setup

1. **Generate Keys**
```bash
# On secure offline machine
selendra key generate --scheme sr25519 --password-interactive
```

2. **Configure Validator**
```bash
# Session keys
curl -H "Content-Type: application/json" -d '{"id":1, "jsonrpc":"2.0", "method": "author_rotateKeys", "params":[]}' http://localhost:9933
```

3. **Start Validation**
```bash
# Bond tokens
selendra tx staking bond <CONTROLLER_ADDRESS> <AMOUNT> <REWARD_DESTINATION>

# Set session keys
selendra tx session set-keys <SESSION_KEYS> <PROOF>

# Validate
selendra tx staking validate --commission <COMMISSION_RATE>
```

### 2.3 Maintenance Procedures

1. **Regular Updates**
```bash
#!/bin/bash
# update_node.sh

# Stop service
systemctl stop selendra

# Update code
cd /path/to/selendra
git pull
cargo build --release

# Start service
systemctl start selendra
```

2. **Performance Monitoring**
```bash
# Check node status
curl -H "Content-Type: application/json" -d '{"id":1, "jsonrpc":"2.0", "method": "system_health", "params":[]}' http://localhost:9933

# Monitor sync status
curl -H "Content-Type: application/json" -d '{"id":1, "jsonrpc":"2.0", "method": "system_syncState", "params":[]}' http://localhost:9933
```

3. **Backup Procedures**
```bash
# Schedule regular backups
(crontab -l 2>/dev/null; echo "0 2 * * * /path/to/backup_node.sh") | crontab -
```

## 3. Emergency Procedures

### 3.1 Network Recovery
```bash
#!/bin/bash
# recover_network.sh

# Check network status
check_network_status() {
    # Implementation
}

# Activate backup validators
activate_backup_validators() {
    # Implementation
}

# Switch to backup RPC nodes
switch_rpc_nodes() {
    # Implementation
}

# Main recovery logic
if ! check_network_status; then
    activate_backup_validators
    switch_rpc_nodes
    notify_team
fi
```

### 3.2 DDoS Mitigation
```bash
# Configure fail2ban
cat > /etc/fail2ban/jail.local << EOF
[selendra-rpc]
enabled = true
port = 9933
filter = selendra-rpc
logpath = /var/log/selendra/rpc.log
maxretry = 3
bantime = 3600
EOF
```

## 4. Monitoring and Alerts

### 4.1 Alert Configuration
```yaml
# alertmanager.yml
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

  - alert: LowPeerCount
    expr: selendra_sub_libp2p_peers_count < 3
    for: 5m
    labels:
      severity: warning
    annotations:
      description: Node has low peer count
```

### 4.2 Performance Metrics
```yaml
# prometheus.yml additions
  - alert: HighMemoryUsage
    expr: node_memory_MemAvailable_bytes / node_memory_MemTotal_bytes * 100 < 10
    for: 5m
    labels:
      severity: warning
    annotations:
      description: Less than 10% memory available

  - alert: HighCPUUsage
    expr: 100 - (avg by(instance) (irate(node_cpu_seconds_total{mode="idle"}[5m])) * 100) > 90
    for: 5m
    labels:
      severity: warning
    annotations:
      description: CPU usage is above 90%
```
