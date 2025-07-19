use serde::{Deserialize, Serialize};

use crate::{
    nbt::tag::{
        BooleanTag, ByteTag, CompoundTag, DoubleTag, IntArrayTag, IntTag, ListTag, LongTag,
        StringTag,
    },
    schema::java::v1_20::entity::Player,
};

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct LevelDat {
    pub Data: LevelDatData,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct LevelDatData {
    pub allowCommands: BooleanTag,
    pub BorderCenterX: DoubleTag,
    pub BorderCenterY: DoubleTag,
    pub BorderDamagePerBlock: DoubleTag,
    pub BorderSize: DoubleTag,
    pub BorderSafeZone: DoubleTag,
    pub BorderSizeLerpTarget: DoubleTag,
    pub BorderSizeLerpTime: LongTag,
    pub BorderWarningBlocks: DoubleTag,
    pub BorderWarningTime: DoubleTag,
    pub clearWeatherTime: IntTag,
    pub CustomBossEvents: CustomBossEvents,
    pub DataPacks: DataPacks,
    pub DataVersion: IntTag,
    pub DayTime: LongTag,
    pub Difficulty: ByteTag<Difficulty>,
    pub DifficultyLocked: BooleanTag,
    pub DimensionData: DimensionData,
    pub GameRules: GameRules,
    pub WorldGenSettings: WorldGenSettings,
    pub GameType: IntTag<GameType>,
    pub generatorName: StringTag<GeneratorName>,
    pub generatorOptions: Option<CompoundTag>, // only used in 1.15 and below, https://minecraft.wiki/w/Java_Edition_level_format#level.dat_format // {}; // `GeneratorOptions`
    pub generatorVersion: Option<IntTag>, // also only used in 1.15 and below
    pub hardcore: BooleanTag,
    pub initialized: BooleanTag,
    pub LastPlayed: LongTag,
    pub LevelName: StringTag,
    pub MapFeatures: BooleanTag,
    pub Player: Player,
    pub raining: BooleanTag,
    pub rainTime: IntTag,
    pub RandomSeed: LongTag,
    pub SizeOnDisk: LongTag,
    pub SpawnX: IntTag,
    pub SpawnY: IntTag,
    pub SpawnZ: IntTag,
    pub thundering: BooleanTag,
    pub thunderTime: IntTag,
    pub Time: LongTag,
    pub version: IntTag,
    pub Version: Version,
    pub WanderingTraderId: IntArrayTag, // `UUIDLike`? Probably, will double check. Moving to Rust will make that really easy, too!! (tuple byte arrays)
    pub WanderingTraderSpawnChance: IntTag,
    pub WanderingTraderSpawnDelay: IntTag,
    pub WasModded: BooleanTag,
}

pub type CustomBossEvents = CompoundTag<CustomBossEvent>;

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct CustomBossEvent {
    pub Players: ListTag<IntArrayTag>,
    pub Color: StringTag<BossBarColor>,
    pub CreateWorldFog: BooleanTag,
    pub DarkenScreen: BooleanTag,
    pub Max: IntTag,
    pub Value: IntTag,
    pub Name: StringTag, // JSON text component?
    pub Overlay: StringTag<BossBarOverlay>,
    pub PlayBossMusic: BooleanTag,
    pub Visible: BooleanTag,
}

#[allow(non_camel_case_types)]
#[derive(Serialize, Deserialize)]
pub enum BossBarColor {
    black,
    dark_blue,
    dark_green,
    dark_aqua,
    dark_red,
    dark_purple,
    gold,
    gray,
    dark_gray,
    blue,
    green,
    aqua,
    red,
    light_purple,
    yellow,
    white,
}

#[allow(non_camel_case_types)]
#[derive(Serialize, Deserialize)]
pub enum BossBarOverlay {
    progress,
    notched_6,
    notched_10,
    notched_12,
    notched_20,
}

#[allow(non_snake_case)]
#[derive(Default, Serialize, Deserialize)]
pub struct DataPacks {
    pub Disabled: ListTag<StringTag>,
    pub Enabled: ListTag<StringTag>,
}

#[derive(Serialize, Deserialize)]
pub enum Difficulty {
    Peaceful = 0,
    Easy,
    Normal,
    Hard,
}

#[derive(Serialize, Deserialize)]
pub struct DimensionData {
    pub the_end: EndData,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct EndData {
    pub DragonFight: Option<DragonFight>,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct DragonFight {
    pub ExitPortalLocation: ExitPortalLocation,
    pub Gateways: [Option<IntTag>; 19], // 0 - 19
    pub DragonKilled: BooleanTag,
    pub DragonUUIDLeast: LongTag,
    pub DragonUUIDMost: LongTag,
    pub PreviouslyKilled: BooleanTag,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct ExitPortalLocation {
    pub X: ByteTag,
    pub Y: ByteTag,
    pub Z: ByteTag,
}

// These are resolved simply as the stringified counterparts to these.
// The numeric values could use more-accurate types.
// I only did simple ones for the time being, which for the most part represent the values that appear to be documented.
#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct GameRules {
    pub forgiveDeadPlayers: BooleanTag,
    pub doInsomnia: BooleanTag,
    pub fallDamage: BooleanTag,
    pub doDaylightCycle: BooleanTag,
    pub spawnRadius: IntTag, // `${number}`,
    pub doWeatherCycle: BooleanTag,
    pub doPatrolSpawning: BooleanTag,
    pub maxCommandChainLength: IntTag, // `${number}`,
    pub universalAnger: BooleanTag,
    pub fireDamage: BooleanTag,
    pub doImmediateRespawn: BooleanTag,
    pub playersSleepingPercentage: ByteTag, // `${number}`,
    pub maxEntityCramming: BooleanTag,
    pub doMobSpawning: BooleanTag,
    pub showDeathMessages: BooleanTag,
    pub announceAdvancements: BooleanTag,
    pub disableRaids: BooleanTag,
    pub naturalRegeneration: BooleanTag,
    pub reducedDebugInfo: BooleanTag,
    pub drowningDamage: BooleanTag,
    pub sendCommandFeedback: BooleanTag,
    pub doLimitedCrafting: BooleanTag,
    pub commandBlockOutput: BooleanTag,
    pub doTraderSpawning: BooleanTag,
    pub doFireTick: BooleanTag,
    pub mobGriefing: BooleanTag,
    pub spectatorsGenerateChunks: BooleanTag,
    pub doEntityDrops: BooleanTag,
    pub doTileDrops: BooleanTag,
    pub keepInventory: BooleanTag,
    pub randomTickSpeed: BooleanTag,
    pub freezeDamage: BooleanTag,
    pub doMobLoot: BooleanTag,
    pub disableElytraMovementCheck: BooleanTag,
    pub logAdminCommands: BooleanTag,
}

#[derive(Serialize, Deserialize)]
pub struct WorldGenSettings {
    pub bonus_chest: BooleanTag,
    pub seed: LongTag,
    pub generate_features: BooleanTag,
    pub dimensions: CompoundTag, // {}; // `WorldGenDimensions`
}

#[derive(Serialize, Deserialize)]
pub enum GameType {
    Survival = 0,
    Creative,
    Adventure,
    Spectator,
}

#[allow(non_camel_case_types)]
#[derive(Serialize, Deserialize)]
pub enum GeneratorName {
    default,
    flat,
    largeBiomes,
    amplified,
    buffet,
    debug_all_block_states,
    default_1_1,
    customized,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct Version {
    pub Id: IntTag,
    pub Name: StringTag,
    pub Series: StringTag,
    pub Snapshot: BooleanTag,
}
