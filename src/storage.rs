#![no_std]

use soroban_sdk::{contracttype, Address, Env, Vec};

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum DataKey {
    Admin,
    Code,
    StartTime,
    EndTime,
    RewardAmount,
    AttendeesList,
    Attendee(Address),
    RewardsDistributed,
}

// Add attendee to list
pub fn push_attendee(env: &Env, addr: &Address) {
    let mut list: Vec<Address> = env
        .storage()
        .persistent()
        .get(&DataKey::AttendeesList)
        .unwrap_or(Vec::new(env));

    if !list.contains(addr) {
        list.push_back(addr.clone());
        env.storage().persistent().set(&DataKey::AttendeesList, &list);
    }
}

// Get all attendees
pub fn get_attendees(env: &Env) -> Vec<Address> {
    env.storage()
        .persistent()
        .get(&DataKey::AttendeesList)
        .unwrap_or(Vec::new(env))
}

// Mark rewards distributed
pub fn mark_reward_distributed(env: &Env) {
    env.storage()
        .persistent()
        .set(&DataKey::RewardsDistributed, &true);
}

// Check if rewards already distributed
pub fn is_reward_distributed(env: &Env) -> bool {
    env.storage()
        .persistent()
        .get::<_, bool>(&DataKey::RewardsDistributed)
        .unwrap_or(false)
}
