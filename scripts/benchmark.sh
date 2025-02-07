#!/bin/bash

# Build the node in release mode
cargo build --release

# Start the node in dev mode
./target/release/selendra --dev &
NODE_PID=$!

# Wait for node to start
sleep 10

# Run benchmark test with 10000 transactions
for i in {1..10000}; do
    curl -H "Content-Type: application/json" -d '{"id":1, "jsonrpc":"2.0", "method":"author_submitExtrinsic", "params":["0x280403000b63ce64c10c05dc1515a6e75bb219324e5c08d4fc716e6dd6d13ac0ad2f2820206060606060606060606060606060606060606060606060606060606060606060606060606"]}' http://localhost:9933
done

# Get end time
END_TIME=$(date +%s)

# Calculate TPS
TOTAL_TX=10000
DURATION=$((END_TIME - START_TIME))
TPS=$((TOTAL_TX / DURATION))

echo "Benchmark Results:"
echo "Total Transactions: $TOTAL_TX"
echo "Duration: $DURATION seconds"
echo "TPS: $TPS"

# Clean up
kill $NODE_PID
