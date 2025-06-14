use std::{
    collections::HashMap,
    fmt::Debug,
    sync::{LazyLock, Mutex},
};

use vcmp_bindings::options::VcmpEntityPool;

use crate::{consts::EntityId, func::player::PlayerPy};

pub trait EntityPoolTrait {
    fn entity_pool_type() -> VcmpEntityPool;
    fn entity_id(&self) -> EntityId;
}

#[derive(Debug, Clone)]
pub struct AnEntityPool<E>
where
    E: EntityPoolTrait + Debug + Clone,
{
    pool: HashMap<EntityId, E>,
}

impl<E: EntityPoolTrait + Debug + Clone> AnEntityPool<E> {
    pub fn pool_type(&self) -> VcmpEntityPool {
        E::entity_pool_type()
    }

    pub fn add_entity(&mut self, entity: E) {
        self.pool.insert(entity.entity_id(), entity);
    }

    pub fn remove_entity(&mut self, entity_id: EntityId) {
        self.pool.remove(&entity_id);
    }

    pub fn new() -> Self {
        Self {
            pool: HashMap::new(),
        }
    }
}

#[derive(Debug, Clone)]
/// 实体池
pub struct EntityPool {
    players: AnEntityPool<PlayerPy>,
}

impl EntityPool {
    fn new() -> Self {
        Self {
            players: AnEntityPool::new(),
        }
    }

    pub fn insert(&mut self, entity_type: VcmpEntityPool, entity_id: EntityId) {
        match entity_type {
            VcmpEntityPool::Player => {
                self.insert_raw_player(entity_id);
            }
            _ => {
                todo!()
            }
        }
    }
    pub fn remove(&self, entity_type: VcmpEntityPool, entity_id: EntityId) {}
    pub fn get_players(&self) -> Vec<&PlayerPy> {
        self.players.values().collect()
    }
    pub fn insert_raw_player(&mut self, player_id: i32) {
        let plr = PlayerPy::new(player_id);
        self.insert_player(plr);
    }
    pub fn remove_player(&mut self, player_id: i32) -> bool {
        // check if player exists
        if self.players.contains_key(&player_id) {
            self.players.remove(&player_id);
            true
        } else {
            false
        }
    }
}

/// 全局实体列表
///
/// Thread safe
pub static ENTITY_POOL: LazyLock<Mutex<EntityPool>> =
    LazyLock::new(|| Mutex::new(EntityPool::new()));
