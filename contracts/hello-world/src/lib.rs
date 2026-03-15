#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, symbol_short, Address, Env};

// ─── Storage Keys ─────────────────────────────────────────────────────────────

#[contracttype]
pub enum DataKey {
    Game(Address),
    HouseBalance,
}

// ─── Types ────────────────────────────────────────────────────────────────────

#[contracttype]
#[derive(Clone, Debug, PartialEq)]
pub enum GameState {
    Active,
    Finished,
}

#[contracttype]
#[derive(Clone, Debug)]
pub struct Game {
    pub player: Address,
    pub secret_number: u32, // 1–100
    pub bet_amount: i128,   // in stroops
    pub state: GameState,
    pub last_result: bool,
    pub rounds_played: u32,
}

// ─── Contract ─────────────────────────────────────────────────────────────────

#[contract]
pub struct Contract;

#[contractimpl]
impl Contract {
    /// Deposit stroops into the house pool to fund player payouts.
    pub fn fund_house(env: Env, funder: Address, amount: i128) {
        funder.require_auth();
        assert!(amount > 0, "amount must be positive");

        let current: i128 = env
            .storage()
            .instance()
            .get(&DataKey::HouseBalance)
            .unwrap_or(0);

        env.storage()
            .instance()
            .set(&DataKey::HouseBalance, &(current + amount));
    }

    /// Start a new HighOrLow round.
    /// `secret_number` : 1–100 (commit-reveal recommended for production).
    /// `bet_amount`    : in stroops (1 XLM = 10_000_000 stroops).
    pub fn start_game(env: Env, player: Address, secret_number: u32, bet_amount: i128) {
        player.require_auth();

        assert!(
            secret_number >= 1 && secret_number <= 100,
            "secret_number must be 1-100"
        );
        assert!(bet_amount > 0, "bet must be positive");

        if let Some(existing) = env
            .storage()
            .persistent()
            .get::<DataKey, Game>(&DataKey::Game(player.clone()))
        {
            assert!(
                existing.state != GameState::Active,
                "finish your current game first"
            );
        }

        let game = Game {
            player: player.clone(),
            secret_number,
            bet_amount,
            state: GameState::Active,
            last_result: false,
            rounds_played: 0,
        };

        env.storage()
            .persistent()
            .set(&DataKey::Game(player.clone()), &game);

        #[allow(deprecated)]
        env.events().publish(
            (symbol_short!("game"), symbol_short!("started")),
            (player, secret_number, bet_amount),
        );
    }

    /// Guess whether the hidden number is High (> 50) or Low (<= 50).
    /// Returns `true` if the player wins.
    pub fn make_guess(env: Env, player: Address, guess_high: bool) -> bool {
        player.require_auth();

        let mut game: Game = env
            .storage()
            .persistent()
            .get(&DataKey::Game(player.clone()))
            .expect("no active game found");

        assert!(game.state == GameState::Active, "game is not active");

        let is_high = game.secret_number > 50;
        let won = guess_high == is_high;

        let mut house: i128 = env
            .storage()
            .instance()
            .get(&DataKey::HouseBalance)
            .unwrap_or(0);

        if won {
            let payout = (game.bet_amount * 18) / 10;
            assert!(house >= payout, "house has insufficient funds");
            house -= payout;
        } else {
            house += game.bet_amount;
        }

        env.storage().instance().set(&DataKey::HouseBalance, &house);

        game.last_result = won;
        game.rounds_played += 1;
        game.state = GameState::Finished;

        env.storage()
            .persistent()
            .set(&DataKey::Game(player.clone()), &game);

        #[allow(deprecated)]
        env.events().publish(
            (symbol_short!("guess"), symbol_short!("result")),
            (player, guess_high, game.secret_number, won),
        );

        won
    }

    /// Clear a finished game so the player can start a new round.
    pub fn reset_game(env: Env, player: Address) {
        player.require_auth();

        let game: Game = env
            .storage()
            .persistent()
            .get(&DataKey::Game(player.clone()))
            .expect("no game found");

        assert!(
            game.state == GameState::Finished,
            "can only reset a finished game"
        );

        env.storage().persistent().remove(&DataKey::Game(player));
    }

    // ─── Views ────────────────────────────────────────────────────────────────

    pub fn get_game(env: Env, player: Address) -> Option<Game> {
        env.storage().persistent().get(&DataKey::Game(player))
    }

    pub fn get_house_balance(env: Env) -> i128 {
        env.storage()
            .instance()
            .get(&DataKey::HouseBalance)
            .unwrap_or(0)
    }

    pub fn reveal_number(env: Env, player: Address) -> u32 {
        let game: Game = env
            .storage()
            .persistent()
            .get(&DataKey::Game(player))
            .expect("no game found");

        assert!(
            game.state == GameState::Finished,
            "number is only revealed after the game ends"
        );

        game.secret_number
    }
}

mod test;
