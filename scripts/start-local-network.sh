#!/bin/bash

# Ensure we're in the project root
cd "$(dirname "$0")/.."

# Clean previous chain data
rm -rf /tmp/selendra

# Start the first validator node
cargo run --release -- \
    --base-path /tmp/selendra/node1 \
    --chain=local \
    --port 30333 \
    --ws-port 9945 \
    --rpc-port 9934 \
    --validator \
    --name selendra-node1 \
    --node-key 0000000000000000000000000000000000000000000000000000000000000001

# Optional: Start a second validator node in another terminal
# cargo run --release -- \
#     --base-path /tmp/selendra/node2 \
#     --chain=local \
#     --port 30334 \
#     --ws-port 9946 \
#     --rpc-port 9935 \
#     --validator \
#     --name selendra-node2 \
#     --bootnodes /ip4/127.0.0.1/tcp/30333/p2p/FIRST_NODE_PEER_ID
