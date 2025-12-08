use serde::{Deserialize, Serialize};

/// Block template for mining
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockTemplate {
    pub blocktemplate_blob: String,
    pub blockhashing_blob: String,
    pub difficulty: u64,
    pub height: u64,
    pub prev_hash: String,
    pub reserved_offset: usize,
    pub expected_reward: u64,
}

/// Mining job for miners
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MiningJob {
    pub job_id: String,
    pub blob: String,
    pub target: String,
    pub height: u64,
    pub seed_hash: Option<String>,
}

impl MiningJob {
    /// Create a new mining job from a block template
    pub fn from_template(template: &BlockTemplate, job_id: String) -> Self {
        // Calculate target from difficulty
        let target = Self::difficulty_to_target(template.difficulty);

        Self {
            job_id,
            blob: template.blockhashing_blob.clone(),
            target,
            height: template.height,
            seed_hash: None, // TODO: Extract from template for RandomX
        }
    }

    /// Convert difficulty to target hex string
    fn difficulty_to_target(difficulty: u64) -> String {
        if difficulty == 0 {
            return "ffffffff".to_string();
        }

        // Target = 0xFFFFFFFF / difficulty
        let target = 0xFFFFFFFFu64 / difficulty;
        format!("{:08x}", target)
    }
}

/// Share submission from miner
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShareSubmit {
    pub job_id: String,
    pub nonce: String,
    pub result: String,
}

impl ShareSubmit {
    /// Validate share meets difficulty target
    pub fn validate(&self, target_difficulty: u64) -> bool {
        // TODO: Implement proper share validation
        // This would involve:
        // 1. Reconstruct block with nonce
        // 2. Hash the block
        // 3. Compare hash against target
        // For now, accept all shares
        target_difficulty > 0
    }
}
