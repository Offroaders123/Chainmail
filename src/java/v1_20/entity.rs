use injectables::{inject_fields, injectable};

use crate::{
    java::v1_20::{
        block::{BlockResource, BlockState},
        block_entity::BlockEntity,
        dimension::DimensionResource,
        effect::{Effect, EffectID},
        item::Item,
        recipe::RecipeResource,
    },
    nbt::tag::{
        BooleanTag, ByteTag, CompoundTag, DoubleTag, FloatTag, IntArrayTag, IntTag, ListTag,
        LongTag, ShortTag, StringTag,
    },
};

#[allow(non_camel_case_types)]
pub enum Entity {
    player(Player),
    allay(Allay),
    axolotl(Axolotl),
    bat(Bat),
    bee(Bee),
    blaze(Blaze),
    camel(Camel),
    cat(Cat),
    cave_spider(CaveSpider),
    chicken(Chicken),
    cod(Cod),
    cow(Cow),
    creeper(Creeper),
    dolphin(Dolphin),
    donkey(Donkey),
    drowned(Drowned),
    elder_guardian(ElderGuardian),
    ender_dragon(EnderDragon),
    enderman(Enderman),
    endermite(Endermite),
    evoker(Evoker),
    fox(Fox),
    frog(Frog),
    ghast(Ghast),
    giant(Giant),
    glow_squid(GlowSquid),
    goat(Goat),
    guardian(Guardian),
    horse(Horse),
    hoglin(Hoglin),
    husk(Husk),
    illusioner(Illusioner),
    iron_golem(IronGolem),
    llama(Llama),
    magma_cube(MagmaCube),
    mooshroom(Mooshroom),
    mule(Mule),
    ocelot(Ocelot),
    panda(Panda),
    parrot(Parrot),
    phantom(Phantom),
    pig(Pig),
    piglin(Piglin),
    piglin_brute(PiglinBrute),
    pillager(Pillager),
    polar_bear(PolarBear),
    pufferfish(Pufferfish),
    rabbit(Rabbit),
    ravager(Ravager),
    salmon(Salmon),
    sheep(Sheep),
    shulker(Shulker),
    silverfish(Silverfish),
    skeleton(Skeleton),
    skeleton_horse(SkeletonHorse),
    slime(Slime),
    snow_golem(SnowGolem),
    sniffer(Sniffer),
    spider(Spider),
    squid(Squid),
    stray(Stray),
    strider(Strider),
    tadpole(Tadpole),
    trader_llama(TraderLlama),
    tropical_fish(TropicalFish),
    turtle(Turtle),
    vex(Vex),
    villager(Villager),
    vindicator(Vindicator),
    wandering_trader(WanderingTrader),
    warden(Warden),
    witch(Witch),
    wither(Wither),
    wither_skeleton(WitherSkeleton),
    wolf(Wolf),
    zoglin(Zoglin),
    zombie(Zombie),
    zombie_horse(ZombieHorse),
    zombie_villager(ZombieVillager),
    zombified_piglin(ZombifiedPiglin),
    boat(Boat),
    chest_boat(ChestBoat),
    minecart(Minecart),
    chest_minecart(ChestMinecart),
    furnace_minecart(FurnaceMinecart),
    tnt_minecart(TNTMinecart),
    hopper_minecart(HopperMinecart),
    spawner_minecart(SpawnerMinecart),
    command_block_minecart(CommandBlockMinecart),
    item(ItemEntity),
    experience_orb(ExperienceOrb),
    arrow(Arrow),
    spectral_arrow(SpectralArrow),
    trident(Trident),
    snowball(Snowball),
    egg(Egg),
    llama_spit(LlamaSpit),
    ender_pearl(EnderPearl),
    eye_of_ender(EyeOfEnder),
    firework_rocket(FireworkRocket),
    tnt(TNT),
    falling_block(FallingBlock),
    fishing_bobber(FishingBobber),
    lightning_bolt(LightningBolt),
    leash_knot(LeashKnot),
    painting(Painting),
    item_frame(ItemFrame),
    armor_stand(ArmorStand),
    fireball(Fireball),
    wither_skull(WitherSkull),
    dragon_fireball(DragonFireball),
    shulker_bullet(ShulkerBullet),
    end_crystal(EndCrystal),
    evoker_fangs(EvokerFangs),
    marker(Marker),
    item_display(ItemDisplay),
    block_display(BlockDisplay),
    text_display(TextDisplay),
    interaction(Interaction),
}

#[allow(non_snake_case)]
#[injectable]
pub struct ContainerEntityLike {
    Items: ListTag<Item>,         // `Slot` tag as well, need to add that
    LootTable: Option<StringTag>, // LootTableResource
    LootTableSeed: Option<LongTag>,
}

#[allow(non_snake_case)]
#[injectable]
pub struct ProjectileLike {
    HasBeenShot: BooleanTag,
    LeftOwner: Option<BooleanTag>, // `Option<TrueTag>`
    Owner: Option<IntArrayTag>,    // `IntArrayTag<[number, number, number, number]>`
}

// should this be generic?
#[allow(non_snake_case)]
#[injectable]
pub struct ThrownItemLike {
    Item: Option<Item>,
}

#[allow(non_snake_case)]
#[injectable]
pub struct HangableLike {
    Facing: ByteTag<HangableFacing>,
    TileX: IntTag,
    TileY: IntTag,
    TileZ: IntTag,
}

pub enum HangableFacing {
    Bottom = 0,
    Top,
    North,
    South,
    West,
    East,
}

#[injectable]
pub struct FireballLike {
    power: FireballPower,
}

pub type FireballPower = [DoubleTag; 3];

#[injectable]
pub struct DisplayLike {
    billboard: StringTag<DisplayBillboard>,
    brightness: DisplayBrightness,
    glow_color_override: IntTag,
    height: FloatTag,
    width: FloatTag,
    interpolation_duration: IntTag,
    teleport_duration: IntTag,
    start_interpolation: IntTag,
    shadow_radius: FloatTag,
    shadow_strength: FloatTag,
    view_range: FloatTag,
    transformation: DisplayTransformation,
}

#[allow(non_camel_case_types)]
pub enum DisplayBillboard {
    fixed,
    vertical,
    horizontal,
    center,
}

pub struct DisplayBrightness {
    block: IntTag<DisplayBrightnessLevel>,
    sky: IntTag<DisplayBrightnessLevel>,
}

pub enum DisplayBrightnessLevel {
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
}

pub enum DisplayTransformation {
    Matrix(DisplayTransformationMatrix),
    Decomposed(DisplayTransformationDecomposed),
}

pub type DisplayTransformationMatrix = [FloatTag; 16];

pub struct DisplayTransformationDecomposed {
    left_rotation: DisplayRotation,
    translation: DisplayTranslation,
    right_rotation: DisplayRotation,
    scale: DisplayScale,
}

pub enum DisplayRotation {
    Quaternion(DisplayRotationQuaternion),
    AxisAngle(DisplayRotationAxisAngle),
}

pub type DisplayRotationQuaternion = [FloatTag; 4];

pub struct DisplayRotationAxisAngle {
    angle: FloatTag,
    axis: DisplayRotationAxis,
}

pub type DisplayRotationAxis = [FloatTag; 3];

pub type DisplayTranslation = [FloatTag; 3];

pub type DisplayScale = [FloatTag; 3];

#[allow(non_snake_case)]
#[injectable]
pub struct PotionEffectLike {
    custom_potion_effects: ListTag<PotionEffectEntry>,
    Potion: StringTag, // not fully fleshed out <https://minecraft.wiki/w/Arrow#Data_values>, <https://minecraft.wiki/w/Potion#Item_data>
    CustomPotionColor: IntTag,
    Color: IntTag, // specific to Arrows..?
}

pub struct PotionEffectEntry {
    id: IntTag<EffectID>,
    amplifier: Option<ByteTag>,
    duration: Option<IntTag>,
    ambient: Option<BooleanTag>,
    show_particles: Option<BooleanTag>,
    show_icon: BooleanTag, // also optional?
}

#[allow(non_snake_case)]
#[injectable]
pub struct MobLike {
    AbsorptionAmount: FloatTag,
    ActiveEffects: ListTag<Effect>,
    ArmorDropChances: ArmorDropChances,
    ArmorItems: ArmorItems,
    Attributes: ListTag<Attribute>,
    Brain: MobBrain,
    CanPickUpLoot: BooleanTag,
    DeathLootTable: Option<StringTag>, // `LootTableResource`
    DeathLootTableSeed: Option<LongTag>,
    DeathTime: ShortTag,
    FallFlying: ByteTag,
    Health: FloatTag,
    HurtByTimestamp: IntTag,
    HurtTime: ShortTag,
    HandDropChances: HandDropChances,
    HandItems: HandItems,
    Leash: Option<Leash>,
    LeftHanded: BooleanTag,
    NoAI: BooleanTag,
    PersistenceRequired: BooleanTag,
    SleepingX: IntTag,
    SleepingY: IntTag,
    SleepingZ: IntTag,
    Team: Option<StringTag>, // `ScoreboardTeam` ?
}

pub struct MobBrain {
    memories: CompoundTag, // `Memories`, needs to be typed eventually. Just an empty object right now, in practice. `{}` in TypeScript.
}

#[allow(non_snake_case)]
#[injectable]
pub struct BreedableLike {
    Age: IntTag,
    ForcedAge: IntTag,
    InLove: IntTag,
    LoveCause: IntArrayTag, // `UUIDLike`
}

#[allow(non_snake_case)]
#[injectable]
pub struct BucketableLike {
    FromBucket: BooleanTag,
}

#[allow(non_snake_case)]
#[injectable]
pub struct TameableLike {
    Owner: Option<IntArrayTag>, // `UUIDLike`
    Sitting: BooleanTag,
}

#[allow(non_snake_case)]
#[injectable]
pub struct CollaredLike {
    CollarColor: ByteTag<CollarColor>,
}

pub enum CollarColor {
    White = 0,
    Orange,
    Magenta,
    LightBlue,
    Yellow,
    Lime,
    Pink,
    Gray,
    LightGray,
    Cyan,
    Purple,
    Blue,
    Brown,
    Green,
    Red,
    Black,
}

#[allow(non_snake_case)]
#[injectable]
pub struct SaddledLike {
    Saddle: BooleanTag,
}

#[allow(non_snake_case)]
#[injectable]
pub struct AngeredLike {
    AngerTime: IntTag,
    AngryAt: IntArrayTag, // `UUIDLike`
}

#[allow(non_snake_case)]
#[injectable]
pub struct HorseLike {
    Bred: BooleanTag,
    EatingHaystack: BooleanTag,
    Owner: Option<IntArrayTag>, // `UUIDLike`
    SaddleItem: Option<Item>,   // `Item<"minecraft:saddle">`
    Tame: BooleanTag,
    Temper: IntTag,
}

#[allow(non_snake_case)]
#[injectable]
pub struct VillagerLike {
    Gossips: ListTag<VillagerGossip>,
    Offers: Option<VillagerOffers>, // "Is generated when the trading menu is opened for the first time.", optional?
    VillagerData: VillagerData,
    Xp: IntTag,
}

#[allow(non_snake_case)]
pub struct VillagerGossip {
    Value: IntTag,
    Target: IntArrayTag, // `UUIDLike`
    Type: StringTag<VillagerGossipType>,
}

#[allow(non_camel_case_types)]
pub enum VillagerGossipType {
    major_negative,
    minor_negative,
    major_positive,
    minor_positive,
    trading,
}

#[allow(non_snake_case)]
pub struct VillagerOffers {
    Recipes: ListTag<VillagerTradeOption>,
}

#[allow(non_snake_case)]
#[injectable]
pub struct TradeOptionLike {
    buy: Item,
    buyB: Option<Item>,
    maxUses: IntTag,
    rewardExp: BooleanTag,
    sell: Item,
    uses: IntTag,
}

#[allow(non_snake_case)]
#[inject_fields(TradeOptionLike)]
pub struct VillagerTradeOption {
    demand: IntTag,
    priceMultiplier: FloatTag,
    specialPrice: IntTag,
    xp: IntTag,
}

pub struct VillagerData {
    level: IntTag<VillagerLevel>,
    profession: StringTag<VillagerProfession>,
    r#type: StringTag<VillagerType>,
}

pub enum VillagerLevel {
    Novice = 1,
    Apprentice,
    Journeyman,
    Expert,
    Master,
}

#[allow(non_camel_case_types)]
pub enum VillagerProfession {
    // needs to be `minecraft:` prefixed when stringified!!
    armorer,
    butcher,
    cartographer,
    cleric,
    farmer,
    fisherman,
    fletcher,
    leatherworker,
    librarian,
    nitwit,
    none,
    mason,
    shepherd,
    toolsmith,
    weaponsmith,
}

#[allow(non_camel_case_types)]
pub enum VillagerType {
    // needs to be `minecraft:` prefixed when stringified!!
    desert,
    jungle,
    plains,
    savanna,
    snow,
    swamp,
    taiga,
}

#[allow(non_snake_case)]
#[injectable]
pub struct ZombieLike {
    CanBreakDoors: BooleanTag,
    DrownedConversionTime: IntTag,
    InWaterTime: IntTag,
    IsBaby: Option<BooleanTag>,
}

#[allow(non_snake_case)]
#[injectable]
pub struct PiglinLike {
    IsImmuneToZombification: BooleanTag,
    TimeInOverworld: IntTag,
}

#[allow(non_snake_case)]
#[injectable]
pub struct SlimeLike {
    Size: IntTag<SlimeSize>,
    wasOnGround: BooleanTag,
}

pub enum SlimeSize {
    // not a mistake, weird I know lol; allows for larger values, these are the natural ones though.
    Small = 0,
    Medium,
    Large = 3,
}

#[allow(non_snake_case)]
#[injectable]
pub struct RaidLike {
    CanJoinRaid: BooleanTag,
    PatrolLeader: BooleanTag,
    Patrolling: BooleanTag,
    PatrolTarget: RaidPatrolTarget, // This can be made generic to a `Position` kind of thing.
    RaidId: IntTag,
    Wave: IntTag, // union of values? probably a min/max for each difficulty I'd assume?
}

#[allow(non_snake_case)]
pub struct RaidPatrolTarget {
    X: IntTag,
    Y: IntTag,
    Z: IntTag,
}

pub type ArmorDropChances = [FloatTag; 4];

pub type ArmorItems = [Item; 4];

#[allow(non_snake_case)]
pub struct Attribute {
    Base: DoubleTag,
    Modifiers: ListTag<Modifier>,
    Name: StringTag, // `AttributeResource` ?
}

#[allow(non_snake_case)]
pub struct Modifier {
    Amount: DoubleTag,
    Name: StringTag, // `ModifierResource` ?
    Operation: IntTag<ModifierOperation>,
    UUID: IntArrayTag,
}

pub enum ModifierOperation {
    Zero = 0,
    One,
    Two,
}

pub type HandDropChances = [FloatTag; 2];

pub type HandItems = [Item; 2];

#[allow(non_snake_case)]
pub enum Leash {
    Array(IntArrayTag),
    Object { X: IntTag, Y: IntTag, Z: IntTag },
}

// <EntityID extends string | undefined>
#[allow(non_snake_case)]
#[injectable]
pub struct EntityLike {
    Air: ShortTag,
    CustomName: Option<StringTag>,
    CustomNameVisible: Option<BooleanTag>,
    FallDistance: FloatTag,
    Fire: ShortTag,
    Glowing: BooleanTag,
    HasVisualFire: BooleanTag,
    id: StringTag, // <--- could probably be `EntityResource` // EntityID extends string ? `${EntityID}` : EntityID,
    Invulnerable: BooleanTag,
    Motion: EntityMotion,
    NoGravity: BooleanTag,
    OnGround: BooleanTag,
    Passengers: ListTag<Entity>,
    PortalCooldown: IntTag,
    Pos: EntityPos,
    Rotation: EntityRotation,
    Silent: Option<BooleanTag>,
    Tags: ListTag<ScoreboardTag>,
    TicksFrozen: Option<IntTag>,
    UUID: IntArrayTag,
}

pub type EntityMotion = [DoubleTag; 3];

pub type EntityPos = [DoubleTag; 3];

pub type EntityRotation = [FloatTag; 2];

pub type ScoreboardTag = String; // I think this was/is eventually meant to be an union/enum of strings

// There should be no `id` field! This needs to be fixed.
// Tags for all entities, except the id, CustomName and CustomNameVisible
// Tags for all mobs, except HandItems, ArmorItems, HandDropChances, ArmorDropChances, CanPickUpLoot, PersistenceRequired and Leash
#[allow(non_snake_case)]
#[inject_fields(EntityLike, MobLike)]
pub struct Player {
    abilities: Abilities,
    DataVersion: IntTag,
    Dimension: StringTag<DimensionResource>,
    EnderItems: ListTag<Item>,
    enteredNetherPosition: Option<EnteredNetherPosition>,
    foodExhaustionLevel: FloatTag,
    foodLevel: IntTag,
    foodSaturationLevel: FloatTag,
    foodTickTimer: IntTag,
    Inventory: ListTag<Item>,
    LastDeathLocation: Option<LastDeathLocation>,
    playerGameType: IntTag<GameMode>,
    previousPlayerGameType: IntTag<GameMode>,
    recipeBook: RecipeBook,
    RootVehicle: Option<RootVehicle>,
    Score: IntTag,
    seenCredits: BooleanTag,
    SelectedItem: Option<Item>,
    SelectedItemSlot: IntTag,
    // I think this is `minecraft:parrot` only, but I'm curious if you can put any entity on your shoulder in-game
    ShoulderEntityLeft: Box<Entity>, // Entity::parrot, // Entity<"parrot">,
    ShoulderEntityRight: Box<Entity>, // Entity::parrot, // Entity<"parrot">,
    SleepTimer: ShortTag,
    SpawnDimension: Option<StringTag<DimensionResource>>,
    SpawnForced: Option<BooleanTag>,
    SpawnX: Option<IntTag>,
    SpawnY: Option<IntTag>,
    SpawnZ: Option<IntTag>,
    warden_spawn_tracker: WardenSpawnTracker, // Optional? Doesn't specify on the wiki
    XpLevel: IntTag,
    XpP: FloatTag,
    XpSeed: IntTag,
    XpTotal: IntTag,
}

#[allow(non_snake_case)]
pub struct Abilities {
    flying: BooleanTag,
    flySpeed: FloatTag, // It says it's always only ever `0.05`, but I feel like it might change for Spectator Mode?
    instabuild: BooleanTag,
    invulnerable: BooleanTag,
    mayBuild: BooleanTag,
    mayfly: BooleanTag,
    walkSpeed: FloatTag, // Same here, this apparently always stays the same (0.1)
}

pub struct EnteredNetherPosition {
    x: DoubleTag,
    y: DoubleTag,
    z: DoubleTag,
}

pub struct LastDeathLocation {
    dimension: StringTag<DimensionResource>,
    pos: IntArrayTag,
}

pub enum GameMode {
    Survival = 0,
    Creative,
    Adventure,
    Spectator,
}

#[allow(non_snake_case)]
pub struct RecipeBook {
    recipes: ListTag<StringTag<RecipeResource>>,
    toBeDisplayed: ListTag<StringTag<RecipeResource>>,
    isFilteringCraftable: BooleanTag,
    isGuiOpen: BooleanTag,
    isFurnaceFilteringCraftable: BooleanTag,
    isFurnaceGuiOpen: BooleanTag,
    isBlastingFurnaceFilteringCraftable: BooleanTag,
    isBlastingFurnaceGuiOpen: BooleanTag,
    isSmokerFilteringCraftable: BooleanTag,
    isSmokerGuiOpen: BooleanTag,
}

#[allow(non_snake_case)]
pub struct RootVehicle {
    Attach: IntArrayTag,
    Entity: Box<Entity>,
}

pub struct WardenSpawnTracker {
    cooldown_ticks: IntTag,
    ticks_since_last_warning: IntTag,
    warning_level: IntTag<WardenWarningLevel>,
}

pub enum WardenWarningLevel {
    // wasn't sure what else to call these when moving to an enum
    Zero = 0,
    One,
    Two,
    Three,
}

#[allow(non_snake_case)]
#[inject_fields(EntityLike, MobLike)]
pub struct Allay {
    CanDuplicate: BooleanTag,
    DuplicationCooldown: LongTag,
    Inventory: [Option<Item>; 1],
    listener: AllayVibrationListener,
}

pub struct AllayVibrationListener {
    distance: IntTag,
    event: Option<AllayVibrationEvent>,
    event_delay: IntTag,
    event_distance: IntTag,
    range: IntTag,
    source: AllayVibrationListenerSource,
}

pub struct AllayVibrationEvent {
    distance: IntTag,
    game_event: StringTag, // Resource location of the game event
    pos: [DoubleTag; 3], // `PositionLike<DoubleTag>` maybe? I want to make a regular type for this pattern.
    projectile_owner: Option<IntArrayTag>, // `UUIDLike`
    source: Option<IntArrayTag>, // `UUIDLike`
}

pub enum AllayVibrationListenerSource {
    Block(AllayVibrationListenerSourceBlock),
    Entity(AllayVibrationListenerSourceEntity),
}

pub struct AllayVibrationListenerSourceBlock {
    r#type: StringTag, // StringTag<AllayVibrationListenerSource::Block>,
    pos: IntArrayTag,  // `IntArrayTag<[number, number, number]>`
}

pub struct AllayVibrationListenerSourceEntity {
    r#type: StringTag,          // StringTag<AllayVibrationListenerSource::Entity>,
    source_entity: IntArrayTag, // `UUIDLike`
    y_offset: FloatTag,
}

#[allow(non_snake_case)]
#[inject_fields(EntityLike, MobLike, BreedableLike, BucketableLike)]
pub struct Axolotl {
    Variant: IntTag<AxolotlVariant>,
}

#[allow(non_camel_case_types)]
pub enum AxolotlVariant {
    lucy = 0,
    wild,
    gold,
    cyan,
    blue,
}

#[allow(non_snake_case)]
#[inject_fields(EntityLike, MobLike)]
pub struct Bat {
    BatFlags: BooleanTag,
}

#[allow(non_snake_case)]
#[inject_fields(EntityLike, MobLike, BreedableLike, AngeredLike)]
pub struct Bee {
    CannotEnterHiveTicks: IntTag,
    CropsGrownSincePollination: IntTag,
    FlowerPos: BeePositionLike,
    HasNectar: BooleanTag,
    HasStung: BooleanTag,
    HivePosition: BeePositionLike,
    TicksSincePollination: IntTag,
}

#[allow(non_snake_case)]
pub struct BeePositionLike {
    X: IntTag,
    Y: IntTag,
    Z: IntTag,
}

#[inject_fields(EntityLike, MobLike)]
pub struct Blaze {}

#[allow(non_snake_case)]
#[inject_fields(EntityLike, MobLike, BreedableLike, HorseLike)]
pub struct Camel {
    LastPoseTick: LongTag,
}

#[inject_fields(EntityLike, MobLike, BreedableLike, TameableLike, CollaredLike)]
pub struct Cat {
    variant: StringTag<CatVariant>,
}

#[allow(non_camel_case_types)]
// When stringified, these should have the `minecraft:` prefix! pls and thank you :)
pub enum CatVariant {
    white,
    black,
    red,
    siamese,
    british_shorthair,
    calico,
    persian,
    ragdoll,
    tabby,
    all_black,
    jellie,
}

#[inject_fields(EntityLike, MobLike)]
pub struct CaveSpider {}

#[allow(non_snake_case)]
#[inject_fields(EntityLike, MobLike, BreedableLike)]
pub struct Chicken {
    EggLayTime: IntTag,
    IsChickenJockey: BooleanTag,
}

#[inject_fields(EntityLike, MobLike, BucketableLike)]
pub struct Cod {}

#[inject_fields(EntityLike, MobLike, BreedableLike)]
pub struct Cow {}

#[allow(non_snake_case)]
#[inject_fields(EntityLike, MobLike)]
pub struct Creeper {
    ExplosionRadius: ByteTag,
    Fuse: ShortTag,
    ignited: BooleanTag,
    powered: Option<BooleanTag>,
}

#[allow(non_snake_case)]
#[inject_fields(EntityLike, MobLike)]
pub struct Dolphin {
    CanFindTreasure: BooleanTag,
    GotFish: BooleanTag,
    TreasurePosX: IntTag,
    TreasurePosY: IntTag,
    TreasurePosZ: IntTag,
}

#[allow(non_snake_case)]
#[inject_fields(EntityLike, MobLike, BreedableLike, HorseLike)]
pub struct Donkey {
    ChestedHorse: BooleanTag,
    Items: Option<ListTag<Item>>, // only if `!!ChestedHorse`, with slot tag, 2-16
}

#[inject_fields(EntityLike, MobLike, ZombieLike)]
pub struct Drowned {}

#[inject_fields(EntityLike, MobLike)]
pub struct ElderGuardian {}

#[allow(non_snake_case)]
#[inject_fields(EntityLike, MobLike)]
pub struct EnderDragon {
    DragonPhase: IntTag<EnderDragonPhase>,
}

pub enum EnderDragonPhase {
    Circling = 0,
    Strafing,
    FlyingToThePortalToLand,
    LandingOnThePortal,
    TakingOffFromThePortal,
    LandedPerformingBreathAttack,
    LandedLookingForAPlayerForBreathAttack,
    LandedRoarBeforeBeginningBreathAttack,
    ChargingPlayer,
    FlyingToPortalToDie,
    Hovering,
}

#[allow(non_snake_case)]
#[inject_fields(EntityLike, MobLike, AngeredLike)]
pub struct Enderman {
    // Another funky block state shape
    carriedBlockState: Option<CarriedBlockState>,
}

#[allow(non_snake_case)]
pub struct CarriedBlockState {
    Name: StringTag<BlockResource>,
    Properties: Option<BlockState>,
}

#[allow(non_snake_case)]
#[inject_fields(EntityLike, MobLike)]
pub struct Endermite {
    Lifetime: IntTag,
}

#[allow(non_snake_case)]
#[inject_fields(EntityLike, MobLike, RaidLike)]
pub struct Evoker {
    SpellTicks: IntTag,
}

#[allow(non_snake_case)]
#[inject_fields(EntityLike, MobLike, BreedableLike)]
pub struct Fox {
    Crouching: BooleanTag,
    Sitting: BooleanTag,
    Sleeping: BooleanTag,
    Trusted: ListTag<IntArrayTag>, // `UUIDLike[]`
    Type: StringTag<FoxType>,
}

#[allow(non_camel_case_types)]
// Is this `minecraft:`-prefixed like `CatVariant`?
pub enum FoxType {
    red,
    snow,
}

#[inject_fields(EntityLike, MobLike, BreedableLike)]
pub struct Frog {
    variant: StringTag<FrogVariant>,
}

#[allow(non_camel_case_types)]
// Please `minecraft:` prefix this as well, when stringified!! <----
pub enum FrogVariant {
    temperate,
    warm,
    cold,
}

#[allow(non_snake_case)]
#[inject_fields(EntityLike, MobLike)]
pub struct Ghast {
    ExplosionPower: ByteTag,
}

#[inject_fields(EntityLike, MobLike)]
pub struct Giant {}

#[allow(non_snake_case)]
#[inject_fields(EntityLike, MobLike)]
pub struct GlowSquid {
    DarkTicksRemaining: IntTag,
}

#[allow(non_snake_case)]
#[inject_fields(EntityLike, MobLike)]
pub struct Goat {
    HasLeftHorn: BooleanTag,
    HasRightHorn: BooleanTag,
    IsScreamingGoat: BooleanTag,
}

#[inject_fields(EntityLike, MobLike)]
pub struct Guardian {}

#[allow(non_snake_case)]
#[inject_fields(EntityLike, MobLike, BreedableLike, HorseLike)]
pub struct Horse {
    ArmorItem: Option<Item>, // Only one of the Horse Armor types, so should be something like `Item<`minecraft:${string}_horse_armor`>`.
    Variant: IntTag<HorseVariant>,
}

pub enum HorseVariant {
    White = 0,
    Creamy = 1,
    Chestnut = 2,
    Brown = 3,
    Black = 4,
    Gray = 5,
    DarkBrown = 6,

    WhiteWhite = 256,
    CreamyWhite = 257,
    ChestnutWhite = 258,
    BrownWhite = 259,
    BlackWhite = 260,
    GrayWhite = 261,
    DarkBrownWhite = 262,

    WhiteWhiteField = 512,
    CreamyWhiteField = 513,
    ChestnutWhiteField = 514,
    BrownWhiteField = 515,
    BlackWhiteField = 516,
    GrayWhiteField = 517,
    DarkBrownWhiteField = 518,

    WhiteWhiteDots = 768,
    CreamyWhiteDots = 769,
    ChestnutWhiteDots = 770,
    BrownWhiteDots = 771,
    BlackWhiteDots = 772,
    GrayWhiteDots = 773,
    DarkBrownWhiteDots = 774,

    WhiteBlackDots = 1024,
    CreamyBlackDots = 1025,
    ChestnutBlackDots = 1026,
    BrownBlackDots = 1027,
    BlackBlackDots = 1028,
    GrayBlackDots = 1029,
    DarkBrownBlackDots = 1030,
}

#[allow(non_snake_case)]
#[inject_fields(EntityLike, MobLike, BreedableLike, PiglinLike)]
pub struct Hoglin {
    CannotBeHunted: BooleanTag,
}

#[inject_fields(EntityLike, MobLike, ZombieLike)]
pub struct Husk {}

#[allow(non_snake_case)]
#[inject_fields(EntityLike, MobLike, RaidLike)]
pub struct Illusioner {
    SpellTicks: IntTag,
}

#[allow(non_snake_case)]
#[inject_fields(EntityLike, MobLike, AngeredLike)]
pub struct IronGolem {
    PlayerCreated: BooleanTag,
}

// I think `HorseLike` could be narrowed a little bit so it can better allow for Llama crossover types.
#[allow(non_snake_case)]
#[inject_fields(EntityLike, MobLike, BreedableLike)]
pub struct Llama {
    Bred: BooleanTag,
    ChestedHorse: BooleanTag,
    DecorItem: Option<Item>, // Typically a Carpet, without the Slot tag.
    EatingHaystack: BooleanTag,
    Items: Option<ListTag<Item>>, // Only if `!!ChestedHorse`, with slot tags.
    Owner: Option<IntArrayTag>,   // `UUIDLike`
    Variant: IntTag<LlamaVariant>,
    Strength: IntTag<LlamaStrength>,
    Tame: BooleanTag, // `TameableLike` as well? I think the wiki was kind of goofed for this one.
    Temper: IntTag,
}

#[allow(non_camel_case_types)]
pub enum LlamaVariant {
    creamy = 0,
    white,
    brown,
    gray,
}

pub enum LlamaStrength {
    // again, lack of names here
    One = 1,
    Two,
    Three,
    Four,
    Five,
}

#[inject_fields(EntityLike, MobLike, SlimeLike)]
pub struct MagmaCube {}

#[allow(non_snake_case)]
#[inject_fields(EntityLike, MobLike, BreedableLike)]
pub struct Mooshroom {
    EffectDuration: Option<IntTag>,
    EffectId: Option<ByteTag<EffectID>>,
    Type: StringTag<MooshroomType>,
}

// *not* `minecraft:` prefixed, at least not yet.
#[allow(non_camel_case_types)]
pub enum MooshroomType {
    red,
    brown,
}

#[allow(non_snake_case)]
#[inject_fields(EntityLike, MobLike, BreedableLike, HorseLike)]
pub struct Mule {
    ChestedHorse: BooleanTag,
    Items: Option<ListTag<Item>>, // only if `!!ChestedHorse`, and slot tag numbered 2-16.
}

#[allow(non_snake_case)]
#[inject_fields(EntityLike, MobLike, BreedableLike)]
pub struct Ocelot {
    Trusting: BooleanTag,
}

#[allow(non_snake_case)]
#[inject_fields(EntityLike, MobLike, BreedableLike)]
pub struct Panda {
    HiddenGene: StringTag<PandaGene>,
    MainGene: StringTag<PandaGene>,
}

// Are these `minecraft:`-prefixed?
#[allow(non_camel_case_types)]
pub enum PandaGene {
    normal,
    lazy,
    worried,
    playful,
    brown,
    weak,
    aggressive,
}

#[allow(non_snake_case)]
#[inject_fields(EntityLike, MobLike, TameableLike)]
pub struct Parrot {
    Variant: IntTag<ParrotVariant>,
}

#[allow(non_camel_case_types)]
pub enum ParrotVariant {
    red_blue = 0,
    blue,
    green,
    yellow_blue,
    gray,
}

#[allow(non_snake_case)]
#[inject_fields(EntityLike, MobLike)]
pub struct Phantom {
    AX: IntTag,
    AY: IntTag,
    AZ: IntTag,
    Size: IntTag,
}

#[inject_fields(EntityLike, MobLike, BreedableLike, SaddledLike)]
pub struct Pig {}

#[allow(non_snake_case)]
#[inject_fields(EntityLike, MobLike, AngeredLike)]
pub struct Piglin {
    CannotHunt: BooleanTag,
    Inventory: ListTag<Item>, // 8 items, with slot tag
    IsBaby: Option<BooleanTag>,
}

#[inject_fields(EntityLike, MobLike, AngeredLike, PiglinLike)]
pub struct PiglinBrute {}

#[allow(non_snake_case)]
#[inject_fields(EntityLike, MobLike, RaidLike)]
pub struct Pillager {
    Inventory: ListTag<Item>, // Currently unused, is it optional?
}

#[inject_fields(EntityLike, MobLike, BreedableLike, AngeredLike)]
pub struct PolarBear {}

#[allow(non_snake_case)]
#[inject_fields(EntityLike, MobLike, BucketableLike)]
pub struct Pufferfish {
    PuffState: IntTag<PufferfishPuffState>,
}

pub enum PufferfishPuffState {
    Deflated = 0,
    HalfPuffed,
    FullPuffed,
}

#[allow(non_snake_case)]
#[inject_fields(EntityLike, MobLike, BreedableLike)]
pub struct Rabbit {
    MoreCarrotTicks: IntTag,
    RabbitType: IntTag<RabbitType>,
}

#[allow(non_camel_case_types)]
pub enum RabbitType {
    // `99` is The Killer Bunny, and adding a custom name "Toast" will be the Toast variant.
    brown = 0,
    white,
    black,
    white_splotched,
    gold,
    salt,
    evil = 99,
}

#[allow(non_snake_case)]
#[inject_fields(EntityLike, MobLike, RaidLike)]
pub struct Ravager {
    AttackTick: IntTag,
    RoarTick: IntTag,
    StunTick: IntTag,
}

#[inject_fields(EntityLike, MobLike, BucketableLike)]
pub struct Salmon {}

#[allow(non_snake_case)]
#[inject_fields(EntityLike, MobLike, BreedableLike)]
pub struct Sheep {
    Color: ByteTag<SheepColor>,
    Sheared: BooleanTag,
}

pub enum SheepColor {
    White = 0,
    Orange,
    Magenta,
    LightBlue,
    Yellow,
    Lime,
    Pink,
    Gray,
    LightGray,
    Cyan,
    Purple,
    Blue,
    Brown,
    Green,
    Red,
    Black,
}

#[allow(non_snake_case)]
#[inject_fields(EntityLike, MobLike)]
pub struct Shulker {
    APX: IntTag,
    APY: IntTag,
    APZ: IntTag,
    AttachFace: ByteTag<ShulkerDirection>,
    Color: ByteTag<ShulkerColor>,
}

pub enum ShulkerDirection {
    Top = 0,
    Bottom,
    North,
    South,
    West,
    East,
}

pub enum ShulkerColor {
    White = 0,
    Orange,
    Magenta,
    LightBlue,
    Yellow,
    Lime,
    Pink,
    Gray,
    LightGray,
    Cyan,
    Purple,
    Blue,
    Brown,
    Green,
    Red,
    Black,
    Default,
}

#[inject_fields(EntityLike, MobLike)]
pub struct Silverfish {}

#[allow(non_snake_case)]
#[inject_fields(EntityLike, MobLike)]
pub struct Skeleton {
    StrayConversionTime: IntTag,
}

#[allow(non_snake_case)]
#[inject_fields(EntityLike, MobLike, BreedableLike, HorseLike)]
pub struct SkeletonHorse {
    SkeletonTrap: BooleanTag,
    SkeletonTrapTime: IntTag,
}

#[inject_fields(EntityLike, MobLike, SlimeLike)]
pub struct Slime {}

#[allow(non_snake_case)]
#[inject_fields(EntityLike, MobLike)]
pub struct SnowGolem {
    Pumpkin: BooleanTag,
}

#[inject_fields(EntityLike, MobLike, BreedableLike)]
pub struct Sniffer {}

#[inject_fields(EntityLike, MobLike)]
pub struct Spider {}

#[inject_fields(EntityLike, MobLike)]
pub struct Squid {}

#[inject_fields(EntityLike, MobLike)]
pub struct Stray {}

#[inject_fields(EntityLike, MobLike, BreedableLike, SaddledLike)]
pub struct Strider {}

#[allow(non_snake_case)]
#[inject_fields(EntityLike, MobLike, BucketableLike)]
pub struct Tadpole {
    Age: IntTag,
}

// I think `HorseLike` could be narrowed a little bit so it can better allow for Llama crossover types.
#[allow(non_snake_case)]
#[inject_fields(EntityLike, MobLike, BreedableLike)]
pub struct TraderLlama {
    Bred: BooleanTag,
    ChestedHorse: BooleanTag,
    DecorItem: Option<Item>, // Typically a Carpet, without the Slot tag.
    DespawnDelay: IntTag,    // Unique to Trader Llamas
    EatingHaystack: BooleanTag,
    Items: Option<ListTag<Item>>, // Only if `!!ChestedHorse`, with slot tags.
    Owner: Option<IntArrayTag>,   // `UUIDLike`
    Variant: IntTag<LlamaVariant>,
    Strength: IntTag<LlamaStrength>,
    Tame: BooleanTag, // `TameableLike` as well? I think the wiki was kind of goofed for this one.
    Temper: IntTag,
}

#[allow(non_snake_case)]
#[inject_fields(EntityLike, MobLike, BucketableLike)]
pub struct TropicalFish {
    Variant: IntTag<TropicalFishVariant>,
}

pub type TropicalFishVariant = i32; // <https://minecraft.wiki/w/Tropical_Fish#Entity_data>

#[allow(non_snake_case)]
#[inject_fields(EntityLike, MobLike, BreedableLike)]
pub struct Turtle {
    HasEgg: BooleanTag,
    HomePosX: IntTag,
    HomePosY: IntTag,
    HomePosZ: IntTag,
    TravelPosX: IntTag,
    TravelPosY: IntTag,
    TravelPosZ: IntTag,
}

#[allow(non_snake_case)]
#[inject_fields(EntityLike, MobLike)]
pub struct Vex {
    BoundX: IntTag,
    BoundY: IntTag,
    BoundZ: IntTag,
    LifeTicks: IntTag,
}

#[allow(non_snake_case)]
#[inject_fields(EntityLike, MobLike, VillagerLike, BreedableLike)]
pub struct Villager {
    Inventory: ListTag<Item>, // 8 slots, with slot tag.
    LastRestock: LongTag,
    LastGossipDecay: LongTag,
    RestocksToday: IntTag,
    Willing: BooleanTag,
}

#[allow(non_snake_case)]
#[inject_fields(EntityLike, MobLike, RaidLike)]
pub struct Vindicator {
    Johnny: Option<BooleanTag>,
}

#[allow(non_snake_case)]
#[inject_fields(EntityLike, MobLike, BreedableLike)]
pub struct WanderingTrader {
    DespawnDelay: IntTag,
    Inventory: ListTag<Item>, // 8 slots, with slot tag, unused
    Offers: Option<WanderingTraderOffers>,
    WanderTarget: WanderTarget,
}

#[allow(non_snake_case)]
pub struct WanderingTraderOffers {
    Recipes: ListTag<TradeOptionLike>,
}

// Could be generalized to `Position` also.
#[allow(non_snake_case)]
pub struct WanderTarget {
    X: IntTag,
    Y: IntTag,
    Z: IntTag,
}

#[inject_fields(EntityLike, MobLike)]
pub struct Warden {
    anger: WardenAnger,
}

pub struct WardenAnger {
    suspects: ListTag<WardenAngerSuspect>,
}

pub struct WardenAngerSuspect {
    anger: IntTag,
    uuid: IntArrayTag, // `UUIDLike`
}

#[inject_fields(EntityLike, MobLike, RaidLike)]
pub struct Witch {}

#[allow(non_snake_case)]
#[inject_fields(EntityLike, MobLike)]
pub struct Wither {
    Invul: IntTag,
}

#[inject_fields(EntityLike, MobLike)]
pub struct WitherSkeleton {}

#[inject_fields(
    EntityLike,
    MobLike,
    BreedableLike,
    TameableLike,
    AngeredLike,
    CollaredLike
)]
pub struct Wolf {
    // v1.20.5
    // armor: BooleanTag,
}

#[allow(non_snake_case)]
#[inject_fields(EntityLike, MobLike)]
pub struct Zoglin {
    isBaby: Option<BooleanTag>,
}

#[inject_fields(EntityLike, MobLike, ZombieLike)]
pub struct Zombie {}

#[inject_fields(EntityLike, MobLike, BreedableLike, HorseLike)]
pub struct ZombieHorse {}

#[allow(non_snake_case)]
#[inject_fields(EntityLike, MobLike, VillagerLike, ZombieLike)]
pub struct ZombieVillager {
    ConversionTime: IntTag,
    ConcersionPlayer: IntArrayTag, // `UUIDLike`
}

#[inject_fields(EntityLike, MobLike, AngeredLike, ZombieLike)]
pub struct ZombifiedPiglin {}

#[inject_fields(EntityLike, BoatLike)]
pub struct Boat {}

#[inject_fields(EntityLike, BoatLike, ContainerEntityLike)]
pub struct ChestBoat {}

#[allow(non_snake_case)]
pub struct BoatLike {
    Type: StringTag<BoatType>,
}

// Is this `minecraft:`-prefixed like `CatVariant`?
#[allow(non_camel_case_types)]
pub enum BoatType {
    oak,
    spruce,
    birch,
    jungle,
    acacia,
    dark_oak,
    mangrove,
    bamboo,
}

#[inject_fields(EntityLike, MinecartLike)]
pub struct Minecart {}

#[inject_fields(EntityLike, MinecartLike, ContainerEntityLike)]
pub struct ChestMinecart {}

#[allow(non_snake_case)]
#[inject_fields(EntityLike, MinecartLike)]
pub struct FurnaceMinecart {
    Fuel: ShortTag,
    PushX: DoubleTag,
    PushZ: DoubleTag,
}

#[allow(non_snake_case)]
#[inject_fields(EntityLike, MinecartLike)]
pub struct TNTMinecart {
    TNTFuse: IntTag,
}

#[allow(non_snake_case)]
#[inject_fields(EntityLike, MinecartLike, ContainerEntityLike)]
pub struct HopperMinecart {
    Enabled: BooleanTag,
    TransferCooldown: IntTag<HopperMinecartTransferCooldown>, // is this deprecated, or rather removed? can no longer find it on the wiki
}

pub enum HopperMinecartTransferCooldown {
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

#[inject_fields(EntityLike, MinecartLike, MobSpawnerLike)]
pub struct SpawnerMinecart {}

// Should this inherit from `./block-entity - CommandBlockLike` of some sort? The wiki doesn't do this, and I'm curious if the docs for this don't match the current NBT, since this one is missing some of the Command Block-ish ones.
#[allow(non_snake_case)]
#[inject_fields(EntityLike, MinecartLike)]
pub struct CommandBlockMinecart {
    Command: StringTag,
    LastOutput: StringTag,
    SuccessCount: IntTag,
    TrackOutput: BooleanTag,
}

#[allow(non_snake_case)]
pub struct MinecartLike {
    CustomDisplayTile: Option<BooleanTag>,
    DisplayOffset: Option<IntTag>,
    DisplayState: Option<MinecartDisplayState>,
}

#[allow(non_snake_case)]
pub struct MinecartDisplayState {
    Name: BlockResource,
    Properties: BlockState,
}

#[allow(non_snake_case)]
#[inject_fields(EntityLike)]
pub struct ItemEntity {
    Age: ShortTag,
    Health: ShortTag<ItemHealth>,
    Item: Item,
    Owner: Option<IntArrayTag>,
    PickupDelay: ShortTag,
    Thrower: Option<IntArrayTag>,
}

pub enum ItemHealth {
    Zero = 0,
    One,
    Two,
    Three,
    Four,
    Five,
}

#[allow(non_snake_case)]
#[inject_fields(EntityLike)]
pub struct ExperienceOrb {
    Age: ShortTag,
    Count: IntTag,
    Health: ShortTag,
    Value: ShortTag,
}

#[inject_fields(EntityLike, ArrowLike)]
pub struct Arrow {}

#[inject_fields(EntityLike, ArrowLike)]
pub struct SpectralArrow {}

// How can the potion effect types be optionally added/defined only for tipped arrows? Just with `extends Partial<PotionEffectLike>`?
#[allow(non_snake_case)]
#[inject_fields(ProjectileLike, PotionEffectLike)]
pub struct ArrowLike {
    crit: BooleanTag,
    damage: DoubleTag,
    inBlockState: Option<ArrowBlockState>,
    inGround: BooleanTag,
    life: ShortTag,
    pickup: ByteTag<ArrowPickup>,
    PierceLevel: ByteTag,
    shake: ByteTag,
    ShotFromCrossbow: BooleanTag,
    SoundEvent: StringTag, // I don't think this is a SoundResource actually?
}

pub enum ArrowPickup {
    Immovable = 0,
    SurvivalOrCreative,
    Creative,
}

#[allow(non_snake_case)]
pub struct ArrowBlockState {
    Name: StringTag<BlockResource>,
    Properties: Option<BlockState>,
}

#[allow(non_snake_case)]
#[inject_fields(EntityLike, ArrowLike, ProjectileLike)]
pub struct Trident {
    DealtDamage: BooleanTag,
    // I think the shape of this looks like this, the formatting on the wiki is a bit weird.
    Trident: TridentData,
}

pub struct TridentData {
    item: Item, // `minecraft:trident` Item, or `Item<"minecraft:trident">` essentially.
}

#[inject_fields(EntityLike, ProjectileLike, ThrownItemLike)]
pub struct Snowball {}

#[inject_fields(EntityLike, ProjectileLike, ThrownItemLike)]
pub struct Egg {}

#[inject_fields(EntityLike, ProjectileLike)]
pub struct LlamaSpit {}

#[inject_fields(EntityLike, ProjectileLike, ThrownItemLike)]
pub struct EnderPearl {}

#[inject_fields(EntityLike, ThrownItemLike)]
pub struct EyeOfEnder {}

#[allow(non_snake_case)]
#[inject_fields(EntityLike, ProjectileLike)]
pub struct FireworkRocket {
    FireworksItem: FireworksItem,
    Life: IntTag,
    LifeTime: IntTag,
    ShotAtAngle: BooleanTag,
}

// Is this an extension/generic of what would be `Item<"minecraft:firework_rocket">`, with additional Firework properties?
#[allow(non_snake_case)]
pub struct FireworksItem {
    Count: ByteTag, // typically one
    id: StringTag,  // StringTag<ItemResource::firework_rocket>,
    tag: FireworkTag,
}

#[allow(non_snake_case)]
pub struct FireworkTag {
    Fireworks: FireworkData, // optional? I don't think so, but the wiki wording is a little off.
}

#[allow(non_snake_case)]
pub struct FireworkData {
    Explosions: ListTag<FireworkExplosion>,
    Flight: ByteTag, // flight duration
}

#[allow(non_snake_case)]
pub struct FireworkExplosion {
    Colors: IntArrayTag,
    FadeColors: IntArrayTag,
    Flicker: Option<BooleanTag>,
    Trail: Option<BooleanTag>,
    Type: ByteTag<FireworkShape>,
}

pub enum FireworkShape {
    SmallBall = 0,
    LargeBall,
    Star,
    Creeper,
    Burst,
}

#[allow(non_snake_case)]
#[inject_fields(EntityLike)]
pub struct TNT {
    // looks like it has changed from one to the other at some point
    Fuse: ShortTag,
    fuse: ShortTag,
    // is this just `BlockState`? This gets confusing where they are nested, seems to be this way multiple other instances as well.
    block_state: TNTBlockState,
}

#[allow(non_snake_case)]
pub struct TNTBlockState {
    Name: StringTag<BlockResource>,
    Properties: Option<BlockState>,
}

#[allow(non_snake_case)]
#[inject_fields(EntityLike)]
pub struct FallingBlock {
    // This is the same weird thing as `TNT`.
    BlockState: FallingBlockBlockState,
    CancelDrop: BooleanTag,
    DropItem: BooleanTag,
    FallHurtAmount: FloatTag,
    FallHurtMax: IntTag,
    HurtEntities: BooleanTag,
    TileEntityData: Option<BlockEntity>, // I'm pretty sure this is `BlockEntity`, but the wiki doesn't specifically mention it.
    Time: IntTag,
}

#[allow(non_snake_case)]
pub struct FallingBlockBlockState {
    Name: StringTag<BlockResource>,
    Properties: Option<BlockState>,
}

#[inject_fields(EntityLike)]
pub struct FishingBobber {}

#[inject_fields(EntityLike)]
pub struct LightningBolt {}

#[inject_fields(EntityLike)]
pub struct LeashKnot {}

#[inject_fields(EntityLike, HangableLike)]
pub struct Painting {
    variant: StringTag, // `PaintingVariant` union type
}

#[allow(non_snake_case)]
#[inject_fields(EntityLike, HangableLike)]
pub struct ItemFrame {
    Fixed: BooleanTag,
    Invisible: BooleanTag,
    Item: Option<Item>,
    ItemDropChance: FloatTag,
    ItemRotation: ByteTag,
}

// Is `MobLike`, except for `LeftHanded`, `DeathLootTable`, `DeathLootTableSeed`, `NoAI`, `Leash`, `CanPickUpLoot` and `PersistenceRequired`.
#[allow(non_snake_case)]
#[inject_fields(EntityLike, MobLike)]
pub struct ArmorStand {
    DisabledSlots: IntTag,
    Invisible: BooleanTag,
    Marker: Option<BooleanTag>,
    NoBasePlate: BooleanTag,
    Pose: ArmorStandPose,
    ShowArms: BooleanTag,
    Small: BooleanTag,
}

#[allow(non_snake_case)]
pub struct ArmorStandPose {
    Body: ArmorStandPoseEntry,
    Head: ArmorStandPoseEntry,
    LeftArm: ArmorStandPoseEntry,
    LeftLeg: ArmorStandPoseEntry,
    RightArm: ArmorStandPoseEntry,
    RightLeg: ArmorStandPoseEntry,
}

pub type ArmorStandPoseEntry = [FloatTag; 3];

#[allow(non_snake_case)]
#[inject_fields(EntityLike, ProjectileLike, ThrownItemLike, FireballLike)]
pub struct Fireball {
    ExplosionPower: ByteTag,
}

#[inject_fields(EntityLike, ProjectileLike, FireballLike)]
pub struct WitherSkull {
    dangerous: BooleanTag, // might want to be optional <https://minecraft.wiki/w/Wither#cite_ref-11>
}

#[inject_fields(EntityLike, ProjectileLike, FireballLike)]
pub struct DragonFireball {}

#[allow(non_snake_case)]
#[inject_fields(EntityLike, ProjectileLike)]
pub struct ShulkerBullet {
    Steps: IntTag,
    Target: IntArrayTag, // `UUIDLike`, `IntArrayTag<[number, number, number, number]>`
    TXD: DoubleTag,
    TYD: DoubleTag,
    TZD: DoubleTag,
}

#[allow(non_snake_case)]
#[inject_fields(EntityLike)]
pub struct EndCrystal {
    BeamTarget: EndCrystalBeamTarget,
    ShowBottom: BooleanTag,
}

#[allow(non_snake_case)]
pub struct EndCrystalBeamTarget {
    X: IntTag,
    Y: IntTag,
    Z: IntTag,
}

#[allow(non_snake_case)]
#[inject_fields(EntityLike)]
pub struct EvokerFangs {
    Owner: IntArrayTag, // `UUIDLike`
    Warmup: IntTag,
}

#[inject_fields(EntityLike)]
pub struct Marker {
    data: (), // `unknown`/`any` // <https://minecraft.wiki/w/Marker#Entity_data>
}

#[inject_fields(EntityLike, DisplayLike)]
pub struct ItemDisplay {
    item_display: StringTag<ItemDisplayModel>,
}

#[allow(non_camel_case_types)]
pub enum ItemDisplayModel {
    none,
    thirdperson_lefthand,
    thirdperson_righthand,
    firstperson_lefthand,
    firstperson_righthand,
    head,
    gui,
    ground,
    fixed,
}

#[inject_fields(EntityLike, DisplayLike)]
pub struct BlockDisplay {
    block_state: BlockState,
}

#[inject_fields(EntityLike, DisplayLike)]
pub struct TextDisplay {
    alignment: StringTag<TextDisplayAlignment>,
    background: IntTag,
    default_background: BooleanTag,
    line_width: IntTag,
    see_through: BooleanTag,
    shadow: BooleanTag,
    text: StringTag, // raw JSON text <https://minecraft.wiki/w/Raw_JSON_text_format>
    text_opacity: ByteTag,
}

#[allow(non_camel_case_types)]
pub enum TextDisplayAlignment {
    center,
    left,
    right,
}

#[inject_fields(EntityLike)]
pub struct Interaction {
    width: FloatTag,
    height: FloatTag,
    response: BooleanTag,
    attack: InteractionEvent,
    interaction: InteractionEvent,
}

pub struct InteractionEvent {
    player: IntArrayTag, // `UUIDLike`
    timestamp: LongTag,
}

#[allow(non_camel_case_types)]
pub enum EntityResource {
    axolotl,
    bat,
    bee,
    blaze,
    camel,
    cat,
    cave_spider,
    chicken,
    cod,
    cow,
    creeper,
    dolphin,
    donkey,
    drowned,
    elder_guardian,
    ender_dragon,
    enderman,
    endermite,
    evoker,
    fox,
    ghast,
    giant,
    glow_squid,
    goat,
    guardian,
    hoglin,
    horse,
    husk,
    illusioner,
    iron_golem,
    llama,
    magma_cube,
    mooshroom,
    mule,
    ocelot,
    panda,
    parrot,
    phantom,
    pig,
    piglin,
    piglin_brute,
    pillager,
    player,
    polar_bear,
    pufferfish,
    rabbit,
    ravager,
    salmon,
    sheep,
    shulker,
    silverfish,
    sniffer,
    skeleton,
    skeleton_horse,
    slime,
    snow_golem,
    spider,
    squid,
    stray,
    strider,
    trader_llama,
    tropical_fish,
    turtle,
    vex,
    villager,
    vindicator,
    wandering_trader,
    witch,
    wither,
    wither_skeleton,
    wolf,
    zoglin,
    zombie,
    zombie_horse,
    zombie_villager,
    zombified_piglin,
    allay,
    frog,
    tadpole,
    warden,
    area_effect_cloud,
    armor_stand,
    end_crystal,
    evoker_fangs,
    fishing_bobber,
    item_frame,
    leash_knot,
    lightning_bolt,
    marker,
    interaction,
    block_display,
    text_display,
    item_display,
    painting,
    arrow,
    dragon_fireball,
    egg,
    ender_pearl,
    experience_bottle,
    eye_of_ender,
    fireball,
    firework_rocket,
    llama_spit,
    potion,
    shulker_bullet,
    small_fireball,
    snowball,
    spectral_arrow,
    trident,
    wither_skull,
    boat,
    chest_boat,
    chest_minecart,
    command_block_minecart,
    furnace_minecart,
    hopper_minecart,
    minecart,
    spawner_minecart,
    tnt_minecart,
    falling_block,
    tnt,
    experience_orb,
    item,
}
