---
title: SEL 2.0
---

# Selendra 2.0

This document is a living document. So it will grow and change as we develop Selendra Open Network. 

Selendra is a multi-chain multi-asset interoperable nominated Proof-of-Stake system for developing and running Substrate-based and EVM compatible blockchain application. 

# Use-cases
- Digital Identity & Signature
- Stable-coins and DeFi
- DEX
- DAOs
- Assets tokenization
- Wallets, custody, payment, remittances & crypto on/off ramp
- Decentralized Storage & Computing

# Features
- On-chain tokens exchange
- Multi-assets 
- Multi-chain token utilities
- Built-in Smart Contract (EVM & Wasm)
- Bridge to EVM chains and Substrate-based networks
- 1000+ TPS on one chain
- Deflationary via fees burned function 

# To-do
Once the Selendra's network usage increase, millions of accounts created, and reach above 10 millions daily transactions, we will; 

- Selendra Native Parachains, create shards to increase TPS to 50,000-100,000
- Parathread to Polkadot Network
- Appchain with Near Protocol
- IBC to Cosmos Network 

# Network

| Key             | Value                                      |
| --------------- | ------------------------------------------ |
| Name, Precision | SEL, 12                                    |
| SS58 Format     | 208                                        |
| EVM chain id    | 222 (temperary)                            |
| SEL ERC20/SRC20 | 0x0000000000000000000000000.....           |
| Block authoring | BABE                                       |
| Finality        | GRANDPA                                    |
| Block Time      | 6s                                         |
| Block Size      | 5mb                                        |

# Endpoint

## Mainnet

| **Network**     | **URL**                                    |
|-----------------|--------------------------------------------|
| HTTP RPC        | https://mainnet.selendra.org                   |
| Websocket       | wss://mainnet.selendra.org                     |

## Testnet

| **Network**     | **URL**                                    |
|-----------------|--------------------------------------------|
| HTTP RPC        | https://testnet.selendra.org               |
| Websocket       | wss://testnet.selendra.org                 |

# Tokenomics 

Issued on three networks; 
| Key                     | SEL                         |
| ------------------------| ----------------------------|  
| Max supply              | 3,141,592,654               | 
| Mainnet	              | 425,233,741                 | 
| ERC-20                  | 87,035,376                  | 
| BEP-20                  | 15,357,667                  | 
| Total supply	          | 527,626,784                 | 

1 SEL = 1.000,000,000,000 Loka (One thousand billion or Plank 10^12)
1 Loka = 0.000,000,000,001 SEL 

## Allocation 

- BEP-20	                15,357,667
- ERC-20	                87,035,376
- Existing users	        19,113,163
- Bridge Liquidity	        180,000,000	
- Foundation Governance	    200,000,000
- Teams	                    31,415,927

## Inflation		

- max:	    10,763,227	2.50%
- average:	4,305,291	1.00%
- min:	    2,152,645	0.50%

## Token supply

All fees charged on Selendra are burned. New SEL token is generated via inflation then distribute in a form of reward to validators and stakers for helping to secure and navigate the network. Inflation is set to be between 0.5% - 2.5% per year, with initial supply of 527,626,784 SEL. Max supply set to be 3,141,592,654 SEL.  


This way SEL token's max supply will never ever reach in this century, leaving enough time for the next generation of users, builders, and governance to decide what they want to do with the max supply.  

SEL to company stocks
- Yield = Dividend  
- Burn  = Buy back
- Earn  = Fees generated

# Node specification

The most common way for a beginner to run a validator is on a cloud server running Linux. You may choose whatever VPS provider that your prefer, and whatever operating system you are comfortable with. 

The transactions weights in Selendra were benchmarked on standard hardware. It is recommended that validators run at least the standard hardware in order to ensure they are able to process all blocks in time. The following are not minimum requirements but if you decide to run with less than this beware that you might have performance issue.

## Standard Hardware

For the full details of the standard hardware please see here.

- CPU - Intel(R) Core(TM) i7-7700K CPU @ 4.20GHz
- Storage - An NVMe solid state drive of 1 TB (As it should be reasonably sized to deal with blockchain growth).
- Memory - 64GB ECC.

The specs posted above are by no means the minimum specs that you could use when running a validator, however you should be aware that if you are using less you may need to toggle some extra optimizations in order to be equal to other validators that are running the standard.

# Potential use cases

```
accelerator
amm
api
art
bridge
cex
culture
custody
dao
dapp
dashboard
defi
dex
digital securities
ecosystem
enterprise
exchange
explorer
fiat ramp
funds
gaming
governance
government
incubators
infrastructure
institutional 
launchpads
lending
marketplace
multi-chain
news
nft
nodes
non-profit
options
oracle
payments
play-to-earn
learn-to-earn
portfolio
sdk
security
stablecoin
stats
swap
tools
trading
utility
wallet
wealth management
yield farming
``` 