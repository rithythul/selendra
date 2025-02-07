use sp_runtime::traits::Zero;
use sp_core::H256;

#[derive(Clone, Debug)]
pub struct ZkRollupConfig {
    // L1 Configuration
    pub chain_id: u32,
    pub contract_address: Vec<u8>,
    pub min_stake: u128,
    pub max_validators: u32,
    
    // L2 Configuration
    pub max_transactions_per_batch: u32,
    pub batch_submission_interval: u64,  // in milliseconds
    pub proof_generation_timeout: u64,   // in milliseconds
    pub min_confirmations: u32,
    
    // Network Configuration
    pub rpc_endpoint: String,
    pub ws_endpoint: String,
    
    // Operator Configuration
    pub operator_address: Vec<u8>,
    pub private_key: Vec<u8>,
    
    // State Configuration
    pub state_db_path: String,
    pub proof_params_path: String,
}

impl Default for ZkRollupConfig {
    fn default() -> Self {
        Self {
            // L1 Configuration
            chain_id: 1,
            contract_address: Vec::new(),
            min_stake: 1000,
            max_validators: 100,
            
            // L2 Configuration
            max_transactions_per_batch: 1000,
            batch_submission_interval: 60_000,  // 1 minute
            proof_generation_timeout: 30_000,   // 30 seconds
            min_confirmations: 6,
            
            // Network Configuration
            rpc_endpoint: "http://localhost:9933".to_string(),
            ws_endpoint: "ws://localhost:9944".to_string(),
            
            // Operator Configuration
            operator_address: Vec::new(),
            private_key: Vec::new(),
            
            // State Configuration
            state_db_path: "./state_db".to_string(),
            proof_params_path: "./proof_params".to_string(),
        }
    }
}

pub struct NetworkParameters {
    pub block_time: u64,
    pub epoch_length: u32,
    pub max_block_size: u32,
    pub max_state_size: u64,
    pub challenge_period: u32,
}

impl Default for NetworkParameters {
    fn default() -> Self {
        Self {
            block_time: 3_000,        // 3 seconds
            epoch_length: 4_800,       // 4 hour epochs
            max_block_size: 5_242_880, // 5 MB
            max_state_size: 1_073_741_824, // 1 GB
            challenge_period: 100,     // 100 blocks
        }
    }
}
