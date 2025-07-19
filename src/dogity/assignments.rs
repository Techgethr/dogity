use crate::types::*;
use ic_cdk::api::caller;

#[ic_cdk_macros::update]
pub fn assign_stake(service: String, amount: u64) -> bool {
    let principal = caller();
    USERS.with(|users| {
        let mut users = users.borrow_mut();
        if let Some(user) = users.get_mut(&principal) {
            let available = user.balance - user.assignments.values().sum::<u64>() - user.slashed;
            if available >= amount {
                *user.assignments.entry(service).or_insert(0) += amount;
                return true;
            }
        }
        false
    })
}

#[ic_cdk_macros::query]
pub fn get_assignments() -> HashMap<ServiceId, u64> {
    let principal = caller();
    USERS.with(|users| {
        users.borrow().get(&principal)
            .map(|u| u.assignments.clone())
            .unwrap_or_default()
    })
} 