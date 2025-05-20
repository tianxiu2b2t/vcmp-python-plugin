
from dataclasses import asdict, dataclass
from enum import IntEnum
from typing import Literal, TypedDict

FROM_DICT_TYPE = Literal["strict", "ignore"]
INT32_MAX = 0x7FFFFFFF
MAX_PLAYERS = 100
MAX_VEHICLES = 1000
MAX_OBJECTS = 3000
MAX_PICKUPS = 2000
MAX_CHECKPOINTS = 2000

@dataclass
class DataclassDict:
    """A dataclass that can be converted to a dictionary"""

    def to_dict(self) -> dict:
        return asdict(self)

    def from_dict(self, data: dict, error: FROM_DICT_TYPE = "strict"):
        for key, value in data.items():
            if not hasattr(self, key):
                if error == "strict":
                    raise AttributeError(f"Attribute {key} not found in {self.__class__.__name__}")
                elif error == "ignore":
                    continue

            setattr(self, key, value)

@dataclass
class WastedSettings(DataclassDict):
    death_timer: int
    fade_timer: int
    fade_in_speed: float
    fade_out_speed: float
    fade_colour: 'RGB'
    corpse_fade_start: int
    corpse_fade_time: int

@dataclass
class ServerSettings(DataclassDict):
    port: int
    maxplayers: int
    servername: str
    flags: int
    locked: bool
    gamemode: str
    password: str
    version: int

class vcmpError(IntEnum):
    vcmpErrorNone = 0
    vcmpErrorNoSuchEntity = 1
    vcmpErrorBufferTooSmall = 2
    vcmpErrorTooLargeInput = 3
    vcmpErrorArgumentOutOfBounds = 4
    vcmpErrorNullArgument = 5
    vcmpErrorPoolExhausted = 6
    vcmpErrorInvalidName = 7
    vcmpErrorRequestDenied = 8
    forceSizeVcmpError = INT32_MAX

class vcmpEntityPool(IntEnum):
    vcmpEntityPoolVehicle = 1
    vcmpEntityPoolObject = 2
    vcmpEntityPoolPickup = 3
    vcmpEntityPoolRadio = 4
    vcmpEntityPoolBlip = 7
    vcmpEntityPoolCheckPoint = 8
    forceSizeVcmpEntityPool = INT32_MAX

class vcmpDisconnectReason(IntEnum):
    vcmpDisconnectReasonTimeout = 0
    vcmpDisconnectReasonQuit = 1
    vcmpDisconnectReasonKick = 2
    vcmpDisconnectReasonCrash = 3
    vcmpDisconnectReasonAntiCheat = 4
    forceSizeVcmpDisconnectReason = INT32_MAX

class vcmpBodyPart(IntEnum):
    vcmpBodyPartBody = 0
    vcmpBodyPartTorso = 1
    vcmpBodyPartLeftArm = 2
    vcmpBodyPartRightArm = 3
    vcmpBodyPartLeftLeg = 4
    vcmpBodyPartRightLeg = 5
    vcmpBodyPartHead = 6
    vcmpBodyPartInVehicle = 7
    forceSizeVcmpBodyPart = INT32_MAX

class vcmpPlayerState(IntEnum):
    vcmpPlayerStateNone = 0
    vcmpPlayerStateNormal = 1
    vcmpPlayerStateAim = 2
    vcmpPlayerStateDriver = 3
    vcmpPlayerStatePassenger = 4
    vcmpPlayerStateEnterDriver = 5
    vcmpPlayerStateEnterPassenger = 6
    vcmpPlayerStateExit = 7
    vcmpPlayerStateUnspawned = 8
    forceSizeVcmpPlayerState = INT32_MAX

class vcmpPlayerUpdate(IntEnum):
    vcmpPlayerUpdateNormal = 0
    vcmpPlayerUpdateAiming = 1
    vcmpPlayerUpdateDriver = 2
    vcmpPlayerUpdatePassenger = 3
    forceSizeVcmpPlayerUpdate = INT32_MAX

class vcmpPlayerVehicle(IntEnum):
    vcmpPlayerVehicleOut = 0
    vcmpPlayerVehicleEntering = 1
    vcmpPlayerVehicleExiting = 2
    vcmpPlayerVehicleIn = 3
    forceSizeVcmpPlayerVehicle = INT32_MAX

class vcmpVehicleSync(IntEnum):
    vcmpVehicleSyncNone = 0
    vcmpVehicleSyncDriver = 1
    vcmpVehicleSyncPassenger = 3
    vcmpVehicleSyncNear = 4
    forceSizeVcmpVehicleSync = INT32_MAX

class vcmpVehicleUpdate(IntEnum):
    vcmpVehicleUpdateDriverSync = 0
    vcmpVehicleUpdateOtherSync = 1
    vcmpVehicleUpdatePosition = 2
    vcmpVehicleUpdateHealth = 4
    vcmpVehicleUpdateColour = 5
    vcmpVehicleUpdateRotation = 6
    forceSizeVcmpVehicleUpdate = INT32_MAX
    
class vcmpServerOption(IntEnum):
    vcmpServerOptionSyncFrameLimiter = 0
    vcmpServerOptionFrameLimiter = 1
    vcmpServerOptionTaxiBoostJump = 2
    vcmpServerOptionDriveOnWater = 3
    vcmpServerOptionFastSwitch = 4
    vcmpServerOptionFriendlyFire = 5
    vcmpServerOptionDisableDriveBy = 6
    vcmpServerOptionPerfectHandling = 7
    vcmpServerOptionFlyingCars = 8
    vcmpServerOptionJumpSwitch = 9
    vcmpServerOptionShowMarkers = 10
    vcmpServerOptionOnlyShowTeamMarkers = 11
    vcmpServerOptionStuntBike = 12
    vcmpServerOptionShootInAir = 13
    vcmpServerOptionShowNameTags = 14
    vcmpServerOptionJoinMessages = 15
    vcmpServerOptionDeathMessages = 16
    vcmpServerOptionChatTagsEnabled = 17
    vcmpServerOptionUseClasses = 18
    vcmpServerOptionWallGlitch = 19
    vcmpServerOptionDisableBackfaceCulling = 20
    vcmpServerOptionDisableHeliBladeDamage = 21
    forceSizeVcmpServerOption = INT32_MAX

class vcmpPlayerOption(IntEnum):
    vcmpPlayerOptionControllable = 0
    vcmpPlayerOptionDriveBy = 1
    vcmpPlayerOptionWhiteScanlines = 2
    vcmpPlayerOptionGreenScanlines = 3
    vcmpPlayerOptionWidescreen = 4
    vcmpPlayerOptionShowMarkers = 5
    vcmpPlayerOptionCanAttack = 6
    vcmpPlayerOptionHasMarker = 7
    vcmpPlayerOptionChatTagsEnabled = 8
    vcmpPlayerOptionDrunkEffects = 9
    forceSizeVcmpPlayerOption = INT32_MAX

class vcmpVehicleOption(IntEnum):
    vcmpVehicleOptionDoorsLocked = 0
    vcmpVehicleOptionAlarm = 1
    vcmpVehicleOptionLights = 2
    vcmpVehicleOptionRadioLocked = 3
    vcmpVehicleOptionGhost = 4
    vcmpVehicleOptionSiren = 5
    vcmpVehicleOptionSingleUse = 6
    forceSizeVcmpVehicleOption = INT32_MAX

class vcmpPickupOption(IntEnum):
    vcmpPickupOptionSingleUse = 0
    forceSizeVcmpPickupOption = INT32_MAX

class Weather(IntEnum):
    """Weather types."""

    MOSTLY_CLEAR_SKIES = 0
    OVERCAST = 1
    RAINY_LIGHTNING = 2
    FOGGY = 3
    CLEAR_SKIES = 4
    RAINY = 5
    DARK_SKY_PARTLY_CLOUDY = 6
    LIGHT_SKY_PARTLY_CLOUDY = 7
    OVERCAST_PARTLY_CLOUDY = 8
    GREY_SKY_BLACK_CLOUDS = 9

    WAVES_TOUCHES_SKY = 386
    BIG_WAVES = 375
    MEDIUM_WAVES = 377
    SMALLER_WAVES_BUT_STILL_BIGGER_THAN_NORMAL = 379
    DUST_ON_STREETS_LIKE_IN_TEXAS = 91

class WeaponField(IntEnum):
    """Weapon fields."""

    FIRE_TYPE = 1
    RANGE = 2
    FIRING_RATE = 3
    RELOAD = 4
    CLIP_SIZE = 5
    DAMAGE = 6
    SPEED = 7
    RADIUS = 8
    LIFESPAN = 9
    SPREAD = 10
    FIRE_OFFSET_X = 11
    FIRE_OFFSET_Y = 12
    FIRE_OFFSET_Z = 13
    ANIM_GROUP = 14
    ANIM_LOOP_START = 15
    ANIM_LOOP_END = 16
    ANIM_FIRE_POS = 17
    ANIM_TWO_LOOP_START = 18
    ANIM_TWO_LOOP_END = 19
    ANIM_TWO_FIRE_POS = 20
    ANIM_BREAKOUT_POS = 21
    MODEL_ID = 22
    MODEL_TWO_ID = 23
    FLAGS = 24
    WEAPON_SLOT = 25

class KeyCode(IntEnum):
    """ Key codes. """


    VK_LBUTTON = 0x01
    VK_RBUTTON = 0x02
    VK_CANCEL = 0x03
    VK_MBUTTON = 0x04
    VK_XBUTTON1 = 0x05
    VK_XBUTTON2 = 0x06
    VK_BACK = 0x08
    VK_TAB = 0x09
    VK_CLEAR = 0x0C
    VK_RETURN = 0x0D
    VK_SHIFT = 0x10
    VK_CONTROL = 0x11
    VK_MENU = 0x12
    VK_PAUSE = 0x13
    VK_CAPITAL = 0x14
    VK_KANA = 0x15
    VK_HANGUEL = 0x15
    VK_HANGUL = 0x15
    VK_JUNJA = 0x17
    VK_FINAL = 0x18
    VK_HANJA = 0x19
    VK_KANJI = 0x19
    VK_IME_OFF = 0x1A
    VK_ESCAPE = 0x1B
    VK_CONVERT = 0x1C
    VK_NONCONVERT = 0x1D
    VK_ACCEPT = 0x1E
    VK_MODECHANGE = 0x1F
    VK_SPACE = 0x20
    VK_PRIOR = 0x21
    VK_NEXT = 0x22
    VK_END = 0x23
    VK_HOME = 0x24
    VK_LEFT = 0x25
    VK_UP = 0x26
    VK_RIGHT = 0x27
    VK_DOWN = 0x28
    VK_SELECT = 0x29
    VK_PRINT = 0x2A
    VK_EXECUTE = 0x2B
    VK_SNAPSHOT = 0x2C
    VK_INSERT = 0x2D
    VK_DELETE = 0x2E
    VK_HELP = 0x2F
    VK_0 = 0x30
    VK_1 = 0x31
    VK_2 = 0x32
    VK_3 = 0x33
    VK_4 = 0x34
    VK_5 = 0x35
    VK_6 = 0x36
    VK_7 = 0x37
    VK_8 = 0x38
    VK_9 = 0x39
    VK_A = 0x41
    VK_B = 0x42
    VK_C = 0x43
    VK_D = 0x44
    VK_E = 0x45
    VK_F = 0x46
    VK_G = 0x47
    VK_H = 0x48
    VK_I = 0x49
    VK_J = 0x4A
    VK_K = 0x4B
    VK_L = 0x4C
    VK_M = 0x4D
    VK_N = 0x4E
    VK_O = 0x4F
    VK_P = 0x50
    VK_Q = 0x51
    VK_R = 0x52
    VK_S = 0x53
    VK_T = 0x54
    VK_U = 0x55
    VK_V = 0x56
    VK_W = 0x57
    VK_X = 0x58
    VK_Y = 0x59
    VK_Z = 0x5A
    VK_LWIN = 0x5B
    VK_RWIN = 0x5C
    VK_APPS = 0x5D
    VK_SLEEP = 0x5F
    VK_NUMPAD0 = 0x60
    VK_NUMPAD1 = 0x61
    VK_NUMPAD2 = 0x62
    VK_NUMPAD3 = 0x63
    VK_NUMPAD4 = 0x64
    VK_NUMPAD5 = 0x65
    VK_NUMPAD6 = 0x66
    VK_NUMPAD7 = 0x67
    VK_NUMPAD8 = 0x68
    VK_NUMPAD9 = 0x69
    VK_MULTIPLY = 0x6A
    VK_ADD = 0x6B
    VK_SEPARATOR = 0x6C
    VK_SUBTRACT = 0x6D
    VK_DECIMAL = 0x6E
    VK_DIVIDE = 0x6F
    VK_F1 = 0x70
    VK_F2 = 0x71
    VK_F3 = 0x72
    VK_F4 = 0x73
    VK_F5 = 0x74
    VK_F6 = 0x75
    VK_F7 = 0x76
    VK_F8 = 0x77
    VK_F9 = 0x78
    VK_F10 = 0x79
    VK_F11 = 0x7A
    VK_F12 = 0x7B
    VK_F13 = 0x7C
    VK_F14 = 0x7D
    VK_F15 = 0x7E
    VK_F16 = 0x7F
    VK_F17 = 0x80
    VK_F18 = 0x81
    VK_F19 = 0x82
    VK_F20 = 0x83
    VK_F21 = 0x84
    VK_F22 = 0x85
    VK_F23 = 0x86
    VK_F24 = 0x87
    VK_NUMLOCK = 0x90
    VK_SCROLL = 0x91
    VK_LSHIFT = 0xA0
    VK_RSHIFT = 0xA1
    VK_LCONTROL = 0xA2
    VK_RCONTROL = 0xA3
    VK_LMENU = 0xA4
    VK_RMENU = 0xA5
    VK_BROWSER_BACK = 0xA6
    VK_BROWSER_FORWARD = 0xA7
    VK_BROWSER_REFRESH = 0xA8
    VK_BROWSER_STOP = 0xA9
    VK_BROWSER_SEARCH = 0xAA
    VK_BROWSER_FAVORITES = 0xAB
    VK_BROWSER_HOME = 0xAC
    VK_VOLUME_MUTE = 0xAD
    VK_VOLUME_DOWN = 0xAE
    VK_VOLUME_UP = 0xAF
    VK_MEDIA_NEXT_TRACK = 0xB0
    VK_MEDIA_PREV_TRACK = 0xB1
    VK_MEDIA_STOP = 0xB2
    VK_MEDIA_PLAY_PAUSE = 0xB3
    VK_LAUNCH_MAIL = 0xB4
    VK_LAUNCH_MEDIA_SELECT = 0xB5
    VK_LAUNCH_APP1 = 0xB6
    VK_LAUNCH_APP2 = 0xB7
    VK_OEM_1 = 0xBA
    VK_OEM_PLUS = 0xBB
    VK_OEM_COMMA = 0xBC
    VK_OEM_MINUS = 0xBD
    VK_OEM_PERIOD = 0xBE
    VK_OEM_2 = 0xBF
    VK_OEM_3 = 0xC0
    VK_OEM_4 = 0xDB
    VK_OEM_5 = 0xDC
    VK_OEM_6 = 0xDD
    VK_OEM_7 = 0xDE
    VK_OEM_8 = 0xDF
    VK_OEM_102 = 0xE2
    VK_PROCESSKEY = 0xE5
    VK_PACKET = 0xE7
    VK_ATTN = 0xF6
    VK_CRSEL = 0xF7
    VK_EXSEL = 0xF8
    VK_EREOF = 0xF9
    VK_PLAY = 0xFA
    VK_ZOOM = 0xFB
    VK_NONAME = 0xFC
    VK_PA1 = 0xFD
    VK_OEM_CLEAR = 0xFE
    VK_UNKNOWN = 0xFF

class Version(IntEnum):
    """ Version enum. """
    v04rel002 = 66215
    v04rel003 = 66230
    v04rel004 = 67000
    v04rel006 = 67400
    v0_4_7_0 = 67700
    v0_4_7_1 = 67710

@dataclass
class AreaPoints:
    x: float
    y: float

@dataclass
class RGB:
    red: int
    green: int
    blue: int
    alpha: int = 255

    def to_int(self) -> int:
        return (self.red << 16) | (self.green << 8) | self.blue
    
    def to_alpha(self) -> int:
        return (self.red << 24) | (self.green << 16) | (self.blue << 8) | self.alpha
    
    def to_hex(self) -> str:
        return f"#{self.red:02x}{self.green:02x}{self.blue:02x}"
    
    def to_alpha_hex(self) -> str:
        return f"#{self.red:02x}{self.green:02x}{self.blue:02x}{self.alpha:02x}"
    
    def to_argb(self) -> int:
        return (self.alpha << 24) | (self.red << 16) | (self.green << 8) | self.blue
    
    def to_argb_hex(self) -> str:
        return f"#{self.alpha:02x}{self.red:02x}{self.green:02x}{self.blue:02x}"
    

    @staticmethod
    def from_int(value: int):
        return RGB((value >> 16) & 0xFF, (value >> 8) & 0xFF, value & 0xFF)
    
    @staticmethod
    def from_alpha(value: int):
        return RGB((value >> 24) & 0xFF, (value >> 16) & 0xFF, (value >> 8) & 0xFF, value & 0xFF)
    
    @staticmethod
    def from_argb(value: int):
        return RGB((value >> 16) & 0xFF, (value >> 8) & 0xFF, value & 0xFF, (value >> 24) & 0xFF)


    
    