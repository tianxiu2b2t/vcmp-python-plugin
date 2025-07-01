import math
from typing import Optional


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
    def color(self) -> 'RGB':
        return RGB.from_int(funcs.get_player_colour(self._id))

    @color.setter
    def color(self, color: 'RGB'):
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
    def position(self, position: 'Vector'):
        funcs.set_player_position(self._id, position.x, position.y, position.z)

    @property
    def speed(self):
        return Vector(
            **funcs.get_player_speed(self._id)
        )

    @speed.setter
    def speed(self, speed: 'Vector'):
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

    def send_raw_message(self, color: 'RGB', message: str):
        """
        Send message to the player
        """

        funcs.send_client_message(self._id, color.to_alpha(), message)

    def send_announce(self, type: int, message: str):
        """
        Send announce message to the player
        """
        funcs.send_game_message(self._id, type, message)

    def send_data(self, stream: 'WriteStream'):
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

    def set_camera_position(self, position: 'Vector', look_at: 'Vector'):
        """
        Set the player camera position
        """
        funcs.set_camera_position(self._id, position.x, position.y, position.z, look_at.x, look_at.y, look_at.z)

    def restore_camera(self):
        """
        Restore the player camera
        """
        funcs.restore_camera(self._id)

    def set_camera(self, position: 'Vector', look_yaw: float, look_pitch: float, range: float = 0.5):
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

    def add_position(self, position: 'Vector'):
        """
        Add position to the player position
        """

        new_pos = Vector(
            self.position.x + position.x,
            self.position.y + position.y,
            self.position.z + position.z
        )
        self.position = new_pos

    def add_speed(self, speed: 'Vector'):
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

import inspect

for member in inspect.getmembers(Player):
    name, value = member
    # if value is function
    if inspect.isfunction(value):
        print(f"fn {name}")
    else:
        print(f"fn prop_{name}")