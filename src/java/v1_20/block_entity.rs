use crate::java::v1_20::{
    block::{BlockResource, BlockState},
    entity::Entity,
    item::Item,
};
use crate::nbt::tag::{
    BooleanTag, ByteTag, CompoundTag, FloatTag, IntArrayTag, IntTag, ListTag, LongTag, ShortTag,
    StringTag,
};

#[allow(non_camel_case_types)]
pub enum BlockEntity {
    banner(Banner),
    barrel(Barrel),
    beacon(Beacon),
    beehive(Beehive),
    blast_furnace(BlastFurnace),
    brewing_stand(BrewingStand),
    brushable_block(BrushableBlock),
    calibrated_sculk_sensor(CalibratedSculkSensor),
    campfire(Campfire),
    chest(Chest),
    chiseled_bookshelf(ChiseledBookshelf),
    comparator(Comparator),
    command_block(CommandBlock),
    conduit(Conduit),
    dispenser(Dispenser),
    dropper(Dropper),
    enchanting_table(EnchantingTable),
    end_gateway(EndGateway),
    furnace(Furnace),
    hanging_sign(HangingSign),
    hopper(Hopper),
    jigsaw(Jigsaw),
    jukebox(Jukebox),
    lectern(Lectern),
    mob_spawner(MobSpawner),
    piston(Piston),
    sculk_catalyst(SculkCatalyst),
    sculk_sensor(SculkSensor),
    sculk_shrieker(SculkShrieker),
    shulker_box(ShulkerBox),
    sign(Sign),
    skull(Skull),
    smoker(Smoker),
    soul_campfire(SoulCampfire),
    structure_block(StructureBlock),
    trapped_chest(TrappedChest),
}

#[allow(non_snake_case)]
pub struct Banner {
    pub block_entity_like: BlockEntityLike,
    pub custom_nameable_like: CustomNameableLike,
    pub Patterns: ListTag<BannerPattern>,
}

#[allow(non_snake_case)]
pub struct BannerPattern {
    pub Color: IntTag<BannerPatternColor>,
    pub Pattern: StringTag<BannerPatternResource>,
}

#[allow(non_camel_case_types)]
pub enum BannerPatternColor {
    white = 0,
    orange,
    magenta,
    light_blue,
    yellow,
    lime,
    pink,
    gray,
    light_gray,
    cyan,
    purple,
    blue,
    brown,
    green,
    red,
    black,
}

// These are used as plain strings
#[allow(non_camel_case_types)]
pub enum BannerPatternResource {
    b,
    bs,
    ts,
    ls,
    rs,
    cs,
    ms,
    drs,
    dls,
    ss,
    cr,
    sc,
    ld,
    rud,
    lud,
    rd,
    vh,
    vhr,
    hh,
    hhb,
    bl,
    br,
    tl,
    tr,
    bt,
    tt,
    bts,
    tts,
    mc,
    mr,
    bo,
    cbo,
    bri,
    gra,
    gru,
    cre,
    sku,
    flo,
    moj,
    glb,
    pig,
}

pub struct Barrel {
    pub block_entity_like: BlockEntityLike,
    pub custom_nameable_like: CustomNameableLike,
    pub loot_table_like: LootTableLike,
    pub chest_like: ChestLike,
}

#[allow(non_snake_case)]
pub struct Beacon {
    pub block_entity_like: BlockEntityLike,
    pub custom_nameable_like: CustomNameableLike,
    pub Lock: Option<StringTag>,
    pub Levels: IntTag<BeaconLevel>,
    pub Primary: IntTag<BeaconEffectID>,
    pub Secondary: IntTag<BeaconEffectID>,
}

pub enum BeaconLevel {
    Zero = 0,
    One,
    Two,
    Three,
    Four,
}

// Needs a proper Rust definition. This was a union before
pub enum BeaconEffectID {
    // -1 | EffectID,
}

#[allow(non_snake_case)]
pub struct Beehive {
    pub block_entity_like: BlockEntityLike,
    pub Bees: ListTag<BeehiveEntity>,
    pub FlowerPos: BeehiveFlower,
}

#[allow(non_snake_case)]
pub struct BeehiveEntity {
    pub EntityData: Entity, // Entity<"bee">, // Probably should allow only `BeeEntity` once that's implemented? Wiki doesn't specify
    pub MinOccupationTicks: IntTag,
    pub TicksInHive: IntTag,
}

#[allow(non_snake_case)]
pub struct BeehiveFlower {
    pub X: IntTag,
    pub Y: IntTag,
    pub Z: IntTag,
}

#[allow(non_snake_case)]
pub struct BlastFurnace {
    pub block_entity_like: BlockEntityLike,
    pub custom_nameable_like: CustomNameableLike,
    pub furnace_like: FurnaceLike,
    pub RecipesUsed: BlastFurnaceRecipesUsed,
}

// pub struct BlastFurnaceRecipesUsed {
//   [id: number]: IntTag, // Use `BlastFurnaceRecipeID` for the index once that is implemented.
// }
pub type BlastFurnaceRecipesUsed = CompoundTag<IntTag>; // temp, look at comment above

#[allow(non_snake_case)]
pub struct BrewingStand {
    pub block_entity_like: BlockEntityLike,
    pub custom_nameable_like: CustomNameableLike,
    pub BrewTime: ShortTag,
    pub Fuel: ByteTag<BrewingStandFuelLevel>,
    pub Items: [Option<Item>; 5], // 0-4, with slot tag
    pub Lock: Option<StringTag>,
}

pub enum BrewingStandFuelLevel {
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
    Eleven,
    Twelve,
    Thirteen,
    Fourteen,
    Fifteen,
    Sixteen,
    Seventeen,
    Eighteen,
    Nineteen,
    Twenty,
}

pub struct BrushableBlock {
    pub block_entity_like: BlockEntityLike,
    pub loot_table_like: LootTableLike,
    pub item: Option<Item>, // Should this be uppercase? The wiki showed lowercase
}

pub struct CalibratedSculkSensor {
    pub block_entity_like: BlockEntityLike,
    // This one doesn't have documention yet :P
}

pub struct Campfire {
    pub block_entity_like: BlockEntityLike,
    pub campfire_like: CampfireLike,
}

pub struct Chest {
    pub block_entity_like: BlockEntityLike,
    pub custom_nameable_like: CustomNameableLike,
    pub loot_table_like: LootTableLike,
    pub chest_like: ChestLike,
}

#[allow(non_snake_case)]
pub struct ChiseledBookshelf {
    pub block_entity_like: BlockEntityLike,
    pub Items: [Option<Item>; 6], // 0-5, with slot tag
    pub last_interacted_slot: IntTag<ChiseledBookshelfLastInteractedSlot>,
}

pub enum ChiseledBookshelfLastInteractedSlot {
    MinusOne = -1,
    Zero,
    One,
    Two,
    Three,
    Four,
    Five,
}

#[allow(non_snake_case)]
pub struct Comparator {
    pub block_entity_like: BlockEntityLike,
    pub OutputSignal: IntTag,
}

#[allow(non_snake_case)]
pub struct CommandBlock {
    pub block_entity_like: BlockEntityLike,
    pub custom_nameable_like: CustomNameableLike,
    pub auto: BooleanTag,
    pub Command: StringTag,
    pub conditionMet: BooleanTag,
    pub LastExecution: LongTag,
    pub LastOutput: StringTag,
    pub powered: BooleanTag,
    pub SuccessCount: IntTag,
    pub TrackOutput: BooleanTag,
    pub UpdateLastExecution: BooleanTag,
}

#[allow(non_snake_case)]
pub struct Conduit {
    pub block_entity_like: BlockEntityLike,
    pub Target: Option<IntArrayTag>, // `UUIDLike`
}

pub struct Dispenser {
    pub block_entity_like: BlockEntityLike,
    pub custom_nameable_like: CustomNameableLike,
    pub loot_table_like: LootTableLike,
    pub dispenser_like: DispenserLike,
}

pub struct Dropper {
    pub block_entity_like: BlockEntityLike,
    pub custom_nameable_like: CustomNameableLike,
    pub loot_table_like: LootTableLike,
    pub dispenser_like: DispenserLike,
}

pub struct EnchantingTable {
    pub block_entity_like: BlockEntityLike,
    pub custom_nameable_like: CustomNameableLike,
}

#[allow(non_snake_case)]
pub struct EndGateway {
    pub block_entity_like: BlockEntityLike,
    pub Age: LongTag,
    pub ExactTeleport: BooleanTag,
    pub ExitPortal: EndGatewayExitPortal,
}

pub type EndGatewayExitPortal = [IntTag; 3];

#[allow(non_snake_case)]
pub struct Furnace {
    pub block_entity_like: BlockEntityLike,
    pub custom_nameable_like: CustomNameableLike,
    pub furnace_like: FurnaceLike,
    pub RecipesUsed: FurnaceRecipesUsed,
}

// pub struct FurnaceRecipesUsed {
//   [id: number]: IntTag; // Use `FurnaceRecipeID` for the index once that is implemented.
// }
pub type FurnaceRecipesUsed = CompoundTag<IntTag>;

pub struct HangingSign {
    pub block_entity_like: BlockEntityLike,
    pub sign_like: SignLike,
}

#[allow(non_snake_case)]
pub struct Hopper {
    pub block_entity_like: BlockEntityLike,
    pub custom_nameable_like: CustomNameableLike,
    pub loot_table_like: LootTableLike,
    pub Items: [Option<Item>; 5], // 0-4, with slot tag
    pub Lock: Option<StringTag>,
    pub TransferCooldown: IntTag<HopperTransferCooldown>,
}

// Needs proper
pub enum HopperTransferCooldown {
    Zero = 0,
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
}

pub struct Jigsaw {
    pub block_entity_like: BlockEntityLike,
    pub final_state: StringTag<BlockResource>,
    pub joint: StringTag<JigsawJoint>,
    pub name: StringTag,
    pub pool: StringTag,
    pub target: StringTag,
}

#[allow(non_camel_case_types)]
pub enum JigsawJoint {
    rollable,
    aligned,
}

#[allow(non_snake_case)]
pub struct Jukebox {
    pub block_entity_like: BlockEntityLike,
    pub IsPlaying: BooleanTag,
    pub RecordItem: Item, // maybe only music disc items at the type level? optional?
    pub RecordStartTick: LongTag,
    pub TickCount: LongTag,
}

#[allow(non_snake_case)]
pub struct Lectern {
    pub block_entity_like: BlockEntityLike,
    pub Book: Option<Item>,
    pub Page: IntTag,
}

pub struct MobSpawner {
    pub block_entity_like: BlockEntityLike,
    pub mob_spawner_like: MobSpawnerLike,
}

#[allow(non_snake_case)]
// Double check that some of these properties are mandatory; the wiki doesn't quite specify, but it make it sound like they could be optional.
pub struct MobSpawnerLike {
    pub Delay: ShortTag,
    pub MaxNearbyEntities: ShortTag,
    pub MaxSpawnDelay: ShortTag,
    pub MinSpawnDelay: ShortTag,
    pub RequiredPlayerRange: ShortTag,
    pub SpawnCount: ShortTag,
    pub SpawnData: Box<Entity>, // Also not sure what this should really be, other than a template of properties to add to your supplied mob type? I think?
    pub SpawnPotentials: Option<ListTag<MobSpawnerSpawnPotential>>,
    pub SpawnRange: ShortTag,
}

#[allow(non_snake_case)]
pub struct MobSpawnerSpawnPotential {
    pub weight: IntTag,
    pub data: Entity, // Related to `SpawnData`, looks like it will override the properties from that one, if this is present instead.
    pub customSpawnRules: MobSpawnerSpawnRules,
}

pub struct MobSpawnerSpawnRules {
    pub block_light_limit: MobSpawnerLightLimit,
    pub sky_light_limit: MobSpawnerLightLimit,
}

pub enum MobSpawnerLightLimit {
    Int(IntTag),
    Compound {
        min_inclusive: IntTag,
        max_inclusive: IntTag,
    },
}

#[allow(non_snake_case)]
pub struct Piston {
    pub block_entity_like: BlockEntityLike,
    pub blockState: BlockState, // I think this is just the raw `BlockState` type, not totally sure though? Might be a wrapper? I don't think so though.
    pub extending: BooleanTag,
    pub facing: IntTag<PistonFacing>,
    pub progress: FloatTag,
    pub source: BooleanTag,
}

pub enum PistonFacing {
    Down = 0,
    Up,
    North,
    South,
    West,
    East,
}

pub struct SculkCatalyst {
    pub block_entity_like: BlockEntityLike,
    pub charges: ListTag<SculkCatalystCharge>,
}

pub struct SculkCatalystCharge {
    pub charge: IntTag,
    pub pos: SculkCatalystChargePosition,
    /**
     * Does not relate to delay.
     *
     * Be 1 if the charge was spread from a sculk or sculk vein, 0 otherwise.
     * The charge can spread to any block if this tag is 1.
     * If it is 0, all the powers in the charge disappear when it spreads to a block not in sculk family.
     */
    pub decay_delay: IntTag<SculkCatalystChargeSpread>,
    pub update_delay: IntTag,
    pub facings: SculkCatalystFacings,
}

pub type SculkCatalystChargePosition = [IntTag; 3];

pub enum SculkCatalystChargeSpread {
    Zero = 0,
    One,
}

// This is probably wrong, the wiki wasn't too clear to me.
pub type SculkCatalystFacings = [StringTag<SculkCatalystFacing>; 6];

#[allow(non_camel_case_types)]
pub enum SculkCatalystFacing {
    north,
    south,
    east,
    west,
    up,
    down,
}

pub struct SculkSensor {
    pub block_entity_like: BlockEntityLike,
    // Can't find documentation about the Block Data here
}

pub struct SculkShrieker {
    pub block_entity_like: BlockEntityLike,
    // Can't find documentation about the Block Data here
}

pub struct ShulkerBox {
    pub block_entity_like: BlockEntityLike,
    pub custom_nameable_like: CustomNameableLike,
    pub loot_table_like: LootTableLike,
    pub chest_like: ChestLike,
}

pub struct Sign {
    pub block_entity_like: BlockEntityLike,
    pub sign_like: SignLike,
}

#[allow(non_snake_case)]
pub struct Skull {
    pub block_entity_like: BlockEntityLike,
    pub note_block_sound: Option<StringTag>, // "The sound event this skull plays when played with a note block." probably should be typed to a Resource Location :)
    pub ExtraType: StringTag,
    pub SkullOwner: Option<SkullOwner>,
}

#[allow(non_snake_case)]
pub struct SkullOwner {
    pub Id: Option<IntArrayTag>, // `UUIDLike`
    pub Name: Option<StringTag>,
    pub Properties: [SkullTexture; 1],
}

#[allow(non_snake_case)]
pub struct SkullTexture {
    /**
     * A Base64-encoded JSON object. Equivalent to the type SkullTextureJSON.
     */
    pub Value: StringTag,
    pub Signature: Option<StringTag>,
}

#[allow(non_snake_case)]
pub struct SkullTextureJSON {
    pub isPublic: Option<bool>,
    pub signatureRequired: bool,
    pub profileId: Option<String>,
    pub textures: SkullTextureTextures,
    /**
     * LongTag (BigInt), but not really because it's from JSON.
     */
    pub timestamp: i64, // `number` apparently, when it was in TypeScript
}

#[allow(non_snake_case)]
pub struct SkullTextureTextures {
    pub CAPE: Option<Cape>,
    pub SKIN: Skin,
}

pub struct Cape {
    pub url: String,
}

pub struct Skin {
    pub url: String,
    pub metadata: SkinMetadata,
}

pub struct SkinMetadata {
    pub model: SkinModel, // this is a string, a JSON string, not necessarily an NBT string
}

#[allow(non_camel_case_types)]
pub enum SkinModel {
    classic,
    slim,
}

#[allow(non_snake_case)]
pub struct Smoker {
    pub block_entity_like: BlockEntityLike,
    pub custom_nameable_like: CustomNameableLike,
    pub furnace_like: FurnaceLike,
    pub RecipesUsed: SmokerRecipesUsed,
}

// pub struct SmokerRecipesUsed {
//   [id: number]: IntTag; // Use `SmokerRecipeID` for the index once that is implemented.
// }
pub type SmokerRecipesUsed = CompoundTag<IntTag>;

pub struct SoulCampfire {
    pub block_entity_like: BlockEntityLike,
    pub campfire_like: CampfireLike,
}

#[allow(non_snake_case)]
pub struct StructureBlock {
    pub block_entity_like: BlockEntityLike,
    pub author: StringTag,
    pub ignoreEntities: BooleanTag,
    pub integrity: FloatTag,
    pub metadata: StringTag,
    pub mirror: StringTag<StructureBlockMirror>,
    pub mode: StringTag<StructureBlockMode>,
    pub name: StringTag,
    pub posX: IntTag,
    pub posY: IntTag,
    pub posZ: IntTag,
    pub powered: BooleanTag,
    pub rotation: StringTag<StructureBlockRotation>,
    pub seed: LongTag,
    pub showboundingbox: BooleanTag,
    pub sizeX: IntTag,
    pub sizeY: IntTag,
    pub sizeZ: IntTag,
}

#[allow(non_camel_case_types)]
pub enum StructureBlockMirror {
    NONE,
    LEFT_RIGHT,
    FRONT_BACK,
}

pub enum StructureBlockMode {
    SAVE,
    LOAD,
    CORNER,
    DATA,
}

#[allow(non_camel_case_types)]
pub enum StructureBlockRotation {
    NONE,
    CLOCKWISE_90,
    CLOCKWISE_180,
    COUNTERCLOCKWISE_90,
}

pub struct TrappedChest {
    pub block_entity_like: BlockEntityLike,
    pub custom_nameable_like: CustomNameableLike,
    pub loot_table_like: LootTableLike,
    pub chest_like: ChestLike,
}

#[allow(non_snake_case)]
pub struct CampfireLike {
    // `IntArrayTag<[number, number, number, number]>`
    pub CookingTimes: IntArrayTag,
    pub CookingTotalTimes: IntArrayTag,
    pub Items: [Option<Item>; 4], // 0-3, with slot tag
}

#[allow(non_snake_case)]
pub struct ChestLike {
    pub Items: ListTag<Item>, // 0-26, with slot tag
    pub Lock: Option<StringTag>,
}

#[allow(non_snake_case)]
pub struct CustomNameableLike {
    pub CustomName: Option<StringTag>, // JSON text component
}

#[allow(non_snake_case)]
pub struct DispenserLike {
    pub Items: ListTag<Item>, // 0-8, with slot tag
    pub Lock: Option<StringTag>,
}

#[allow(non_snake_case)]
pub struct FurnaceLike {
    pub BurnTime: ShortTag,
    pub CookTime: ShortTag,
    pub CookTimeTotal: ShortTag,
    pub Items: [Option<Item>; 3], // 0-2, with slot tag
    pub Lock: Option<StringTag>,
}

#[allow(non_snake_case)]
pub struct LootTableLike {
    pub LootTable: Option<StringTag>, // `LootTableResource`
    pub LootTableSeed: Option<LongTag>,
}

pub struct SignLike {
    pub is_waxed: BooleanTag,
    pub front_text: SignText,
    pub back_text: SignText,
}

pub struct SignText {
    pub has_glowing_text: BooleanTag,
    pub color: StringTag<SignColor>,
    pub messages: ListTag<StringTag>,
}

#[allow(non_camel_case_types)]
pub enum SignColor {
    white,
    orange,
    magenta,
    light_blue,
    yellow,
    lime,
    pink,
    gray,
    light_gray,
    cyan,
    purple,
    blue,
    brown,
    green,
    red,
    black,
}

// <BlockEntityID extends string>
#[allow(non_snake_case)]
pub struct BlockEntityLike {
    pub id: StringTag, // `BlockEntityResource`?
    pub keepPacked: BooleanTag,
    pub x: IntTag,
    pub y: IntTag,
    pub z: IntTag,
}

// Prefixed with `minecraft:` when stringified
#[allow(non_camel_case_types)]
pub enum BlockEntityResource {
    banner,
    barrel,
    beacon,
    bed,
    beehive,
    bell,
    blast_furnace,
    brewing_stand,
    brushable_block,
    calibrated_sculk_sensor,
    campfire,
    chest,
    chiseled_bookshelf,
    comparator,
    command_block,
    conduit,
    daylight_detector,
    decorated_pot,
    dispenser,
    dropper,
    enchanting_table,
    ender_chest,
    end_gateway,
    end_portal,
    furnace,
    hanging_sign,
    hopper,
    jigsaw,
    jukebox,
    lectern,
    mob_spawner,
    piston,
    sculk_catalyst,
    sculk_sensor,
    sculk_shrieker,
    shulker_box,
    sign,
    skull,
    smoker,
    soul_campfire,
    structure_block,
    trapped_chest,
}
