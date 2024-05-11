#![no_std]
use soroban_sdk::mock::MockEnvBuilder;
use soroban_sdk::storage::{MockStorage, MockStorageInstance};
use soroban_sdk::{Address, Env};

use crate::{VotingContract, StorageKey};

#[test]
fn test_vote() {
    let mut storage = MockStorage::new();
    let mut instance = MockStorageInstance::new();
    instance.set(&StorageKey::TotalVotes, &0);
    storage.set_instance(instance);

    let env = MockEnvBuilder::new()
        .set_storage(storage)
        .set_sender(Address::new([0; 20]))
        .build();

    VotingContract::vote(env.clone(), env.sender(), 1);

    assert_eq!(VotingContract::get_total_votes(env.clone()), 1);
    assert_eq!(VotingContract::get_candidate_votes(env.clone(), 1), 1);

    assert!(std::panic::catch_unwind(|| VotingContract::vote(env.clone(), env.sender(), 1)).is_err());
}

#[test]
fn test_has_voted() {
    let mut storage = MockStorage::new();
    let mut instance = MockStorageInstance::new();
    instance.set(&StorageKey::VotedUsers(Address::new([0; 20])), &true);
    storage.set_instance(instance);

    let env = MockEnvBuilder::new()
        .set_storage(storage)
        .set_sender(Address::new([0; 20]))
        .build();

    assert_eq!(VotingContract::has_voted(env.clone(), env.sender()), true);

    let different_user = Address::new([1; 20]);
    assert_eq!(VotingContract::has_voted(env.clone(), different_user), false);
}
