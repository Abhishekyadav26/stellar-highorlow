#![cfg(test)]

use super::*;
use soroban_sdk::{testutils::Address as _, Address, Env};

fn setup() -> (Env, ContractClient<'static>, Address) {
    let env = Env::default();
    env.mock_all_auths();

    #[allow(deprecated)]
    let contract_id = env.register_contract(None, Contract);
    let client = ContractClient::new(&env, &contract_id);

    let player = Address::generate(&env);
    (env, client, player)
}

#[test]
fn test_fund_house() {
    let (_env, client, funder) = setup();
    client.fund_house(&funder, &5_000_000_000_i128);
    assert_eq!(client.get_house_balance(), 5_000_000_000_i128);
}

#[test]
fn test_start_game() {
    let (_env, client, player) = setup();
    client.start_game(&player, &75_u32, &1_000_000_i128);

    let game = client.get_game(&player).unwrap();
    assert_eq!(game.state, GameState::Active);
    assert_eq!(game.secret_number, 75);
    assert_eq!(game.bet_amount, 1_000_000_i128);
}

#[test]
fn test_player_wins() {
    let (_env, client, player) = setup();
    client.fund_house(&player, &10_000_000_i128);

    // Secret = 75 (high). Guess high → win.
    client.start_game(&player, &75_u32, &1_000_000_i128);
    let won = client.make_guess(&player, &true);
    assert!(won);

    let game = client.get_game(&player).unwrap();
    assert!(game.last_result);
    assert_eq!(game.state, GameState::Finished);
}

#[test]
fn test_player_loses() {
    let (_env, client, player) = setup();
    client.fund_house(&player, &10_000_000_i128);

    // Secret = 30 (low). Guess high → lose.
    client.start_game(&player, &30_u32, &1_000_000_i128);
    let won = client.make_guess(&player, &true);
    assert!(!won);
}

#[test]
fn test_reveal_after_finish() {
    let (_env, client, player) = setup();
    client.fund_house(&player, &10_000_000_i128);

    // 42 <= 50, guess low → win
    client.start_game(&player, &42_u32, &1_000_000_i128);
    client.make_guess(&player, &false);

    assert_eq!(client.reveal_number(&player), 42_u32);
}

#[test]
fn test_reset_and_replay() {
    let (_env, client, player) = setup();
    client.fund_house(&player, &10_000_000_i128);

    client.start_game(&player, &60_u32, &500_000_i128);
    client.make_guess(&player, &true); // 60 > 50 → win

    client.reset_game(&player);
    assert!(client.get_game(&player).is_none());

    // Fresh game
    client.start_game(&player, &20_u32, &500_000_i128);
    assert_eq!(client.get_game(&player).unwrap().state, GameState::Active);
}
