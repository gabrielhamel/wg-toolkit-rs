pub use glam::{Vec2, Vec3, Vec4};

pub type Python = ();
pub type Mailbox = ();

pub type BOOL = u8;
pub type OBJECT_ID = i32;
pub type SHOT_ID = i32;
pub type DB_ID = i64;
pub type EXTRA_ID = u8;
pub type VEH_TYPE_CD = u32;

pub struct SERVER_STATISTICS {
    pub clusterCCU: u32,
    pub regionCCU: u32,
}

pub type QUEUE_INFO = Python;

pub struct DEFAULT_QUEUE_INFO {
    pub classes: Vec<u32>,
    pub players: u32,
}

pub struct MAPS_TRAINING_QUEUE_INFO {
    pub queues: Python,
}

pub struct FUN_RANDOM_QUEUE_INFO {
    pub events: Python,
}

pub struct PUBLIC_ARENA_INFO {
    pub id: OBJECT_ID,
    pub typeID: i32,
    pub roundLength: i32,
    pub roundStart: f32,
}

pub struct ATTACK_RESULTS {
    pub targetID: OBJECT_ID,
    pub targetVehicleIndex: u8,
    pub targetTeam: u8,
    pub targetTypeCompDescr: VEH_TYPE_CD,
    pub targetIsTeamKiller: BOOL,
    pub targetIsOnTheIgnoredBase: BOOL,
    pub targetIsOnTheCapturableBase: BOOL,
    pub targetIsNotSpotted: BOOL,
    pub targetMaxHealth: u16,
    pub targetHealthBeforeDamage: i16,
    pub enemiesNearTarget: u8,
    pub isRecoil: BOOL,
    pub reason: u8,
    pub shellCompDescr: i32,
    pub hitIndirection: u8,
    pub shotID: SHOT_ID,
    pub numVehiclesAffected: i16,
    pub hitFlags: i32,
    pub crits: i32,
    pub stunDuration: f32,
    pub allCrits: i32,
    pub anyDeviceWasDamaged: BOOL,
    pub damage: i32,
    pub repairCost: u32,
    pub critBonusFactor: f32,
    pub droppedCapturePoints: f32,
    pub trackAssistants: Vec<OBJECT_ID>,
    pub stunAssistants: Vec<OBJECT_ID>,
    pub smokeAssistants: Vec<OBJECT_ID>,
    pub distance: f32,
    pub targetInitialSpeed: f32,
    pub attackerInitialSpeed: f32,
    pub attackerHullDamage: u16,
    pub attackerKilledHimself: BOOL,
    pub attackerHealthBeforeDamage: i16,
    pub circularVisionRadius: f32,
    pub attackerWasInvisible: BOOL,
    pub equipmentID: u16,
    pub isIronShieldDamage: BOOL,
    pub attackerType: u8,
}

pub struct PREBATTLE_INVITE {
    pub createTime: u32,
    pub r#type: u16,
    pub comment: String,
    pub creator: String,
    pub creatorBadges: Python,
    pub creatorDBID: DB_ID,
    pub creatorClanAbbrev: String,
    pub extraData: Python,
}

pub struct PREBATTLE_RESULTS {
    pub winner: u8,
    pub finishReason: u8,
    pub kickReason: u8,
    pub extendedResults: Python,
}

pub struct PUBLIC_CHAT_CHANNEL_INFO {
    pub id: OBJECT_ID,
    pub channelName: String,
    pub isReadOnly: BOOL,
    pub isSecured: BOOL,
    pub flags: u8,
}

pub struct PUBLIC_USERS_ROSTER_MEMBER_INFO {
    pub id: DB_ID,
    pub nickName: String,
    pub accessFlags: u8,
}

pub struct CHAT_ACTION_DATA {
    pub requestID: i64,
    pub action: u8,
    pub actionResponse: u8,
    pub time: f64,
    pub sentTime: f64,
    pub channel: OBJECT_ID,
    pub originator: DB_ID,
    pub originatorNickName: String,
    pub group: u8,
    pub data: Python,
    pub flags: u8,
}

pub struct GENERIC_MESSENGER_ARGS_chat2 {
    pub int32Arg1: i32,
    pub int64Arg1: i64,
    pub floatArg1: f64,
    pub strArg1: String,
    pub strArg2: String,
}

pub struct DISCLOSE_EVENT {
    pub vehicleID: OBJECT_ID,
    pub playerName: BOOL,
    pub vehicleType: BOOL,
}

pub struct DOG_TAG_COMPONENT {
    pub id: i32,
    pub progress: f32,
    pub grade: i8,
}

pub struct DOG_TAG {
    pub components: Vec<DOG_TAG_COMPONENT>,
}

pub struct BATTLE_DOG_TAG {
    pub dogTag: DOG_TAG,
    pub defaultDogTag: DOG_TAG,
    pub showDogTagToKiller: BOOL,
}

pub struct VEHICLE_DOG_TAG {
    pub vehicleId: OBJECT_ID,
    pub dogTag: DOG_TAG,
}

pub struct DOG_TAG_SETTINGS {
    pub showVictimsDogTag: BOOL,
    pub showDogTagToKiller: BOOL,
}

pub struct PUBLIC_VEHICLE_INFO {
    pub name: String,
    pub compDescr: String,
    pub outfit: String,
    pub index: u8,
    pub team: u8,
    pub prebattleID: OBJECT_ID,
    pub marksOnGun: u8,
    pub crewGroups: Vec<u16>,
    pub commanderSkinID: u16,
    pub maxHealth: u16,
}

pub struct ARENA_VEH_INFO {
    pub vehInvID: i32,
    pub vehCompDescr: String,
    pub vehOutfit: String,
    pub vehAmmo: Vec<i32>,
    pub vehSetups: Python,
    pub vehSetupsIndexes: Python,
    pub vehCrew: Vec<String>,
    pub vehCrewInvIDs: Vec<i32>,
    pub vehCrewSkins: Python,
    pub marksOnGun: u8,
    pub isRent: BOOL,
    pub activeRent: u8,
    pub settings: u16,
    pub enhancements: Python,
    pub vehPerks: Python,
    pub customRoleSlotTypeId: u8,
    pub vehPostProgression: Vec<i32>,
    pub vehDisabledSetupSwitches: Vec<u8>,
    pub isSuitableForReferralBonus: BOOL,
    pub isElite: BOOL,
    pub prestigeLevel: i32,
    pub anonymizedPrestigeLevel: i32,
    pub prestigeGradeMarkID: i32,
}

pub struct USER_EXTRA_SETTINGS {
    pub hideNonHistoric: u8,
    pub dogTagsUserSettings: DOG_TAG_SETTINGS,
    pub moduleRepairTimer: BOOL,
    pub minimapMinSpottingRange: BOOL,
    pub battleNotifier: BOOL,
    pub additionalzoom: BOOL,
    pub hpinminimap: u8,
    pub hpinplayerspanels: u8,
    pub commandercam: BOOL,
    pub contour: BOOL,
    pub contourPenetrableZone: u8,
    pub contourImpenetrableZone: u8,
    pub crewPerks: BOOL,
    pub mapsInDevelopment: BOOL,
    pub postmortemMode: u8,
}

pub struct ARENA_ADDPLAYER_INFO {
    pub name: String,
    pub attrs: u64,
    pub databaseID: DB_ID,
    pub centerID: i32,
    pub clanAbbrev: String,
    pub clanDBID: DB_ID,
    pub prebattle: Mailbox,
    pub isPrebattleCreator: BOOL,
    pub forbidInBattleInvitations: BOOL,
    pub arenaUniqueID: u64,
    pub team: u8,
    pub tkillRating: f32,
    pub cybersportRating: Vec<f32>,
    pub globalRating: f32,
    pub igrType: i8,
    pub potapovQuestIDs: Vec<u16>,
    pub potapovQuestInfo: Python,
    pub vehiclesInfo: Vec<ARENA_VEH_INFO>,
    pub avatarAmmo: Vec<i32>,
    pub needCheckPenalties: BOOL,
    pub fairplayState: Python,
    pub battlesNum: u32,
    pub ranked: Vec<u16>,
    pub group: u8,
    pub recruiterId: DB_ID,
    pub premMask: i8,
    pub needsAnonymization: BOOL,
    pub wtr: i16,
    pub badges: Python,
    pub overriddenBadge: u8,
    pub dogTag: BATTLE_DOG_TAG,
    pub userExtraSettings: USER_EXTRA_SETTINGS,
    pub isSsrRecordEnabled: BOOL,
    pub isSsrPlayEnabled: BOOL,
    pub componentsData: Python,
}

pub struct AVATAR_VEHICLE_ROSTER {
    pub vehicleID: OBJECT_ID,
    pub prebattleID: OBJECT_ID,
    pub team: i8,
    pub observer: BOOL,
}

pub struct ATTACKER_INFO {
    pub baseMB: Mailbox,
    pub receiveAttackResultsMB: Mailbox,
    pub team: u8,
    pub position: Vec3,
    pub circularVisionRadius: f32,
    pub health: i16,
    pub noOwner: BOOL,
    pub attackerInitialSpeed: f32,
    pub attackerWasInvisible: BOOL,
    pub attackerTypeCompactDescr: VEH_TYPE_CD,
    pub attackerVehicleIndex: u8,
    pub attackerGunBurstCount: u8,
    pub equipmentID: u16,
    pub attackerType: u8,
    pub attackerSiegeState: i8,
    pub attackerMasterVehID: OBJECT_ID,
    pub needsCount: BOOL,
    pub damageDistributionLowerBound: f32,
    pub piercingDistributionLowerBound: f32,
    pub damageDistributionUpperBound: f32,
    pub piercingDistributionUpperBound: f32,
    pub criticalHitChanceBoost: f32,
    pub attackerDualAccuracyState: i8,
}

pub struct DESTRUCTIBLE_ATTACK_INFO {
    pub hitPoint: Vec3,
    pub shotID: i32,
    pub attacker: ATTACKER_INFO,
}

pub struct CLIENT_STATUS_STATISTICS {
    pub ping_lt_50: f32,
    pub ping_51_100: f32,
    pub ping_101_150: f32,
    pub ping_151_400: f32,
    pub ping_gt_400: f32,
    pub lag: f32,
}

pub struct VEHICLE_SPATIAL_INFO {
    pub vehicleID: OBJECT_ID,
    pub team: u8,
    pub position: Vec3,
    pub isAlive: BOOL,
    pub vehClass: String,
    pub prebattleID: OBJECT_ID,
}

pub struct VEHICLE_SYNC_ATTRS {
    pub circularVisionRadius: u16,
    pub gunPiercing: f32,
    pub gunShotsSpeed: f32,
    pub gunCanShoot: BOOL,
}

pub struct IS_OBSERVED_BY_ENEMY {
    pub endTime: f32,
}

pub struct SIEGE_STATE_STATUS {
    pub status: u8,
    pub endTime: f32,
    pub timeLeft: f32,
}

pub struct BURNOUT_WARNING {
    pub status: u8,
}

pub struct DUAL_GUN_STATUS_TIMES {
    pub baseTime: f32,
    pub timeLeft: f32,
    pub endTime: f32,
}

pub struct DUAL_GUN_STATUS {
    pub status: u8,
    pub times: DUAL_GUN_STATUS_TIMES,
}

pub struct DESTROYED_DEVICE_IS_REPAIRING {
    pub extraIndex: u8,
    pub progress: u8,
    pub endTime: f32,
    pub timeLeft: f32,
    pub repairMode: u8,
}

pub struct IS_OTHER_VEHICLE_DAMAGED_DEVICES_VISIBLE {
    pub status: u8,
}

pub struct BURNOUT_UNAVAILABLE {
    pub status: BOOL,
}

pub struct OVERTURN_LEVEL {
    pub level: u8,
    pub times: Vec<f64>,
}

pub struct IN_AOE_ZONE_STATUS {
    pub zoneID: OBJECT_ID,
    pub equipmentID: u16,
    pub team: u8,
    pub startTime: f64,
    pub endTime: f64,
}

pub struct DROWN_LEVEL {
    pub level: u8,
    pub times: Vec<f64>,
}

pub struct BATTLE_EVENT {
    pub eventType: u8,
    pub targetID: OBJECT_ID,
    pub details: u64,
    pub count: u16,
}

pub struct BATTLE_EVENTS_SUMMARY {
    pub damage: u32,
    pub trackAssist: u32,
    pub radioAssist: u32,
    pub stunAssist: u32,
    pub smokeAssist: u32,
    pub inspireAssist: u32,
    pub tankings: u32,
    pub lastKillerID: OBJECT_ID,
    pub lastDeathReasonID: u8,
}

pub struct REMOTE_CAMERA_DATA {
    pub time: f64,
    pub shotPoint: Vec3,
    pub zoom: u8,
    pub mode: u8,
}

pub type STUN_INFO = f64;

pub struct FRONT_LINE_DATA {
    pub columnWidth: f32,
    pub frontDropPerColumn: f32,
    pub outlierFraction: f32,
    pub outlierVerticalDistance: f32,
    pub intrusionVerticalTolerance: f32,
    pub intrusionCheckExtendBounds: f32,
    pub defenderTeam: u8,
    pub frontEdgeExtendColumns: u8,
    pub frontLineIds: Vec<u8>,
    pub frontLineBounds: Vec<Box<[Vec2; 2]>>,
    pub frontLineMainDirVecs: Vec<Vec2>,
}

pub struct AVATAR_AMMO_VIEWS {
    pub vehTypeCompDescrs: Vec<i32>,
    pub compDescrs: Vec<Vec<i32>>,
}

pub struct AVATAR_AMMO_FOR_CELL {
    pub abilitiesIDs: Python,
    pub purchasedAbilities: Vec<i8>,
}

pub struct POST_PROGRESSION_SETUPS {
    pub devicesSetups: Vec<Vec<u32>>,
    pub eqsSetups: Vec<Vec<u32>>,
    pub shellsSetups: Vec<Vec<u32>>,
    pub boostersSetups: Vec<Vec<u32>>,
}

pub struct RESPAWN_AVAILABLE_VEHICLE {
    pub compDescr: String,
    pub crewCompactDescrs: Vec<String>,
    pub settings: u16,
    pub vehSetups: POST_PROGRESSION_SETUPS,
    pub vehSetupsIndexes: Vec<u8>,
    pub vehPostProgression: Vec<i32>,
    pub customRoleSlotTypeId: u8,
    pub vehDisabledSetupSwitches: Vec<u8>,
}

pub struct RESPAWN_COOLDOWN_ITEM {
    pub vehTypeCompDescr: VEH_TYPE_CD,
    pub endOfCooldownPiT: f32,
}

pub struct RESPAWN_ZONE {
    pub position: Vec3,
    pub isEnemyNear: BOOL,
}

pub struct RESPAWN_INFO {
    pub compDescr: String,
    pub respawnType: u8,
    pub autoRespawnPiT: f32,
    pub manualRespawnPiT: f32,
    pub respawnZones: Vec<RESPAWN_ZONE>,
    pub chosenRespawnZone: Vec3,
    pub vehSetupsIndexes: Vec<u8>,
}

pub struct RESPAWN_LIMITED_VEHICLES {
    pub group: u8,
    pub vehTypeCompDescrs: Vec<VEH_TYPE_CD>,
}

pub struct RESPAWN_INFO_VEHICLE {
    pub compDescr: String,
    pub crewCompactDescrs: Vec<String>,
    pub commanderSkinID: u16,
    pub marksOnGun: u8,
    pub index: u16,
    pub position: Vec3,
    pub yaw: f32,
    pub prevGroup: u8,
    pub group: u8,
    pub policyID: u8,
    pub onRespawnSettings: Python,
    pub ammo: Vec<i32>,
    pub outfit: String,
    pub vehPerks: Python,
    pub vehSetups: Python,
    pub vehSetupsIndexes: Python,
    pub vehPostProgression: Vec<i32>,
    pub customRoleSlotTypeId: u8,
    pub vehDisabledSetupSwitches: Vec<u8>,
    pub prestigeLevel: i32,
    pub prestigeGradeMarkID: i32,
}

pub struct BUFF_EFFECT {
    pub radius: f32,
    pub startTime: f64,
    pub endTime: f64,
    pub inactivationDelay: f32,
}

pub struct DOT_EFFECT {
    pub endTime: f64,
    pub period: f32,
    pub groupId: u8,
    pub attackReasonID: u8,
}

pub struct BUFF_EFFECT_INACTIVATION {
    pub senderKey: String,
    pub startTime: f64,
    pub endTime: f64,
    pub inactivationStartTime: f64,
    pub inactivationEndTime: f64,
}

pub struct HOT_EFFECT {
    pub senderKey: String,
    pub startTime: f64,
    pub endTime: f64,
    pub inactivationStartTime: f64,
    pub inactivationEndTime: f64,
    pub isInfluenceZone: BOOL,
}

pub struct INSPIRED_EFFECT {
    pub primary: BOOL,
    pub startTime: f64,
    pub endTime: f64,
    pub inactivationStartTime: f64,
    pub inactivationEndTime: f64,
    pub inactivationSource: BOOL,
    pub equipmentID: u16,
}

pub struct SMOKE_INFO {
    pub smokeID: f64,
    pub equipmentID: u16,
    pub endTime: f32,
    pub team: u8,
    pub expiring: BOOL,
}

pub struct VEHICLE_DETECTOR_INFO {
    pub detectorID: u16,
    pub point: Vec3,
    pub radius: f32,
    pub aliveOnly: BOOL,
}

pub struct COOLDOWN_INFO {
    pub id: u8,
    pub endTime: f32,
    pub baseTime: f32,
    pub leftTime: f32,
}

pub struct CLIENT_VEHICLE_PUBLIC_INFO {
    pub vehID: OBJECT_ID,
    pub name: String,
    pub realName: String,
    pub compDescr: String,
    pub team: i8,
    pub isAlive: i8,
    pub isAvatarReady: i8,
    pub isTeamKiller: i8,
    pub accountDBID: DB_ID,
    pub clanAbbrev: String,
    pub clanDBID: DB_ID,
    pub prebattleID: OBJECT_ID,
    pub isPrebattleCreator: i8,
    pub forbidInBattleInvitations: i8,
    pub events: Python,
    pub igrType: i8,
    pub potapovQuestIDs: Vec<u16>,
    pub potapovQuestInfo: Python,
    pub ranked: Python,
    pub outfit: String,
    pub sessionID: String,
    pub wtr: i16,
    pub badges: Python,
    pub overriddenBadge: u8,
    pub maxHealth: u16,
    pub vehPostProgression: Vec<i32>,
    pub customRoleSlotTypeId: u8,
    pub botDisplayStatus: u8,
    pub prestigeLevel: i32,
    pub anonymizedPrestigeLevel: i32,
    pub prestigeGradeMarkID: i32,
}

pub struct PLAY_LIMITS {
    pub curfew: i32,
    pub weeklyPlayLimit: i32,
    pub dailyPlayLimit: i32,
    pub sessionLimit: i32,
}

pub struct VEHICLE_IN_DEATHZONE {
    pub vehicleId: OBJECT_ID,
    pub nextStrikeTime: f32,
    pub waveDuration: f32,
}

pub struct TIME_WITH_REASON {
    pub endTime: i32,
    pub totalTime: i32,
    pub reason: u8,
}

pub struct PERK_INFO_HUD {
    pub perkID: OBJECT_ID,
    pub state: u8,
    pub coolDown: f32,
    pub lifeTime: f64,
}

pub struct PERK_INFO_RIBBON {
    pub perkID: OBJECT_ID,
    pub endTime: f64,
}

pub struct TOURNAMEMT_PARTICIPANT {
    pub databaseID: DB_ID,
    pub name: String,
    pub role: u8,
    pub teamID: u8,
    pub typeCD: i32,
}

pub struct STEALTH_RADAR_INFO {
    pub equipmentID: u16,
    pub isActive: BOOL,
    pub duration: f32,
    pub endTime: f32,
}

pub struct REGENERATION_KIT_INFO {
    pub isActive: BOOL,
    pub duration: f32,
    pub endTime: f32,
}

pub struct VEHICLE_HEALTH_INFO {
    pub id: OBJECT_ID,
    pub health: i16,
    pub deathReasonID: i8,
    pub isCrewActive: BOOL,
    pub isRespawnActive: BOOL,
}

pub struct GUN_MARKER {
    pub gunPosition: Vec3,
    pub shotVector: Vec3,
    pub dispersion: f32,
}

pub struct KILL_CAM_ATTACKER_SPOTTED {
    pub position: Vec3,
    pub rotation: Vec3,
    pub gunAngles: Vec2,
    pub velocity: Vec3,
    pub angVelocity: Vec3,
}

pub struct KILL_CAM_ATTACKER_UNSPOTTED {
    pub health: u16,
    pub vehicleType: String,
}

pub struct KILL_CAM_ATTACKER {
    pub attackerID: OBJECT_ID,
    pub spottedData: KILL_CAM_ATTACKER_SPOTTED,
    pub unspottedData: KILL_CAM_ATTACKER_UNSPOTTED,
}

pub struct KILL_CAM_VICTIM {
    pub position: Vec3,
    pub rotation: Vec3,
    pub health: u16,
    pub relativeArmor: f32,
    pub victimIsNotSpotted: BOOL,
    pub siegeState: u16,
    pub damageStickers: Vec<u64>,
    pub causeOfDeath: String,
}

pub struct KILL_CAM_PROJECTILE_UNSPOTTED {
    pub shotID: SHOT_ID,
    pub trajectoryData: Vec<Box<[Vec3; 2]>>,
    pub gravity: f32,
    pub impactPoint: Vec3,
    pub isExplosion: BOOL,
    pub explosionRadius: f32,
    pub nominalArmor: f32,
    pub angleGain: f32,
    pub hitAngleCos: f32,
    pub triNormal: Vec3,
    pub hasProjectilePierced: BOOL,
    pub hasNonPiercedDamage: BOOL,
    pub ricochetCount: u16,
    pub ricochetAngleCos: f32,
    pub effectiveShellDamage: f32,
    pub damageRandomizationFactor: f32,
    pub is2CaliberRuleActive: BOOL,
    pub is3CaliberRuleActive: BOOL,
    pub impactType: u16,
    pub maxPenetrationAngle: f32,
    pub armorProtectionHE: f32,
    pub spallLinerProtectionHE: f32,
    pub velocity: Vec3,
    pub shellCompDescr: i32,
    pub segments: Vec<u64>,
    pub piercingPower: f32,
    pub nominalPiercingPower: f32,
    pub shellDamageBurstHE: f32,
    pub distanceLossHE: f32,
    pub averageDamageOfShell: f32,
    pub hasDistanceFalloff: BOOL,
}

pub struct KILL_CAM_PROJECTILE_SPOTTED {
    pub distanceOfShot: f32,
    pub damageDistanceModifier: f32,
}

pub struct KILL_CAM_PROJECTILE {
    pub unspottedData: KILL_CAM_PROJECTILE_UNSPOTTED,
    pub spottedData: KILL_CAM_PROJECTILE_SPOTTED,
}

pub struct KILL_CAM_DATA {
    pub attacker: KILL_CAM_ATTACKER,
    pub victim: KILL_CAM_VICTIM,
    pub projectile: KILL_CAM_PROJECTILE,
    pub statusCode: u8,
}

pub struct TRACK_STATE {
    pub isBroken: BOOL,
    pub hitPoint: Vec3,
    pub isDebris: BOOL,
}

pub struct VEHICLE_HIT_DIRECTION {
    pub hitDirYaw: f32,
    pub attackerID: OBJECT_ID,
    pub damage: u16,
    pub crits: u32,
    pub isBlocked: BOOL,
    pub isShellHE: BOOL,
    pub damagedID: OBJECT_ID,
    pub attackReasonID: i8,
}

pub struct VEHICLE_CLIP_RELOAD_TIME {
    pub endTime: f32,
    pub baseTime: f32,
    pub timeLeft: f32,
    pub firstTime: f32,
    pub stunned: BOOL,
    pub isBoostApplicable: BOOL,
}

pub struct VEHICLE_GUN_RELOAD_TIME {
    pub endTime: f32,
    pub baseTime: f32,
    pub timeLeft: f32,
}

pub struct OWN_VEHICLE_POSITION {
    pub position: Vec3,
    pub direction: Vec3,
    pub speed: f32,
    pub rotationSpeed: f32,
}

pub struct TARGET_VEHICLE_ID {
    pub targetID: OBJECT_ID,
}

pub struct DUAL_GUN_STATE {
    pub activeGun: u8,
    pub gunStates: Vec<u8>,
    pub cooldowns: Vec<COOLDOWN_INFO>,
}

pub struct VEHICLE_AMMO {
    pub compactDescr: i32,
    pub quantity: u16,
    pub quantityInClip: u16,
    pub previousStage: u16,
    pub endTime: f32,
    pub totalTime: i16,
    pub index: u8,
}

pub struct VEHICLE_DAMAGE_INFO {
    pub extraIndex: EXTRA_ID,
    pub damageIndex: u8,
    pub entityID: OBJECT_ID,
    pub equipmentID: u16,
}

pub struct VEHICLE_OPTIONAL_DEVICE_STATUS {
    pub deviceID: u8,
    pub isOn: u8,
}

pub struct VEHICLE_SETTING {
    pub vehicleID: OBJECT_ID,
    pub code: u8,
    pub value: i32,
}

pub struct TARGETING_INFO {
    pub turretYaw: f32,
    pub gunPitch: f32,
    pub maxTurretRotationSpeed: f32,
    pub maxGunRotationSpeed: f32,
    pub shotDispMultiplierFactor: f32,
    pub gunShotDispersionFactorsTurretRotation: f32,
    pub chassisShotDispersionFactorsMovement: f32,
    pub chassisShotDispersionFactorsRotation: f32,
    pub aimingTime: f32,
}

pub struct BATTLE_EVENTS {
    pub events: Vec<BATTLE_EVENT>,
    pub endTime: f32,
}

pub struct WELCOME_TO_SECTOR {
    pub sectorID: u8,
    pub groupID: u8,
    pub groupState: u8,
    pub goodGroup: BOOL,
    pub actionTime: f32,
    pub actionDuration: f32,
}

pub struct SECTOR_SHOOTING {
    pub sectorID: u8,
}

pub struct PLANE_TRAJECTORY {
    pub equipmentID: u16,
    pub team: u8,
    pub curTime: f64,
    pub curPos: Vec3,
    pub curDir: Vec2,
    pub nextTime: f64,
    pub nextPos: Vec3,
    pub nextDir: Vec2,
    pub isEndOfFlight: BOOL,
}

pub struct FIRE_INFO {
    pub deviceExtraIndex: EXTRA_ID,
    pub notificationIndex: u8,
    pub attackerID: OBJECT_ID,
    pub equipmentID: u16,
    pub startTime: f64,
}

pub struct DEBUFF_INFO {
    pub attackReasonID: u16,
    pub finishTime: f32,
}

pub struct TIMER_INFO {
    pub factor: f32,
    pub left: f32,
    pub endTime: f32,
}

pub struct HEALTH_INFO {
    pub vehicleID: OBJECT_ID,
    pub health: u16,
}

pub struct UDO_INFO {
    pub guid: String,
    pub position: Vec2,
}

pub struct ROCKET_ACCELERATION_STATE_STATUS {
    pub status: u8,
    pub endTime: f32,
    pub timeLeft: u16,
    pub reuseCount: i8,
}

pub struct AUTO_SHOOT_GUN_STATE_STATUS {
    pub state: u8,
    pub stateActivationTime: f32,
    pub dispersionFactor: f32,
    pub updateTime: f32,
    pub shotDispersionPerSec: f32,
    pub maxShotDispersion: f32,
}

pub struct GOODIE_RESOURCE {
    pub r#type: u8,
    pub value: u16,
    pub isPercentage: BOOL,
}

pub struct GOODIE_STATE_INFO {
    pub state: u8,
    pub finishTime: f64,
    pub count: u16,
    pub expirations: Python,
}

pub struct BATTLE_GOODIE_RECORD {
    pub goodieID: u32,
    pub lifetime: u16,
    pub useby: u64,
    pub resource: GOODIE_RESOURCE,
    pub stateInfo: GOODIE_STATE_INFO,
}

pub struct ROLE_EQUIPMENT_STATE {
    pub level: u8,
    pub progress: f32,
}

pub struct STATUS_WITH_TIME_INTERVAL {
    pub statusID: u8,
    pub startTime: f32,
    pub endTime: f32,
}

pub struct TIME_INTERVAL {
    pub startTime: f32,
    pub endTime: f32,
}

pub struct STATE_WITH_TIME_INTERVAL {
    pub stateID: u8,
    pub timeInterval: TIME_INTERVAL,
    pub isSourceVehicle: BOOL,
}

pub struct VISUAL_SCRIPT_EQUIPMENT_STATE {
    pub quantity: i32,
    pub endTime: f32,
    pub totalTime: f32,
    pub prevStage: u8,
    pub stage: u8,
    pub locked: BOOL,
}

pub struct VISUAL_SCRIPT_EQUIPMENT_PUBLIC_STATE {
    pub stage: u8,
}

pub struct SPAWN_AVAILABLE_VEHICLE {
    pub compDescr: String,
    pub settings: u16,
    pub isRent: BOOL,
    pub isElite: BOOL,
}

pub struct SPAWN_POINT_INFO {
    pub vehicleID: OBJECT_ID,
    pub number: i8,
}

pub struct SPAWN_KEY_POINT {
    pub guid: String,
    pub position: Vec2,
}

pub struct TEAM_SPAWN_KEY_POINT {
    pub vehID: OBJECT_ID,
    pub guid: String,
    pub placed: BOOL,
}

pub struct GAME_OBJECT_STATE {
    pub id: String,
    pub state: BOOL,
}

pub struct GAME_OBJECT_ACTIVATION {
    pub id: String,
    pub start: f32,
    pub end: f32,
}

pub struct POLYGON_TRIGGER_PARAMS {
    pub step: u16,
    pub origin: Vec3,
    pub dimensions: Vec3,
    pub verts: Vec<Vec3>,
    pub segments: Vec<Vec<u16>>,
}

pub struct SPAWNGROUP_INFO {
    pub name: String,
    pub position: Vec2,
}

pub struct TEAM_LIVES {
    pub vehicleID: OBJECT_ID,
    pub lives: i8,
    pub usedLives: i8,
    pub lockedLives: i8,
}

pub struct DEATH_INFO {
    pub victimID: OBJECT_ID,
    pub killerID: OBJECT_ID,
    pub reasonID: i8,
    pub equipmentID: i16,
    pub numVehiclesAffected: i16,
}

pub struct VEHICLES_INFO {
    pub vehicleID: OBJECT_ID,
    pub isAlive: BOOL,
    pub outfitCD: String,
    pub compDescr: String,
    pub fakeName: String,
    pub name: String,
    pub team: i8,
    pub isAvatarReady: BOOL,
    pub isTeamKiller: BOOL,
    pub accountDBID: u64,
    pub clanAbbrev: String,
    pub clanDBID: DB_ID,
    pub prebattleID: OBJECT_ID,
    pub isPrebattleCreator: BOOL,
    pub forbidInBattleInvitations: BOOL,
    pub igrType: i8,
    pub avatarSessionID: String,
    pub overriddenBadge: i8,
    pub customRoleSlotTypeId: i8,
    pub botDisplayStatus: i8,
    pub teamPanelMode: u8,
    pub maxHealth: i16,
    pub prestigeLevel: i32,
    pub prestigeGradeMarkID: i32,
    pub vehPostProgression: Vec<i32>,
    pub personalMissionIDs: Vec<i32>,
    pub personalMissionInfo: Python,
    pub events: Python,
    pub badges: Python,
    pub ranked: Vec<u16>,
    pub deathInfo: DEATH_INFO,
    pub __generation: u32,
}

pub struct PVE_MINIMAP_DATA {
    pub minimapBorders: Box<[Vec2; 2]>,
    pub zoomLevel: f32,
}

pub struct PVE_TARGET_MARKER {
    pub settingId: i16,
    pub targetId: i32,
    pub visibleStyle: String,
    pub invisibleStyle: String,
    pub lastVisiblePosition: Vec3,
}

pub struct RESPAWN_TIME_INFO {
    pub vehicleID: OBJECT_ID,
    pub spawnTime: f32,
    pub delay: f32,
}

pub struct CIRCUIT_OVERLOAD_INFO {
    pub notificationIndex: u8,
    pub prefabPath: String,
}

pub struct IMPACT_AREA_INFO {
    pub timeBeforeDamage: f32,
    pub isVisible: BOOL,
}

pub struct ANON71 {
    pub equipment: Vec<String>,
}

pub struct ANON164 {
    pub points: Vec<Vec3>,
    pub width: f32,
}

pub struct ANON166 {
    pub position: Vec3,
    pub size: Vec3,
}

pub struct ANON168 {
    pub position: Vec3,
    pub radius: Vec3,
}

pub struct ANON170 {
    pub position: Vec3,
    pub text: String,
    pub color: Vec4,
    pub textSize: f32,
}

pub struct ANON172 {
    pub name: String,
    pub version: u32,
    pub destroyTime: f32,
    pub lines: Vec<ANON164>,
    pub cubes: Vec<ANON166>,
    pub spheres: Vec<ANON168>,
    pub texts: Vec<ANON170>,
}