import math
from typing import (
    Optional, 
    Type, 
    TypeVar
)

from vcmp.vec import (
    Vector, 
    Quaternion
)
from vcmp.__export import (
    calls, 
    funcs
)
from vcmp.streams import WriteStream
from vcmp.types import (
    vcmpEntityPool, 
    vcmpPlayerOption, 
    vcmpPlayerState, 
    vcmpVehicleSync, 
    vcmpVehicleOption,
    MAX_VEHICLES,
    MAX_OBJECTS,
    MAX_PICKUPS,
    MAX_CHECKPOINTS,
    RGB
)

T = TypeVar('T')

class Player:
    def __init__(
        self,
        id: int
    ):
        self._id: int = id

    # use id as hash
    def __hash__(self) -> int:
        return hash(self._id)

    def __eq__(self, value: object) -> bool:
        if isinstance(value, Player):
            return self._id == value._id
        return False

    @property
    def id(self) -> int:
        return self._id

    @property
    def is_alive(self) -> bool:
        return funcs.is_player_connected(self._id)
    
    @property
    def admin(self):
        return funcs.is_player_admin(self._id)

    @admin.setter
    def admin(self, admin: bool):
        funcs.set_player_admin(self._id, admin)
    
    @property
    def name(self) -> str:
        return funcs.get_player_name(self._id)
    
    @name.setter
    def name(self, name: str):
        funcs.set_player_name(self._id, name)

    @property
    def ip(self) -> str:
        return funcs.get_player_ip(self._id)
    
    @property
    def unique_id(self) -> str:
        return funcs.get_player_uid(self._id)
    
    @property
    def unique_id2(self) -> str:
        return funcs.get_player_uid2(self._id)

    @property
    def ping(self) -> int:
        return funcs.get_player_ping(self._id)
    
    @property
    def score(self) -> int:
        return funcs.get_player_score(self._id)

    @score.setter
    def score(self, score: int):
        funcs.set_player_score(self._id, score)
    
    @property
    def color(self) -> RGB:
        return RGB.from_int(funcs.get_player_colour(self._id))

    @color.setter
    def color(self, color: RGB):
        funcs.set_player_colour(self._id, color.to_int())

    @property
    def key(self):
        """
            Player key (I didn't know what it is)
        """
        return funcs.get_player_key(self._id)

    @property
    def state(self):
        state = funcs.get_player_state(self._id)
        return vcmpPlayerState(state)
    

        #  Player Options

    @property
    def controllable(self) -> bool:
        return funcs.get_player_option(self._id, vcmpPlayerOption.vcmpPlayerOptionControllable)

    @controllable.setter
    def controllable(self, controllable: bool):
        funcs.set_player_option(self._id, vcmpPlayerOption.vcmpPlayerOptionControllable, controllable)

    @property
    def frozen(self) -> bool:
        return not self.controllable
    
    @frozen.setter
    def frozen(self, frozen: bool):
        self.controllable = not frozen

    @property
    def drive_by(self) -> bool:
        return funcs.get_player_option(self._id, vcmpPlayerOption.vcmpPlayerOptionDriveBy)

    @drive_by.setter
    def drive_by(self, drive_by: bool):
        funcs.set_player_option(self._id, vcmpPlayerOption.vcmpPlayerOptionDriveBy, drive_by)

    @property
    def white_scanlines(self) -> bool:
        return funcs.get_player_option(self._id, vcmpPlayerOption.vcmpPlayerOptionWhiteScanlines)

    @white_scanlines.setter
    def white_scanlines(self, white_scanlines: bool):
        funcs.set_player_option(self._id, vcmpPlayerOption.vcmpPlayerOptionWhiteScanlines, white_scanlines)

    @property
    def green_scanlines(self) -> bool:
        return funcs.get_player_option(self._id, vcmpPlayerOption.vcmpPlayerOptionGreenScanlines)

    @green_scanlines.setter
    def green_scanlines(self, green_scanlines: bool):
        funcs.set_player_option(self._id, vcmpPlayerOption.vcmpPlayerOptionGreenScanlines, green_scanlines)

    @property
    def widescreen(self) -> bool:
        return funcs.get_player_option(self._id, vcmpPlayerOption.vcmpPlayerOptionWidescreen)

    @widescreen.setter
    def widescreen(self, widescreen: bool):
        funcs.set_player_option(self._id, vcmpPlayerOption.vcmpPlayerOptionWidescreen, widescreen)

    @property
    def show_markers(self) -> bool:
        return funcs.get_player_option(self._id, vcmpPlayerOption.vcmpPlayerOptionShowMarkers)

    @show_markers.setter
    def show_markers(self, show_markers: bool):
        funcs.set_player_option(self._id, vcmpPlayerOption.vcmpPlayerOptionShowMarkers, show_markers)

    @property
    def can_attack(self) -> bool:
        return funcs.get_player_option(self._id, vcmpPlayerOption.vcmpPlayerOptionCanAttack)

    @can_attack.setter
    def can_attack(self, can_attack: bool):
        funcs.set_player_option(self._id, vcmpPlayerOption.vcmpPlayerOptionCanAttack, can_attack)

    @property
    def has_marker(self) -> bool:
        return funcs.get_player_option(self._id, vcmpPlayerOption.vcmpPlayerOptionHasMarker)

    @has_marker.setter
    def has_marker(self, has_marker: bool):
        funcs.set_player_option(self._id, vcmpPlayerOption.vcmpPlayerOptionHasMarker, has_marker)

    @property
    def chat_tags_enabled(self) -> bool:
        return funcs.get_player_option(self._id, vcmpPlayerOption.vcmpPlayerOptionChatTagsEnabled)
    
    @chat_tags_enabled.setter
    def chat_tags_enabled(self, chat_tags_enabled: bool):
        funcs.set_player_option(self._id, vcmpPlayerOption.vcmpPlayerOptionChatTagsEnabled, chat_tags_enabled)

    @property
    def drunk_effects(self) -> bool:
        return funcs.get_player_option(self._id, vcmpPlayerOption.vcmpPlayerOptionDrunkEffects)

    @drunk_effects.setter
    def drunk_effects(self, drunk_effects: bool):
        funcs.set_player_option(self._id, vcmpPlayerOption.vcmpPlayerOptionDrunkEffects, drunk_effects)

    @property
    def world(self) -> int:
        return funcs.get_player_world(self._id)
    
    @world.setter
    def world(self, world: int):
        funcs.set_player_world(self._id, world)

    @property
    def secondary_world(self) -> int:
        return funcs.get_player_secondary_world(self._id)

    @secondary_world.setter
    def secondary_world(self, secondary_world: int):
        funcs.set_player_secondary_world(self._id, secondary_world)

    @property
    def sec_world(self) -> int:
        return self.secondary_world
    
    @sec_world.setter
    def sec_world(self, secondary_world: int):
        self.secondary_world = secondary_world

    @property
    def unique_world(self) -> int:
        return funcs.get_player_unique_world(self._id)

    @property
    def clazz(self) -> int:
        return funcs.get_player_class(self._id)

    @property
    def team(self) -> int:
        return funcs.get_player_team(self._id)

    @team.setter
    def team(self, team: int):
        funcs.set_player_team(self._id, team)

    @property
    def skin(self) -> int:
        return funcs.get_player_skin(self._id)

    @skin.setter
    def skin(self, skin: int):
        funcs.set_player_skin(self._id, skin)

    @property
    def spawned(self):
        return funcs.is_player_spawned(self._id)

    @property
    def typing(self):
        return funcs.is_player_typing(self._id)

    @property
    def cash(self):
        return funcs.get_player_money(self._id)

    @cash.setter
    def cash(self, cash: int):
        funcs.set_player_money(self._id, cash)

    @property
    def wanted_level(self):
        return funcs.get_player_wanted_level(self._id)

    @wanted_level.setter
    def wanted_level(self, wanted_level: int):
        funcs.set_player_wanted_level(self._id, wanted_level)

    @property
    def fps(self):
        return funcs.get_player_fps(self._id)
    
    @property
    def health(self):
        return funcs.get_player_health(self._id)

    @health.setter
    def health(self, health: float):
        funcs.set_player_health(self._id, health)

    @property
    def armour(self):
        return funcs.get_player_armour(self._id)

    @armour.setter
    def armour(self, armour: int):
        funcs.set_player_armour(self._id, armour)

    @property
    def immunity(self):
        return funcs.get_player_immunity_flags(self._id)
    
    @immunity.setter
    def immunity(self, immunity: int):
        funcs.set_player_immunity_flags(self._id, immunity)

    @property
    def position(self):
        return Vector(
            **funcs.get_player_position(self._id)
        )
    
    @position.setter
    def position(self, position: Vector):
        funcs.set_player_position(self._id, position.x, position.y, position.z)

    @property
    def speed(self):
        return Vector(
            **funcs.get_player_speed(self._id)
        )

    @speed.setter
    def speed(self, speed: Vector):
        funcs.set_player_speed(self._id, speed.x, speed.y, speed.z)

    @property
    def angle(self):
        return funcs.get_player_heading(self._id)
    
    @angle.setter
    def angle(self, heading: float):
        funcs.set_player_heading(self._id, heading)

    @property
    def alpha(self):
        return funcs.get_player_alpha(self._id)
    
    def set_alpha(self, alpha: int, fade: int):
        funcs.set_player_alpha(self._id, alpha, fade)

    @property
    def aim_position(self):
        return Vector(
            **funcs.get_player_aim_position(self._id)
        )
    
    @property
    def aim_direction(self):
        return Vector(
            **funcs.get_player_aim_direction(self._id)
        )
    
    @property
    def on_fire(self):
        """
        :return: True if the player is firing, not weapon firing
        """
        return funcs.is_player_on_fire(self._id)
    
    @property
    def crouching(self):
        return funcs.is_player_crouching(self._id)
    
    @property
    def action(self):
        return funcs.get_player_action(self._id)
    
    @property
    def game_keys(self):
        return funcs.get_player_game_keys(self._id)
    
    @property
    def vehicle(self):
        return Vehicle(funcs.get_player_vehicle_id(self._id))
    
    @vehicle.setter
    def vehicle(self, vehicle: Optional['Vehicle']):
        id = None
        if vehicle is not None:
            id = vehicle.id
        if id is not None and funcs.check_entity_exists(vcmpEntityPool.vcmpEntityPoolVehicle, id):
            origin_id = funcs.get_player_vehicle_id(self._id)
            if origin_id != id:
                # remove from old vehicle
                funcs.remove_player_from_vehicle(self._id)
                funcs.put_player_in_vehicle(self._id, id, 0, 0, 1)
        else:
            funcs.remove_player_from_vehicle(self._id)

    @property
    def vehicle_status(self):
        return funcs.get_player_in_vehicle_status(self._id)
    
    @property
    def weapon(self):
        """
        Get the player weapon
        """
        return funcs.get_player_weapon(self._id)
    
    @property
    def weapon_ammo(self):
        """
        Get the player weapon ammo
        """
        return funcs.get_player_weapon_ammo(self._id)
    
    @weapon_ammo.setter
    def weapon_ammo(self, ammo: int):
        """
        Set the player weapon ammo
        """
        funcs.set_player_weapon(self._id, self.weapon, ammo)

    @property
    def weapon_slot(self):
        """
        Get the player weapon slot
        """
        return funcs.get_player_weapon_slot(self._id)
    
    @weapon_slot.setter
    def weapon_slot(self, slot: int):
        """
        Set the player weapon slot
        """
        funcs.set_player_weapon_slot(self._id, slot)
    
    @property
    def camera_locked(self):
        return funcs.is_camera_locked(self._id)
    
    @property
    def standing_vehicle(self):
        return Vehicle(funcs.get_player_standing_on_vehicle(self._id))
    
    @property
    def standing_object(self):
        # TODO: 
        return funcs.get_player_standing_on_object(self._id)
    
    @property
    def away(self):
        return funcs.is_player_away(self._id)

    @property
    def spectate_target(self):
        id = funcs.get_player_spectate_target(self._id)
        if funcs.is_player_connected(id):
            return Player(id)
        return None
    
    @spectate_target.setter
    def spectate_target(self, target: Optional['Player']):
        id = -1
        if target is not None:
            id = target.id
        funcs.set_player_spectate_target(self._id, id)

    def kick(self, reason: Optional[str] = None):
        """
            Kick the player

            :param reason: The reason for the kick, optional
        """
        if reason is not None:
            self.send_message(reason)
        funcs.kick_player(self._id)

    def ban(self, reason: Optional[str] = None):
        """
            Ban the player

            :param reason: The reason for the ban, optional
        """
        if reason is not None:
            self.send_message(reason)
        funcs.ban_player(self._id)

    def send_message(self, message: str):
        """
        Send message to the player
        """
        return self.send_raw_message(RGB.from_alpha(0x0b5fa5ff), message)

    def send_raw_message(self, color: RGB, message: str):
        """
        Send message to the player
        """

        funcs.send_client_message(self._id, color.to_alpha(), message)

    def send_announce(self, type: int, message: str):
        """
        Send announce message to the player
        """
        funcs.send_game_message(self._id, type, message)

    def send_data(self, stream: WriteStream):
        """
        Send stream data to the player
        """
        buffer = stream._buffer.getvalue()
        funcs.send_client_script_data(self._id, buffer)

    def is_player_streamed_for_target(
        self,
        target: 'Player'
    ) -> bool:
        """
        Check if the player is streamed for the target player
        """
        return funcs.is_player_streamed_for_player(self._id, target.id)

    def spawn(self):
        """
        Spawn the player
        """
        funcs.force_player_spawn(self._id)

    def select(self):
        """
        Select the player
        """
        funcs.force_player_select(self._id)

    def play_sound(self, sound_id: int, x: float = math.nan, y: float = math.nan, z: float = math.nan):
        """
        Play sound to the player
        """
        funcs.play_sound(self.unique_world, sound_id, x, y, z)
        
    def set_vehicle_slot(self, vehicle: 'Vehicle', slot: int):
        """
        Set the player vehicle slot
        """
        if vehicle is None:
            funcs.remove_player_from_vehicle(self._id)
            return

        funcs.put_player_in_vehicle(self.id, vehicle.id, slot, 1, 0)

    def give_weapon(self, weapon: int, ammo: int):
        """
        Give the player a weapon
        """
        funcs.give_player_weapon(self._id, weapon, ammo)

    def set_weapon(self, weapon: int, ammo: int):
        """
        Set the player weapon
        """
        funcs.set_player_weapon(self._id, weapon, ammo)

    def get_weapon(self):
        """
        Get the player weapon
        """
        return funcs.get_player_weapon(self._id)

    def get_weapon_ammo(self):
        """
        Get the player weapon ammo
        """
        return funcs.get_player_weapon_ammo(self._id)
    
    def set_weapon_slot(self, slot: int):
        """
        Set the player weapon slot
        maybe let the player select the weapon slot
        """
        funcs.set_player_weapon_slot(self._id, slot)

    def get_weapon_slot(self):
        """
        Get the player weapon slot
        """
        return funcs.get_player_weapon_slot(self._id)
    
    def get_weapon_at_slot(self, slot: int):
        """
        Get the player weapon at slot
        """
        return funcs.get_player_weapon_at_slot(self._id, slot)
    
    def get_weapon_ammo_at_slot(self, slot: int):
        """
        Get the player weapon ammo at slot
        """
        return funcs.get_player_ammo_at_slot(self._id, slot)
    
    def remove_weapon(self, weapon: int):
        """
        Remove the player weapon
        """
        funcs.remove_player_weapon(self._id, weapon)

    def disarm(self):
        """
        Remove the player weapons
        same as clear_weapons
        """
        funcs.remove_all_weapons(self._id)

    def clear_weapons(self):
        """
        Remove the player weapons
        same as disarm
        """
        funcs.remove_all_weapons(self._id)

    def set_camera_position(self, position: Vector, look_at: Vector):
        """
        Set the player camera position
        """
        funcs.set_camera_position(self._id, position.x, position.y, position.z, look_at.x, look_at.y, look_at.z)

    def restore_camera(self):
        """
        Restore the player camera
        """
        funcs.restore_camera(self._id)

    def set_camera(self, position: Vector, look_yaw: float, look_pitch: float, range: float = 0.5):
        """
        Set the player camera with look yaw and look pitch,
        yaw is degrees from 0 to 360
        pitch is degrees from -90 to 90
        range is the distance from the player to the camera
        
        """
        look = Vector(0, 0, 0)
        look.x = math.cos(math.radians(look_yaw)) * range
        look.y = math.sin(math.radians(look_yaw)) * range
        look.z = math.sin(math.radians(look_pitch)) * range
        look = look + position

        funcs.set_camera_position(self._id, position.x, position.y, position.z, look.x, look.y, look.z)

    def play_animation(self, group: int, animation: int):
        """
        Play the player animation
        """
        funcs.set_player_animation(self._id, group, animation)

    def redirect(self, ip: str, port: int, nick: str, password: str, user_password: str):
        """
        Redirect the player to a new server
        """
        funcs.redirect_player_to_server(self._id, ip, port, nick, password, user_password)

    def request_module_list(self):
        """
        Request the player module list
        Need to recieve the module list from the PlayerModuleListEvent
        """
        funcs.get_player_module_list(self._id)

    def kill(self):
        """
        Kill the player
        """
        funcs.kill_player(self._id)

    def add_position(self, position: Vector):
        """
        Add position to the player position
        """

        new_pos = Vector(
            self.position.x + position.x,
            self.position.y + position.y,
            self.position.z + position.z
        )
        self.position = new_pos

    def add_speed(self, speed: Vector):
        """
        Add speed to the player speed
        """

        new_speed = Vector(
            self.speed.x + speed.x,
            self.speed.y + speed.y,
            self.speed.z + speed.z
        )
        self.speed = new_speed

    def __repr__(self) -> str:
        return f"Player(id={self.id}, name='{self.name}')"

    def __new__(cls, player_id: int):
        plr = get_player_from_id(player_id)
        if plr is not None:
            return plr
        plr = super().__new__(cls)
        _players.append(plr)
        return plr

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
    def position(self, value: Vector):
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
    def rotation(self, value: Quaternion):
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
    def rotation_euler(self, value: Vector):
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
    def speed(self, value: Vector):
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
    def relative_speed(self, value: Vector):
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
    def turn_speed(self, value: Vector):
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
    def relative_turn_speed(self, value: Vector):
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
    def spawn_position(self, value: Vector):
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
    def spawn_rotation(self, value: Quaternion):
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
    def spawn_rotation_euler(self, value: Vector):
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

    def is_streamed_for_player(self, player_id: int | Player):
        """
        Check if the vehicle is streamed for the player
        """
        id = player_id if isinstance(player_id, int) else player_id.id
        return funcs.is_vehicle_streamed_for_player(self._id, id)
    
    def get_occupant(self, slot: int) -> Optional[Player]:
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
        
    def set_position(self, position: Vector, remove_occupants: bool = False):
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

    def add_position(self, position: Vector):
        """
        Add position to the vehicle position
        """
        new_pos = Vector(
            self.position.x + position.x,
            self.position.y + position.y,
            self.position.z + position.z
        )
        self.position = new_pos

    def add_speed(self, speed: Vector):
        """
        Add speed to the vehicle speed
        """
        new_speed = Vector(
            self.speed.x + speed.x,
            self.speed.y + speed.y,
            self.speed.z + speed.z
        )
        self.speed = new_speed

    def add_relative_speed(self, speed: Vector):
        """
        Add relative speed to the vehicle speed
        """
        new_speed = Vector(
            self.relative_speed.x + speed.x,
            self.relative_speed.y + speed.y,
            self.relative_speed.z + speed.z
        )
        self.relative_speed = new_speed

    def add_turn_speed(self, speed: Vector):
        """
        Add turn speed to the vehicle turn speed
        """
        new_speed = Vector(
            self.turn_speed.x + speed.x,
            self.turn_speed.y + speed.y,
            self.turn_speed.z + speed.z
        )
        self.turn_speed = new_speed

    def add_relative_turn_speed(self, speed: Vector):
        """
        Add relative turn speed to the vehicle turn speed
        """
        new_speed = Vector(
            self.relative_turn_speed.x + speed.x,
            self.relative_turn_speed.y + speed.y,
            self.relative_turn_speed.z + speed.z
        )
        self.relative_turn_speed = new_speed

    def add_rotation(self, rotation: Quaternion):
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

    def add_rotation_euler(self, rotation: Vector):
        """
        Add rotation to the vehicle rotation
        """
        new_rotation = Vector(
            self.rotation_euler.x + rotation.x,
            self.rotation_euler.y + rotation.y,
            self.rotation_euler.z + rotation.z
        )
        self.rotation_euler = new_rotation

    def add_spawn_position(self, position: Vector):
        """
        Add position to the vehicle spawn position
        """
        new_pos = Vector(
            self.spawn_position.x + position.x,
            self.spawn_position.y + position.y,
            self.spawn_position.z + position.z
        )
        self.spawn_position = new_pos

    def add_spawn_rotation(self, rotation: Quaternion):
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

    def add_spawn_rotation_euler(self, rotation: Vector):
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

class Pickup:
    def __init__(self, pickup_id: int):
        self._id = pickup_id

    def __hash__(self) -> int:
        return hash(self._id)

    def __eq__(self, value: object) -> bool:
        if isinstance(value, Pickup):
            return self._id == value._id
        return False

    @property
    def id(self) -> int:
        return self._id

    @property
    def is_alive(self) -> bool:
        return funcs.check_entity_exists(vcmpEntityPool.vcmpEntityPoolPickup, self._id)

    @property
    def world(self):
        return funcs.get_pickup_world(self._id)
    
    @world.setter
    def world(self, world: int):
        funcs.set_pickup_world(self._id, world)

    @property
    def alpha(self):
        return funcs.get_pickup_alpha(self._id)

    @alpha.setter
    def alpha(self, alpha: int):
        funcs.set_pickup_alpha(self._id, alpha)

    @property
    def automatic(self):
        return funcs.is_pickup_automatic(self._id)

    @automatic.setter
    def automatic(self, automatic: bool):
        funcs.set_pickup_is_automatic(self._id, automatic)

    @property
    def timer(self):
        """
        Auto timer
        """
        return funcs.get_pickup_auto_timer(self._id)

    @timer.setter
    def timer(self, timer: int):
        """
        Auto timer
        """
        funcs.set_pickup_auto_timer(self._id, timer)

    @property
    def position(self):
        return Vector(
            **funcs.get_pickup_position(self._id)
        )
    
    @position.setter
    def position(self, position: Vector):
        funcs.set_pickup_position(self._id, position.x, position.y, position.z)

    @property
    def model(self):
        return funcs.get_pickup_model(self._id)

    @property
    def quantity(self):
        return funcs.get_pickup_quantity(self._id)

    @property
    def single_use(self):
        return funcs.get_pickup_option(self._id, 0)
    
    @single_use.setter
    def single_use(self, single_use: bool):
        funcs.set_pickup_option(self._id, 0, single_use)

    def refresh(self):
        """
        Refresh the pickup
        """
        funcs.refresh_pickup(self._id)
        
    def is_streamed_for_player(self, player: int | Player) -> bool:
        """
        Check if the pickup is streamed for the player
        """
        id = player if isinstance(player, int) else player.id
        return funcs.is_pickup_streamed_for_player(self._id, id)
    
    def add_position(self, position: Vector):
        """
        Add position to the pickup position
        """
        new_pos = Vector(
            self.position.x + position.x,
            self.position.y + position.y,
            self.position.z + position.z
        )
        self.position = new_pos

    def delete(self):
        """
        Delete the pickup
        """
        if not funcs.check_entity_exists(vcmpEntityPool.vcmpEntityPoolPickup, self._id):
            return
        funcs.delete_pickup(self._id)
        _pickups.remove(self)

    def __del__(self):
        self.delete()

    def __new__(cls, pickup_id: int):
        if not funcs.check_entity_exists(vcmpEntityPool.vcmpEntityPoolPickup, pickup_id):
            return None
        pickup = next((pickup for pickup in _pickups if pickup.id == pickup_id), None)
        if pickup is None:
            pickup = super().__new__(cls)
            _pickups.append(pickup)
        return pickup

class CheckPoint:
    def __init__(self, id: int):
        self._id = id

    def __hash__(self) -> int:
        return hash(self._id)

    def __eq__(self, value: object) -> bool:
        if isinstance(value, CheckPoint):
            return self._id == value._id
        return False

    @property
    def id(self) -> int:
        return self._id

    @property
    def is_alive(self) -> bool:
        return funcs.check_entity_exists(vcmpEntityPool.vcmpEntityPoolCheckPoint, self._id)

    @property
    def sphere(self):
        return funcs.is_check_point_sphere(self._id)
    
    @property
    def world(self):
        return funcs.get_check_point_world(self._id)
    
    @world.setter
    def world(self, world: int):
        funcs.set_check_point_world(self._id, world)

    @property
    def position(self):
        return Vector(
            **funcs.get_check_point_position(self._id)
        )

    @position.setter
    def position(self, position: Vector):
        funcs.set_check_point_position(self._id, position.x, position.y, position.z)

    @property
    def color(self) -> RGB:
        return RGB(**funcs.get_check_point_colour(self._id))

    @color.setter
    def color(self, color: RGB):
        funcs.set_check_point_colour(self._id, color.red, color.green, color.blue, color.alpha)

    @property
    def radius(self):
        return funcs.get_check_point_radius(self._id)

    @radius.setter
    def radius(self, radius: float):
        funcs.set_check_point_radius(self._id, radius)

    @property
    def owner(self):
        return get_player_from_id(funcs.get_check_point_owner(self._id))

    def delete(self):
        """
        Delete the pickup
        """
        if not funcs.check_entity_exists(vcmpEntityPool.vcmpEntityPoolCheckPoint, self._id):
            return
        funcs.delete_check_point(self._id)
        _checkpoints.remove(self)

    def is_streamed_for_player(self, player: int | Player) -> bool:
        id = player if isinstance(player, int) else player.id
        return funcs.is_check_point_streamed_for_player(self._id, id)

    def add_position(self, position: Vector):
        """
        Add position to the pickup position
        """
        new_pos = Vector(
            self.position.x + position.x,
            self.position.y + position.y,
            self.position.z + position.z
        )
        self.position = new_pos

    def __del__(self):
        self.delete()

    def __new__(cls, checkpoint_id: int):
        if not funcs.check_entity_exists(vcmpEntityPool.vcmpEntityPoolCheckPoint, checkpoint_id):
            return None
        
        checkpoint = next((checkpoint for checkpoint in _checkpoints if checkpoint.id == checkpoint_id), None)
        if checkpoint is None:
            checkpoint = super().__new__(cls)
            _checkpoints.append(checkpoint)
        return checkpoint

class Object:
    def __init__(self, id: int):
        self._id = id

    def __hash__(self) -> int:
        return hash(self._id)

    def __eq__(self, value: object) -> bool:
        if isinstance(value, Object):
            return self._id == value._id
        return False

    @property
    def id(self) -> int:
        return self._id
    
    @property
    def is_alive(self) -> bool:
        return funcs.check_entity_exists(vcmpEntityPool.vcmpEntityPoolObject, self._id)

    @property
    def model(self):
        return funcs.get_object_model(self._id)
    
    @property
    def world(self):
        return funcs.get_object_world(self._id)

    @world.setter
    def world(self, world: int):
        funcs.set_object_world(self._id, world)

    @property
    def alpha(self):
        return funcs.get_object_alpha(self._id)

    @property
    def position(self):
        return Vector(
            **funcs.get_object_position(self._id)
        )

    @position.setter
    def position(self, position: Vector):
        funcs.set_object_position(self._id, position.x, position.y, position.z)

    @property
    def shot_report(self):
        return funcs.is_object_shot_report_enabled(self._id)
    
    @shot_report.setter
    def shot_report(self, shot_report: bool):
        funcs.set_object_shot_report_enabled(self._id, shot_report)

    @property
    def touched_report(self):
        return funcs.is_object_touched_report_enabled(self._id)

    @touched_report.setter
    def touched_report(self, touched_report: bool):
        funcs.set_object_touched_report_enabled(self._id, touched_report)

    def delete(self):
        """
        Delete the pickup
        """
        if not funcs.check_entity_exists(vcmpEntityPool.vcmpEntityPoolObject, self._id):
            return
        funcs.delete_object(self._id)
        _objects.remove(self)

    def is_streamed_for_player(self, player: int | Player) -> bool:
        id = player if isinstance(player, int) else player.id
        return funcs.is_object_streamed_for_player(self._id, id)
    
    def add_position(self, position: Vector):
        """
        Add position to the pickup position
        """
        new_pos = Vector(
            self.position.x + position.x,
            self.position.y + position.y,
            self.position.z + position.z
        )
        self.position = new_pos

    def set_alpha(self, alpha: int, duration: int = 0):
        funcs.set_object_alpha(self._id, alpha, duration)

    def move_to(self, position: Vector, duration: int = 0):
        """
        Move the object to the given position
        """
        funcs.move_object_to(self._id, position.x, position.y, position.z, duration)

    def move_by(self, position: Vector, duration: int = 0):
        """
        Move the object by the given position
        """
        funcs.move_object_by(self._id, position.x, position.y, position.z, duration)

    def rotate_to(self, rotation: Quaternion, duration: int = 0):
        """
        Rotate the object to the given rotation
        """
        funcs.rotate_object_to(self._id, rotation.x, rotation.y, rotation.z, rotation.w, duration)

    def rotate_to_euler(self, rotation: Vector, duration: int = 0):
        """
        Rotate the object to the given euler rotation
        """
        funcs.rotate_object_to_euler(self._id, rotation.x, rotation.y, rotation.z, duration)

    def rotate_by(self, rotation: Quaternion, duration: int = 0):
        """
        Rotate the object by the given rotation
        """
        funcs.rotate_object_by(self._id, rotation.x, rotation.y, rotation.z, rotation.w, duration)

    def rotate_by_euler(self, rotation: Vector, duration: int = 0):
        """
        Rotate the object by the given euler rotation
        """
        funcs.rotate_object_by_euler(self._id, rotation.x, rotation.y, rotation.z, duration)
        
    def __del__(self):
        self.delete()

    def __new__(cls, object_id: int):
        if not funcs.check_entity_exists(vcmpEntityPool.vcmpEntityPoolObject, object_id):
            return None
        object = next((object for object in _objects if object.id == object_id), None)
        if object is None:
            object = super().__new__(cls)
            _objects.append(object)
        return object

class Marker:
    def __init__(self, id: int):
        self._id = id

    def __hash__(self) -> int:
        return hash(self._id)

    def __eq__(self, value: object) -> bool:
        if isinstance(value, Marker):
            return self._id == value._id
        return False

    @property
    def id(self) -> int:
        return self._id

    @property
    def is_alive(self) -> bool:
        return funcs.check_entity_exists(vcmpEntityPool.vcmpEntityPoolBlip, self._id)

    @property
    def world(self):
        return funcs.get_coord_blip_info(self._id)[0]
    
    @property
    def model(self):
        return funcs.get_coord_blip_info(self._id)[6]

    @property
    def position(self):
        info = funcs.get_coord_blip_info(self._id)
        return Vector(info[1], info[2], info[3])

    @property
    def scale(self):
        return funcs.get_coord_blip_info(self._id)[4]
    
    @property
    def color(self):
        return RGB.from_alpha(funcs.get_coord_blip_info(self._id)[5])

    def delete(self):
        """
        Delete the pickup
        """
        if not funcs.check_entity_exists(vcmpEntityPool.vcmpEntityPoolBlip, self._id):
            return
        funcs.destroy_coord_blip(self._id)
        _markers.remove(self)

    def __new__(cls, marker_id: int) -> Optional['Marker']:
        if not funcs.check_entity_exists(vcmpEntityPool.vcmpEntityPoolBlip, marker_id):
            return None
        marker = next((marker for marker in _markers if marker.id == marker_id), None)
        if marker is None:
            marker = super().__new__(cls)
            _markers.append(marker)
        return marker

_players: list[Player] = []
_vehicles: list[Vehicle] = []
_pickups: list[Pickup] = []
_checkpoints: list[CheckPoint] = []
_objects: list[Object] = []
_markers: list[Marker] = []
_pools_max: dict[vcmpEntityPool, int] = {
    vcmpEntityPool.vcmpEntityPoolVehicle: MAX_VEHICLES,
    vcmpEntityPool.vcmpEntityPoolPickup: MAX_PICKUPS,
    vcmpEntityPool.vcmpEntityPoolObject: MAX_OBJECTS,
    vcmpEntityPool.vcmpEntityPoolCheckPoint: MAX_CHECKPOINTS,
    vcmpEntityPool.vcmpEntityPoolBlip: 99999
}

def get_player_from_id(
    player_id: int
) -> Optional[Player]:
    return next((player for player in _players if player.id == player_id), None)

def get_vehicle_from_id(
    vehicle_id: int
) -> Optional[Vehicle]:
    return next((vehicle for vehicle in _vehicles if vehicle.id == vehicle_id), None)

def get_pickup_from_id(
    pickup_id: int
) -> Optional[Pickup]:
    return next((pickup for pickup in _pickups if pickup.id == pickup_id), None)

def get_object_from_id(
    object_id: int
) -> Optional[Object]:
    return next((object for object in _objects if object.id == object_id), None)

def get_marker_from_id(
    marker_id: int
) -> Optional[Marker]:
    return next((marker for marker in _markers if marker.id == marker_id), None)

def get_checkpoint_from_id(
    checkpoint_id: int
) -> Optional[CheckPoint]:
    return next((checkpoint for checkpoint in _checkpoints if checkpoint.id == checkpoint_id), None)
    
def search_from_pool(
    pool: vcmpEntityPool,
) -> list[int]:
    max = _pools_max[pool]
    return [i for i in range(max) if funcs.check_entity_exists(pool, i)]

def _update_pool(
    pool: vcmpEntityPool,
    instance: Type[T]
) -> list[T]:
    searched = search_from_pool(pool)
    res = []
    for i in searched:
        obj = instance(i) # type: ignore
        if obj is None:
            continue
        res.append(obj)
    return res

def update_pool(
    pool: vcmpEntityPool,
):
    global _vehicles, _pickups, _objects, _checkpoints, _markers
    match pool:
        case vcmpEntityPool.vcmpEntityPoolVehicle:
            _vehicles = _update_pool(pool, Vehicle)
        case vcmpEntityPool.vcmpEntityPoolPickup:
            _pickups = _update_pool(pool, Pickup)
        case vcmpEntityPool.vcmpEntityPoolObject:
            _objects = _update_pool(pool, Object)
        case vcmpEntityPool.vcmpEntityPoolCheckPoint:
            _checkpoints = _update_pool(pool, CheckPoint)
        case vcmpEntityPool.vcmpEntityPoolBlip:
            _markers = _update_pool(pool, Marker)

def update_pools():
    for pool in _pools_max.keys():
        update_pool(pool)

def _on_pre_player_connect(
    player_id: int
):
    _players.append(Player(player_id))

def _on_post_player_disconnect(
    player_id: int,
    *args,
):
    instance = next((player for player in _players if player.id == player_id), None)
    if instance is not None:
        _players.remove(instance)

def _on_pre_entity_pool_update(
    pool: int,
    id: int,
    deleted: bool
):
    if deleted:
        return
    
    match pool:
        case vcmpEntityPool.vcmpEntityPoolVehicle:
            Vehicle(id)
        case vcmpEntityPool.vcmpEntityPoolPickup:
            Pickup(id)
        case vcmpEntityPool.vcmpEntityPoolObject:
            Object(id)
        case vcmpEntityPool.vcmpEntityPoolCheckPoint:
            CheckPoint(id)
        case vcmpEntityPool.vcmpEntityPoolBlip:
            Marker(id)

def _on_post_entity_pool_update(
    pool: int,
    id: int,
    deleted: bool
):
    if not deleted:
        return
    match pool:
        case vcmpEntityPool.vcmpEntityPoolVehicle:
            instance = next((vehicle for vehicle in _vehicles if vehicle.id == id), None)
            if instance is not None:
                _vehicles.remove(instance)
        case vcmpEntityPool.vcmpEntityPoolPickup:
            instance = next((pickup for pickup in _pickups if pickup.id == id), None)
            if instance is not None:
                _pickups.remove(instance)

setattr(calls, "on_pre_player_connect", _on_pre_player_connect)
setattr(calls, "on_post_player_disconnect", _on_post_player_disconnect)
setattr(calls, "on_pre_entity_pool_change", _on_pre_entity_pool_update)
setattr(calls, "on_post_entity_pool_change", _on_post_entity_pool_update)

#update_pools()