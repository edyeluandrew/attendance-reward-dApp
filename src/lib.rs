#![no_std]

use soroban_sdk::{contract, contractimpl, Address, Env, log, Vec, Symbol, symbol_short};

mod storage;
use storage::*;

#[contract]
pub struct AttendanceContract;

#[contractimpl]
impl AttendanceContract {
    // Initialize admin
    pub fn init_admin(env: Env, admin: Address) {
        assert!(
            env.storage().persistent().get::<_, Address>(&DataKey::Admin).is_none(),
            "Admin already set"
        );
        env.storage().persistent().set(&DataKey::Admin, &admin);
        log!(&env, "Admin initialized");
    }

    // Organizer sets event code (using Symbol instead of Vec<u8>)
    pub fn set_code(env: Env, admin: Address, code: Symbol) {
        Self::require_admin(&env, &admin);
        env.storage().persistent().set(&DataKey::Code, &code);
        log!(&env, "Event code set");
    }

    // Set session time window
    pub fn set_time_window(env: Env, admin: Address, start: u64, end: u64) {
        Self::require_admin(&env, &admin);
        env.storage().persistent().set(&DataKey::StartTime, &start);
        env.storage().persistent().set(&DataKey::EndTime, &end);
        log!(&env, "Time window set");
    }

    // Set reward amount in XLM
    pub fn set_reward_amount(env: Env, admin: Address, amount: i128) {
        Self::require_admin(&env, &admin);
        env.storage().persistent().set(&DataKey::RewardAmount, &amount);
        log!(&env, "Reward amount set to {} XLM", amount);
    }

    // Attendee check-in during session
    pub fn attend(env: Env, user: Address, code: Symbol) {
        user.require_auth();

        let stored_code: Symbol = env
            .storage()
            .persistent()
            .get(&DataKey::Code)
            .expect("Event code not set");
        assert_eq!(stored_code, code, "Invalid code");

        // Check time window
        if let (Some(start), Some(end)) = (
            env.storage().persistent().get::<_, u64>(&DataKey::StartTime),
            env.storage().persistent().get::<_, u64>(&DataKey::EndTime),
        ) {
            let now = env.ledger().timestamp();
            assert!(now >= start && now <= end, "Outside attendance window");
        }

        // Prevent duplicate attendance
        let attended: Option<i128> = env.storage().persistent().get(&DataKey::Attendee(user.clone()));
        assert!(attended.is_none(), "Already checked in");

        // Record attendance (no XLM transfer yet)
        let reward: i128 = env.storage().persistent().get(&DataKey::RewardAmount).unwrap_or(100);
        env.storage().persistent().set(&DataKey::Attendee(user.clone()), &reward);
        push_attendee(&env, &user);
        log!(&env, "{} checked in; reward pending", user);
    }

    // After session, admin distributes rewards
    pub fn distribute_rewards(env: Env, admin: Address) {
        Self::require_admin(&env, &admin);
        assert!(!is_reward_distributed(&env), "Rewards already distributed");

        // Ensure session has ended
        let now = env.ledger().timestamp();
        let end_time: u64 = env
            .storage()
            .persistent()
            .get(&DataKey::EndTime)
            .expect("End time not set");
        assert!(now >= end_time, "Session not over yet");

        let attendees = get_attendees(&env);
        let reward: i128 = env.storage().persistent().get(&DataKey::RewardAmount).unwrap_or(100);

        for addr in attendees.iter() {
            // TODO: Replace with actual XLM transfer logic
            log!(&env, "Distributed {} XLM to {}", reward, addr);
        }

        mark_reward_distributed(&env);
        log!(&env, "All rewards distributed");
    }

    // Read attendees
    pub fn get_attendees(env: Env) -> Vec<Address> {
        get_attendees(&env)
    }

    // Check if user attended
    pub fn has_attended(env: Env, user: Address) -> bool {
        env.storage()
            .persistent()
            .get::<_, i128>(&DataKey::Attendee(user))
            .is_some()
    }

    // Admin check helper
    fn require_admin(env: &Env, caller: &Address) {
        let admin: Address = env
            .storage()
            .persistent()
            .get(&DataKey::Admin)
            .expect("Admin not set");
        assert_eq!(admin, *caller, "Not authorized");
    }
}