use crate::types::*;
use ic_cdk::api::{caller, env};

fn ck_doge_canister_id() -> Principal {
    let id = env::var("CK_DOGE_CANISTER_ID").unwrap_or_else(|_| "aaaaa-aa".to_string());
    Principal::from_text(&id).expect("Invalid CK_DOGE_CANISTER_ID")
}

#[ic_cdk_macros::query]
pub async fn get_deposit_address() -> String {
    let principal = caller();
    let canister_id = ck_doge_canister_id();
    let (address,): (String,) = ic_cdk::call(canister_id, "get_deposit_address", (principal,))
        .await
        .unwrap_or(("error_getting_address".to_string(),));
    address
}

#[ic_cdk_macros::update]
pub async fn verify_and_register_deposit(txid: String, amount: u64) -> bool {
    let principal = caller();
    let canister_id = ck_doge_canister_id();
    let (verified,): (bool,) = ic_cdk::call(canister_id, "verify_deposit", (txid.clone(), amount, principal))
        .await
        .unwrap_or((false,));
    if verified {
        USERS.with(|users| {
            let mut users = users.borrow_mut();
            let user = users.entry(principal).or_default();
            user.balance += amount;
        });
        true
    } else {
        false
    }
}

#[ic_cdk_macros::update]
pub async fn withdraw(to: String, amount: u64) -> bool {
    let principal = caller();
    let mut success = false;
    USERS.with(|users| {
        let mut users = users.borrow_mut();
        if let Some(user) = users.get_mut(&principal) {
            let available = user.balance - user.assignments.values().sum::<u64>() - user.slashed;
            if available >= amount {
                user.balance -= amount;
                success = true;
            }
        }
    });
    if success {
        let canister_id = ck_doge_canister_id();
        let (txid,): (String,) = ic_cdk::call(canister_id, "send_transaction", (to.clone(), amount, principal))
            .await
            .unwrap_or(("error_sending_tx".to_string(),));
        txid != "error_sending_tx"
    } else {
        false
    }
}

#[ic_cdk_macros::query]
pub fn get_balance() -> u64 {
    let principal = caller();
    USERS.with(|users| {
        users.borrow().get(&principal).map(|u| u.balance).unwrap_or(0)
    })
}

#[ic_cdk_macros::query]
pub fn get_slashed() -> u64 {
    let principal = caller();
    USERS.with(|users| {
        users.borrow().get(&principal).map(|u| u.slashed).unwrap_or(0)
    })
} 