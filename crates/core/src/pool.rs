use std::{
    collections::HashMap,
    fmt::Debug,
    sync::{LazyLock, Mutex},
};

use tracing::{Level, event};
use vcmp_bindings::options::VcmpEntityPool;

use crate::{
    consts::EntityId,
    functions::{
        checkpoint::CheckPointPy, marker::MarkerPy, object::ObjectPy, pickup::PickupPy,
        player::PlayerPy, vehicle::VehiclePy,
    },
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

    pub fn get_mut_entity(&mut self, entity_id: EntityId) -> Option<&mut E> {
        self.pool.get_mut(&entity_id)
    }

    pub fn entities(&self) -> impl Iterator<Item = &E> {
        self.pool.values()
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

#[derive(Debug, Clone, Default)]
/// 实体池
pub struct EntityPool {
    players: AnEntityPool<PlayerPy>,
    vehicles: AnEntityPool<VehiclePy>,
    objects: AnEntityPool<ObjectPy>,
    pickups: AnEntityPool<PickupPy>,
    markers: AnEntityPool<MarkerPy>,
    checkpoints: AnEntityPool<CheckPointPy>,
}

impl EntityPool {
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
            VcmpEntityPool::Object => {
                self.objects.insert_raw_entity(entity_id);
            }
            VcmpEntityPool::Pickup => {
                self.pickups.insert_raw_entity(entity_id);
            }
            VcmpEntityPool::Marker => {
                self.markers.insert_raw_entity(entity_id);
            }
            VcmpEntityPool::CheckPoint => {
                self.checkpoints.insert_raw_entity(entity_id);
            }
            _ => {
                event!(Level::ERROR, "Unknown entity type: {:?}", entity_type);
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
            VcmpEntityPool::Radio => {
                // ignore
            }
            VcmpEntityPool::Object => {
                self.objects.remove_entity(entity_id);
            }
            VcmpEntityPool::Pickup => {
                self.pickups.remove_entity(entity_id);
            }
            VcmpEntityPool::Marker => {
                self.markers.remove_entity(entity_id);
            }
            VcmpEntityPool::CheckPoint => {
                self.checkpoints.remove_entity(entity_id);
            }
            _ => {
                event!(Level::ERROR, "Unknown entity type: {:?}", entity_type);
            }
        }
    }

    pub fn contains(&self, entity_type: VcmpEntityPool, entity_id: EntityId) -> bool {
        match entity_type {
            VcmpEntityPool::Player => self.players.have_entity(entity_id),
            VcmpEntityPool::Vehicle => self.vehicles.have_entity(entity_id),
            VcmpEntityPool::Radio => false,
            VcmpEntityPool::Object => self.objects.have_entity(entity_id),
            VcmpEntityPool::Pickup => self.pickups.have_entity(entity_id),
            VcmpEntityPool::Marker => self.markers.have_entity(entity_id),
            VcmpEntityPool::CheckPoint => self.checkpoints.have_entity(entity_id),
            _ => false,
        }
    }

    // 更具体的获取方法
    pub fn get_player(&self, player_id: EntityId) -> Option<&PlayerPy> {
        self.players.get_entity(player_id)
    }

    pub fn get_mut_player(&mut self, player_id: EntityId) -> Option<&mut PlayerPy> {
        self.players.get_mut_entity(player_id)
    }

    pub fn get_vehicle(&self, vehicle_id: EntityId) -> Option<&VehiclePy> {
        self.vehicles.get_entity(vehicle_id)
    }

    pub fn get_mut_vehicle(&mut self, vehicle_id: EntityId) -> Option<&mut VehiclePy> {
        self.vehicles.get_mut_entity(vehicle_id)
    }

    pub fn get_object(&self, object_id: EntityId) -> Option<&ObjectPy> {
        self.objects.get_entity(object_id)
    }

    pub fn get_pickup(&self, pickup_id: EntityId) -> Option<&PickupPy> {
        self.pickups.get_entity(pickup_id)
    }

    pub fn get_marker(&self, marker_id: EntityId) -> Option<&MarkerPy> {
        self.markers.get_entity(marker_id)
    }

    pub fn get_checkpoint(&self, checkpoint_id: EntityId) -> Option<&CheckPointPy> {
        self.checkpoints.get_entity(checkpoint_id)
    }

    pub fn get_players(&self) -> Vec<PlayerPy> {
        self.players
            .entities()
            .cloned()
            .filter(|p| p.get_var_reload_joined())
            .collect()
    }

    pub fn get_all_players(&self) -> Vec<PlayerPy> {
        self.players.entities().cloned().collect()
    }

    pub fn get_vehicles(&self) -> Vec<VehiclePy> {
        self.vehicles.entities().cloned().collect()
    }

    pub fn get_objects(&self) -> Vec<ObjectPy> {
        self.objects.entities().cloned().collect()
    }

    pub fn get_pickups(&self) -> Vec<PickupPy> {
        self.pickups.entities().cloned().collect()
    }

    pub fn get_markers(&self) -> Vec<MarkerPy> {
        self.markers.entities().cloned().collect()
    }

    pub fn get_checkpoints(&self) -> Vec<CheckPointPy> {
        self.checkpoints.entities().cloned().collect()
    }
}

/// 全局实体列表
///
/// Thread safe
pub static ENTITY_POOL: LazyLock<Mutex<EntityPool>> =
    LazyLock::new(|| Mutex::new(EntityPool::default()));
