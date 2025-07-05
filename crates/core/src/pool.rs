use std::{
    collections::HashMap,
    fmt::Debug,
    sync::{LazyLock, Mutex},
};

use tracing::{Level, event};
use vcmp_bindings::options::VcmpEntityPool;

use crate::{
    consts::EntityId,
    functions::{object::ObjectPy, player::PlayerPy, vehicle::VehiclePy},
};

pub trait EntityPoolTrait: Debug + Clone {
    fn entity_pool_type() -> VcmpEntityPool;
    fn entity_id(&self) -> EntityId;
}

#[derive(Debug, Clone)]
pub struct AnEntityPool<E>
where
    E: EntityPoolTrait,
{
    pool: HashMap<EntityId, E>,
}

impl<E: EntityPoolTrait> AnEntityPool<E> {
    pub fn pool_type(&self) -> VcmpEntityPool {
        E::entity_pool_type()
    }

    pub fn add_entity(&mut self, entity: E) {
        self.pool.insert(entity.entity_id(), entity);
    }

    pub fn remove_entity(&mut self, entity_id: EntityId) {
        self.pool.remove(&entity_id);
    }

    pub fn have_entity(&self, entity_id: EntityId) -> bool {
        self.pool.contains_key(&entity_id)
    }

    pub fn insert_raw_entity(&mut self, entity: impl Into<E>) {
        self.add_entity(entity.into());
    }

    pub fn get_entity(&self, entity_id: EntityId) -> Option<&E> {
        self.pool.get(&entity_id)
    }

    pub fn new() -> Self {
        Self {
            pool: HashMap::new(),
        }
    }
}

impl<E: EntityPoolTrait> Default for AnEntityPool<E> {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone)]
/// 实体池
pub struct EntityPool {
    players: AnEntityPool<PlayerPy>,
    vehicles: AnEntityPool<VehiclePy>,
    objects: AnEntityPool<ObjectPy>,
}

impl EntityPool {
    fn new() -> Self {
        Self {
            players: AnEntityPool::new(),
            vehicles: AnEntityPool::new(),
            objects: AnEntityPool::new(),
        }
    }

    pub fn insert(&mut self, entity_type: VcmpEntityPool, entity_id: EntityId) {
        match entity_type {
            VcmpEntityPool::Player => {
                self.players.insert_raw_entity(entity_id);
            }
            VcmpEntityPool::Vehicle => {
                self.vehicles.insert_raw_entity(entity_id);
            }
            VcmpEntityPool::Radio => {
                // ignore
            }
            _ => {
                event!(Level::TRACE, "Unknown entity type: {entity_type:?}");
                todo!()
            }
        }
    }
    pub fn remove(&mut self, entity_type: VcmpEntityPool, entity_id: EntityId) {
        match entity_type {
            VcmpEntityPool::Player => {
                self.players.remove_entity(entity_id);
            }
            VcmpEntityPool::Vehicle => {
                self.vehicles.remove_entity(entity_id);
            }
            _ => {
                event!(Level::TRACE, "Unknown entity type: {entity_type:?}");
                todo!()
            }
        }
    }

    pub fn contains(&self, entity_type: VcmpEntityPool, entity_id: EntityId) -> bool {
        match entity_type {
            VcmpEntityPool::Player => self.players.have_entity(entity_id),
            VcmpEntityPool::Vehicle => self.vehicles.have_entity(entity_id),
            _ => {
                event!(Level::TRACE, "Unknown entity type: {entity_type:?}");
                todo!()
            }
        }
    }

    // 更具体的获取方法
    pub fn get_player(&self, player_id: EntityId) -> Option<&PlayerPy> {
        self.players.get_entity(player_id)
    }

    pub fn get_vehicle(&self, vehicle_id: EntityId) -> Option<&VehiclePy> {
        self.vehicles.get_entity(vehicle_id)
    }

    pub fn get_object(&self, object_id: EntityId) -> Option<&ObjectPy> {
        self.objects.get_entity(object_id)
    }

    pub fn get_players(&self) -> Vec<PlayerPy> {
        self.players.pool.values().cloned().collect()
    }
}

/// 全局实体列表
///
/// Thread safe
pub static ENTITY_POOL: LazyLock<Mutex<EntityPool>> =
    LazyLock::new(|| Mutex::new(EntityPool::new()));
