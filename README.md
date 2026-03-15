# 🎲 HighOrLow — Soroban Smart Contract on Stellar

> A provably fair on-chain guessing game built with Rust + Soroban SDK on the Stellar network.

---



## 📌 Project Description

**HighOrLow** is a minimalist on-chain betting game deployed on the **Stellar blockchain** using **Soroban** smart contracts. Players wager XLM (in stroops) on whether a hidden number is **High (> 50)** or **Low (≤ 50)**. If correct, the house pays out **1.8×** the bet. If wrong, the house collects the wager.

The contract is written in **Rust** using the official [Soroban SDK](https://soroban.stellar.org/) and follows the Soroban storage and auth patterns for persistent, instance, and temporary data.

---

## ⚙️ What It Does

1. **House funding** — An admin funds the contract with XLM to cover player payouts.
2. **Start a round** — A player commits a secret number (1–100) and a bet amount (stroops).
3. **Make a guess** — The player guesses `high` or `low`.
4. **Resolution** — The contract compares the guess against the secret number, updates balances, emits an event, and marks the game finished.
5. **Reveal & Reset** — The secret number is revealed after the round ends; the player resets to start a fresh game.

```
Player                    Contract
  │                          │
  │── start_game(75, bet) ──>│  stores Game { secret=75, state=Active }
  │                          │
  │── make_guess(high) ─────>│  75 > 50 → player WINS
  │<── true ─────────────────│  house pays 1.8× bet
  │                          │
  │── reveal_number() ──────>│  returns 75
  │── reset_game() ─────────>│  clears state
```

---

## ✨ Features

| Feature | Details |
|---|---|
| 🔐 **Auth enforcement** | Every mutating call requires `player.require_auth()` |
| 🎰 **1.8× payout** | Win: receive 1.8× your bet. Lose: house keeps the bet |
| 📦 **Persistent storage** | Per-player game state stored in Soroban persistent storage |
| 📡 **On-chain events** | `game/started` and `guess/resolved` events emitted for indexers |
| 🔢 **Number reveal** | Secret is revealed post-game for full transparency |
| 🔄 **Multi-round** | Reset and play again as many times as you want |
| 🧪 **Unit tests** | Full test suite covering win, loss, reveal, and reset flows |

---

## 🗂️ Project Structure

```
highorlow/
├── Cargo.toml          # Rust workspace & Soroban dependencies
└── src/
    ├── lib.rs          # Main contract logic
    └── test.rs         # Unit tests (Soroban testutils)
```

---

## 🚀 Getting Started

### Prerequisites

```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Add WASM target
rustup target add wasm32-unknown-unknown

# Install Stellar CLI
cargo install --locked stellar-cli --features opt
```

### Build

```bash
cd highorlow
stellar contract build
# Output: target/wasm32-unknown-unknown/release/highorlow.wasm
```

### Test

```bash
cargo test
```

### Deploy to Testnet

```bash
# Configure testnet identity
stellar keys generate alice --network testnet
stellar keys fund alice --network testnet

# Deploy
stellar contract deploy \
  --wasm target/wasm32-unknown-unknown/release/highorlow.wasm \
  --source alice \
  --network testnet
```

### Invoke — Fund the House

```bash
stellar contract invoke \
  --id <CONTRACT_ID> \
  --source alice \
  --network testnet \
  -- fund_house \
  --funder alice \
  --amount 10000000000
```

### Invoke — Start a Game

```bash
stellar contract invoke \
  --id <CONTRACT_ID> \
  --source alice \
  --network testnet \
  -- start_game \
  --player alice \
  --secret_number 75 \
  --bet_amount 1000000
```

### Invoke — Make a Guess

```bash
stellar contract invoke \
  --id <CONTRACT_ID> \
  --source alice \
  --network testnet \
  -- make_guess \
  --player alice \
  --guess_high true
```

---

## 🔗 Deployed Contract

| Network | Contract ID |
|---|---|
| **Stellar Testnet** | [`highorlow`](https://stellar.expert/explorer/testnet/contract/highorlow) |

> The contract alias `highorlow` is registered on Stellar Testnet.  
> View transactions, events, and storage live on [Stellar Expert](https://stellar.expert/explorer/testnet/contract/highorlow).

---

## 🧠 Contract Functions

| Function | Description |
|---|---|
| `fund_house(funder, amount)` | Admin deposits XLM into house pool |
| `start_game(player, secret_number, bet_amount)` | Begin a new round |
| `make_guess(player, guess_high)` | Submit High/Low guess; returns `bool` (won?) |
| `reset_game(player)` | Clear finished game state |
| `get_game(player)` | View current game struct |
| `get_house_balance()` | View house pool balance |
| `reveal_number(player)` | Reveal secret number after game ends |

---

## 📐 Game Logic

```
secret_number = 1–100 (committed at game start)

is_high = secret_number > 50

if player_guess == is_high:
    payout = bet × 1.8    ← player wins
else:
    house keeps bet        ← player loses
```

---

## 🛣️ Roadmap

- [ ] **VRF / Commit-Reveal** — Replace plaintext secret with Chainlink-style VRF for true on-chain randomness
- [ ] **Token support** — Accept any SEP-41 token, not just native XLM
- [ ] **Multiplayer** — Two players bet against each other instead of the house
- [ ] **Frontend** — Next.js + Freighter Wallet UI
- [ ] **Leaderboard** — Track wins/losses per player on-chain

---

## 📄 License

MIT © [Abhishek Kumar Yadav](https://github.com/abhishek)

---

<p align="center">Built with ❤️ on <strong>Stellar</strong> using <strong>Soroban</strong></p>
