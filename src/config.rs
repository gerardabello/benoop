use rand::distributions::WeightedIndex;
use rand::prelude::*;

use serde::{Deserialize, Serialize};

fn default_weight() -> i32 {
    1
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, Deserialize, Serialize)]
pub struct RequestConfig {
    pub url: String,

    #[serde(default = "default_weight")]
    pub weight: i32,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Config {
    pub concurrent: usize,
    pub total: usize,
    pub requests: Vec<RequestConfig>,
}

impl Config {
    pub fn get_random_request(&self) -> &RequestConfig {
        let weights: Vec<i32> = self.requests.iter().map(|r| r.weight).collect();
        let dist = WeightedIndex::new(&weights).unwrap();
        let mut rng = thread_rng();
        &self.requests[dist.sample(&mut rng)]
    }
}
