use serde::{Deserialize, Serialize};

use crate::schema::bedrock::block_entity::BlockEntity;
use crate::schema::bedrock::dimension::DimensionID;
use crate::schema::bedrock::entity::{Entity, Player};
use crate::nbt::tag::{
    BooleanTag, ByteArrayTag, ByteTag, CompoundTag, FloatTag, IntArrayTag, IntTag, ListTag,
    LongTag, ShortTag, StringTag,
};

pub type Buffer = Vec<u8>; // temp

#[allow(non_camel_case_types)]
pub type number = f64; // temp alias, from TypeScript, each type should be resolved at usage site accordingly

pub type NBTData<T> = T; // temp NBT wrapper type, persisted from TS

// export type Key = WorldKey | SuffixKeyEntry | ChunkKeyEntry,

// export type WorldKey<K extends keyof WorldKeyNameMap = keyof WorldKeyNameMap> = K,

// export type WorldValue<K extends keyof WorldKeyNameMap = keyof WorldKeyNameMap> = WorldKeyNameMap[K],

// pub struct WorldKeyNameMap {
//   BiomeData: NBTData<BiomeData>,
//   dimension0: NBTData<LegacyDimension0>,
//   dimension1: NBTData<LegacyDimension1>,
//   mVillages: NBTData<LegacyMVillages>,
//   LevelChunkMetaDataDictionary: LevelChunkMetaDataDictionary,
//   game_flatworldlayers: GameFlatWorldLayers,
//   "PositionTrackDB-LastId": NBTData<PositionTrackDBLastId>,
//   "~local_player": NBTData<LocalPlayer>,
//   AutonomousEntities: NBTData<AutonomousEntities>,
//   Overworld: NBTData<Overworld>,
//   Nether: NBTData<Nether>,
//   TheEnd: NBTData<TheEnd>,
//   mobevents: NBTData<MobEvents>,
//   portals: NBTData<Portals>,
//   schedulerWT: NBTData<SchedulerWT>,
//   scoreboard: NBTData<Scoreboard>,
// }

// export type SuffixKey<K extends keyof SuffixKeyNameMap = keyof SuffixKeyNameMap> = SuffixKeyEntry<K>,

// pub struct SuffixKeyEntry<K extends keyof SuffixKeyNameMap = keyof SuffixKeyNameMap> {
//   type: K,
//   // value: SuffixKeyNameMap[K],
//   key: Buffer,
// }

// export type SuffixValue<K extends keyof SuffixKeyNameMap = keyof SuffixKeyNameMap> = SuffixKeyNameMap[K],

// pub struct SuffixKeyNameMap {
//   actorprefix: ActorPrefix,
//   digp: DigP,
//   posTrackDB: NBTData<PosTrackDB>,
//   player: NBTData<PlayerServerDef>,
//   player_server: NBTData<PlayerServer>,
//   tickingarea: NBTData<TickingArea>,
//   VILLAGE_DWELLERS: NBTData<VillageDwellers>,
//   VILLAGE_INFO: NBTData<VillageInfo>,
//   VILLAGE_PLAYERS: NBTData<VillagePlayers>,
//   VILLAGE_POI: NBTData<VillagePois>,
//   map: NBTData<Map>,
// }

// export type ChunkKey<K extends keyof ChunkKeyNameMap = keyof ChunkKeyNameMap> = ChunkKeyEntry<K>,

// pub struct ChunkKeyEntry<K extends keyof ChunkKeyNameMap = keyof ChunkKeyNameMap> {
//   x: number,
//   y: number,
//   type: K,
//   dimension: DimensionID,
//   subchunk: number | null,
//   // value: ChunkKeyNameMap[K],
// }

// export type ChunkValue<K extends keyof ChunkKeyNameMap = keyof ChunkKeyNameMap> = ChunkKeyNameMap[K],

#[allow(non_camel_case_types)]
#[derive(Serialize, Deserialize)]
pub enum CHUNK_KEY {
    Data3D = 43,
    Version,
    Data2D,
    Data2DLegacy,
    SubChunkPrefix,
    LegacyTerrain,
    BlockEntity,
    Entity,
    PendingTicks,
    LegacyBlockExtraData,
    BiomeState,
    FinalizedState,
    ConversionData,
    BorderBlocks,
    HardcodedSpawners,
    RandomTicks,
    CheckSums,
    GenerationSeed,
    GeneratedPreCavesAndCliffsBlending,
    BlendingBiomeHeight,
    MetaDataHash,
    BlendingData,
    ActorDigestVersion,
    LegacyVersion = 118,
}

#[allow(non_camel_case_types)]
#[derive(Serialize, Deserialize)]
pub enum ChunkKey {
    Data3D(Data3D),
    Version(Version),
    Data2D(Data2D),
    Data2DLegacy(Data2DLegacy),
    SubChunkPrefix(SubChunkPrefix),
    LegacyTerrain(LegacyTerrain),
    BlockEntity(BlockEntities),
    Entity(Entities),
    PendingTicks(PendingTicks),
    LegacyBlockExtraData(LegacyBlockExtraData),
    BiomeState(BiomeState),
    FinalizedState(FinalizedState),
    ConversionData(ConversionData),
    BorderBlocks(BorderBlocks),
    HardcodedSpawners(HardcodedSpawners),
    RandomTicks(RandomTicks),
    CheckSums(CheckSums),
    GenerationSeed(GenerationSeed),
    GeneratedPreCavesAndCliffsBlending(GeneratedPreCavesAndCliffsBlending),
    BlendingBiomeHeight(BlendingBiomeHeight),
    MetaDataHash(MetaDataHash),
    BlendingData(BlendingData),
    ActorDigestVersion(ActorDigestVersion),
    LegacyVersion(LegacyVersion),
}

// export type Value = WorldValue | SuffixValue | ChunkValue,

// WorldKey

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct AutonomousEntities {
    AutonomousEntityList: ListTag<Entity>, // approximated, guessing this is entity. I've only ever seen it empty
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct BiomeData {
    list: ListTag<BiomeDataList>,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct BiomeDataList {
    id: ByteTag, // numerical biome ID, likely a resource from Region-Types
    snowAccumulation: FloatTag,
}

pub type GameFlatWorldLayers = Buffer;

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct LevelChunkMetaDataDictionary {
    count: number,
    entries: ListTag<LevelChunkMetaDataDictionaryEntry>,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct LevelChunkMetaDataDictionaryEntry {
    key: Buffer, // temporary, I'm not sure how this is supposed to be parsed nicely
    tag: NBTData<LevelChunkMetaDataDictionaryTag>,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct LevelChunkMetaDataDictionaryTag {
    BiomeBaseGameVersion: StringTag,
    DimensionName: DimensionName,
    GenerationSeed: LongTag,
    GeneratorType: IntTag, // could be a union instead? `1` appears to be the regular setting.
    LastSavedBaseGameVersion: Option<StringTag>,
    LastSavedDimensionHeightRange: Option<DimensionHeightRange>,
    OriginalBaseGameVersion: StringTag,
    OriginalDimensionHeightRange: DimensionHeightRange,
    // Why are these `ShortTag`s? The values look like booleans! dangit Mojang.
    // and I think these are unique to the Overworld, at the moment at least.
    Overworld1_18HeightExtended: Option<ShortTag>,
    UnderwaterLavaLakeFixed: Option<ShortTag>,
    WorldGenBelowZeroFixed: Option<ShortTag>,
}

#[allow(non_camel_case_types)]
#[derive(Serialize, Deserialize)]
pub enum DimensionName {
    Overworld,
    Nether,
    TheEnd,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct DimensionHeightRange {
    max: ShortTag,
    min: ShortTag,
}

pub type LocalPlayer = Player;

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct MobEvents {
    events_enabled: BooleanTag,
    #[serde(rename = "minecraft:ender_dragon_event")]
    ender_dragon_event: BooleanTag,
    #[serde(rename = "minecraft:pillager_patrols_event")]
    pillager_patrols_event: BooleanTag,
    #[serde(rename = "minecraft:wandering_trader_event")]
    wandering_trader_event: BooleanTag,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct Overworld {
    // [name: string]: never, // untyped atm
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct SchedulerWT {
    daysSinceLastWTSpawn: IntTag,
    isSpawningWT: BooleanTag, // most likely boolean?
    nextWTSpawnCheckTick: LongTag,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct Scoreboard {
    Criteria: ListTag<CompoundTag>, // object[], // needs to be typed?
    DisplayObjectives: ListTag<ScoreboardDisplayObjective>,
    Entries: ListTag<ScoreboardEntry>,
    Objectives: ListTag<ScoreboardObjective>,
    LastUniqueId: LongTag,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct ScoreboardDisplayObjective {
    Name: StringTag,
    ObjectiveName: StringTag, // the internal name of the objective displayed; Resource ID of some sort?
    SortOrder: ListTag<ByteTag<ScoreboardDisplayObjectiveSortOrder>>, // seems to be optional; 'if not specified'
}

#[allow(non_camel_case_types)]
#[derive(Serialize, Deserialize)]
pub enum ScoreboardDisplayObjectiveSortOrder {
    Zero = 0,
    One,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct ScoreboardEntry {
    IdentityType: ByteTag<ScoreboardEntryType>,
    EntityId: Option<LongTag>,
    PlayerId: Option<LongTag>,
    ScoreboardId: LongTag,
}

#[allow(non_camel_case_types)]
#[derive(Serialize, Deserialize)]
pub enum ScoreboardEntryType {
    One = 1,
    Two,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct ScoreboardObjective {
    Criteria: ScoreboardObjectiveCriteria,
    DisplayName: StringTag,
    Name: StringTag, // internal name of this objective; maybe resource ID? This might be user-defined actually, though
    Scores: ListTag<ScoreboardObjectiveScore>,
}

#[allow(non_camel_case_types)]
#[derive(Serialize, Deserialize)]
pub enum ScoreboardObjectiveCriteria {
    dummy,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct ScoreboardObjectiveScore {
    Score: IntTag,
    ScoreboardId: LongTag,
}

// ChunkKey

pub type Data3D = Buffer;
pub type Version = number;
pub type Data2D = Buffer;
pub type Data2DLegacy = Buffer;
pub type SubChunkPrefix = Buffer;
pub type LegacyTerrain = Buffer;

pub type BlockEntities = Vec<NBTData<BlockEntity>>;

pub type Entities = Vec<NBTData<Entity>>;

pub type PendingTicks = Vec<NBTData<PendingTick>>;

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct PendingTick {
    // [name: string]: never, // untyped atm
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct LegacyBlockExtraData {
    entries: number,
    entriesKey: number,
    value: number,
}

pub type BiomeState = Buffer; // NBT?
pub type FinalizedState = number;
pub type ConversionData = Buffer; // NBT?
pub type BorderBlocks = Buffer; // NBT?
pub type HardcodedSpawners = Buffer; // NBT?

pub type RandomTicks = Vec<NBTData<RandomTick>>;

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct RandomTick {
    // [name: string]: never, // untyped atm
}

pub type CheckSums = Buffer;
pub type GenerationSeed = Buffer;
pub type GeneratedPreCavesAndCliffsBlending = bool;
pub type BlendingBiomeHeight = Buffer;
pub type MetaDataHash = Buffer;
pub type BlendingData = Buffer;
pub type ActorDigestVersion = number;
pub type LegacyVersion = number;

// SuffixKey

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct ActorPrefix {
    // [name: string]: never, // untyped atm
}

pub type DigP = Buffer;

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct VillageDwellers {
    Dwellers: ListTag<VillageDweller>,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct VillageDweller {
    actors: ListTag<VillageDwellerActor>,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct VillageDwellerActor {
    ID: LongTag,
    last_saved_pos: VillageDwellerActorPos,
    TS: LongTag,
}

pub type VillageDwellerActorPos = [IntTag; 3];

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct VillageInfo {
    BDTime: LongTag,
    GDTime: LongTag,
    Initialized: BooleanTag, // maybe boolean?
    MTick: LongTag,
    PDTick: LongTag,
    RX0: IntTag,
    RX1: IntTag,
    RY0: IntTag,
    RY1: IntTag,
    RZ0: IntTag,
    RZ1: IntTag,
    Tick: LongTag,
    Version: ByteTag,
    X0: IntTag,
    X1: IntTag,
    Y0: IntTag,
    Y1: IntTag,
    Z0: IntTag,
    Z1: IntTag,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct VillagePlayers {
    Players: ListTag<Player>, // approximated, maybe `Player[]`, but I nor the wiki know for sure
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct VillagePois {
    POI: ListTag<VillagePoi>,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct VillagePoi {
    instances: ListTag<VillagePoiInstance>,
    VillagerID: LongTag,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct VillagePoiInstance {
    Capacity: LongTag,
    InitEvent: StringTag, // some kind of resource?
    Name: StringTag,      // resource?
    OwnerCount: LongTag,
    Radius: FloatTag,
    Skip: BooleanTag,      // maybe boolean?
    SoundEvent: StringTag, // resource?
    Type: IntTag,          // some kind of union type?
    UseAABB: BooleanTag,   // boolean?
    Weight: LongTag,
    X: IntTag,
    Y: IntTag,
    Z: IntTag,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct Map {
    colors: ByteArrayTag,
    decorations: ListTag<CompoundTag>, // object[],
    dimension: ByteTag<DimensionID>,
    fullyExplored: BooleanTag,
    height: ShortTag,
    mapId: LongTag,
    mapLocked: BooleanTag,
    parentMapId: LongTag,
    scale: ByteTag<MapScale>, // ~~might be a union of only a few values~~
    unlimitedTracking: BooleanTag,
    width: ShortTag,
    xCenter: IntTag,
    zCenter: IntTag,
}

#[allow(non_camel_case_types)]
#[derive(Serialize, Deserialize)]
pub enum MapScale {
    // this is approximated, there are 5 levels of maps, looks like? The DB can account for this as well, there were 5 entries for a single map in the world, all with the same reference `parentMapId`.
    Zero = 0,
    One,
    Two,
    Three,
    Four,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct Portals {
    data: PortalsData,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct PortalsData {
    PortalRecords: ListTag<PortalRecord>,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct PortalRecord {
    DimId: IntTag<DimensionID>,
    Span: ByteTag,
    TpX: IntTag,
    TpY: IntTag,
    TpZ: IntTag,
    Xa: ByteTag,
    Za: ByteTag,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct Nether {
    data: NetherData,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct NetherData {
    LimboEntities: ListTag<Entity>, // `Entity[]`? approximated, only ever seen empty
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct TheEnd {
    data: TheEndData,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct TheEndData {
    DragonFight: DragonFight,
    LimboEntities: ListTag<Entity>, // approximated
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct DragonFight {
    DragonFightVersion: ByteTag,
    DragonKilled: BooleanTag,
    DragonSpawned: BooleanTag,
    DragonUUID: LongTag,
    ExitPortalLocation: [IntTag; 3],
    Gateways: ListTag<IntTag>, // Maybe tuple of 20x`IntTag`? Not sure how this works exactly
    IsRespawning: BooleanTag,
    PreviouslyKilled: BooleanTag,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct TickingArea {
    Dimension: IntTag<DimensionID>,
    IsCircle: BooleanTag,
    MaxX: IntTag,
    MaxZ: IntTag,
    MinX: IntTag,
    MinZ: IntTag,
    Name: StringTag,
    Preload: BooleanTag,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct PosTrackDB {
    dim: IntTag<DimensionID>,
    id: StringTag,
    pos: [IntTag; 3],
    status: ByteTag,  // `BooleanTag`?
    version: ByteTag, // same here
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct PositionTrackDBLastId {
    id: StringTag,
    version: ByteTag,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct PlayerServerDef {
    MsaId: StringTag,
    PlatformOfflineId: Option<StringTag>,
    PlatformOnlineId: Option<StringTag>,
    SelfSignedId: StringTag,
    ServerId: StringTag,
}

// `Player` I'm pretty sure essentially, there might be more keys for server players than the plain `LocalPlayer` though.
pub type PlayerServer = Player;

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct LegacyDimension0 {
    mansion: LegacyDimensionStructure,
    mineshaft: LegacyDimensionStructure,
    oceans: LegacyDimensionStructure,
    stronghold: LegacyDimensionStructure,
    village: LegacyDimensionStructure,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct LegacyDimension1 {
    bridge: LegacyDimensionStructure,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct LegacyDimensionStructure {
    structures: Option<ListTag<Structure>>,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct Structure {
    BB: IntArrayTag,
    Children: ListTag<StructureChildren>,
    ChunkX: IntTag,
    ChunkZ: IntTag,
    ID: IntTag, // union, or unique? I think maybe union of structure piece variants; should be unique to each 'structure' if made into a union type.
    iscreated: Option<BooleanTag>, // likely boolean hehe, maybe optional? *might be only for Ocean Monument
    Valid: Option<BooleanTag>,     // villages
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct StructureChildren {
    Abandoned: Option<BooleanTag>,       // villages
    Entrances: Option<[IntArrayTag; 3]>, // mineshaft, this might not be a tuple, this is just what the raw value was at the time if I remember
    BB: IntArrayTag,
    CA: Option<IntTag>,         // villages
    CB: Option<IntTag>,         // villages
    CC: Option<IntTag>,         // villages
    CD: Option<IntTag>,         // villages
    Chest: Option<BooleanTag>,  // villages; bridge (fortress)
    D: Option<IntTag>,          // mineshaft
    Desert: Option<BooleanTag>, // villages
    HPos: Option<IntTag>,       // villages
    ID: IntTag,
    Num: Option<IntTag>,          // mineshaft
    Mob: Option<BooleanTag>,      // bridge (fortress)
    hps: Option<ByteTag>,         // mineshaft
    hr: Option<ByteTag>,          // mineshaft
    sc: Option<ByteTag>,          // mineshaft
    Savannah: Option<BooleanTag>, // villages
    Seed: Option<IntTag>,         // bridge (fortress)
    Taiga: Option<BooleanTag>,    // villages
    Terrace: Option<BooleanTag>,  // villages
    tf: Option<ByteTag>,          // mineshaft
    VCount: Option<IntTag>,       // villages
    gendepth: IntTag,
    orientation: IntTag, // union of 4 cardinal directions? `255 (-1) | 0 | 1 | 2 | 3`
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct LegacyMVillages {
    data: LegacyMVillagesData,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct LegacyMVillagesData {
    Tick: IntTag,
    Villages: ListTag<CompoundTag>, // object[], // not quite typed out just yet
}
