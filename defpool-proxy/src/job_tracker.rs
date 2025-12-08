use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use tracing::{debug, warn};

/// Job information for tracking difficulty
#[derive(Debug, Clone)]
pub struct JobInfo {
    pub job_id: String,
    pub target: String,
    pub difficulty: f64,
    pub height: u64,
}

impl JobInfo {
    /// Calculate difficulty from target hex string
    pub fn difficulty_from_target(target: &str) -> f64 {
        // Target is a hex string representing the difficulty target
        // Lower target = higher difficulty
        // Difficulty = 0xFFFFFFFF / target_value
        
        if target.is_empty() {
            return 1000.0; // Default difficulty
        }
        
        // Parse hex target (remove 0x prefix if present)
        let target_str = target.trim_start_matches("0x");
        
        match u64::from_str_radix(target_str, 16) {
            Ok(target_val) if target_val > 0 => {
                let difficulty = 0xFFFFFFFFu64 as f64 / target_val as f64;
                debug!("Calculated difficulty {} from target {}", difficulty, target);
                difficulty
            }
            _ => {
                warn!("Failed to parse target: {}, using default", target);
                1000.0
            }
        }
    }
}

/// Tracks mining jobs and their difficulties
pub struct JobTracker {
    jobs: Arc<Mutex<HashMap<String, JobInfo>>>,
    max_jobs: usize,
}

impl JobTracker {
    pub fn new(max_jobs: usize) -> Self {
        Self {
            jobs: Arc::new(Mutex::new(HashMap::new())),
            max_jobs,
        }
    }

    /// Add a new job
    pub fn add_job(&self, job_id: String, target: String, height: u64) {
        let difficulty = JobInfo::difficulty_from_target(&target);
        
        let job_info = JobInfo {
            job_id: job_id.clone(),
            target,
            difficulty,
            height,
        };

        let mut jobs = self.jobs.lock().unwrap();
        
        // Limit memory usage by removing old jobs
        if jobs.len() >= self.max_jobs {
            // Remove oldest job (simple approach - could use LRU)
            if let Some(oldest_key) = jobs.keys().next().cloned() {
                jobs.remove(&oldest_key);
            }
        }
        
        jobs.insert(job_id.clone(), job_info);
        debug!("Added job {} with difficulty {}", job_id, difficulty);
    }

    /// Get job information
    pub fn get_job(&self, job_id: &str) -> Option<JobInfo> {
        let jobs = self.jobs.lock().unwrap();
        jobs.get(job_id).cloned()
    }

    /// Get difficulty for a job
    pub fn get_difficulty(&self, job_id: &str) -> f64 {
        self.get_job(job_id)
            .map(|job| job.difficulty)
            .unwrap_or(1000.0) // Default difficulty if job not found
    }

    /// Clear all jobs
    pub fn clear(&self) {
        let mut jobs = self.jobs.lock().unwrap();
        jobs.clear();
    }

    /// Get job count
    pub fn count(&self) -> usize {
        let jobs = self.jobs.lock().unwrap();
        jobs.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_difficulty_calculation() {
        // High difficulty (low target)
        let diff1 = JobInfo::difficulty_from_target("00010000");
        assert!(diff1 > 1000.0);

        // Low difficulty (high target)
        let diff2 = JobInfo::difficulty_from_target("01000000");
        assert!(diff2 < diff1);
    }

    #[test]
    fn test_job_tracker() {
        let tracker = JobTracker::new(10);
        
        tracker.add_job("job1".to_string(), "00010000".to_string(), 100);
        tracker.add_job("job2".to_string(), "00020000".to_string(), 101);
        
        assert_eq!(tracker.count(), 2);
        
        let job1 = tracker.get_job("job1").unwrap();
        assert_eq!(job1.job_id, "job1");
        assert!(job1.difficulty > 0.0);
        
        let diff = tracker.get_difficulty("job1");
        assert!(diff > 0.0);
    }

    #[test]
    fn test_max_jobs_limit() {
        let tracker = JobTracker::new(3);
        
        for i in 0..5 {
            tracker.add_job(format!("job{}", i), "00010000".to_string(), i);
        }
        
        // Should only keep 3 jobs
        assert!(tracker.count() <= 3);
    }
}
