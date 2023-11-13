use std::sync::{
    atomic::{AtomicUsize, Ordering},
    Arc, Mutex,
};

use rand::RngCore;

use crate::{
    dice::{DiceSet, RollResult},
    tileboard::{find_next_eel, find_next_escalator, Tile},
    TurnCount,
};

pub fn play_all_games(
    tileboard: &[Tile],
    games_played: Arc<AtomicUsize>,
    games_to_play: usize,
    player_count: usize,
    results: Arc<Mutex<Vec<TurnCount>>>,
) {
    let mut rng = rand::thread_rng();
    let mut current_games_played = games_played.load(Ordering::Relaxed);
    while current_games_played < games_to_play {
        match games_played.compare_exchange_weak(
            current_games_played,
            current_games_played + 1,
            Ordering::AcqRel,
            Ordering::Relaxed,
        ) {
            Ok(_) => {
                let turn_count = play_game(&tileboard, &mut rng, player_count);
                // Ignore poisoned mutexes
                match results.lock().map_err(|e| e.into_inner()) {
                    Ok(mut results_guard) | Err(mut results_guard) => {
                        results_guard.push(turn_count)
                    }
                }
                current_games_played = games_played.load(Ordering::Relaxed);
            }
            Err(current) => current_games_played = current,
        }
    }
}

fn play_game<R>(tileboard: &[Tile], rng: &mut R, player_count: usize) -> TurnCount
where
    R: RngCore,
{
    let mut dice_set = DiceSet::new();
    let mut players = vec![Player::default(); player_count];

    let mut turn_count = 0;
    let mut game_over = false;

    while !game_over {
        for player in players.iter_mut() {
            player.play_turn(tileboard, rng, &mut dice_set);
            if player.has_won {
                game_over = true;
                break;
            }
        }
        turn_count += 1;
    }

    turn_count
}

#[derive(Clone)]
struct Player {
    location: usize,
    has_won: bool,
}

impl Player {
    fn play_turn<R>(&mut self, tileboard: &[Tile], rng: &mut R, dice_set: &mut DiceSet)
    where
        R: RngCore,
    {
        dice_set.roll_all(rng);
        match dice_set.get_result() {
            RollResult::Eels(roll) => {
                if let Some(position) = find_next_eel(tileboard, self.location) {
                    self.location = position;
                } else {
                    self.location += roll as usize;
                }
            }
            RollResult::Escalator(roll) => {
                if let Some(position) = find_next_escalator(tileboard, self.location) {
                    self.location = position;
                } else {
                    self.location += roll as usize;
                }
            }
            RollResult::Number(roll) => {
                self.location += roll as usize;
            }
        }

        self.follow_tile(tileboard)
    }

    fn follow_tile(&mut self, tileboard: &[Tile]) {
        match tileboard.get(self.location) {
            Some(Tile::Eel(dest) | Tile::Escalator(dest)) => self.location = *dest as usize,
            Some(Tile::Normal) => (),
            None => self.has_won = true,
        }
    }
}

impl Default for Player {
    fn default() -> Self {
        Player {
            location: 0,
            has_won: false,
        }
    }
}
