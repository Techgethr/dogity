use crate::types::*;
use ic_cdk::api::caller;

#[ic_cdk_macros::update]
pub fn register_service(service: String, slasher: Principal) -> bool {
    // Only allow canister controller to register services (for demo, allow anyone)
    SERVICES.with(|services| {
        let mut services = services.borrow_mut();
        let entry = services.entry(service).or_default();
        entry.slashers.insert(slasher)
    })
}

#[ic_cdk_macros::update]
pub fn slash(service: String, user: Principal, amount: u64, _proof: String) -> bool {
    // Only allow authorized slasher for the service
    let slasher = caller();
    let authorized = SERVICES.with(|services| {
        services.borrow().get(&service)
            .map(|s| s.slashers.contains(&slasher))
            .unwrap_or(false)
    });
    if !authorized { return false; }
    USERS.with(|users| {
        let mut users = users.borrow_mut();
        if let Some(u) = users.get_mut(&user) {
            let assigned = u.assignments.get_mut(&service);
            if let Some(staked) = assigned {
                let to_slash = amount.min(*staked);
                *staked -= to_slash;
                u.slashed += to_slash;
                return true;
            }
        }
        false
    })
} 