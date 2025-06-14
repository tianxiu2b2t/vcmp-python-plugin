use std::{collections::HashMap, sync::{LazyLock, OnceLock}};
use crate::func::player::RustPlayer;

pub const PLUGIN_VERSION: u32 = 1;


pub struct Pools {
    pub players: HashMap<i32, RustPlayer>,
}

impl Pools {
    fn new() -> Self {
        Self {
            players: HashMap::default(),
        }
    }
    pub fn get_players(&self) -> Vec<RustPlayer> {
        self.players.values().cloned().collect()
    }
    pub fn insert_player(&mut self, player: RustPlayer) {
        self.players.insert(player.get_id(), player);
    }
    pub fn remove_player(&mut self, player_id: i32) {
        // check if player exists
        if self.players.contains_key(&player_id) {
            self.players.remove(&player_id);
        }
    }
}

pub static POOLS: OnceLock<Pools> = OnceLock::new();


pub fn get_pool() -> &'static Pools {
    POOLS.get_or_init(|| Pools::new())
}