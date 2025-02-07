# Selendra Network API Specification

## Table of Contents
1. [RPC Endpoints](#rpc-endpoints)
2. [WebSocket Interface](#websocket-interface)
3. [HTTP API](#http-api)
4. [SDK Documentation](#sdk-documentation)
5. [Example Integrations](#example-integrations)

## RPC Endpoints

### Chain State Queries

```json
// Get block details
{
    "jsonrpc": "2.0",
    "id": 1,
    "method": "chain_getBlock",
    "params": ["0x..."]
}

// Response
{
    "jsonrpc": "2.0",
    "id": 1,
    "result": {
        "block": {
            "header": {
                "parentHash": "0x...",
                "number": "0x...",
                "stateRoot": "0x...",
                "extrinsicsRoot": "0x...",
                "digest": {
                    "logs": [...]
                }
            },
            "extrinsics": [...]
        }
    }
}
```

### Account Management

```json
// Get account balance
{
    "jsonrpc": "2.0",
    "id": 1,
    "method": "state_getStorage",
    "params": [
        "0x26aa394eea5630e07c48ae0c9558cef7b99d880ec681799c0cf30e8886371da9"
    ]
}

// Get account nonce
{
    "jsonrpc": "2.0",
    "id": 1,
    "method": "system_accountNextIndex",
    "params": ["5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY"]
}
```

### Transaction Submission

```json
// Submit transaction
{
    "jsonrpc": "2.0",
    "id": 1,
    "method": "author_submitExtrinsic",
    "params": ["0x..."]
}

// Watch transaction status
{
    "jsonrpc": "2.0",
    "id": 1,
    "method": "author_submitAndWatchExtrinsic",
    "params": ["0x..."]
}
```

## WebSocket Interface

### Connection Setup

```javascript
const ws = new WebSocket('ws://localhost:9944');

ws.onopen = () => {
    // Subscribe to new blocks
    ws.send(JSON.stringify({
        id: 1,
        jsonrpc: '2.0',
        method: 'chain_subscribeNewHeads',
        params: []
    }));
};

ws.onmessage = (message) => {
    const response = JSON.parse(message.data);
    console.log('New block:', response.params.result);
};
```

### Event Subscription

```javascript
// Subscribe to events
{
    "jsonrpc": "2.0",
    "id": 1,
    "method": "state_subscribeStorage",
    "params": [
        [
            "0x26aa394eea5630e07c48ae0c9558cef7b99d880ec681799c0cf30e8886371da9"
        ]
    ]
}
```

## HTTP API

### RESTful Endpoints

```typescript
interface APIEndpoints {
    // Block queries
    GET /api/v1/block/latest
    GET /api/v1/block/{blockHash}
    GET /api/v1/block/{blockNumber}

    // Account queries
    GET /api/v1/account/{address}/balance
    GET /api/v1/account/{address}/transactions
    GET /api/v1/account/{address}/assets

    // Transaction operations
    POST /api/v1/transaction/submit
    GET /api/v1/transaction/{txHash}/status
    GET /api/v1/transaction/{txHash}/events
}
```

### Response Format

```typescript
interface APIResponse<T> {
    success: boolean;
    data?: T;
    error?: {
        code: number;
        message: string;
        details?: any;
    };
    meta: {
        timestamp: number;
        blockNumber: number;
    };
}
```

## SDK Documentation

### JavaScript/TypeScript SDK

```typescript
import { SelendraAPI } from '@selendra/sdk';

// Initialize API
const api = new SelendraAPI({
    nodeUrl: 'wss://rpc.selendra.org',
    types: {
        // Custom type definitions
    }
});

// Connect to node
await api.connect();

// Query chain state
const balance = await api.query.system.account(address);

// Submit transaction
const tx = api.tx.balances.transfer(recipient, amount);
await tx.signAndSend(account);
```

### Python SDK

```python
from selendra_sdk import SelendraAPI

# Initialize API
api = SelendraAPI(
    node_url='wss://rpc.selendra.org',
    types={
        # Custom type definitions
    }
)

# Connect to node
api.connect()

# Query chain state
balance = api.query.system.account(address)

# Submit transaction
tx = api.tx.balances.transfer(recipient, amount)
result = tx.sign_and_send(account)
```

## Example Integrations

### Wallet Integration

```typescript
class SelendraWallet {
    async connect() {
        // Connect to extension
        const injector = await web3Enable('Selendra Wallet');
        const accounts = await web3Accounts();
        return accounts;
    }

    async transfer(from: string, to: string, amount: number) {
        const tx = api.tx.balances.transfer(to, amount);
        const result = await tx.signAndSend(from);
        return result;
    }

    async getBalance(address: string) {
        const { data } = await api.query.system.account(address);
        return data.free.toNumber();
    }
}
```

### Smart Contract Integration

```typescript
class SelendraContract {
    async deploy(abi: any, bytecode: string, ...args: any[]) {
        const contract = new Contract(api, abi, bytecode);
        const deployed = await contract.deploy(...args);
        return deployed;
    }

    async call(address: string, method: string, ...args: any[]) {
        const contract = new Contract(api, abi, address);
        const result = await contract.call(method, ...args);
        return result;
    }
}
```

### Block Explorer Integration

```typescript
class SelendraExplorer {
    async getBlock(blockHash: string) {
        const block = await api.rpc.chain.getBlock(blockHash);
        return this.formatBlock(block);
    }

    async getTransaction(txHash: string) {
        const tx = await api.rpc.author.submitAndWatchExtrinsic(txHash);
        return this.formatTransaction(tx);
    }

    async getEvents(blockHash: string) {
        const events = await api.query.system.events.at(blockHash);
        return this.formatEvents(events);
    }
}
```
