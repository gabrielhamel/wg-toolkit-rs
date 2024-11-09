use wgtk::net::app::common::element::Method;

use super::super::alias::*;
use super::interface::*;

/// Entity 0x01
/// Methods for Account on client component
pub enum AccountMethod { 
    Chat(ChatMethod),
    PlayerMessenger_chat2(PlayerMessenger_chat2Method),
    ClientCommandsPort(ClientCommandsPortMethod),
    AccountAuthTokenProviderClient(AccountAuthTokenProviderClientMethod),
    InvitationsClient(InvitationsClientMethod),
    onKickedFromServer(String, u8, u32), // idx(0)
    onEnqueued(u8), // idx(1)
    onEnqueueFailure(u8, u8, String), // idx(2)
    onDequeued(u8), // idx(3)
    onKickedFromQueue(u8), // idx(4)
    onIGRTypeChanged(String), // idx(5)
    onArenaJoinFailure(u8, String), // idx(6)
    onPrebattleJoined(OBJECT_ID), // idx(7)
    onPrebattleJoinFailure(u8), // idx(8)
    onKickedFromArena(u8), // idx(9)
    onKickedFromPrebattle(u8), // idx(10)
    onCenterIsLongDisconnected(BOOL), // idx(11)
    showGUI(String), // idx(12)
    receiveActiveArenas(Vec<PUBLIC_ARENA_INFO>), // idx(13)
    receiveServerStats(SERVER_STATISTICS), // idx(14)
    receiveQueueInfo(QUEUE_INFO), // idx(15)
    updatePrebattle(u8, String), // idx(16)
    update(String), // idx(17)
    resyncDossiers(BOOL), // idx(18)
    onUnitUpdate(u64, String, String), // idx(19)
    onUnitCallOk(i32), // idx(20)
    onUnitNotify(u64, i32, String, Python), // idx(21)
    onUnitError(i32, u64, i32, String), // idx(22)
    onUnitBrowserError(i32, String), // idx(23)
    onUnitBrowserResultsSet(String), // idx(24)
    onUnitBrowserResultsUpdate(String), // idx(25)
    onGlobalMapUpdate(String, String), // idx(26)
    onGlobalMapReply(u64, i32, String), // idx(27)
    onSendPrebattleInvites(DB_ID, String, DB_ID, String, u64, u8), // idx(28)
    onClanInfoReceived(DB_ID, String, String, String, String), // idx(29)
    receiveNotification(String), // idx(30)
}

// 0: onEnqueued [Fixed(1)] @ []
// 1: onDequeued [Fixed(1)] @ []
// 2: onKickedFromQueue [Fixed(1)] @ []
// 3: onPrebattleJoinFailure [Fixed(1)] @ []
// 4: onKickedFromArena [Fixed(1)] @ []
// 5: onKickedFromPrebattle [Fixed(1)] @ []
// 6: onCenterIsLongDisconnected [Fixed(1)] @ []
// 7: resyncDossiers [Fixed(1)] @ []
// 8: onPrebattleJoined [Fixed(4)] @ []
// 9: onUnitCallOk [Fixed(4)] @ []
// 10: receiveServerStats [Fixed(8)] @ []
// 11: onKickedFromServer [Variable(Variable8)] @ []
// 12: onEnqueueFailure [Variable(Variable8)] @ []
// 13: onIGRTypeChanged [Variable(Variable8)] @ []
// 14: onArenaJoinFailure [Variable(Variable8)] @ []
// 15: receiveActiveArenas [Variable(Variable8)] @ []
// 16: receiveQueueInfo [Variable(Variable8)] @ []
// 17: updatePrebattle [Variable(Variable8)] @ []
// 18: update [Variable(Variable8)] @ []
// 19: onUnitUpdate [Variable(Variable8)] @ []
// 20: onUnitNotify [Variable(Variable8)] @ []
// 21: onUnitError [Variable(Variable8)] @ []
// 22: onUnitBrowserError [Variable(Variable8)] @ []
// 23: onUnitBrowserResultsSet [Variable(Variable8)] @ []
// 24: onUnitBrowserResultsUpdate [Variable(Variable8)] @ []
// 25: onGlobalMapUpdate [Variable(Variable8)] @ []
// 26: onGlobalMapReply [Variable(Variable8)] @ []
// 27: onSendPrebattleInvites [Variable(Variable8)] @ []
// 28: onClanInfoReceived [Variable(Variable8)] @ []
// 29: receiveNotification [Variable(Variable8)] @ []
// 30: onChatAction [Variable(Variable8)] @ ["Chat"]
// 31: messenger_onActionByServer_chat2 [Variable(Variable8)] @ ["PlayerMessenger_chat2"]
// 32: onCmdResponse [Variable(Variable8)] @ ["ClientCommandsPort"]
// 33: onCmdResponseExt [Variable(Variable8)] @ ["ClientCommandsPort"]
// 34: onTokenReceived [Variable(Variable8)] @ ["AccountAuthTokenProviderClient"]
// 35: processInvitations [Variable(Variable8)] @ ["InvitationsClient"]
// 36: showGUI [Variable(Variable16)] @ []

/// Entity 0x02
/// Methods for Avatar on client component
pub enum AvatarMethod { 
    Chat(ChatMethod),
    PlayerMessenger_chat2(PlayerMessenger_chat2Method),
    ClientCommandsPort(ClientCommandsPortMethod),
    InvitationsClient(InvitationsClientMethod),
    AccountAuthTokenProviderClient(AccountAuthTokenProviderClientMethod),
    TeamHealthBar_Avatar(TeamHealthBar_AvatarMethod),
    RecoveryMechanic_Avatar(RecoveryMechanic_AvatarMethod),
    RespawnController_Avatar(RespawnController_AvatarMethod),
    VehiclesSpawnListStorage_Avatar(VehiclesSpawnListStorage_AvatarMethod),
    VehicleRemovalController_Avatar(VehicleRemovalController_AvatarMethod),
    VehicleHealthBroadcastListenerComponent_Avatar(VehicleHealthBroadcastListenerComponent_AvatarMethod),
    TriggersController_Avatar(TriggersController_AvatarMethod),
    AvatarEpic(AvatarEpicMethod),
    update(String), // idx(0)
    onKickedFromServer(String, u8, u32), // idx(1)
    onIGRTypeChanged(String), // idx(2)
    onAutoAimVehicleLost(u8), // idx(3)
    receiveAccountStats(u32, String), // idx(4)
    updateVehicleHealth(OBJECT_ID, i16, i8, BOOL, BOOL), // idx(5)
    updateVehicleAmmo(OBJECT_ID, i32, u16, u8, u8, i16, i16, i16), // idx(6)
    onSwitchViewpoint(OBJECT_ID, Vec3), // idx(7)
    updateVehicleSetting(OBJECT_ID, u8, i32), // idx(8)
    updateTargetingInfo(f32, f32, f32, f32, f32, f32, f32, f32, f32), // idx(9)
    updateTargetVehicleID(OBJECT_ID), // idx(10)
    showOwnVehicleHitDirection(f32, OBJECT_ID, u16, u32, BOOL, BOOL, OBJECT_ID, u8), // idx(11)
    showOtherVehicleDamagedDevices(OBJECT_ID, Vec<EXTRA_ID>, Vec<EXTRA_ID>), // idx(12)
    showShotResults(Vec<u64>), // idx(13)
    showDevelopmentInfo(u8, String), // idx(14)
    showHittingArea(u16, Vec3, Vec3, f64), // idx(15)
    showCarpetBombing(u16, Vec3, Vec3, f64), // idx(16)
    showTracer(OBJECT_ID, SHOT_ID, BOOL, u8, Vec3, Vec3, f32, f32, u8), // idx(17)
    stopTracer(SHOT_ID, Vec3), // idx(18)
    explodeProjectile(SHOT_ID, u8, u8, Vec3, Vec3, Vec<u32>), // idx(19)
    onRoundFinished(i8, u8), // idx(20)
    onKickedFromArena(u8), // idx(21)
    onBattleEvents(Vec<BATTLE_EVENT>), // idx(22)
    battleEventsSummary(BATTLE_EVENTS_SUMMARY), // idx(23)
    updateArena(u8, String), // idx(24)
    updatePositions(Vec<u16>, Vec<i16>), // idx(25)
    receivePhysicsDebugInfo(String), // idx(26)
    updateCarriedFlagPositions(Vec<u8>, Vec<i16>), // idx(27)
    receiveNotification(String), // idx(28)
    onRepairPointAction(u8, u8, f32), // idx(29)
    updateAvatarPrivateStats(String), // idx(30)
    updateResourceAmount(u8, u32), // idx(31)
    onFrictionWithVehicle(OBJECT_ID, Vec3, u8), // idx(32)
    onCollisionWithVehicle(Vec3, f32), // idx(33)
    onSmoke(SMOKE_INFO), // idx(34)
    onCombatEquipmentShotLaunched(u16, Vec3), // idx(35)
    updateQuestProgress(String, Python), // idx(36)
    updateVehicleQuickShellChanger(OBJECT_ID, BOOL), // idx(37)
    enemySPGHit(Vec3), // idx(38)
    enemySPGShotSound(Vec3, Vec3), // idx(39)
    handleScriptEventFromServer(String, String, String, String, String), // idx(40)
    setUpdatedGoodiesSnapshot(Vec<BATTLE_GOODIE_RECORD>), // idx(41)
    onRandomEvent(String), // idx(42)
}

// 0: onAutoAimVehicleLost [Fixed(1)] @ []
// 1: onKickedFromArena [Fixed(1)] @ []
// 2: updatePlayerLives [Fixed(1)] @ ["RespawnController_Avatar"]
// 3: enteringProtectionZone [Fixed(1)] @ ["AvatarEpic"]
// 4: leavingProtectionZone [Fixed(1)] @ ["AvatarEpic"]
// 5: protectionZoneShooting [Fixed(1)] @ ["AvatarEpic"]
// 6: onSectorShooting [Fixed(1)] @ ["AvatarEpic"]
// 7: onRankUpdate [Fixed(1)] @ ["AvatarEpic"]
// 8: onRoundFinished [Fixed(2)] @ []
// 9: onXPUpdated [Fixed(2)] @ ["AvatarEpic"]
// 10: updateTargetVehicleID [Fixed(4)] @ []
// 11: explodeVehicleBeforeRespawn [Fixed(4)] @ ["RespawnController_Avatar"]
// 12: removeVehicle [Fixed(4)] @ ["VehicleRemovalController_Avatar"]
// 13: updateResourceAmount [Fixed(5)] @ []
// 14: updateVehicleQuickShellChanger [Fixed(5)] @ []
// 15: onDestructibleDestroyed [Fixed(5)] @ ["AvatarEpic"]
// 16: onRepairPointAction [Fixed(6)] @ []
// 17: onSectorBaseAction [Fixed(6)] @ ["AvatarEpic"]
// 18: updateVehicleHealth [Fixed(9)] @ []
// 19: updateVehicleSetting [Fixed(9)] @ []
// 20: onStepRepairPointAction [Fixed(11)] @ ["AvatarEpic"]
// 21: enemySPGHit [Fixed(12)] @ []
// 22: onVehicleHealthChanged [Fixed(12)] @ ["VehicleHealthBroadcastListenerComponent_Avatar"]
// 23: welcomeToSector [Fixed(12)] @ ["AvatarEpic"]
// 24: updateState [Fixed(13)] @ ["RecoveryMechanic_Avatar"]
// 25: onCrewRoleFactorAndRankUpdate [Fixed(13)] @ ["AvatarEpic"]
// 26: onCombatEquipmentShotLaunched [Fixed(14)] @ []
// 27: onSwitchViewpoint [Fixed(16)] @ []
// 28: stopTracer [Fixed(16)] @ []
// 29: onCollisionWithVehicle [Fixed(16)] @ []
// 30: onSmoke [Fixed(16)] @ []
// 31: onFrictionWithVehicle [Fixed(17)] @ []
// 32: updateVehicleAmmo [Fixed(18)] @ []
// 33: showOwnVehicleHitDirection [Fixed(21)] @ []
// 34: enemySPGShotSound [Fixed(24)] @ []
// 35: showHittingArea [Fixed(34)] @ []
// 36: showCarpetBombing [Fixed(34)] @ []
// 37: battleEventsSummary [Fixed(34)] @ []
// 38: updateTargetingInfo [Fixed(36)] @ []
// 39: showTracer [Fixed(43)] @ []
// 40: update [Variable(Variable8)] @ []
// 41: onKickedFromServer [Variable(Variable8)] @ []
// 42: onIGRTypeChanged [Variable(Variable8)] @ []
// 43: receiveAccountStats [Variable(Variable8)] @ []
// 44: showOtherVehicleDamagedDevices [Variable(Variable8)] @ []
// 45: showShotResults [Variable(Variable8)] @ []
// 46: showDevelopmentInfo [Variable(Variable8)] @ []
// 47: explodeProjectile [Variable(Variable8)] @ []
// 48: onBattleEvents [Variable(Variable8)] @ []
// 49: updateArena [Variable(Variable8)] @ []
// 50: updatePositions [Variable(Variable8)] @ []
// 51: receivePhysicsDebugInfo [Variable(Variable8)] @ []
// 52: updateCarriedFlagPositions [Variable(Variable8)] @ []
// 53: receiveNotification [Variable(Variable8)] @ []
// 54: updateAvatarPrivateStats [Variable(Variable8)] @ []
// 55: updateQuestProgress [Variable(Variable8)] @ []
// 56: handleScriptEventFromServer [Variable(Variable8)] @ []
// 57: setUpdatedGoodiesSnapshot [Variable(Variable8)] @ []
// 58: onRandomEvent [Variable(Variable8)] @ []
// 59: onChatAction [Variable(Variable8)] @ ["Chat"]
// 60: messenger_onActionByServer_chat2 [Variable(Variable8)] @ ["PlayerMessenger_chat2"]
// 61: onCmdResponse [Variable(Variable8)] @ ["ClientCommandsPort"]
// 62: onCmdResponseExt [Variable(Variable8)] @ ["ClientCommandsPort"]
// 63: processInvitations [Variable(Variable8)] @ ["InvitationsClient"]
// 64: onTokenReceived [Variable(Variable8)] @ ["AccountAuthTokenProviderClient"]
// 65: updateTeamsHealthPercentage [Variable(Variable8)] @ ["TeamHealthBar_Avatar"]
// 66: redrawVehicleOnRespawn [Variable(Variable8)] @ ["RespawnController_Avatar"]
// 67: updateRespawnVehicles [Variable(Variable8)] @ ["RespawnController_Avatar"]
// 68: updateRespawnCooldowns [Variable(Variable8)] @ ["RespawnController_Avatar"]
// 69: updateRespawnInfo [Variable(Variable8)] @ ["RespawnController_Avatar"]
// 70: updateVehicleLimits [Variable(Variable8)] @ ["RespawnController_Avatar"]
// 71: onTeamLivesRestored [Variable(Variable8)] @ ["RespawnController_Avatar"]
// 72: externalTrigger [Variable(Variable8)] @ ["TriggersController_Avatar"]
// 73: syncPurchasedAbilities [Variable(Variable8)] @ ["AvatarEpic"]
// 74: onRandomReserveOffer [Variable(Variable8)] @ ["AvatarEpic"]
// 75: showDestructibleShotResults [Variable(Variable8)] @ ["AvatarEpic"]
// 76: updateSpawnList [Variable(Variable16)] @ ["VehiclesSpawnListStorage_Avatar"]

/// Entity 0x03
/// Methods for ArenaInfo on client component
pub enum ArenaInfoMethod { 
    showCarpetBombing(u16, Vec3, Vec3, f32), // idx(0)
}

// 0: showCarpetBombing [Fixed(30)] @ []

/// Entity 0x04
/// Methods for ClientSelectableObject on client component
pub enum ClientSelectableObjectMethod { 
}


/// Entity 0x05
/// Methods for HangarVehicle on client component
pub enum HangarVehicleMethod { 
}


/// Entity 0x06
/// Methods for Vehicle on client component
pub enum VehicleMethod { 
    onExtraHitted(i16, Vec3), // idx(0)
    onHealthChanged(i16, i16, OBJECT_ID, u8, i8), // idx(1)
    showShooting(u8, i8), // idx(2)
    updateLaserSight(OBJECT_ID, BOOL, String), // idx(3)
    showDamageFromShot(OBJECT_ID, Vec<u64>, u8, i32, u8, BOOL), // idx(4)
    showDamageFromExplosion(OBJECT_ID, Vec3, u8, i32, u8), // idx(5)
    showAmmoBayEffect(u8, f32, f32), // idx(6)
    onPushed(f32, f32), // idx(7)
    onStaticCollision(f32, Vec3, Vec3, u8, f32, i8, u16), // idx(8)
    showRammingEffect(f32, Vec3), // idx(9)
}

// 0: showShooting [Fixed(2)] @ []
// 1: onPushed [Fixed(8)] @ []
// 2: showAmmoBayEffect [Fixed(9)] @ []
// 3: onHealthChanged [Fixed(10)] @ []
// 4: onExtraHitted [Fixed(14)] @ []
// 5: showRammingEffect [Fixed(16)] @ []
// 6: showDamageFromExplosion [Fixed(22)] @ []
// 7: onStaticCollision [Fixed(36)] @ []
// 8: updateLaserSight [Variable(Variable8)] @ []
// 9: showDamageFromShot [Variable(Variable8)] @ []

/// Entity 0x07
/// Methods for AreaDestructibles on client component
pub enum AreaDestructiblesMethod { 
}


/// Entity 0x08
/// Methods for OfflineEntity on client component
pub enum OfflineEntityMethod { 
}


/// Entity 0x09
/// Methods for Flock on client component
pub enum FlockMethod { 
}


/// Entity 0x0A
/// Methods for FlockExotic on client component
pub enum FlockExoticMethod { 
}


/// Entity 0x0B
/// Methods for Login on client component
pub enum LoginMethod { 
    onKickedFromServer(i32), // idx(0)
    receiveLoginQueueNumber(u64), // idx(1)
    setPeripheryRoutingGroup(String, Python), // idx(2)
}

// 0: onKickedFromServer [Fixed(4)] @ []
// 1: receiveLoginQueueNumber [Fixed(8)] @ []
// 2: setPeripheryRoutingGroup [Variable(Variable8)] @ []

/// Entity 0x0C
/// Methods for DetachedTurret on client component
pub enum DetachedTurretMethod { 
    onStaticCollision(f32, Vec3, Vec3), // idx(0)
    showDamageFromShot(Vec<u64>, u8), // idx(1)
}

// 0: onStaticCollision [Fixed(28)] @ []
// 1: showDamageFromShot [Variable(Variable8)] @ []

/// Entity 0x0D
/// Methods for DebugDrawEntity on client component
pub enum DebugDrawEntityMethod { 
}


/// Entity 0x0E
/// Methods for ClientSelectableCameraObject on client component
pub enum ClientSelectableCameraObjectMethod { 
}


/// Entity 0x0F
/// Methods for ClientSelectableCameraVehicle on client component
pub enum ClientSelectableCameraVehicleMethod { 
}


/// Entity 0x10
/// Methods for ClientSelectableWebLinksOpener on client component
pub enum ClientSelectableWebLinksOpenerMethod { 
}


/// Entity 0x11
/// Methods for ClientSelectableEasterEgg on client component
pub enum ClientSelectableEasterEggMethod { 
}


/// Entity 0x12
/// Methods for EmptyEntity on client component
pub enum EmptyEntityMethod { 
}


/// Entity 0x13
/// Methods for LimitedVisibilityEntity on client component
pub enum LimitedVisibilityEntityMethod { 
}


/// Entity 0x14
/// Methods for HeroTank on client component
pub enum HeroTankMethod { 
}


/// Entity 0x15
/// Methods for PlatoonTank on client component
pub enum PlatoonTankMethod { 
}


/// Entity 0x16
/// Methods for PlatoonLighting on client component
pub enum PlatoonLightingMethod { 
}


/// Entity 0x17
/// Methods for SectorBase on client component
pub enum SectorBaseMethod { 
}


/// Entity 0x18
/// Methods for Sector on client component
pub enum SectorMethod { 
    showBomb(Vec3), // idx(0)
}

// 0: showBomb [Fixed(12)] @ []

/// Entity 0x19
/// Methods for DestructibleEntity on client component
pub enum DestructibleEntityMethod { 
    onHealthChanged(i16, OBJECT_ID, u8, i32), // idx(0)
    showDamageFromShot(OBJECT_ID, u8, i32), // idx(1)
    showDamageFromExplosion(OBJECT_ID, i32), // idx(2)
}

// 0: showDamageFromExplosion [Fixed(8)] @ []
// 1: showDamageFromShot [Fixed(9)] @ []
// 2: onHealthChanged [Fixed(11)] @ []

/// Entity 0x1A
/// Methods for StepRepairPoint on client component
pub enum StepRepairPointMethod { 
}


/// Entity 0x1B
/// Methods for ProtectionZone on client component
pub enum ProtectionZoneMethod { 
}


/// Entity 0x1C
/// Methods for HangarPoster on client component
pub enum HangarPosterMethod { 
}


/// Entity 0x1D
/// Methods for TeamInfo on client component
pub enum TeamInfoMethod { 
    onCombatEquipmentUsed(OBJECT_ID, OBJECT_ID), // idx(0)
    showHittingArea(u16, Vec3, Vec3, f64), // idx(1)
}

// 0: onCombatEquipmentUsed [Fixed(8)] @ []
// 1: showHittingArea [Fixed(34)] @ []

/// Entity 0x1E
/// Methods for AvatarInfo on client component
pub enum AvatarInfoMethod { 
}


/// Entity 0x1F
/// Methods for ArenaObserverInfo on client component
pub enum ArenaObserverInfoMethod { 
}


/// Entity 0x20
/// Methods for AreaOfEffect on client component
pub enum AreaOfEffectMethod { 
    playEffect(String, Vec3, f32), // idx(0)
}

// 0: playEffect [Variable(Variable8)] @ []

/// Entity 0x21
/// Methods for AttackBomber on client component
pub enum AttackBomberMethod { 
}


/// Entity 0x22
/// Methods for AttackArtilleryFort on client component
pub enum AttackArtilleryFortMethod { 
}


/// Entity 0x23
/// Methods for PersonalDeathZone on client component
pub enum PersonalDeathZoneMethod { 
}


/// Entity 0x24
/// Methods for ClientSelectableRankedObject on client component
pub enum ClientSelectableRankedObjectMethod { 
}


/// Entity 0x25
/// Methods for SimulatedVehicle on client component
pub enum SimulatedVehicleMethod { 
}


/// Entity 0x26
/// Methods for ClientSelectableHangarsSwitcher on client component
pub enum ClientSelectableHangarsSwitcherMethod { 
}


/// Entity 0x27
/// Methods for StaticDeathZone on client component
pub enum StaticDeathZoneMethod { 
    onDeathZoneDamage(OBJECT_ID, String), // idx(0)
    onDeathZoneNotification(BOOL, OBJECT_ID, f32, f32), // idx(1)
    onEntityEnteredInZone(OBJECT_ID), // idx(2)
    onEntityLeftZone(OBJECT_ID), // idx(3)
}

// 0: onEntityEnteredInZone [Fixed(4)] @ []
// 1: onEntityLeftZone [Fixed(4)] @ []
// 2: onDeathZoneNotification [Fixed(13)] @ []
// 3: onDeathZoneDamage [Variable(Variable8)] @ []

/// Entity 0x28
/// Methods for BasicMine on client component
pub enum BasicMineMethod { 
}


/// Entity 0x29
/// Methods for ApplicationPoint on client component
pub enum ApplicationPointMethod { 
}


/// Entity 0x2A
/// Methods for NetworkEntity on client component
pub enum NetworkEntityMethod { 
    activateGameObject(), // idx(0)
    activateGameObjectUnique(), // idx(1)
    deactivateGameObject(), // idx(2)
    deactivateGameObjectUnique(), // idx(3)
    createGameObject(), // idx(4)
    removeGameObject(), // idx(5)
    removeGameObjectUnique(), // idx(6)
}

// 0: activateGameObject [Fixed(0)] @ []
// 1: activateGameObjectUnique [Fixed(0)] @ []
// 2: deactivateGameObject [Fixed(0)] @ []
// 3: deactivateGameObjectUnique [Fixed(0)] @ []
// 4: createGameObject [Fixed(0)] @ []
// 5: removeGameObject [Fixed(0)] @ []
// 6: removeGameObjectUnique [Fixed(0)] @ []

/// Entity 0x2B
/// Methods for Comp7Lighting on client component
pub enum Comp7LightingMethod { 
}


