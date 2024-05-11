#![no_std]

use soroban_sdk::{ contract, contractimpl, contracttype, Address, Env };

#[contracttype]
pub enum StorageKey {
    TotalVotes,
    CandidateVotes(u64),
    VotedUsers(Address),
}

#[contract]
pub struct VotingContract;

#[contractimpl]
impl VotingContract {
    pub fn vote(env: Env, from: Address, candidate_id: u64) {
        from.require_auth();

        let voted_status: bool = env
            .storage()
            .instance()
            .get(&StorageKey::VotedUsers(from.clone()))
            .unwrap();
        if voted_status {
            panic!("You have already voted");
        }

        let mut total_votes: u64 = env
            .storage()
            .instance()
            .get(&StorageKey::TotalVotes)
            .unwrap_or(0);
        total_votes += 1;
        env.storage().instance().set(&StorageKey::TotalVotes, &total_votes);

        let mut candidate_votes: u64 = env
            .storage()
            .instance()
            .get(&StorageKey::CandidateVotes(candidate_id))
            .unwrap_or(0);
        candidate_votes += 1;
        env.storage().instance().set(&StorageKey::CandidateVotes(candidate_id), &candidate_votes);

        env.storage().instance().set(&StorageKey::VotedUsers(from), &true);
    }

    pub fn get_total_votes(env: Env) -> u64 {
        env.storage().instance().get(&StorageKey::TotalVotes).unwrap_or(0)
    }

    pub fn get_candidate_votes(env: Env, candidate_id: u64) -> u64 {
        env.storage().instance().get(&StorageKey::CandidateVotes(candidate_id)).unwrap_or(0)
    }

    pub fn has_voted(env: Env, user: Address) -> bool {
        let votingstatus: bool = env
            .storage()
            .instance()
            .get(&StorageKey::VotedUsers(user))
            .unwrap();
        votingstatus
    }
}
