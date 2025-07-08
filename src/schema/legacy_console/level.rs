use serde::{Deserialize, Serialize};

use crate::nbt::tag::{BooleanTag, ByteTag, IntTag, LongTag, StringTag};

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct LevelDat {
    Data: LevelDatData,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct LevelDatData {
    BiomeCentreXChunk: IntTag,
    BiomeScale: IntTag,
    XZSize: IntTag,
    StrongholdEndPortalZ: IntTag,
    StrongholdEndPortalX: IntTag,
    hasStrongholdEndPortal: BooleanTag,
    StrongholdZ: IntTag,
    StrongholdY: IntTag,
    StrongholdX: IntTag,
    HellScale: IntTag,
    SizeOnDisk: LongTag,
    DimensionData: DimensionData,
    BiomeCentreZChunk: IntTag,
    newSeaLevel: BooleanTag,
    RandomSeed: LongTag,
    thunderTime: IntTag,
    raining: BooleanTag,
    initialized: BooleanTag,
    version: IntTag,
    ModernEnd: BooleanTag,
    generatorVersion: IntTag,
    LevelName: StringTag,
    Difficulty: ByteTag<Difficulty>,
    DataVersion: IntTag,
    hasStronghold: BooleanTag,
    SpawnX: IntTag,
    DayTime: LongTag,
    hasBeenInCreative: BooleanTag,
    thundering: BooleanTag,
    GameType: IntTag,
    Time: LongTag,
    spawnBonusChest: BooleanTag,
    clearWeatherTime: IntTag,
    MapFeatures: BooleanTag,
    SpawnY: IntTag,
    LastPlayed: LongTag,
    allowCommands: BooleanTag,
    hardcore: BooleanTag,
    generatorName: StringTag,
    DifficultyLocked: BooleanTag,
    rainTime: IntTag,
    SpawnZ: IntTag,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct DimensionData {
    #[serde(rename = "The End")]
    TheEnd: TheEndData,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct TheEndData {
    DragonFight: DragonFightData,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct DragonFightData {
    DragonKilled: BooleanTag,
    PreviouslyKilled: BooleanTag,
    Gateways: [Option<IntTag>; 19], // assumed length, based on the other versions' typings, likely the same as those
}

#[derive(Serialize, Deserialize)]
pub enum Difficulty {
    Peaceful = 0,
    Easy,
    Normal,
    Hard,
}
