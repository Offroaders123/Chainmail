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
// extends BlockEntityLike<BlockEntityResource.banner>, CustomNameableLike
pub struct Banner {
    Patterns: ListTag<BannerPattern>,
}

#[allow(non_snake_case)]
pub struct BannerPattern {
    Color: IntTag<BannerPatternColor>,
    Pattern: StringTag<BannerPatternResource>,
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

// extends BlockEntityLike<BlockEntityResource.barrel>, CustomNameableLike, LootTableLike, ChestLike
pub struct Barrel {}

#[allow(non_snake_case)]
// extends BlockEntityLike<BlockEntityResource.beacon>, CustomNameableLike
pub struct Beacon {
    Lock: Option<StringTag>,
    Levels: IntTag<BeaconLevel>,
    Primary: IntTag<BeaconEffectID>,
    Secondary: IntTag<BeaconEffectID>,
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
// extends BlockEntityLike<BlockEntityResource.beehive>
pub struct Beehive {
    Bees: ListTag<BeehiveEntity>,
    FlowerPos: BeehiveFlower,
}

#[allow(non_snake_case)]
pub struct BeehiveEntity {
    EntityData: Entity, // Entity<"bee">, // Probably should allow only `BeeEntity` once that's implemented? Wiki doesn't specify
    MinOccupationTicks: IntTag,
    TicksInHive: IntTag,
}

#[allow(non_snake_case)]
pub struct BeehiveFlower {
    X: IntTag,
    Y: IntTag,
    Z: IntTag,
}

#[allow(non_snake_case)]
// extends BlockEntityLike<BlockEntityResource.blast_furnace>, CustomNameableLike, FurnaceLike
pub struct BlastFurnace {
    RecipesUsed: BlastFurnaceRecipesUsed,
}

// pub struct BlastFurnaceRecipesUsed {
//   [id: number]: IntTag, // Use `BlastFurnaceRecipeID` for the index once that is implemented.
// }
pub type BlastFurnaceRecipesUsed = CompoundTag<IntTag>; // temp, look at comment above

#[allow(non_snake_case)]
// extends BlockEntityLike<BlockEntityResource.brewing_stand>, CustomNameableLike
pub struct BrewingStand {
    BrewTime: ShortTag,
    Fuel: ByteTag<BrewingStandFuelLevel>,
    Items: [Option<Item>; 5], // 0-4, with slot tag
    Lock: Option<StringTag>,
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

// extends BlockEntityLike<BlockEntityResource.brushable_block>, LootTableLike
pub struct BrushableBlock {
    item: Option<Item>, // Should this be uppercase? The wiki showed lowercase
}

// extends BlockEntityLike<BlockEntityResource.calibrated_sculk_sensor>
pub struct CalibratedSculkSensor {
    // This one doesn't have documention yet :P
}

// extends BlockEntityLike<BlockEntityResource.campfire>, CampfireLike
pub struct Campfire {}

// extends BlockEntityLike<BlockEntityResource.chest>, CustomNameableLike, LootTableLike, ChestLike
pub struct Chest {}

#[allow(non_snake_case)]
// extends BlockEntityLike<BlockEntityResource.chiseled_bookshelf> {
pub struct ChiseledBookshelf {
    Items: [Option<Item>; 6], // 0-5, with slot tag
    last_interacted_slot: IntTag<ChiseledBookshelfLastInteractedSlot>,
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
// extends BlockEntityLike<BlockEntityResource.comparator> {
pub struct Comparator {
    OutputSignal: IntTag,
}

#[allow(non_snake_case)]
// extends BlockEntityLike<BlockEntityResource.command_block>, CustomNameableLike
pub struct CommandBlock {
    auto: BooleanTag,
    Command: StringTag,
    conditionMet: BooleanTag,
    LastExecution: LongTag,
    LastOutput: StringTag,
    powered: BooleanTag,
    SuccessCount: IntTag,
    TrackOutput: BooleanTag,
    UpdateLastExecution: BooleanTag,
}

#[allow(non_snake_case)]
// extends BlockEntityLike<BlockEntityResource.conduit> {
pub struct Conduit {
    Target: Option<IntArrayTag>, // `UUIDLike`
}

// extends BlockEntityLike<BlockEntityResource.dispenser>, CustomNameableLike, LootTableLike, DispenserLike
pub struct Dispenser {}

// extends BlockEntityLike<BlockEntityResource.dropper>, CustomNameableLike, LootTableLike, DispenserLike
pub struct Dropper {}

// extends BlockEntityLike<BlockEntityResource.enchanting_table>, CustomNameableLike
pub struct EnchantingTable {}

#[allow(non_snake_case)]
// extends BlockEntityLike<BlockEntityResource.end_gateway> {
pub struct EndGateway {
    Age: LongTag,
    ExactTeleport: BooleanTag,
    ExitPortal: EndGatewayExitPortal,
}

pub type EndGatewayExitPortal = [IntTag; 3];

#[allow(non_snake_case)]
// extends BlockEntityLike<BlockEntityResource.furnace>, CustomNameableLike, FurnaceLike
pub struct Furnace {
    RecipesUsed: FurnaceRecipesUsed,
}

// pub struct FurnaceRecipesUsed {
//   [id: number]: IntTag; // Use `FurnaceRecipeID` for the index once that is implemented.
// }
pub type FurnaceRecipesUsed = CompoundTag<IntTag>;

// extends BlockEntityLike<BlockEntityResource.hanging_sign>, SignLike {
pub struct HangingSign {}

#[allow(non_snake_case)]
// extends BlockEntityLike<BlockEntityResource.hopper>, CustomNameableLike, LootTableLike {
pub struct Hopper {
    Items: [Option<Item>; 5], // 0-4, with slot tag
    Lock: Option<StringTag>,
    TransferCooldown: IntTag<HopperTransferCooldown>,
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

// extends BlockEntityLike<BlockEntityResource.jigsaw> {
pub struct Jigsaw {
    final_state: StringTag<BlockResource>,
    joint: StringTag<JigsawJoint>,
    name: StringTag,
    pool: StringTag,
    target: StringTag,
}

#[allow(non_camel_case_types)]
pub enum JigsawJoint {
    rollable,
    aligned,
}

#[allow(non_snake_case)]
// extends BlockEntityLike<BlockEntityResource.jukebox> {
pub struct Jukebox {
    IsPlaying: BooleanTag,
    RecordItem: Item, // maybe only music disc items at the type level? optional?
    RecordStartTick: LongTag,
    TickCount: LongTag,
}

#[allow(non_snake_case)]
// extends BlockEntityLike<BlockEntityResource.lectern> {
pub struct Lectern {
    Book: Option<Item>,
    Page: IntTag,
}

// extends BlockEntityLike<BlockEntityResource.mob_spawner>, MobSpawnerLike {
pub struct MobSpawner {}

#[allow(non_snake_case)]
// Double check that some of these properties are mandatory; the wiki doesn't quite specify, but it make it sound like they could be optional.
pub struct MobSpawnerLike {
    Delay: ShortTag,
    MaxNearbyEntities: ShortTag,
    MaxSpawnDelay: ShortTag,
    MinSpawnDelay: ShortTag,
    RequiredPlayerRange: ShortTag,
    SpawnCount: ShortTag,
    SpawnData: Entity, // Also not sure what this should really be, other than a template of properties to add to your supplied mob type? I think?
    SpawnPotentials: Option<ListTag<MobSpawnerSpawnPotential>>,
    SpawnRange: ShortTag,
}

#[allow(non_snake_case)]
pub struct MobSpawnerSpawnPotential {
    weight: IntTag,
    data: Entity, // Related to `SpawnData`, looks like it will override the properties from that one, if this is present instead.
    customSpawnRules: MobSpawnerSpawnRules,
}

pub struct MobSpawnerSpawnRules {
    block_light_limit: MobSpawnerLightLimit,
    sky_light_limit: MobSpawnerLightLimit,
}

pub enum MobSpawnerLightLimit {
    Int(IntTag),
    Compound {
        min_inclusive: IntTag,
        max_inclusive: IntTag,
    },
}

#[allow(non_snake_case)]
// extends BlockEntityLike<BlockEntityResource.piston> {
pub struct Piston {
    blockState: BlockState, // I think this is just the raw `BlockState` type, not totally sure though? Might be a wrapper? I don't think so though.
    extending: BooleanTag,
    facing: IntTag<PistonFacing>,
    progress: FloatTag,
    source: BooleanTag,
}

pub enum PistonFacing {
    Down = 0,
    Up,
    North,
    South,
    West,
    East,
}

// extends BlockEntityLike<BlockEntityResource.sculk_catalyst> {
pub struct SculkCatalyst {
    charges: ListTag<SculkCatalystCharge>,
}

pub struct SculkCatalystCharge {
    charge: IntTag,
    pos: SculkCatalystChargePosition,
    /**
     * Does not relate to delay.
     *
     * Be 1 if the charge was spread from a sculk or sculk vein, 0 otherwise.
     * The charge can spread to any block if this tag is 1.
     * If it is 0, all the powers in the charge disappear when it spreads to a block not in sculk family.
     */
    decay_delay: IntTag<SculkCatalystChargeSpread>,
    update_delay: IntTag,
    facings: SculkCatalystFacings,
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

// extends BlockEntityLike<BlockEntityResource.sculk_sensor> {
pub struct SculkSensor {
    // Can't find documentation about the Block Data here
}

// extends BlockEntityLike<BlockEntityResource.sculk_shrieker> {
pub struct SculkShrieker {
    // Can't find documentation about the Block Data here
}

// extends BlockEntityLike<BlockEntityResource.shulker_box>, CustomNameableLike, LootTableLike, ChestLike {
pub struct ShulkerBox {}

// extends BlockEntityLike<BlockEntityResource.sign>, SignLike {
pub struct Sign {}

#[allow(non_snake_case)]
// extends BlockEntityLike<BlockEntityResource.skull> {
pub struct Skull {
    note_block_sound: Option<StringTag>, // "The sound event this skull plays when played with a note block." probably should be typed to a Resource Location :)
    ExtraType: StringTag,
    SkullOwner: Option<SkullOwner>,
}

#[allow(non_snake_case)]
pub struct SkullOwner {
    Id: Option<IntArrayTag>, // `UUIDLike`
    Name: Option<StringTag>,
    Properties: [SkullTexture; 1],
}

#[allow(non_snake_case)]
pub struct SkullTexture {
    /**
     * A Base64-encoded JSON object. Equivalent to the type SkullTextureJSON.
     */
    Value: StringTag,
    Signature: Option<StringTag>,
}

#[allow(non_snake_case)]
pub struct SkullTextureJSON {
    isPublic: Option<bool>,
    signatureRequired: bool,
    profileId: Option<String>,
    textures: SkullTextureTextures,
    /**
     * LongTag (BigInt), but not really because it's from JSON.
     */
    timestamp: i64, // `number` apparently, when it was in TypeScript
}

#[allow(non_snake_case)]
pub struct SkullTextureTextures {
    CAPE: Option<Cape>,
    SKIN: Skin,
}

pub struct Cape {
    url: String,
}

pub struct Skin {
    url: String,
    metadata: SkinMetadata,
}

pub struct SkinMetadata {
    model: SkinModel, // this is a string, a JSON string, not necessarily an NBT string
}

#[allow(non_camel_case_types)]
pub enum SkinModel {
    classic,
    slim,
}

#[allow(non_snake_case)]
// extends BlockEntityLike<BlockEntityResource.smoker>, CustomNameableLike, FurnaceLike
pub struct Smoker {
    RecipesUsed: SmokerRecipesUsed,
}

// pub struct SmokerRecipesUsed {
//   [id: number]: IntTag; // Use `SmokerRecipeID` for the index once that is implemented.
// }
pub type SmokerRecipesUsed = CompoundTag<IntTag>;

// extends BlockEntityLike<BlockEntityResource.soul_campfire>, CampfireLike
pub struct SoulCampfire {}

#[allow(non_snake_case)]
// extends BlockEntityLike<BlockEntityResource.structure_block>
pub struct StructureBlock {
    author: StringTag,
    ignoreEntities: BooleanTag,
    integrity: FloatTag,
    metadata: StringTag,
    mirror: StringTag<StructureBlockMirror>,
    mode: StringTag<StructureBlockMode>,
    name: StringTag,
    posX: IntTag,
    posY: IntTag,
    posZ: IntTag,
    powered: BooleanTag,
    rotation: StringTag<StructureBlockRotation>,
    seed: LongTag,
    showboundingbox: BooleanTag,
    sizeX: IntTag,
    sizeY: IntTag,
    sizeZ: IntTag,
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

// extends BlockEntityLike<BlockEntityResource.trapped_chest>, CustomNameableLike, LootTableLike, ChestLike
pub struct TrappedChest {}

#[allow(non_snake_case)]
pub struct CampfireLike {
    // `IntArrayTag<[number, number, number, number]>`
    CookingTimes: IntArrayTag,
    CookingTotalTimes: IntArrayTag,
    Items: [Option<Item>; 4], // 0-3, with slot tag
}

#[allow(non_snake_case)]
pub struct ChestLike {
    Items: ListTag<Item>, // 0-26, with slot tag
    Lock: Option<StringTag>,
}

#[allow(non_snake_case)]
pub struct CustomNameableLike {
    CustomName: Option<StringTag>, // JSON text component
}

#[allow(non_snake_case)]
pub struct DispenserLike {
    Items: ListTag<Item>, // 0-8, with slot tag
    Lock: Option<StringTag>,
}

#[allow(non_snake_case)]
pub struct FurnaceLike {
    BurnTime: ShortTag,
    CookTime: ShortTag,
    CookTimeTotal: ShortTag,
    Items: [Option<Item>; 3], // 0-2, with slot tag
    Lock: Option<StringTag>,
}

#[allow(non_snake_case)]
pub struct LootTableLike {
    LootTable: Option<StringTag>, // `LootTableResource`
    LootTableSeed: Option<LongTag>,
}

pub struct SignLike {
    is_waxed: BooleanTag,
    front_text: SignText,
    back_text: SignText,
}

pub struct SignText {
    has_glowing_text: BooleanTag,
    color: StringTag<SignColor>,
    messages: ListTag<StringTag>,
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
    id: StringTag, // `BlockEntityResource`?
    keepPacked: BooleanTag,
    x: IntTag,
    y: IntTag,
    z: IntTag,
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
