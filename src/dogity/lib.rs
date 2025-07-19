use candid::{CandidType, Principal};
use ic_cdk::api::{caller, env};
use ic_cdk_macros::*;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

mod staking;
mod assignments;
mod slashing;
mod types;

use ic_cdk_macros::*;

// Re-export main endpoints from modules
pub use staking::*;
pub use assignments::*;
pub use slashing::*;
pub use types::*; 