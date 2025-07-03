import math
from typing import Optional


class Vehicle:
    # if vehicle in _vehicles, use it, else create new
    def __init__(self, vehicle_id: int):
        self._id = vehicle_id

    def __hash__(self) -> int:
        return hash(self._id)

    def __eq__(self, value: object) -> bool:
        if isinstance(value, Vehicle):
            return self._id == value._id
        return False

    @property
    def id(self):
        return self._id

    @property
    def is_alive(self):
        return funcs.check_entity_exists(vcmpEntityPool.vcmpEntityPoolVehicle, self._id)

    @property
    def doors_locked(self):
        """
        Get the vehicle doors locked
        """
        return funcs.get_vehicle_option(self._id, vcmpVehicleOption.vcmpVehicleOptionDoorsLocked)

    @doors_locked.setter
    def doors_locked(self, value: bool):
        """
        Set the vehicle doors locked
        """
        funcs.set_vehicle_option(self._id, vcmpVehicleOption.vcmpVehicleOptionDoorsLocked, value)

    @property
    def alarm(self):
        """
        Get the vehicle alarm
        """
        return funcs.get_vehicle_option(self._id, vcmpVehicleOption.vcmpVehicleOptionAlarm)
    
    @alarm.setter
    def alarm(self, value: bool):
        """
        Set the vehicle alarm
        """
        funcs.set_vehicle_option(self._id, vcmpVehicleOption.vcmpVehicleOptionAlarm, value)

    @property
    def lights(self):
        """
        Get the vehicle lights
        """
        return funcs.get_vehicle_option(self._id, vcmpVehicleOption.vcmpVehicleOptionLights)

    @lights.setter
    def lights(self, value: bool):
        """
        Set the vehicle lights
        """
        funcs.set_vehicle_option(self._id, vcmpVehicleOption.vcmpVehicleOptionLights, value)

    @property
    def radio_locked(self):
        """
        Get the vehicle radio locked
        """
        return funcs.get_vehicle_option(self._id, vcmpVehicleOption.vcmpVehicleOptionRadioLocked)

    @radio_locked.setter
    def radio_locked(self, value: bool):
        """
        Set the vehicle radio locked
        """

        funcs.set_vehicle_option(self._id, vcmpVehicleOption.vcmpVehicleOptionRadioLocked, value)

    @property
    def ghost(self):
        """
        Get the vehicle ghost
        """
        return funcs.get_vehicle_option(self._id, vcmpVehicleOption.vcmpVehicleOptionGhost)

    @ghost.setter
    def ghost(self, value: bool):
        """
        Set the vehicle ghost
        """
        funcs.set_vehicle_option(self._id, vcmpVehicleOption.vcmpVehicleOptionGhost, value)

    @property
    def siren(self):
        """
        Get the vehicle siren
        """
        return funcs.get_vehicle_option(self._id, vcmpVehicleOption.vcmpVehicleOptionSiren)
    
    @siren.setter
    def siren(self, value: bool):
        """
        Set the vehicle siren
        """
        funcs.set_vehicle_option(self._id, vcmpVehicleOption.vcmpVehicleOptionSiren, value)

    @property
    def single_use(self):
        """
        Get the vehicle single use
        """
        return funcs.get_vehicle_option(self._id, vcmpVehicleOption.vcmpVehicleOptionSingleUse)

    @single_use.setter
    def single_use(self, value: bool):
        """
        Set the vehicle single use
        """
        funcs.set_vehicle_option(self._id, vcmpVehicleOption.vcmpVehicleOptionSingleUse, value)
    
    @property
    def sync_source(self):
        """
        Get the vehicle sync source
        """
        return funcs.get_vehicle_sync_source(self._id)
    
    @property
    def sync_type(self):
        """
        Get the vehicle sync type
        """
        return vcmpVehicleSync(funcs.get_vehicle_sync_type(self._id))
    
    @property
    def world(self):
        """
        Get the vehicle world
        """
        return funcs.get_vehicle_world(self._id)

    @world.setter
    def world(self, value: int):
        """
        Set the vehicle world
        """
        funcs.set_vehicle_world(self._id, value)

    @property
    def model(self):
        """
        Get the vehicle model
        """
        return funcs.get_vehicle_model(self._id)

    @property
    def immunity(self):
        """
        Get the vehicle immutability
        """
        return funcs.get_vehicle_immunity_flags(self._id)
    
    @immunity.setter
    def immunity(self, value: int):
        """
        Set the vehicle immutability
        """
        funcs.set_vehicle_immunity_flags(self._id, value)

    @property
    def wrecked(self):
        """
        Get the vehicle wrecked
        """
        return funcs.is_vehicle_wrecked(self._id)
    
    @property
    def position(self):
        """
        Get the vehicle position
        """
        return Vector(
            **funcs.get_vehicle_position(self._id)
        )

    @position.setter
    def position(self, value: 'Vector'):
        """
        Set the vehicle position
        """
        funcs.set_vehicle_position(self._id, value.x, value.y, value.z, False)

    @property
    def rotation(self):
        """
        Get the vehicle rotation
        """
        return Quaternion(**funcs.get_vehicle_rotation(self._id))

    @rotation.setter
    def rotation(self, value: 'Quaternion'):
        """
        Set the vehicle rotation
        """
        funcs.set_vehicle_rotation(self._id, value.x, value.y, value.z, value.w)

    @property
    def rotation_euler(self):
        """
        Get the vehicle rotation euler
        """
        return Vector(**funcs.get_vehicle_rotation_euler(self._id))
    
    @rotation_euler.setter
    def rotation_euler(self, value: 'Vector'):
        """
        Set the vehicle rotation euler
        """
        funcs.set_vehicle_rotation_euler(self._id, value.x, value.y, value.z)

    @property
    def speed(self):
        """
        Get the vehicle speed
        """
        return Vector(
            **funcs.get_vehicle_speed(self._id, False)
        )
    
    @speed.setter
    def speed(self, value: 'Vector'):
        """
        Set the vehicle speed
        """
        funcs.set_vehicle_speed(self._id, value.x, value.y, value.z, False, False)

    @property
    def relative_speed(self):
        """
        Get the vehicle relative speed
        """
        return Vector(
            **funcs.get_vehicle_speed(self._id, True)
        )
    
    @relative_speed.setter
    def relative_speed(self, value: 'Vector'):
        """
        Set the vehicle relative speed
        """
        funcs.set_vehicle_speed(self._id, value.x, value.y, value.z, False, True)

    @property
    def turn_speed(self):
        """
        Get the vehicle turn speed
        """
        return Vector(
            **funcs.get_vehicle_turn_speed(self._id, False)
        )

    @turn_speed.setter
    def turn_speed(self, value: 'Vector'):
        """
        Set the vehicle turn speed
        """
        funcs.set_vehicle_turn_speed(self._id, value.x, value.y, value.z, False, False)

    @property
    def relative_turn_speed(self):
        """
        Get the vehicle relative turn speed
        """
        return Vector(
            **funcs.get_vehicle_turn_speed(self._id, True)
        )

    @relative_turn_speed.setter
    def relative_turn_speed(self, value: 'Vector'):
        """
        Set the vehicle relative turn speed
        """
        funcs.set_vehicle_turn_speed(self._id, value.x, value.y, value.z, False, True)
    
    @property
    def spawn_position(self):
        """
        Get the vehicle spawn position
        """
        return Vector(
            **funcs.get_vehicle_spawn_position(self._id)
        )

    @spawn_position.setter
    def spawn_position(self, value: 'Vector'):
        """
        Set the vehicle spawn position
        """
        funcs.set_vehicle_spawn_position(self._id, value.x, value.y, value.z)

    @property
    def spawn_rotation(self):
        """
        Get the vehicle spawn rotation
        """
        return Quaternion(**funcs.get_vehicle_spawn_rotation(self._id))

    @spawn_rotation.setter
    def spawn_rotation(self, value: 'Quaternion'):
        """
        Set the vehicle spawn rotation
        """
        funcs.set_vehicle_spawn_rotation(self._id, value.x, value.y, value.z, value.w)

    @property
    def spawn_rotation_euler(self):
        """
        Get the vehicle spawn rotation euler
        """
        return Vector(
            **funcs.get_vehicle_spawn_rotation_euler(self._id)
        )
    
    @spawn_rotation_euler.setter
    def spawn_rotation_euler(self, value: 'Vector'):
        """
        Set the vehicle spawn rotation euler
        """
        funcs.set_vehicle_spawn_rotation_euler(self._id, value.x, value.y, value.z)

    @property
    def idle_respawn_timer(self):
        """
        Get the vehicle idle respawn timer
        """
        return funcs.get_vehicle_idle_respawn_timer(self._id)
    
    @idle_respawn_timer.setter
    def idle_respawn_timer(self, value: int):
        """
        Set the vehicle idle respawn timer
        """
        funcs.set_vehicle_idle_respawn_timer(self._id, value)

    @property
    def health(self):
        """
        Get the vehicle health
        """
        return funcs.get_vehicle_health(self._id)
    
    @health.setter
    def health(self, value: int):
        """
        Set the vehicle health
        """
        funcs.set_vehicle_health(self._id, value)

    @property
    def primary_color(self):
        """
        Get the vehicle primary color
        """
        return funcs.get_vehicle_colour(self._id)[0]

    @primary_color.setter
    def primary_color(self, value: int):
        """
        Set the vehicle primary color
        """
        funcs.set_vehicle_colour(self._id, value, self.secondary_color)

    @property
    def secondary_color(self):
        """
        Get the vehicle secondary color
        """
        return funcs.get_vehicle_colour(self._id)[1]
    
    @secondary_color.setter
    def secondary_color(self, value: int):
        """
        Set the vehicle secondary color
        """
        funcs.set_vehicle_colour(self._id, self.primary_color, value)

    @property
    def color(self) -> tuple[int, int]:
        """
        Get the vehicle color
        """
        return funcs.get_vehicle_colour(self._id)

    @color.setter
    def color(self, value: tuple[int, int]):
        """
        Set the vehicle color
        """
        funcs.set_vehicle_colour(self._id, value[0], value[1])

    @property
    def damage(self):
        """
        Get the vehicle damage
        """
        return funcs.get_vehicle_damage_data(self._id)

    @damage.setter
    def damage(self, value: int):
        """
        Set the vehicle damage
        """
        funcs.set_vehicle_damage_data(self._id, value)

    @property
    def radio(self):
        """
        Get the vehicle radio
        """
        return funcs.get_vehicle_radio(self._id)

    @radio.setter
    def radio(self, value: int):
        """
        Set the vehicle radio
        """
        funcs.set_vehicle_radio(self._id, value)

    @property
    def turret_rotation(self) -> tuple[int, int]:
        """
        Get the vehicle turret rotation
        """
        return funcs.get_vehicle_turret_rotation(self._id)

    @property
    def lights_data(self):
        """
        Get the vehicle lights data
        """
        return funcs.get_vehicle_lights_data(self._id)
    
    @lights_data.setter
    def lights_data(self, value: int):
        """
        Set the vehicle lights data
        """
        funcs.set_vehicle_lights_data(self._id, value)

    def delete(self):
        """
        Delete the vehicle
        """
        if not funcs.check_entity_exists(vcmpEntityPool.vcmpEntityPoolVehicle, self._id):
            return
        funcs.delete_vehicle(self._id)
        _vehicles.remove(self)

    def is_streamed_for_player(self, player_id: int):
        """
        Check if the vehicle is streamed for the player
        """
        id = player_id if isinstance(player_id, int) else player_id.id
        return funcs.is_vehicle_streamed_for_player(self._id, id)
    
    def get_occupant(self, slot: int) -> Optional['Player']:
        """
        Get the vehicle occupant
        """
        return get_player_from_id(funcs.get_vehicle_occupant(self._id, slot))

    def respawn(self):
        """
        Respawn the vehicle
        """
        funcs.respawn_vehicle(self._id)

    def explode(self):  
        """
        Explode the vehicle
        """
        funcs.explode_vehicle(self._id)

    def kill(self):
        """
        Kill the vehicle, same as explode
        """
        funcs.explode_vehicle(self._id)

    def fix(self):
        """
        Fix the vehicle
        """
        funcs.set_vehicle_health(self._id, 1000)
        funcs.set_vehicle_damage_data(self._id, 0)
        dwLightsData = funcs.get_vehicle_lights_data(self._id)
        dwLightsData &= 0xFFFFFF00
        funcs.set_vehicle_lights_data(self._id, dwLightsData)
        
    def set_position(self, position: 'Vector', remove_occupants: bool = False):
        """
        Set the vehicle position
        """
        funcs.set_vehicle_position(self._id, position.x, position.y, position.z, remove_occupants)

    def get_part_status(self, part: int) -> int:
        """
        Get the vehicle part status
        """
        return funcs.get_vehicle_part_status(self._id, part)

    def set_part_status(self, part: int, status: int):
        """
        Set the vehicle part status
        """
        funcs.set_vehicle_part_status(self._id, part, status)

    def get_tyre_status(self, tyre: int) -> int:
        """
        Get the vehicle tyre status
        """
        return funcs.get_vehicle_tyre_status(self._id, tyre)

    def set_tyre_status(self, tyre: int, status: int):
        """
        Set the vehicle tyre status
        """
        funcs.set_vehicle_tyre_status(self._id, tyre, status)

    def exists_handling_rule(self, rule: int) -> bool:
        """
        Check if the vehicle exists handling rule
        """
        return funcs.exists_inst_handling_rule(self._id, rule)
    
    def set_handling_rule(self, rule: int, value: float):
        """
        Set the vehicle handling rule
        """
        funcs.set_inst_handling_rule(self._id, rule, value)

    def get_handling_rule(self, rule: int) -> float:
        """
        Get the vehicle handling rule
        """
        return funcs.get_inst_handling_rule(self._id, rule)
    
    def reset_handling_rule(self, rule: int):
        """
        Reset the vehicle handling rule
        """
        funcs.reset_inst_handling_rule(self._id, rule)

    def reset_handling(self):
        """
        Reset the vehicle handling
        """
        funcs.reset_inst_handling(self._id)

    def add_position(self, position: 'Vector'):
        """
        Add position to the vehicle position
        """
        new_pos = Vector(
            self.position.x + position.x,
            self.position.y + position.y,
            self.position.z + position.z
        )
        self.position = new_pos

    def add_speed(self, speed: 'Vector'):
        """
        Add speed to the vehicle speed
        """
        new_speed = Vector(
            self.speed.x + speed.x,
            self.speed.y + speed.y,
            self.speed.z + speed.z
        )
        self.speed = new_speed

    def add_relative_speed(self, speed: 'Vector'):
        """
        Add relative speed to the vehicle speed
        """
        new_speed = Vector(
            self.relative_speed.x + speed.x,
            self.relative_speed.y + speed.y,
            self.relative_speed.z + speed.z
        )
        self.relative_speed = new_speed

    def add_turn_speed(self, speed: 'Vector'):
        """
        Add turn speed to the vehicle turn speed
        """
        new_speed = Vector(
            self.turn_speed.x + speed.x,
            self.turn_speed.y + speed.y,
            self.turn_speed.z + speed.z
        )
        self.turn_speed = new_speed

    def add_relative_turn_speed(self, speed: 'Vector'):
        """
        Add relative turn speed to the vehicle turn speed
        """
        new_speed = Vector(
            self.relative_turn_speed.x + speed.x,
            self.relative_turn_speed.y + speed.y,
            self.relative_turn_speed.z + speed.z
        )
        self.relative_turn_speed = new_speed

    def add_rotation(self, rotation: 'Quaternion'):
        """
        Add rotation to the vehicle rotation
        """
        new_rotation = Quaternion(
            self.rotation.x + rotation.x,
            self.rotation.y + rotation.y,
            self.rotation.z + rotation.z,
            self.rotation.w + rotation.w
        )
        self.rotation = new_rotation

    def add_rotation_euler(self, rotation: 'Vector'):
        """
        Add rotation to the vehicle rotation
        """
        new_rotation = Vector(
            self.rotation_euler.x + rotation.x,
            self.rotation_euler.y + rotation.y,
            self.rotation_euler.z + rotation.z
        )
        self.rotation_euler = new_rotation

    def add_spawn_position(self, position: 'Vector'):
        """
        Add position to the vehicle spawn position
        """
        new_pos = Vector(
            self.spawn_position.x + position.x,
            self.spawn_position.y + position.y,
            self.spawn_position.z + position.z
        )
        self.spawn_position = new_pos

    def add_spawn_rotation(self, rotation: 'Quaternion'):
        """
        Add rotation to the vehicle spawn rotation
        """
        new_rotation = Quaternion(
            self.spawn_rotation.x + rotation.x,
            self.spawn_rotation.y + rotation.y,
            self.spawn_rotation.z + rotation.z,
            self.spawn_rotation.w + rotation.w
        )
        self.spawn_rotation = new_rotation

    def add_spawn_rotation_euler(self, rotation: 'Vector'):
        """
        Add rotation to the vehicle spawn rotation
        """
        new_rotation = Vector(
            self.spawn_rotation_euler.x + rotation.x,
            self.spawn_rotation_euler.y + rotation.y,
            self.spawn_rotation_euler.z + rotation.z
        )
        self.spawn_rotation_euler = new_rotation

    def __repr__(self) -> str:
        return f"Vehicle(id={self.id}, model={self.model})"

    def __del__(self):
        self.delete()

    def __new__(cls, vehicle_id: int):
        if not funcs.check_entity_exists(vcmpEntityPool.vcmpEntityPoolVehicle, vehicle_id):
            return None
        vehicle = next((vehicle for vehicle in _vehicles if vehicle.id == vehicle_id), None)
        if vehicle is None:
            vehicle = super().__new__(cls)
            _vehicles.append(vehicle)
        return vehicle


import inspect

for member in inspect.getmembers(object=Vehicle):
    name, value = member
    # if value is function
    if inspect.isfunction(value):
        print(f"fn {name}")
    else:
        print(f"fn prop_{name}")