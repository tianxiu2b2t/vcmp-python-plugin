from enum import IntEnum
from __vcmp import functions as funcs # type: ignore
from __vcmp import callbacks as calls # type: ignore

INT32_MAX = 2147483647

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