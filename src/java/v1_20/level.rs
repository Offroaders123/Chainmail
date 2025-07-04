use crate::java::alpha::entity::Player;
use crate::nbt::tag::{
    BooleanTag, ByteTag, CompoundTag, DoubleTag, IntArrayTag, IntTag, ListTag, LongTag, StringTag,
};

#[allow(non_snake_case)]
pub struct LevelDat {
    Data: LevelDatData,
}

#[allow(non_snake_case)]
pub struct LevelDatData {
    allowCommands: BooleanTag,
    BorderCenterX: DoubleTag,
    BorderCenterY: DoubleTag,
    BorderDamagePerBlock: DoubleTag,
    BorderSize: DoubleTag,
    BorderSafeZone: DoubleTag,
    BorderSizeLerpTarget: DoubleTag,
    BorderSizeLerpTime: LongTag,
    BorderWarningBlocks: DoubleTag,
    BorderWarningTime: DoubleTag,
    clearWeatherTime: IntTag,
    CustomBossEvents: CustomBossEvents,
    DataPacks: DataPacks,
    DataVersion: IntTag,
    DayTime: LongTag,
    Difficulty: ByteTag<Difficulty>,
    DifficultyLocked: BooleanTag,
    DimensionData: DimensionData,
    GameRules: GameRules,
    WorldGenSettings: WorldGenSettings,
    GameType: IntTag<GameType>,
    generatorName: StringTag<GeneratorName>,
    generatorOptions: CompoundTag, // {}; // `GeneratorOptions`
    generatorVersion: IntTag,
    hardcore: BooleanTag,
    initialized: BooleanTag,
    LastPlayed: LongTag,
    LevelName: StringTag,
    MapFeatures: BooleanTag,
    Player: Player,
    raining: BooleanTag,
    rainTime: IntTag,
    RandomSeed: LongTag,
    SizeOnDisk: LongTag,
    SpawnX: IntTag,
    SpawnY: IntTag,
    SpawnZ: IntTag,
    thundering: BooleanTag,
    thunderTime: IntTag,
    Time: LongTag,
    version: IntTag,
    Version: Version,
    WanderingTraderId: IntArrayTag, // `UUIDLike`? Probably, will double check. Moving to Rust will make that really easy, too!! (tuple byte arrays)
    WanderingTraderSpawnChance: IntTag,
    WanderingTraderSpawnDelay: IntTag,
    WasModded: BooleanTag,
}

pub type CustomBossEvents = CompoundTag<CustomBossEvent>;

#[allow(non_snake_case)]
pub struct CustomBossEvent {
    Players: ListTag<IntArrayTag>,
    Color: StringTag<BossBarColor>,
    CreateWorldFog: BooleanTag,
    DarkenScreen: BooleanTag,
    Max: IntTag,
    Value: IntTag,
    Name: StringTag, // JSON text component?
    Overlay: StringTag<BossBarOverlay>,
    PlayBossMusic: BooleanTag,
    Visible: BooleanTag,
}

#[allow(non_camel_case_types)]
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
pub enum BossBarOverlay {
    progress,
    notched_6,
    notched_10,
    notched_12,
    notched_20,
}

#[allow(non_snake_case)]
pub struct DataPacks {
    Disabled: ListTag<StringTag>,
    Enabled: ListTag<StringTag>,
}

pub enum Difficulty {
    Peaceful = 0,
    Easy,
    Normal,
    Hard,
}

pub struct DimensionData {
    the_end: EndData,
}

#[allow(non_snake_case)]
pub struct EndData {
    DragonFight: Option<DragonFight>,
}

#[allow(non_snake_case)]
pub struct DragonFight {
    ExitPortalLocation: ExitPortalLocation,
    Gateways: ListTag<IntTag>, // 0 - 19
    DragonKilled: BooleanTag,
    DragonUUIDLeast: LongTag,
    DragonUUIDMost: LongTag,
    PreviouslyKilled: BooleanTag,
}

#[allow(non_snake_case)]
pub struct ExitPortalLocation {
    X: ByteTag,
    Y: ByteTag,
    Z: ByteTag,
}

// These are resolved simply as the stringified counterparts to these.
// The numeric values could use more-accurate types.
// I only did simple ones for the time being, which for the most part represent the values that appear to be documented.
#[allow(non_snake_case)]
pub struct GameRules {
    forgiveDeadPlayers: BooleanTag,
    doInsomnia: BooleanTag,
    fallDamage: BooleanTag,
    doDaylightCycle: BooleanTag,
    spawnRadius: IntTag, // `${number}`,
    doWeatherCycle: BooleanTag,
    doPatrolSpawning: BooleanTag,
    maxCommandChainLength: IntTag, // `${number}`,
    universalAnger: BooleanTag,
    fireDamage: BooleanTag,
    doImmediateRespawn: BooleanTag,
    playersSleepingPercentage: ByteTag, // `${number}`,
    maxEntityCramming: BooleanTag,
    doMobSpawning: BooleanTag,
    showDeathMessages: BooleanTag,
    announceAdvancements: BooleanTag,
    disableRaids: BooleanTag,
    naturalRegeneration: BooleanTag,
    reducedDebugInfo: BooleanTag,
    drowningDamage: BooleanTag,
    sendCommandFeedback: BooleanTag,
    doLimitedCrafting: BooleanTag,
    commandBlockOutput: BooleanTag,
    doTraderSpawning: BooleanTag,
    doFireTick: BooleanTag,
    mobGriefing: BooleanTag,
    spectatorsGenerateChunks: BooleanTag,
    doEntityDrops: BooleanTag,
    doTileDrops: BooleanTag,
    keepInventory: BooleanTag,
    randomTickSpeed: BooleanTag,
    freezeDamage: BooleanTag,
    doMobLoot: BooleanTag,
    disableElytraMovementCheck: BooleanTag,
    logAdminCommands: BooleanTag,
}

pub struct WorldGenSettings {
    bonus_chest: BooleanTag,
    seed: LongTag,
    generate_features: BooleanTag,
    dimensions: CompoundTag, // {}; // `WorldGenDimensions`
}

pub enum GameType {
    Survival = 0,
    Creative,
    Adventure,
    Spectator,
}

#[allow(non_camel_case_types)]
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
pub struct Version {
    Id: IntTag,
    Name: StringTag,
    Series: StringTag,
    Snapshot: BooleanTag,
}
