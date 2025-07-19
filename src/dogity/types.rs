use candid::{CandidType, Principal};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

pub type ServiceId = String;
pub type OperatorId = Principal;

#[derive(Default, CandidType, Serialize, Deserialize, Clone)]
pub struct UserData {
    pub balance: u64, // total DOGE deposited
    pub assignments: HashMap<ServiceId, u64>, // service_id -> staked amount
    pub slashed: u64, // total slashed
}

#[derive(Default, CandidType, Serialize, Deserialize, Clone)]
pub struct ServiceData {
    pub slashers: HashSet<Principal>, // canisters allowed to slash
}

thread_local! {
    pub static USERS: std::cell::RefCell<HashMap<Principal, UserData>> = Default::default();
    pub static SERVICES: std::cell::RefCell<HashMap<ServiceId, ServiceData>> = Default::default();
} 