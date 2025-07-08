use serde::{Deserialize, Serialize};

use crate::{
    bedrock::{
        block::{Block, BlockResource},
        entity::{Entity, EntityResource},
        item::{Item, ItemResource},
    },
    nbt::tag::{BooleanTag, ByteTag, FloatTag, IntTag, ListTag, LongTag, ShortTag, StringTag},
};

#[allow(non_camel_case_types)]
#[derive(Serialize, Deserialize)]
pub enum BlockEntity {
    Banner(Banner),
    Barrel(Barrel),
    Beacon(Beacon),
    Bed(Bed),
    Beehive(Beehive),
    Bell(Bell),
    BlastFurnace(BlastFurnace),
    BrewingStand(BrewingStand),
    BrushableBlock(BrushableBlock),
    CalibratedSculkSensor(CalibratedSculkSensor),
    Campfire(Campfire),
    Cauldron(Cauldron),
    Chalkboard(Chalkboard),
    Chest(Chest),
    ChiseledBookshelf(ChiseledBookshelf),
    CommandBlock(CommandBlock),
    Comparator(Comparator),
    CompoundCreator(CompoundCreator),
    Conduit(Conduit),
    DaylightDetector(DaylightDetector),
    DecoratedPot(DecoratedPot),
    Dispenser(Dispenser),
    Dropper(Dropper),
    ElementConstructor(ElementConstructor),
    EnchantTable(EnchantTable),
    EndGateway(EndGateway),
    FlowerPot(FlowerPot),
    Furnace(Furnace),
    HangingSign(HangingSign),
    Hopper(Hopper),
    ItemFrame(ItemFrame),
    JigsawBlock(JigsawBlock),
    Jukebox(Jukebox),
    LabTable(LabTable),
    Lectern(Lectern),
    Lodestone(Lodestone),
    MaterialReducer(MaterialReducer),
    MobSpawner(MobSpawner),
    MovingBlock(MovingBlock),
    Music(Music),
    NetherReactor(NetherReactor),
    PistonArm(PistonArm),
    Poster(Poster),
    SculkCatalyst(SculkCatalyst),
    SculkSensor(SculkSensor),
    SculkShrieker(SculkShrieker),
    ShulkerBox(ShulkerBox),
    Sign(Sign),
    Skull(Skull),
    Slate(Slate),
    Smoker(Smoker),
    StructureBlock(StructureBlock),
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct Banner {
    block_entity_like: BlockEntityLike,
    Base: IntTag, // BannerColor
    Patterns: Option<ListTag<BannerPattern>>,
    Type: IntTag<BannerType>,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct BannerPattern {
    Color: IntTag,      // BannerColor
    Pattern: StringTag, // PatternID
}

#[allow(non_camel_case_types)]
#[derive(Serialize, Deserialize)]
pub enum BannerType {
    Zero = 0,
    One,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct Barrel {
    block_entity_like: BlockEntityLike,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct Beacon {
    block_entity_like: BlockEntityLike,
    primary: IntTag,   // EffectID
    secondary: IntTag, // EffectID
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct Bed {
    block_entity_like: BlockEntityLike,
    color: ByteTag, // Bed#Metadata
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct Beehive {
    block_entity_like: BlockEntityLike,
    Occupants: Option<ListTag<BeehiveOccupant>>,
    ShouldSpawnBees: BooleanTag,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct BeehiveOccupant {
    ActorIdentifier: StringTag, // `${EntityResource.bee}<>`, // Always `minecraft:bee<>`, probably should be from a type `ActorResource.bee`
    SaveData: Entity,           // `Entity<"bee">`
    TicksLeftToStay: IntTag,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct Bell {
    block_entity_like: BlockEntityLike,
    Direction: IntTag, // May be only two/four values? unknown for sure
    Ringing: BooleanTag,
    Ticks: IntTag,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct BlastFurnace {
    block_entity_like: BlockEntityLike,
    furnace_like: FurnaceLike,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct BrewingStand {
    block_entity_like: BlockEntityLike,
    CookTime: ShortTag,
    FuelAmount: ShortTag,
    FuelTotal: ShortTag,
    Items: ListTag<Item>, // Maybe just `[Item,Item,Item]`, but the values should be optional because they aren't always be present.
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct BrushableBlock {
    block_entity_like: BlockEntityLike,
    brush_count: IntTag<BrushCount>,
    brush_direction: ByteTag<BrushDirection>,
    item: Option<Item>,
    LootTable: Option<StringTag>, // `LootTableResource`
    LootTableSeed: Option<IntTag>,
    r#type: BrushBlockType,
}

#[allow(non_camel_case_types)]
#[derive(Serialize, Deserialize)]
pub enum BrushCount {
    Zero = 0,
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
}

#[allow(non_camel_case_types)]
#[derive(Serialize, Deserialize)]
pub enum BrushDirection {
    Zero = 0,
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
}

#[allow(non_camel_case_types)]
#[derive(Serialize, Deserialize)]
pub enum BrushBlockType {
    // How are you supposed to shadow properties from another enum in Rust?
    // suspicious_gravel = BlockResource::suspicious_gravel,
    // suspicious_sand = BlockResource::suspicious_sand,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct CalibratedSculkSensor {
    block_entity_like: BlockEntityLike,
    VibrationListener: SculkVibrationListener,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct Campfire {
    block_entity_like: BlockEntityLike,
    Item1: Option<Item>,
    ItemTime1: IntTag,
    Item2: Option<Item>,
    ItemTime2: IntTag,
    Item3: Option<Item>,
    ItemTime3: IntTag,
    Item4: Option<Item>,
    ItemTime4: IntTag,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct Cauldron {
    block_entity_like: BlockEntityLike,
    CustomColor: Option<IntTag>, // 32-bit ARGB-encoded color
    Items: ListTag<Item>,
    PotionId: ShortTag, // `PotionID | -1`
    PotionType: ShortTag<PotionType>,
}

#[allow(non_camel_case_types)]
#[derive(Serialize, Deserialize)]
pub enum PotionType {
    Zero = 0,
    One,
    Two,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct Chalkboard {
    block_entity_like: BlockEntityLike,
    BaseX: IntTag,
    BaseY: IntTag,
    BaseZ: IntTag,
    Locked: BooleanTag,
    OnGround: BooleanTag,
    Owner: LongTag,
    Size: IntTag,
    Text: StringTag,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct Chest {
    block_entity_like: BlockEntityLike,
    chest_like: ChestLike,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct ChiseledBookshelf {
    block_entity_like: BlockEntityLike,
    Items: ListTag<Item>,
    LastInteractedSlot: IntTag<ChiseledBookshelfLastInteractedSlot>,
}

#[allow(non_camel_case_types)]
#[derive(Serialize, Deserialize)]
pub enum ChiseledBookshelfLastInteractedSlot {
    Zero = 0,
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
// This needs to omit "CustomName"
// Omit<BlockEntityLike<BlockEntityResource.CommandBlock>, "CustomName">, CommandBlockLike
pub struct CommandBlock {
    block_entity_like: BlockEntityLike,
    command_block_like: CommandBlockLike,
    auto: BooleanTag,
    conditionalMode: Option<BooleanTag>, // I think it's a boolean, not just `ByteTag`, missing docs
    conditionMet: BooleanTag,
    LPConditionalMode: BooleanTag, // Also guessing; This was also named `LPCondionalMode`, double-check the wiki is misspelt.
    LPRedstoneMode: BooleanTag,    // Also guessing
    LPCommandMode: BooleanTag,     // Also guessing
    powered: BooleanTag,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct Comparator {
    block_entity_like: BlockEntityLike,
    OutputSignal: IntTag,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct CompoundCreator {
    block_entity_like: BlockEntityLike,
    chemistry_table_like: ChemistryTableLike,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct Conduit {
    block_entity_like: BlockEntityLike,
    Active: BooleanTag,
    Target: LongTag,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct DaylightDetector {
    block_entity_like: BlockEntityLike,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct DecoratedPot {
    block_entity_like: BlockEntityLike,
    sherds: ListTag<ItemResource>, // Probably should only be the sherd item ID's and `minecraft:brick`?
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct Dispenser {
    block_entity_like: BlockEntityLike,
    Items: ListTag<Item>,
    LootTable: Option<StringTag>, // `LootTableResource`
    LootTableSeed: Option<IntTag>,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct Dropper {
    block_entity_like: BlockEntityLike,
    Items: ListTag<Item>,
    LootTable: Option<StringTag>, // `LootTableResource`
    LootTableSeed: Option<IntTag>,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct ElementConstructor {
    block_entity_like: BlockEntityLike,
    chemistry_table_like: ChemistryTableLike,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct EnchantTable {
    block_entity_like: BlockEntityLike,
    CustomName: Option<StringTag>,
    rott: FloatTag, // Really two t's?
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct EndGateway {
    block_entity_like: BlockEntityLike,
    Age: IntTag,
    ExitPortal: ExitPortal,
}

pub type ExitPortal = [IntTag; 3];

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct FlowerPot {
    block_entity_like: BlockEntityLike,
    PlantBlock: Option<Block>,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct Furnace {
    block_entity_like: BlockEntityLike,
    furnace_like: FurnaceLike,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct HangingSign {
    block_entity_like: BlockEntityLike,
    sign_like: SignLike,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct Hopper {
    block_entity_like: BlockEntityLike,
    Items: ListTag<Item>, // `[Item,Item,Item,Item,Item]`?
    TransferCooldown: IntTag,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct ItemFrame {
    block_entity_like: BlockEntityLike,
    ItemDropChance: Option<FloatTag>,
    ItemRotation: Option<FloatTag>,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct JigsawBlock {
    block_entity_like: BlockEntityLike,
    final_state: BlockResource,
    joint: StringTag<JigsawBlockJoint>,
    name: StringTag,
    target: StringTag,
    target_pool: StringTag, // `TargetPoolResource`? Still not sure what Target Pool means.
}

#[allow(non_camel_case_types)]
#[derive(Serialize, Deserialize)]
pub enum JigsawBlockJoint {
    rollable,
    aligned,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct Jukebox {
    block_entity_like: BlockEntityLike,
    RecordItem: Option<Item>,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct LabTable {
    block_entity_like: BlockEntityLike,
    chemistry_table_like: ChemistryTableLike,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct Lectern {
    block_entity_like: BlockEntityLike,
    book: Option<Item>,
    hasBook: Option<BooleanTag>,
    page: Option<IntTag>,
    totalPages: Option<IntTag>,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct Lodestone {
    block_entity_like: BlockEntityLike,
    trackingHandle: Option<IntTag>,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct MaterialReducer {
    block_entity_like: BlockEntityLike,
    chemistry_table_like: ChemistryTableLike,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct MobSpawner {
    block_entity_like: BlockEntityLike,
    Delay: ShortTag,
    DisplayEntityHeight: FloatTag,
    DisplayEntityScale: FloatTag,
    DisplayEntityWidth: FloatTag,
    EntityIdentifier: EntityResource,
    MaxNearbyEntities: ShortTag,
    MaxSpawnDelay: ShortTag,
    MinSpawnDelay: ShortTag,
    RequiredPlayerRange: ShortTag,
    SpawnCount: ShortTag,
    SpawnData: Option<SpawnData>,
    SpawnPotentials: Option<ListTag<SpawnPotential>>,
    SpawnRange: ShortTag,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct SpawnData {
    Properties: (), // {}
    TypeId: EntityResource,
    Weight: IntTag,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct SpawnPotential {
    Properties: (), // {}
    TypeId: EntityResource,
    Weight: IntTag,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct MovingBlock {
    block_entity_like: BlockEntityLike,
    movingBlock: Block,
    movingBlockExtra: Block,
    movingEntity: Option<Box<BlockEntity>>,
}

// I'm actually a Note Block :)
#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct Music {
    block_entity_like: BlockEntityLike,
    note: ByteTag, // Might be a union type since there are only a certain amount of keys/notes?
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct NetherReactor {
    block_entity_like: BlockEntityLike,
    HasFinished: BooleanTag,
    IsInitialized: BooleanTag,
    Progress: ShortTag,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct PistonArm {
    block_entity_like: BlockEntityLike,
    AttachedBlocks: PistonAssociatedBlocks,
    BreakBlocks: PistonAssociatedBlocks,
    LastProgress: FloatTag,
    NewState: ByteTag<PistonState>,
    Progress: FloatTag<PistonProgress>,
    State: ByteTag<PistonState>,
    Sticky: BooleanTag,
}

pub type PistonAssociatedBlocks = ListTag<IntTag>; // Essentially a long list of `x,y,z` pairs for individual blocks. So, `( ...[IntTag,IntTag,IntTag] )[]` if that makes any sense.

#[allow(non_camel_case_types)]
#[derive(Serialize, Deserialize)]
pub enum PistonState {
    Zero = 0,
    One,
    Two,
    Three,
}

#[allow(non_camel_case_types)]
#[derive(Serialize, Deserialize)]
pub enum PistonProgress {
    Zero = 0,
    // OneAndHalf = 0.5, // is this allowed, and or possible?
    One = 1,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct Poster {
    block_entity_like: BlockEntityLike,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct SculkCatalyst {
    block_entity_like: BlockEntityLike,
    cursors: ListTag<SculkCatalystCharge>,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct SculkCatalystCharge {
    charge: ShortTag,
    decay: ShortTag,
    facing: ShortTag,
    update: ShortTag,
    x: IntTag,
    y: IntTag,
    z: IntTag,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct SculkSensor {
    block_entity_like: BlockEntityLike,
    VibrationListener: SculkVibrationListener,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct SculkShrieker {
    block_entity_like: BlockEntityLike,
    VibrationListener: SculkVibrationListener,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct ShulkerBox {
    block_entity_like: BlockEntityLike,
    chest_like: ChestLike,
    facing: FloatTag,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct Sign {
    block_entity_like: BlockEntityLike,
    sign_like: SignLike,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct Skull {
    block_entity_like: BlockEntityLike,
    MouthMoving: BooleanTag,
    MouthTickCount: IntTag,
    Rotation: FloatTag,
    SkullType: ByteTag<SkullType>,
}

#[allow(non_camel_case_types)]
#[derive(Serialize, Deserialize)]
pub enum SkullType {
    Zero = 0,
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct Slate {
    block_entity_like: BlockEntityLike,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct Smoker {
    block_entity_like: BlockEntityLike,
    furnace_like: FurnaceLike,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct StructureBlock {
    block_entity_like: BlockEntityLike,
    animationMode: ByteTag, // Union type? boolean?
    animationSeconds: FloatTag,
    data: IntTag<StructureBlockMode>,
    dataField: StringTag,
    ignoreEntities: BooleanTag,
    integrity: FloatTag,
    isPowered: BooleanTag,
    mirror: ByteTag,          // Union type? boolean?
    redstoneSaveMode: IntTag, // Union?
    removeBlocks: ByteTag,
    rotation: ByteTag, // Union?
    seed: LongTag,
    showBoundingBox: ByteTag,
    structureName: StringTag,
    xStructureOffset: IntTag,
    yStructureOffset: IntTag,
    zStructureOffset: IntTag,
    xStructureSize: IntTag,
    yStructureSize: IntTag,
    zStructureSize: IntTag,
}

#[allow(non_camel_case_types)]
#[derive(Serialize, Deserialize)]
pub enum StructureBlockMode {
    Zero = 0,
    One,
    Two,
    Three,
    Four,
    Five,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct ChemistryTableLike {
    itemAux: ShortTag, // Wiki doesn't have much info, double check these
    itemId: IntTag,
    itemStack: ByteTag,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct ChestLike {
    Findable: ByteTag,
    forceunpair: BooleanTag,
    Item: ListTag<Item>,
    LootTable: Option<StringTag>, // `LootTableResource`
    LootTableSeed: Option<IntTag>,
    pairlead: Option<ByteTag>,
    pairx: Option<IntTag>,
    pairz: Option<IntTag>,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct CommandBlockLike {
    Command: StringTag,
    CustomName: StringTag, // I'd think this is optional?
    ExecuteOnFirstTick: BooleanTag,
    LastExecution: LongTag,
    LastOutput: StringTag,
    LastOutputParams: ListTag<StringTag>, // This could be `[StringTag]`, but I don't think so
    SuccessCount: IntTag,
    TickDelay: IntTag,
    TrackOutput: BooleanTag,
    Version: IntTag,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct FurnaceLike {
    BurnDuration: ShortTag,
    BurnTime: ShortTag,
    CookTime: ShortTag,
    Items: ListTag<Item>,
    StoredXPInt: IntTag,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct SculkVibrationListener {
    event: IntTag, // Likely a union type of a resource ID?
    pending: SculkPendingVibration,
    selector: (), // {}
    ticks: IntTag,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct SculkPendingVibration {
    distance: FloatTag,
    source: LongTag,
    vibration: IntTag, // Maybe also a union type?
    x: IntTag,
    y: IntTag,
    z: IntTag,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct SignLike {
    BackText: SignText,
    FrontText: SignText,
    IsWaxed: BooleanTag,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct SignText {
    HideGlowOutline: BooleanTag,
    IgnoreLighting: BooleanTag,
    PersistFormatting: ByteTag, // ByteTag<1>, // Wiki doesn't have info on the value meaning, other than it only being known to be `1`.
    SignTextColor: IntTag<SignTextColor>, // 32-bit encoded color.
    Text: StringTag,
    TextOwner: StringTag, // wiki doesn't specify the meaning for this tag. Maybe it's a player's username?
}

#[allow(non_camel_case_types)]
#[derive(Serialize, Deserialize)]
pub enum SignTextColor {
    // temp names, these probably have real color names!
    A = -986896,
    B = -425955,
    C = -3715395,
    D = -12930086,
    E = -75715,
    F = -8337633,
    G = -816214,
    H = -12103854,
    I = -6447721,
    J = -15295332,
    K = -7785800,
    L = -12827478,
    M = -8170446,
    N = -10585066,
    O = -5231066,
    P = -16777216,
}

// <BlockEntityID extends string>
#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct BlockEntityLike {
    CustomName: Option<StringTag>,
    id: StringTag, // `${BlockEntityID}`,
    isMovable: BooleanTag,
    x: IntTag,
    y: IntTag,
    z: IntTag,
}

#[allow(non_camel_case_types)]
#[derive(Serialize, Deserialize)]
pub enum BlockEntityResource {
    Banner,
    Barrel,
    Beacon,
    Bed,
    Beehive,
    Bell,
    BlastFurnace,
    BrewingStand,
    BrushableBlock, // Sus
    CalibratedSculkSensor,
    Campfire,
    Cauldron,
    Chalkboard, // unknown, not on the wiki
    Chest,
    ChiseledBookshelf,
    Comparator,
    CompoundCreator, // ID not on the wiki
    CommandBlock,
    Conduit,
    DaylightDetector,
    DecoratedPot,
    Dispenser,
    Dropper,
    EnchantTable,
    EnderChest,
    EndGateway,
    EndPortal,
    ElementConstructor, // also guessed
    FlowerPot,          // ID not listed on the wiki
    Furnace,
    GlowItemFrame,
    HangingSign,
    Hopper,
    ItemFrame,
    JigsawBlock,
    Jukebox,
    LabTable, // guessed
    Lectern,
    Lodestone,       // ID not on the wiki
    MaterialReducer, // guessed
    MobSpawner,
    MovingBlock, // Related to Piston block movement
    Music,       // Note Block
    NetherReactor,
    PistonArm,
    Poster, // might be part of Chalkboard?
    SculkCatalyst,
    SculkSensor,
    SculkShrieker,
    ShulkerBox,
    Sign,
    Skull,
    Slate, // also unknown
    Smoker,
    StructureBlock,
}
