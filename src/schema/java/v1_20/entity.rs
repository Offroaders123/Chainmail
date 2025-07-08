use serde::{Deserialize, Serialize};

use crate::{
    schema::java::v1_20::{
        block::{BlockResource, BlockState},
        block_entity::{BlockEntity, MobSpawnerLike},
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
#[derive(Serialize, Deserialize)]
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
#[derive(Serialize, Deserialize)]
pub struct ContainerEntityLike {
    pub Items: ListTag<Item>,         // `Slot` tag as well, need to add that
    pub LootTable: Option<StringTag>, // LootTableResource
    pub LootTableSeed: Option<LongTag>,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct ProjectileLike {
    pub HasBeenShot: BooleanTag,
    pub LeftOwner: Option<BooleanTag>, // `Option<TrueTag>`
    pub Owner: Option<IntArrayTag>,    // `IntArrayTag<[number, number, number, number]>`
}

// should this be generic?
#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct ThrownItemLike {
    pub Item: Option<Item>,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct HangableLike {
    pub Facing: ByteTag<HangableFacing>,
    pub TileX: IntTag,
    pub TileY: IntTag,
    pub TileZ: IntTag,
}

#[derive(Serialize, Deserialize)]
pub enum HangableFacing {
    Bottom = 0,
    Top,
    North,
    South,
    West,
    East,
}

#[derive(Serialize, Deserialize)]
pub struct FireballLike {
    pub power: FireballPower,
}

pub type FireballPower = [DoubleTag; 3];

#[derive(Serialize, Deserialize)]
pub struct DisplayLike {
    pub billboard: StringTag<DisplayBillboard>,
    pub brightness: DisplayBrightness,
    pub glow_color_override: IntTag,
    pub height: FloatTag,
    pub width: FloatTag,
    pub interpolation_duration: IntTag,
    pub teleport_duration: IntTag,
    pub start_interpolation: IntTag,
    pub shadow_radius: FloatTag,
    pub shadow_strength: FloatTag,
    pub view_range: FloatTag,
    pub transformation: DisplayTransformation,
}

#[allow(non_camel_case_types)]
#[derive(Serialize, Deserialize)]
pub enum DisplayBillboard {
    fixed,
    vertical,
    horizontal,
    center,
}

#[derive(Serialize, Deserialize)]
pub struct DisplayBrightness {
    pub block: IntTag<DisplayBrightnessLevel>,
    pub sky: IntTag<DisplayBrightnessLevel>,
}

#[derive(Serialize, Deserialize)]
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

#[derive(Serialize, Deserialize)]
pub enum DisplayTransformation {
    Matrix(DisplayTransformationMatrix),
    Decomposed(DisplayTransformationDecomposed),
}

pub type DisplayTransformationMatrix = [FloatTag; 16];

#[derive(Serialize, Deserialize)]
pub struct DisplayTransformationDecomposed {
    pub left_rotation: DisplayRotation,
    pub translation: DisplayTranslation,
    pub right_rotation: DisplayRotation,
    pub scale: DisplayScale,
}

#[derive(Serialize, Deserialize)]
pub enum DisplayRotation {
    Quaternion(DisplayRotationQuaternion),
    AxisAngle(DisplayRotationAxisAngle),
}

pub type DisplayRotationQuaternion = [FloatTag; 4];

#[derive(Serialize, Deserialize)]
pub struct DisplayRotationAxisAngle {
    pub angle: FloatTag,
    pub axis: DisplayRotationAxis,
}

pub type DisplayRotationAxis = [FloatTag; 3];

pub type DisplayTranslation = [FloatTag; 3];

pub type DisplayScale = [FloatTag; 3];

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct PotionEffectLike {
    pub custom_potion_effects: ListTag<PotionEffectEntry>,
    pub Potion: StringTag, // not fully fleshed out <https://minecraft.wiki/w/Arrow#Data_values>, <https://minecraft.wiki/w/Potion#Item_data>
    pub CustomPotionColor: IntTag,
    pub Color: IntTag, // specific to Arrows..?
}

#[derive(Serialize, Deserialize)]
pub struct PotionEffectEntry {
    pub id: IntTag<EffectID>,
    pub amplifier: Option<ByteTag>,
    pub duration: Option<IntTag>,
    pub ambient: Option<BooleanTag>,
    pub show_particles: Option<BooleanTag>,
    pub show_icon: BooleanTag, // also optional?
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct MobLike {
    pub AbsorptionAmount: FloatTag,
    pub ActiveEffects: ListTag<Effect>,
    pub ArmorDropChances: ArmorDropChances,
    pub ArmorItems: ArmorItems,
    pub Attributes: ListTag<Attribute>,
    pub Brain: MobBrain,
    pub CanPickUpLoot: BooleanTag,
    pub DeathLootTable: Option<StringTag>, // `LootTableResource`
    pub DeathLootTableSeed: Option<LongTag>,
    pub DeathTime: ShortTag,
    pub FallFlying: ByteTag,
    pub Health: FloatTag,
    pub HurtByTimestamp: IntTag,
    pub HurtTime: ShortTag,
    pub HandDropChances: HandDropChances,
    pub HandItems: HandItems,
    pub Leash: Option<Leash>,
    pub LeftHanded: BooleanTag,
    pub NoAI: BooleanTag,
    pub PersistenceRequired: BooleanTag,
    pub SleepingX: IntTag,
    pub SleepingY: IntTag,
    pub SleepingZ: IntTag,
    pub Team: Option<StringTag>, // `ScoreboardTeam` ?
}

#[derive(Serialize, Deserialize)]
pub struct MobBrain {
    pub memories: CompoundTag, // `Memories`, needs to be typed eventually. Just an empty object right now, in practice. `{}` in TypeScript.
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct BreedableLike {
    pub Age: IntTag,
    pub ForcedAge: IntTag,
    pub InLove: IntTag,
    pub LoveCause: IntArrayTag, // `UUIDLike`
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct BucketableLike {
    pub FromBucket: BooleanTag,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct TameableLike {
    pub Owner: Option<IntArrayTag>, // `UUIDLike`
    pub Sitting: BooleanTag,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct CollaredLike {
    pub CollarColor: ByteTag<CollarColor>,
}

#[derive(Serialize, Deserialize)]
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
#[derive(Serialize, Deserialize)]
pub struct SaddledLike {
    pub Saddle: BooleanTag,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct AngeredLike {
    pub AngerTime: IntTag,
    pub AngryAt: IntArrayTag, // `UUIDLike`
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct HorseLike {
    pub Bred: BooleanTag,
    pub EatingHaystack: BooleanTag,
    pub Owner: Option<IntArrayTag>, // `UUIDLike`
    pub SaddleItem: Option<Item>,   // `Item<"minecraft:saddle">`
    pub Tame: BooleanTag,
    pub Temper: IntTag,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct VillagerLike {
    pub Gossips: ListTag<VillagerGossip>,
    pub Offers: Option<VillagerOffers>, // "Is generated when the trading menu is opened for the first time.", optional?
    pub VillagerData: VillagerData,
    pub Xp: IntTag,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct VillagerGossip {
    pub Value: IntTag,
    pub Target: IntArrayTag, // `UUIDLike`
    pub Type: StringTag<VillagerGossipType>,
}

#[allow(non_camel_case_types)]
#[derive(Serialize, Deserialize)]
pub enum VillagerGossipType {
    major_negative,
    minor_negative,
    major_positive,
    minor_positive,
    trading,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct VillagerOffers {
    pub Recipes: ListTag<VillagerTradeOption>,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct TradeOptionLike {
    pub buy: Item,
    pub buyB: Option<Item>,
    pub maxUses: IntTag,
    pub rewardExp: BooleanTag,
    pub sell: Item,
    pub uses: IntTag,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct VillagerTradeOption {
    #[serde(flatten)]
    pub trade_option_like: TradeOptionLike,
    pub demand: IntTag,
    pub priceMultiplier: FloatTag,
    pub specialPrice: IntTag,
    pub xp: IntTag,
}

#[derive(Serialize, Deserialize)]
pub struct VillagerData {
    pub level: IntTag<VillagerLevel>,
    pub profession: StringTag<VillagerProfession>,
    pub r#type: StringTag<VillagerType>,
}

#[derive(Serialize, Deserialize)]
pub enum VillagerLevel {
    Novice = 1,
    Apprentice,
    Journeyman,
    Expert,
    Master,
}

#[allow(non_camel_case_types)]
#[derive(Serialize, Deserialize)]
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
#[derive(Serialize, Deserialize)]
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
#[derive(Serialize, Deserialize)]
pub struct ZombieLike {
    pub CanBreakDoors: BooleanTag,
    pub DrownedConversionTime: IntTag,
    pub InWaterTime: IntTag,
    pub IsBaby: Option<BooleanTag>,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct PiglinLike {
    pub IsImmuneToZombification: BooleanTag,
    pub TimeInOverworld: IntTag,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct SlimeLike {
    pub Size: IntTag<SlimeSize>,
    pub wasOnGround: BooleanTag,
}

#[derive(Serialize, Deserialize)]
pub enum SlimeSize {
    // not a mistake, weird I know lol; allows for larger values, these are the natural ones though.
    Small = 0,
    Medium,
    Large = 3,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct RaidLike {
    pub CanJoinRaid: BooleanTag,
    pub PatrolLeader: BooleanTag,
    pub Patrolling: BooleanTag,
    pub PatrolTarget: RaidPatrolTarget, // This can be made generic to a `Position` kind of thing.
    pub RaidId: IntTag,
    pub Wave: IntTag, // union of values? probably a min/max for each difficulty I'd assume?
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct RaidPatrolTarget {
    pub X: IntTag,
    pub Y: IntTag,
    pub Z: IntTag,
}

pub type ArmorDropChances = [FloatTag; 4];

pub type ArmorItems = [Item; 4];

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct Attribute {
    pub Base: DoubleTag,
    pub Modifiers: ListTag<Modifier>,
    pub Name: StringTag, // `AttributeResource` ?
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct Modifier {
    pub Amount: DoubleTag,
    pub Name: StringTag, // `ModifierResource` ?
    pub Operation: IntTag<ModifierOperation>,
    pub UUID: IntArrayTag,
}

#[derive(Serialize, Deserialize)]
pub enum ModifierOperation {
    Zero = 0,
    One,
    Two,
}

pub type HandDropChances = [FloatTag; 2];

pub type HandItems = [Item; 2];

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub enum Leash {
    Array(IntArrayTag),
    Object { X: IntTag, Y: IntTag, Z: IntTag },
}

// <EntityID extends string | undefined>
#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct EntityLike {
    pub Air: ShortTag,
    pub CustomName: Option<StringTag>,
    pub CustomNameVisible: Option<BooleanTag>,
    pub FallDistance: FloatTag,
    pub Fire: ShortTag,
    pub Glowing: BooleanTag,
    pub HasVisualFire: BooleanTag,
    pub id: StringTag, // <--- could probably be `EntityResource` // EntityID extends string ? `${EntityID}` : EntityID,
    pub Invulnerable: BooleanTag,
    pub Motion: EntityMotion,
    pub NoGravity: BooleanTag,
    pub OnGround: BooleanTag,
    pub Passengers: ListTag<Entity>,
    pub PortalCooldown: IntTag,
    pub Pos: EntityPos,
    pub Rotation: EntityRotation,
    pub Silent: Option<BooleanTag>,
    pub Tags: ListTag<ScoreboardTag>,
    pub TicksFrozen: Option<IntTag>,
    pub UUID: IntArrayTag,
}

pub type EntityMotion = [DoubleTag; 3];

pub type EntityPos = [DoubleTag; 3];

pub type EntityRotation = [FloatTag; 2];

pub type ScoreboardTag = String; // I think this was/is eventually meant to be an union/enum of strings

// There should be no `id` field! This needs to be fixed.
// Tags for all entities, except the id, CustomName and CustomNameVisible
// Tags for all mobs, except HandItems, ArmorItems, HandDropChances, ArmorDropChances, CanPickUpLoot, PersistenceRequired and Leash
#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct Player {
    #[serde(flatten)]
    pub entity_like: EntityLike,
    #[serde(flatten)]
    pub mob_like: MobLike,
    pub abilities: Abilities,
    pub DataVersion: IntTag,
    pub Dimension: StringTag<DimensionResource>,
    pub EnderItems: ListTag<Item>,
    pub enteredNetherPosition: Option<EnteredNetherPosition>,
    pub foodExhaustionLevel: FloatTag,
    pub foodLevel: IntTag,
    pub foodSaturationLevel: FloatTag,
    pub foodTickTimer: IntTag,
    pub Inventory: ListTag<Item>,
    pub LastDeathLocation: Option<LastDeathLocation>,
    pub playerGameType: IntTag<GameMode>,
    pub previousPlayerGameType: IntTag<GameMode>,
    pub recipeBook: RecipeBook,
    pub RootVehicle: Option<RootVehicle>,
    pub Score: IntTag,
    pub seenCredits: BooleanTag,
    pub SelectedItem: Option<Item>,
    pub SelectedItemSlot: IntTag,
    // I think this is `minecraft:parrot` only, but I'm curious if you can put any entity on your shoulder in-game
    pub ShoulderEntityLeft: Box<Entity>, // Entity::parrot, // Entity<"parrot">,
    pub ShoulderEntityRight: Box<Entity>, // Entity::parrot, // Entity<"parrot">,
    pub SleepTimer: ShortTag,
    pub SpawnDimension: Option<StringTag<DimensionResource>>,
    pub SpawnForced: Option<BooleanTag>,
    pub SpawnX: Option<IntTag>,
    pub SpawnY: Option<IntTag>,
    pub SpawnZ: Option<IntTag>,
    pub warden_spawn_tracker: WardenSpawnTracker, // Optional? Doesn't specify on the wiki
    pub XpLevel: IntTag,
    pub XpP: FloatTag,
    pub XpSeed: IntTag,
    pub XpTotal: IntTag,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct Abilities {
    pub flying: BooleanTag,
    pub flySpeed: FloatTag, // It says it's always only ever `0.05`, but I feel like it might change for Spectator Mode?
    pub instabuild: BooleanTag,
    pub invulnerable: BooleanTag,
    pub mayBuild: BooleanTag,
    pub mayfly: BooleanTag,
    pub walkSpeed: FloatTag, // Same here, this apparently always stays the same (0.1)
}

#[derive(Serialize, Deserialize)]
pub struct EnteredNetherPosition {
    pub x: DoubleTag,
    pub y: DoubleTag,
    pub z: DoubleTag,
}

#[derive(Serialize, Deserialize)]
pub struct LastDeathLocation {
    pub dimension: StringTag<DimensionResource>,
    pub pos: IntArrayTag,
}

#[derive(Serialize, Deserialize)]
pub enum GameMode {
    Survival = 0,
    Creative,
    Adventure,
    Spectator,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct RecipeBook {
    pub recipes: ListTag<StringTag<RecipeResource>>,
    pub toBeDisplayed: ListTag<StringTag<RecipeResource>>,
    pub isFilteringCraftable: BooleanTag,
    pub isGuiOpen: BooleanTag,
    pub isFurnaceFilteringCraftable: BooleanTag,
    pub isFurnaceGuiOpen: BooleanTag,
    pub isBlastingFurnaceFilteringCraftable: BooleanTag,
    pub isBlastingFurnaceGuiOpen: BooleanTag,
    pub isSmokerFilteringCraftable: BooleanTag,
    pub isSmokerGuiOpen: BooleanTag,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct RootVehicle {
    pub Attach: IntArrayTag,
    pub Entity: Box<Entity>,
}

#[derive(Serialize, Deserialize)]
pub struct WardenSpawnTracker {
    pub cooldown_ticks: IntTag,
    pub ticks_since_last_warning: IntTag,
    pub warning_level: IntTag<WardenWarningLevel>,
}

#[derive(Serialize, Deserialize)]
pub enum WardenWarningLevel {
    // wasn't sure what else to call these when moving to an enum
    Zero = 0,
    One,
    Two,
    Three,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct Allay {
    #[serde(flatten)]
    pub entity_like: EntityLike,
    #[serde(flatten)]
    pub mob_like: MobLike,
    pub CanDuplicate: BooleanTag,
    pub DuplicationCooldown: LongTag,
    pub Inventory: [Option<Item>; 1],
    pub listener: AllayVibrationListener,
}

#[derive(Serialize, Deserialize)]
pub struct AllayVibrationListener {
    pub distance: IntTag,
    pub event: Option<AllayVibrationEvent>,
    pub event_delay: IntTag,
    pub event_distance: IntTag,
    pub range: IntTag,
    pub source: AllayVibrationListenerSource,
}

#[derive(Serialize, Deserialize)]
pub struct AllayVibrationEvent {
    pub distance: IntTag,
    pub game_event: StringTag, // Resource location of the game event
    pub pos: [DoubleTag; 3], // `PositionLike<DoubleTag>` maybe? I want to make a regular type for this pattern.
    pub projectile_owner: Option<IntArrayTag>, // `UUIDLike`
    pub source: Option<IntArrayTag>, // `UUIDLike`
}

#[derive(Serialize, Deserialize)]
pub enum AllayVibrationListenerSource {
    Block(AllayVibrationListenerSourceBlock),
    Entity(AllayVibrationListenerSourceEntity),
}

#[derive(Serialize, Deserialize)]
pub struct AllayVibrationListenerSourceBlock {
    pub r#type: StringTag, // StringTag<AllayVibrationListenerSource::Block>,
    pub pos: IntArrayTag,  // `IntArrayTag<[number, number, number]>`
}

#[derive(Serialize, Deserialize)]
pub struct AllayVibrationListenerSourceEntity {
    pub r#type: StringTag, // StringTag<AllayVibrationListenerSource::Entity>,
    pub source_entity: IntArrayTag, // `UUIDLike`
    pub y_offset: FloatTag,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct Axolotl {
    #[serde(flatten)]
    pub entity_like: EntityLike,
    #[serde(flatten)]
    pub mob_like: MobLike,
    #[serde(flatten)]
    pub breedable_like: BreedableLike,
    #[serde(flatten)]
    pub bucketable_like: BucketableLike,
    pub Variant: IntTag<AxolotlVariant>,
}

#[allow(non_camel_case_types)]
#[derive(Serialize, Deserialize)]
pub enum AxolotlVariant {
    lucy = 0,
    wild,
    gold,
    cyan,
    blue,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct Bat {
    #[serde(flatten)]
    pub entity_like: EntityLike,
    #[serde(flatten)]
    pub mob_like: MobLike,
    pub BatFlags: BooleanTag,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct Bee {
    #[serde(flatten)]
    pub entity_like: EntityLike,
    #[serde(flatten)]
    pub mob_like: MobLike,
    #[serde(flatten)]
    pub breedable_like: BreedableLike,
    #[serde(flatten)]
    pub angered_like: AngeredLike,
    pub CannotEnterHiveTicks: IntTag,
    pub CropsGrownSincePollination: IntTag,
    pub FlowerPos: BeePositionLike,
    pub HasNectar: BooleanTag,
    pub HasStung: BooleanTag,
    pub HivePosition: BeePositionLike,
    pub TicksSincePollination: IntTag,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct BeePositionLike {
    pub X: IntTag,
    pub Y: IntTag,
    pub Z: IntTag,
}

#[derive(Serialize, Deserialize)]
pub struct Blaze {
    #[serde(flatten)]
    pub entity_like: EntityLike,
    #[serde(flatten)]
    pub mob_like: MobLike,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct Camel {
    #[serde(flatten)]
    pub entity_like: EntityLike,
    #[serde(flatten)]
    pub mob_like: MobLike,
    #[serde(flatten)]
    pub breedable_like: BreedableLike,
    #[serde(flatten)]
    pub horse_like: HorseLike,
    pub LastPoseTick: LongTag,
}

#[derive(Serialize, Deserialize)]
pub struct Cat {
    #[serde(flatten)]
    pub entity_like: EntityLike,
    #[serde(flatten)]
    pub mob_like: MobLike,
    #[serde(flatten)]
    pub breedable_like: BreedableLike,
    #[serde(flatten)]
    pub tameable_like: TameableLike,
    #[serde(flatten)]
    pub collared_like: CollaredLike,
    pub variant: StringTag<CatVariant>,
}

#[allow(non_camel_case_types)]
// When stringified, these should have the `minecraft:` prefix! pls and thank you :)
#[derive(Serialize, Deserialize)]
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

#[derive(Serialize, Deserialize)]
pub struct CaveSpider {
    #[serde(flatten)]
    pub entity_like: EntityLike,
    #[serde(flatten)]
    pub mob_like: MobLike,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct Chicken {
    #[serde(flatten)]
    pub entity_like: EntityLike,
    #[serde(flatten)]
    pub mob_like: MobLike,
    #[serde(flatten)]
    pub breedable_like: BreedableLike,
    pub EggLayTime: IntTag,
    pub IsChickenJockey: BooleanTag,
}

#[derive(Serialize, Deserialize)]
pub struct Cod {
    #[serde(flatten)]
    pub entity_like: EntityLike,
    #[serde(flatten)]
    pub mob_like: MobLike,
    #[serde(flatten)]
    pub bucketable_like: BucketableLike,
}

#[derive(Serialize, Deserialize)]
pub struct Cow {
    #[serde(flatten)]
    pub entity_like: EntityLike,
    #[serde(flatten)]
    pub mob_like: MobLike,
    #[serde(flatten)]
    pub breedable_like: BreedableLike,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct Creeper {
    #[serde(flatten)]
    pub entity_like: EntityLike,
    #[serde(flatten)]
    pub mob_like: MobLike,
    pub ExplosionRadius: ByteTag,
    pub Fuse: ShortTag,
    pub ignited: BooleanTag,
    pub powered: Option<BooleanTag>,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct Dolphin {
    #[serde(flatten)]
    pub entity_like: EntityLike,
    #[serde(flatten)]
    pub mob_like: MobLike,
    pub CanFindTreasure: BooleanTag,
    pub GotFish: BooleanTag,
    pub TreasurePosX: IntTag,
    pub TreasurePosY: IntTag,
    pub TreasurePosZ: IntTag,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct Donkey {
    #[serde(flatten)]
    pub entity_like: EntityLike,
    #[serde(flatten)]
    pub mob_like: MobLike,
    #[serde(flatten)]
    pub breedable_like: BreedableLike,
    #[serde(flatten)]
    pub horse_like: HorseLike,
    pub ChestedHorse: BooleanTag,
    pub Items: Option<ListTag<Item>>, // only if `!!ChestedHorse`, with slot tag, 2-16
}

#[derive(Serialize, Deserialize)]
pub struct Drowned {
    #[serde(flatten)]
    pub entity_like: EntityLike,
    #[serde(flatten)]
    pub mob_like: MobLike,
    #[serde(flatten)]
    pub zombie_like: ZombieLike,
}

#[derive(Serialize, Deserialize)]
pub struct ElderGuardian {
    #[serde(flatten)]
    pub entity_like: EntityLike,
    #[serde(flatten)]
    pub mob_like: MobLike,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct EnderDragon {
    #[serde(flatten)]
    pub entity_like: EntityLike,
    #[serde(flatten)]
    pub mob_like: MobLike,
    pub DragonPhase: IntTag<EnderDragonPhase>,
}

#[derive(Serialize, Deserialize)]
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
#[derive(Serialize, Deserialize)]
pub struct Enderman {
    #[serde(flatten)]
    pub entity_like: EntityLike,
    #[serde(flatten)]
    pub mob_like: MobLike,
    #[serde(flatten)]
    pub angered_like: AngeredLike,
    // Another funky block state shape
    pub carriedBlockState: Option<CarriedBlockState>,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct CarriedBlockState {
    pub Name: StringTag<BlockResource>,
    pub Properties: Option<BlockState>,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct Endermite {
    #[serde(flatten)]
    pub entity_like: EntityLike,
    #[serde(flatten)]
    pub mob_like: MobLike,
    pub Lifetime: IntTag,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct Evoker {
    #[serde(flatten)]
    pub entity_like: EntityLike,
    #[serde(flatten)]
    pub mob_like: MobLike,
    #[serde(flatten)]
    pub raid_like: RaidLike,
    pub SpellTicks: IntTag,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct Fox {
    #[serde(flatten)]
    pub entity_like: EntityLike,
    #[serde(flatten)]
    pub mob_like: MobLike,
    #[serde(flatten)]
    pub breedable_like: BreedableLike,
    pub Crouching: BooleanTag,
    pub Sitting: BooleanTag,
    pub Sleeping: BooleanTag,
    pub Trusted: ListTag<IntArrayTag>, // `UUIDLike[]`
    pub Type: StringTag<FoxType>,
}

#[allow(non_camel_case_types)]
// Is this `minecraft:`-prefixed like `CatVariant`?
#[derive(Serialize, Deserialize)]
pub enum FoxType {
    red,
    snow,
}

#[derive(Serialize, Deserialize)]
pub struct Frog {
    #[serde(flatten)]
    pub entity_like: EntityLike,
    #[serde(flatten)]
    pub mob_like: MobLike,
    #[serde(flatten)]
    pub breedable_like: BreedableLike,
    pub variant: StringTag<FrogVariant>,
}

#[allow(non_camel_case_types)]
// Please `minecraft:` prefix this as well, when stringified!! <----
#[derive(Serialize, Deserialize)]
pub enum FrogVariant {
    temperate,
    warm,
    cold,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct Ghast {
    #[serde(flatten)]
    pub entity_like: EntityLike,
    #[serde(flatten)]
    pub mob_like: MobLike,
    pub ExplosionPower: ByteTag,
}

#[derive(Serialize, Deserialize)]
pub struct Giant {
    #[serde(flatten)]
    pub entity_like: EntityLike,
    #[serde(flatten)]
    pub mob_like: MobLike,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct GlowSquid {
    #[serde(flatten)]
    pub entity_like: EntityLike,
    #[serde(flatten)]
    pub mob_like: MobLike,
    pub DarkTicksRemaining: IntTag,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct Goat {
    #[serde(flatten)]
    pub entity_like: EntityLike,
    #[serde(flatten)]
    pub mob_like: MobLike,
    pub HasLeftHorn: BooleanTag,
    pub HasRightHorn: BooleanTag,
    pub IsScreamingGoat: BooleanTag,
}

#[derive(Serialize, Deserialize)]
pub struct Guardian {
    #[serde(flatten)]
    pub entity_like: EntityLike,
    #[serde(flatten)]
    pub mob_like: MobLike,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct Horse {
    #[serde(flatten)]
    pub entity_like: EntityLike,
    #[serde(flatten)]
    pub mob_like: MobLike,
    #[serde(flatten)]
    pub breedable_like: BreedableLike,
    #[serde(flatten)]
    pub horse_like: HorseLike,
    pub ArmorItem: Option<Item>, // Only one of the Horse Armor types, so should be something like `Item<`minecraft:${string}_horse_armor`>`.
    pub Variant: IntTag<HorseVariant>,
}

#[derive(Serialize, Deserialize)]
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
#[derive(Serialize, Deserialize)]
pub struct Hoglin {
    #[serde(flatten)]
    pub entity_like: EntityLike,
    #[serde(flatten)]
    pub mob_like: MobLike,
    #[serde(flatten)]
    pub breedable_like: BreedableLike,
    #[serde(flatten)]
    pub piglin_like: PiglinLike,
    pub CannotBeHunted: BooleanTag,
}

#[derive(Serialize, Deserialize)]
pub struct Husk {
    #[serde(flatten)]
    pub entity_like: EntityLike,
    #[serde(flatten)]
    pub mob_like: MobLike,
    #[serde(flatten)]
    pub zombie_like: ZombieLike,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct Illusioner {
    #[serde(flatten)]
    pub entity_like: EntityLike,
    #[serde(flatten)]
    pub mob_like: MobLike,
    #[serde(flatten)]
    pub raid_like: RaidLike,
    pub SpellTicks: IntTag,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct IronGolem {
    #[serde(flatten)]
    pub entity_like: EntityLike,
    #[serde(flatten)]
    pub mob_like: MobLike,
    #[serde(flatten)]
    pub angered_like: AngeredLike,
    pub PlayerCreated: BooleanTag,
}

// I think `HorseLike` could be narrowed a little bit so it can better allow for Llama crossover types.
#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct Llama {
    #[serde(flatten)]
    pub entity_like: EntityLike,
    #[serde(flatten)]
    pub mob_like: MobLike,
    #[serde(flatten)]
    pub breedable_like: BreedableLike,
    pub Bred: BooleanTag,
    pub ChestedHorse: BooleanTag,
    pub DecorItem: Option<Item>, // Typically a Carpet, without the Slot tag.
    pub EatingHaystack: BooleanTag,
    pub Items: Option<ListTag<Item>>, // Only if `!!ChestedHorse`, with slot tags.
    pub Owner: Option<IntArrayTag>,   // `UUIDLike`
    pub Variant: IntTag<LlamaVariant>,
    pub Strength: IntTag<LlamaStrength>,
    pub Tame: BooleanTag, // `TameableLike` as well? I think the wiki was kind of goofed for this one.
    pub Temper: IntTag,
}

#[allow(non_camel_case_types)]
#[derive(Serialize, Deserialize)]
pub enum LlamaVariant {
    creamy = 0,
    white,
    brown,
    gray,
}

#[derive(Serialize, Deserialize)]
pub enum LlamaStrength {
    // again, lack of names here
    One = 1,
    Two,
    Three,
    Four,
    Five,
}

#[derive(Serialize, Deserialize)]
pub struct MagmaCube {
    #[serde(flatten)]
    pub entity_like: EntityLike,
    #[serde(flatten)]
    pub mob_like: MobLike,
    #[serde(flatten)]
    pub slime_like: SlimeLike,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct Mooshroom {
    #[serde(flatten)]
    pub entity_like: EntityLike,
    #[serde(flatten)]
    pub mob_like: MobLike,
    #[serde(flatten)]
    pub breedable_like: BreedableLike,
    pub EffectDuration: Option<IntTag>,
    pub EffectId: Option<ByteTag<EffectID>>,
    pub Type: StringTag<MooshroomType>,
}

// *not* `minecraft:` prefixed, at least not yet.
#[allow(non_camel_case_types)]
#[derive(Serialize, Deserialize)]
pub enum MooshroomType {
    red,
    brown,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct Mule {
    #[serde(flatten)]
    pub entity_like: EntityLike,
    #[serde(flatten)]
    pub mob_like: MobLike,
    #[serde(flatten)]
    pub breedable_like: BreedableLike,
    #[serde(flatten)]
    pub horse_like: HorseLike,
    pub ChestedHorse: BooleanTag,
    pub Items: Option<ListTag<Item>>, // only if `!!ChestedHorse`, and slot tag numbered 2-16.
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct Ocelot {
    #[serde(flatten)]
    pub entity_like: EntityLike,
    #[serde(flatten)]
    pub mob_like: MobLike,
    #[serde(flatten)]
    pub breedable_like: BreedableLike,
    pub Trusting: BooleanTag,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct Panda {
    #[serde(flatten)]
    pub entity_like: EntityLike,
    #[serde(flatten)]
    pub mob_like: MobLike,
    #[serde(flatten)]
    pub breedable_like: BreedableLike,
    pub HiddenGene: StringTag<PandaGene>,
    pub MainGene: StringTag<PandaGene>,
}

// Are these `minecraft:`-prefixed?
#[allow(non_camel_case_types)]
#[derive(Serialize, Deserialize)]
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
#[derive(Serialize, Deserialize)]
pub struct Parrot {
    #[serde(flatten)]
    pub entity_like: EntityLike,
    #[serde(flatten)]
    pub mob_like: MobLike,
    #[serde(flatten)]
    pub tameable_like: TameableLike,
    pub Variant: IntTag<ParrotVariant>,
}

#[allow(non_camel_case_types)]
#[derive(Serialize, Deserialize)]
pub enum ParrotVariant {
    red_blue = 0,
    blue,
    green,
    yellow_blue,
    gray,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct Phantom {
    #[serde(flatten)]
    pub entity_like: EntityLike,
    #[serde(flatten)]
    pub mob_like: MobLike,
    pub AX: IntTag,
    pub AY: IntTag,
    pub AZ: IntTag,
    pub Size: IntTag,
}

#[derive(Serialize, Deserialize)]
pub struct Pig {
    #[serde(flatten)]
    pub entity_like: EntityLike,
    #[serde(flatten)]
    pub mob_like: MobLike,
    #[serde(flatten)]
    pub breedable_like: BreedableLike,
    #[serde(flatten)]
    pub saddled_like: SaddledLike,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct Piglin {
    #[serde(flatten)]
    pub entity_like: EntityLike,
    #[serde(flatten)]
    pub mob_like: MobLike,
    #[serde(flatten)]
    pub angered_like: AngeredLike,
    pub CannotHunt: BooleanTag,
    pub Inventory: ListTag<Item>, // 8 items, with slot tag
    pub IsBaby: Option<BooleanTag>,
}

#[derive(Serialize, Deserialize)]
pub struct PiglinBrute {
    #[serde(flatten)]
    pub entity_like: EntityLike,
    #[serde(flatten)]
    pub mob_like: MobLike,
    #[serde(flatten)]
    pub angered_like: AngeredLike,
    #[serde(flatten)]
    pub piglin_like: PiglinLike,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct Pillager {
    #[serde(flatten)]
    pub entity_like: EntityLike,
    #[serde(flatten)]
    pub mob_like: MobLike,
    #[serde(flatten)]
    pub raid_like: RaidLike,
    pub Inventory: ListTag<Item>, // Currently unused, is it optional?
}

#[derive(Serialize, Deserialize)]
pub struct PolarBear {
    #[serde(flatten)]
    pub entity_like: EntityLike,
    #[serde(flatten)]
    pub mob_like: MobLike,
    #[serde(flatten)]
    pub breedable_like: BreedableLike,
    #[serde(flatten)]
    pub angered_like: AngeredLike,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct Pufferfish {
    #[serde(flatten)]
    pub entity_like: EntityLike,
    #[serde(flatten)]
    pub mob_like: MobLike,
    #[serde(flatten)]
    pub bucketable_like: BucketableLike,
    pub PuffState: IntTag<PufferfishPuffState>,
}

#[derive(Serialize, Deserialize)]
pub enum PufferfishPuffState {
    Deflated = 0,
    HalfPuffed,
    FullPuffed,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct Rabbit {
    #[serde(flatten)]
    pub entity_like: EntityLike,
    #[serde(flatten)]
    pub mob_like: MobLike,
    #[serde(flatten)]
    pub breedable_like: BreedableLike,
    pub MoreCarrotTicks: IntTag,
    pub RabbitType: IntTag<RabbitType>,
}

#[allow(non_camel_case_types)]
#[derive(Serialize, Deserialize)]
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
#[derive(Serialize, Deserialize)]
pub struct Ravager {
    #[serde(flatten)]
    pub entity_like: EntityLike,
    #[serde(flatten)]
    pub mob_like: MobLike,
    #[serde(flatten)]
    pub raid_like: RaidLike,
    pub AttackTick: IntTag,
    pub RoarTick: IntTag,
    pub StunTick: IntTag,
}

#[derive(Serialize, Deserialize)]
pub struct Salmon {
    #[serde(flatten)]
    pub entity_like: EntityLike,
    #[serde(flatten)]
    pub mob_like: MobLike,
    #[serde(flatten)]
    pub bucketable_like: BucketableLike,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct Sheep {
    #[serde(flatten)]
    pub entity_like: EntityLike,
    #[serde(flatten)]
    pub mob_like: MobLike,
    #[serde(flatten)]
    pub breedable_like: BreedableLike,
    pub Color: ByteTag<SheepColor>,
    pub Sheared: BooleanTag,
}

#[derive(Serialize, Deserialize)]
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
#[derive(Serialize, Deserialize)]
pub struct Shulker {
    #[serde(flatten)]
    pub entity_like: EntityLike,
    #[serde(flatten)]
    pub mob_like: MobLike,
    pub APX: IntTag,
    pub APY: IntTag,
    pub APZ: IntTag,
    pub AttachFace: ByteTag<ShulkerDirection>,
    pub Color: ByteTag<ShulkerColor>,
}

#[derive(Serialize, Deserialize)]
pub enum ShulkerDirection {
    Top = 0,
    Bottom,
    North,
    South,
    West,
    East,
}

#[derive(Serialize, Deserialize)]
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

#[derive(Serialize, Deserialize)]
pub struct Silverfish {
    #[serde(flatten)]
    pub entity_like: EntityLike,
    #[serde(flatten)]
    pub mob_like: MobLike,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct Skeleton {
    #[serde(flatten)]
    pub entity_like: EntityLike,
    #[serde(flatten)]
    pub mob_like: MobLike,
    pub StrayConversionTime: IntTag,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct SkeletonHorse {
    #[serde(flatten)]
    pub entity_like: EntityLike,
    #[serde(flatten)]
    pub mob_like: MobLike,
    #[serde(flatten)]
    pub breedable_like: BreedableLike,
    #[serde(flatten)]
    pub horse_like: HorseLike,
    pub SkeletonTrap: BooleanTag,
    pub SkeletonTrapTime: IntTag,
}

#[derive(Serialize, Deserialize)]
pub struct Slime {
    #[serde(flatten)]
    pub entity_like: EntityLike,
    #[serde(flatten)]
    pub mob_like: MobLike,
    #[serde(flatten)]
    pub slime_like: SlimeLike,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct SnowGolem {
    #[serde(flatten)]
    pub entity_like: EntityLike,
    #[serde(flatten)]
    pub mob_like: MobLike,
    pub Pumpkin: BooleanTag,
}

#[derive(Serialize, Deserialize)]
pub struct Sniffer {
    #[serde(flatten)]
    pub entity_like: EntityLike,
    #[serde(flatten)]
    pub mob_like: MobLike,
    #[serde(flatten)]
    pub breedable_like: BreedableLike,
}

#[derive(Serialize, Deserialize)]
pub struct Spider {
    #[serde(flatten)]
    pub entity_like: EntityLike,
    #[serde(flatten)]
    pub mob_like: MobLike,
}

#[derive(Serialize, Deserialize)]
pub struct Squid {
    #[serde(flatten)]
    pub entity_like: EntityLike,
    #[serde(flatten)]
    pub mob_like: MobLike,
}

#[derive(Serialize, Deserialize)]
pub struct Stray {
    #[serde(flatten)]
    pub entity_like: EntityLike,
    #[serde(flatten)]
    pub mob_like: MobLike,
}

#[derive(Serialize, Deserialize)]
pub struct Strider {
    #[serde(flatten)]
    pub entity_like: EntityLike,
    #[serde(flatten)]
    pub mob_like: MobLike,
    #[serde(flatten)]
    pub breedable_like: BreedableLike,
    #[serde(flatten)]
    pub saddled_like: SaddledLike,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct Tadpole {
    #[serde(flatten)]
    pub entity_like: EntityLike,
    #[serde(flatten)]
    pub mob_like: MobLike,
    #[serde(flatten)]
    pub bucketable_like: BucketableLike,
    pub Age: IntTag,
}

// I think `HorseLike` could be narrowed a little bit so it can better allow for Llama crossover types.
#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct TraderLlama {
    #[serde(flatten)]
    pub entity_like: EntityLike,
    #[serde(flatten)]
    pub mob_like: MobLike,
    #[serde(flatten)]
    pub breedable_like: BreedableLike,
    pub Bred: BooleanTag,
    pub ChestedHorse: BooleanTag,
    pub DecorItem: Option<Item>, // Typically a Carpet, without the Slot tag.
    pub DespawnDelay: IntTag,    // Unique to Trader Llamas
    pub EatingHaystack: BooleanTag,
    pub Items: Option<ListTag<Item>>, // Only if `!!ChestedHorse`, with slot tags.
    pub Owner: Option<IntArrayTag>,   // `UUIDLike`
    pub Variant: IntTag<LlamaVariant>,
    pub Strength: IntTag<LlamaStrength>,
    pub Tame: BooleanTag, // `TameableLike` as well? I think the wiki was kind of goofed for this one.
    pub Temper: IntTag,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct TropicalFish {
    #[serde(flatten)]
    pub entity_like: EntityLike,
    #[serde(flatten)]
    pub mob_like: MobLike,
    #[serde(flatten)]
    pub bucketable_like: BucketableLike,
    pub Variant: IntTag<TropicalFishVariant>,
}

pub type TropicalFishVariant = i32; // <https://minecraft.wiki/w/Tropical_Fish#Entity_data>

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct Turtle {
    #[serde(flatten)]
    pub entity_like: EntityLike,
    #[serde(flatten)]
    pub mob_like: MobLike,
    #[serde(flatten)]
    pub breedable_like: BreedableLike,
    pub HasEgg: BooleanTag,
    pub HomePosX: IntTag,
    pub HomePosY: IntTag,
    pub HomePosZ: IntTag,
    pub TravelPosX: IntTag,
    pub TravelPosY: IntTag,
    pub TravelPosZ: IntTag,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct Vex {
    #[serde(flatten)]
    pub entity_like: EntityLike,
    #[serde(flatten)]
    pub mob_like: MobLike,
    pub BoundX: IntTag,
    pub BoundY: IntTag,
    pub BoundZ: IntTag,
    pub LifeTicks: IntTag,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct Villager {
    #[serde(flatten)]
    pub entity_like: EntityLike,
    #[serde(flatten)]
    pub mob_like: MobLike,
    #[serde(flatten)]
    pub villager_like: VillagerLike,
    #[serde(flatten)]
    pub breedable_like: BreedableLike,
    pub Inventory: ListTag<Item>, // 8 slots, with slot tag.
    pub LastRestock: LongTag,
    pub LastGossipDecay: LongTag,
    pub RestocksToday: IntTag,
    pub Willing: BooleanTag,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct Vindicator {
    #[serde(flatten)]
    pub entity_like: EntityLike,
    #[serde(flatten)]
    pub mob_like: MobLike,
    #[serde(flatten)]
    pub raid_like: RaidLike,
    pub Johnny: Option<BooleanTag>,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct WanderingTrader {
    #[serde(flatten)]
    pub entity_like: EntityLike,
    #[serde(flatten)]
    pub mob_like: MobLike,
    #[serde(flatten)]
    pub breedable_like: BreedableLike,
    pub DespawnDelay: IntTag,
    pub Inventory: ListTag<Item>, // 8 slots, with slot tag, unused
    pub Offers: Option<WanderingTraderOffers>,
    pub WanderTarget: WanderTarget,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct WanderingTraderOffers {
    pub Recipes: ListTag<TradeOptionLike>,
}

// Could be generalized to `Position` also.
#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct WanderTarget {
    pub X: IntTag,
    pub Y: IntTag,
    pub Z: IntTag,
}

#[derive(Serialize, Deserialize)]
pub struct Warden {
    #[serde(flatten)]
    pub entity_like: EntityLike,
    #[serde(flatten)]
    pub mob_like: MobLike,
    pub anger: WardenAnger,
}

#[derive(Serialize, Deserialize)]
pub struct WardenAnger {
    pub suspects: ListTag<WardenAngerSuspect>,
}

#[derive(Serialize, Deserialize)]
pub struct WardenAngerSuspect {
    pub anger: IntTag,
    pub uuid: IntArrayTag, // `UUIDLike`
}

#[derive(Serialize, Deserialize)]
pub struct Witch {
    #[serde(flatten)]
    pub entity_like: EntityLike,
    #[serde(flatten)]
    pub mob_like: MobLike,
    #[serde(flatten)]
    pub raid_like: RaidLike,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct Wither {
    #[serde(flatten)]
    pub entity_like: EntityLike,
    #[serde(flatten)]
    pub mob_like: MobLike,
    pub Invul: IntTag,
}

#[derive(Serialize, Deserialize)]
pub struct WitherSkeleton {
    #[serde(flatten)]
    pub entity_like: EntityLike,
    #[serde(flatten)]
    pub mob_like: MobLike,
}

#[derive(Serialize, Deserialize)]
pub struct Wolf {
    #[serde(flatten)]
    pub entity_like: EntityLike,
    #[serde(flatten)]
    pub mob_like: MobLike,
    #[serde(flatten)]
    pub breedable_like: BreedableLike,
    #[serde(flatten)]
    pub tameable_like: TameableLike,
    #[serde(flatten)]
    pub angered_like: AngeredLike,
    #[serde(flatten)]
    pub collared_like: CollaredLike,
    // v1.20.5
    // pub armor: BooleanTag,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct Zoglin {
    #[serde(flatten)]
    pub entity_like: EntityLike,
    #[serde(flatten)]
    pub mob_like: MobLike,
    pub isBaby: Option<BooleanTag>,
}

#[derive(Serialize, Deserialize)]
pub struct Zombie {
    #[serde(flatten)]
    pub entity_like: EntityLike,
    #[serde(flatten)]
    pub mob_like: MobLike,
    #[serde(flatten)]
    pub zombie_like: ZombieLike,
}

#[derive(Serialize, Deserialize)]
pub struct ZombieHorse {
    #[serde(flatten)]
    pub entity_like: EntityLike,
    #[serde(flatten)]
    pub mob_like: MobLike,
    #[serde(flatten)]
    pub breedable_like: BreedableLike,
    #[serde(flatten)]
    pub horse_like: HorseLike,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct ZombieVillager {
    #[serde(flatten)]
    pub entity_like: EntityLike,
    #[serde(flatten)]
    pub mob_like: MobLike,
    #[serde(flatten)]
    pub villager_like: VillagerLike,
    #[serde(flatten)]
    pub zombie_like: ZombieLike,
    pub ConversionTime: IntTag,
    pub ConcersionPlayer: IntArrayTag, // `UUIDLike`
}

#[derive(Serialize, Deserialize)]
pub struct ZombifiedPiglin {
    #[serde(flatten)]
    pub entity_like: EntityLike,
    #[serde(flatten)]
    pub mob_like: MobLike,
    #[serde(flatten)]
    pub angered_like: AngeredLike,
    #[serde(flatten)]
    pub zombie_like: ZombieLike,
}

#[derive(Serialize, Deserialize)]
pub struct Boat {
    #[serde(flatten)]
    pub entity_like: EntityLike,
    #[serde(flatten)]
    pub boat_like: BoatLike,
}

#[derive(Serialize, Deserialize)]
pub struct ChestBoat {
    #[serde(flatten)]
    pub entity_like: EntityLike,
    #[serde(flatten)]
    pub boat_like: BoatLike,
    #[serde(flatten)]
    pub container_entity_like: ContainerEntityLike,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct BoatLike {
    pub Type: StringTag<BoatType>,
}

// Is this `minecraft:`-prefixed like `CatVariant`?
#[allow(non_camel_case_types)]
#[derive(Serialize, Deserialize)]
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

#[derive(Serialize, Deserialize)]
pub struct Minecart {
    #[serde(flatten)]
    pub entity_like: EntityLike,
    #[serde(flatten)]
    pub minecart_like: MinecartLike,
}

#[derive(Serialize, Deserialize)]
pub struct ChestMinecart {
    #[serde(flatten)]
    pub entity_like: EntityLike,
    #[serde(flatten)]
    pub minecart_like: MinecartLike,
    #[serde(flatten)]
    pub container_entity_like: ContainerEntityLike,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct FurnaceMinecart {
    #[serde(flatten)]
    pub entity_like: EntityLike,
    #[serde(flatten)]
    pub minecart_like: MinecartLike,
    pub Fuel: ShortTag,
    pub PushX: DoubleTag,
    pub PushZ: DoubleTag,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct TNTMinecart {
    #[serde(flatten)]
    pub entity_like: EntityLike,
    #[serde(flatten)]
    pub minecart_like: MinecartLike,
    pub TNTFuse: IntTag,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct HopperMinecart {
    #[serde(flatten)]
    pub entity_like: EntityLike,
    #[serde(flatten)]
    pub minecart_like: MinecartLike,
    #[serde(flatten)]
    pub container_entity_like: ContainerEntityLike,
    pub Enabled: BooleanTag,
    pub TransferCooldown: IntTag<HopperMinecartTransferCooldown>, // is this deprecated, or rather removed? can no longer find it on the wiki
}

#[derive(Serialize, Deserialize)]
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

#[derive(Serialize, Deserialize)]
pub struct SpawnerMinecart {
    #[serde(flatten)]
    pub entity_like: EntityLike,
    #[serde(flatten)]
    pub minecart_like: MinecartLike,
    #[serde(flatten)]
    pub mob_spawner_like: MobSpawnerLike,
}

// Should this inherit from `./block-entity - CommandBlockLike` of some sort? The wiki doesn't do this, and I'm curious if the docs for this don't match the current NBT, since this one is missing some of the Command Block-ish ones.
#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct CommandBlockMinecart {
    #[serde(flatten)]
    pub entity_like: EntityLike,
    #[serde(flatten)]
    pub minecart_like: MinecartLike,
    pub Command: StringTag,
    pub LastOutput: StringTag,
    pub SuccessCount: IntTag,
    pub TrackOutput: BooleanTag,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct MinecartLike {
    pub CustomDisplayTile: Option<BooleanTag>,
    pub DisplayOffset: Option<IntTag>,
    pub DisplayState: Option<MinecartDisplayState>,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct MinecartDisplayState {
    pub Name: BlockResource,
    pub Properties: BlockState,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct ItemEntity {
    #[serde(flatten)]
    pub entity_like: EntityLike,
    pub Age: ShortTag,
    pub Health: ShortTag<ItemHealth>,
    pub Item: Item,
    pub Owner: Option<IntArrayTag>,
    pub PickupDelay: ShortTag,
    pub Thrower: Option<IntArrayTag>,
}

#[derive(Serialize, Deserialize)]
pub enum ItemHealth {
    Zero = 0,
    One,
    Two,
    Three,
    Four,
    Five,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct ExperienceOrb {
    #[serde(flatten)]
    pub entity_like: EntityLike,
    pub Age: ShortTag,
    pub Count: IntTag,
    pub Health: ShortTag,
    pub Value: ShortTag,
}

#[derive(Serialize, Deserialize)]
pub struct Arrow {
    #[serde(flatten)]
    pub entity_like: EntityLike,
    #[serde(flatten)]
    pub arrow_like: ArrowLike,
}

#[derive(Serialize, Deserialize)]
pub struct SpectralArrow {
    #[serde(flatten)]
    pub entity_like: EntityLike,
    #[serde(flatten)]
    pub arrow_like: ArrowLike,
}

// How can the potion effect types be optionally added/defined only for tipped arrows? Just with `extends Partial<PotionEffectLike>`?
#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct ArrowLike {
    // please move this to the use sites, rather than in this *-Like* type!!! <---------------------------------
    #[serde(flatten)]
    pub projectile_like: ProjectileLike, // <-------------------
    #[serde(flatten)]
    pub potion_effect_like: PotionEffectLike, // <----------------
    pub crit: BooleanTag,
    pub damage: DoubleTag,
    pub inBlockState: Option<ArrowBlockState>,
    pub inGround: BooleanTag,
    pub life: ShortTag,
    pub pickup: ByteTag<ArrowPickup>,
    pub PierceLevel: ByteTag,
    pub shake: ByteTag,
    pub ShotFromCrossbow: BooleanTag,
    pub SoundEvent: StringTag, // I don't think this is a SoundResource actually?
}

#[derive(Serialize, Deserialize)]
pub enum ArrowPickup {
    Immovable = 0,
    SurvivalOrCreative,
    Creative,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct ArrowBlockState {
    pub Name: StringTag<BlockResource>,
    pub Properties: Option<BlockState>,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct Trident {
    #[serde(flatten)]
    pub entity_like: EntityLike,
    #[serde(flatten)]
    pub arrow_like: ArrowLike,
    #[serde(flatten)]
    pub projectile_like: ProjectileLike,
    pub DealtDamage: BooleanTag,
    // I think the shape of this looks like this, the formatting on the wiki is a bit weird.
    pub Trident: TridentData,
}

#[derive(Serialize, Deserialize)]
pub struct TridentData {
    pub item: Item, // `minecraft:trident` Item, or `Item<"minecraft:trident">` essentially.
}

#[derive(Serialize, Deserialize)]
pub struct Snowball {
    #[serde(flatten)]
    pub entity_like: EntityLike,
    #[serde(flatten)]
    pub projectile_like: ProjectileLike,
    #[serde(flatten)]
    pub thrown_item_like: ThrownItemLike,
}

#[derive(Serialize, Deserialize)]
pub struct Egg {
    #[serde(flatten)]
    pub entity_like: EntityLike,
    #[serde(flatten)]
    pub projectile_like: ProjectileLike,
    #[serde(flatten)]
    pub thrown_item_like: ThrownItemLike,
}

#[derive(Serialize, Deserialize)]
pub struct LlamaSpit {
    #[serde(flatten)]
    pub entity_like: EntityLike,
    #[serde(flatten)]
    pub projectile_like: ProjectileLike,
}

#[derive(Serialize, Deserialize)]
pub struct EnderPearl {
    #[serde(flatten)]
    pub entity_like: EntityLike,
    #[serde(flatten)]
    pub projectile_like: ProjectileLike,
    #[serde(flatten)]
    pub thrown_item_like: ThrownItemLike,
}

#[derive(Serialize, Deserialize)]
pub struct EyeOfEnder {
    #[serde(flatten)]
    pub entity_like: EntityLike,
    #[serde(flatten)]
    pub thrown_item_like: ThrownItemLike,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct FireworkRocket {
    #[serde(flatten)]
    pub entity_like: EntityLike,
    #[serde(flatten)]
    pub projectile_like: ProjectileLike,
    pub FireworksItem: FireworksItem,
    pub Life: IntTag,
    pub LifeTime: IntTag,
    pub ShotAtAngle: BooleanTag,
}

// Is this an extension/generic of what would be `Item<"minecraft:firework_rocket">`, with additional Firework properties?
#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct FireworksItem {
    pub Count: ByteTag, // typically one
    pub id: StringTag,  // StringTag<ItemResource::firework_rocket>,
    pub tag: FireworkTag,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct FireworkTag {
    pub Fireworks: FireworkData, // optional? I don't think so, but the wiki wording is a little off.
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct FireworkData {
    pub Explosions: ListTag<FireworkExplosion>,
    pub Flight: ByteTag, // flight duration
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct FireworkExplosion {
    pub Colors: IntArrayTag,
    pub FadeColors: IntArrayTag,
    pub Flicker: Option<BooleanTag>,
    pub Trail: Option<BooleanTag>,
    pub Type: ByteTag<FireworkShape>,
}

#[derive(Serialize, Deserialize)]
pub enum FireworkShape {
    SmallBall = 0,
    LargeBall,
    Star,
    Creeper,
    Burst,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct TNT {
    #[serde(flatten)]
    pub entity_like: EntityLike,
    // looks like it has changed from one to the other at some point
    pub Fuse: ShortTag,
    pub fuse: ShortTag,
    // is this just `BlockState`? This gets confusing where they are nested, seems to be this way multiple other instances as well.
    pub block_state: TNTBlockState,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct TNTBlockState {
    pub Name: StringTag<BlockResource>,
    pub Properties: Option<BlockState>,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct FallingBlock {
    #[serde(flatten)]
    pub entity_like: EntityLike,
    // This is the same weird thing as `TNT`.
    pub BlockState: FallingBlockBlockState,
    pub CancelDrop: BooleanTag,
    pub DropItem: BooleanTag,
    pub FallHurtAmount: FloatTag,
    pub FallHurtMax: IntTag,
    pub HurtEntities: BooleanTag,
    pub TileEntityData: Option<BlockEntity>, // I'm pretty sure this is `BlockEntity`, but the wiki doesn't specifically mention it.
    pub Time: IntTag,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct FallingBlockBlockState {
    pub Name: StringTag<BlockResource>,
    pub Properties: Option<BlockState>,
}

#[derive(Serialize, Deserialize)]
pub struct FishingBobber {
    #[serde(flatten)]
    pub entity_like: EntityLike,
}

#[derive(Serialize, Deserialize)]
pub struct LightningBolt {
    #[serde(flatten)]
    pub entity_like: EntityLike,
}

#[derive(Serialize, Deserialize)]
pub struct LeashKnot {
    #[serde(flatten)]
    pub entity_like: EntityLike,
}

#[derive(Serialize, Deserialize)]
pub struct Painting {
    #[serde(flatten)]
    pub entity_like: EntityLike,
    #[serde(flatten)]
    pub hangable_like: HangableLike,
    pub variant: StringTag, // `PaintingVariant` union type
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct ItemFrame {
    #[serde(flatten)]
    pub entity_like: EntityLike,
    #[serde(flatten)]
    pub hangable_like: HangableLike,
    pub Fixed: BooleanTag,
    pub Invisible: BooleanTag,
    pub Item: Option<Item>,
    pub ItemDropChance: FloatTag,
    pub ItemRotation: ByteTag,
}

// Is `MobLike`, except for `LeftHanded`, `DeathLootTable`, `DeathLootTableSeed`, `NoAI`, `Leash`, `CanPickUpLoot` and `PersistenceRequired`.
#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct ArmorStand {
    #[serde(flatten)]
    pub entity_like: EntityLike,
    #[serde(flatten)]
    pub mob_like: MobLike,
    pub DisabledSlots: IntTag,
    pub Invisible: BooleanTag,
    pub Marker: Option<BooleanTag>,
    pub NoBasePlate: BooleanTag,
    pub Pose: ArmorStandPose,
    pub ShowArms: BooleanTag,
    pub Small: BooleanTag,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct ArmorStandPose {
    pub Body: ArmorStandPoseEntry,
    pub Head: ArmorStandPoseEntry,
    pub LeftArm: ArmorStandPoseEntry,
    pub LeftLeg: ArmorStandPoseEntry,
    pub RightArm: ArmorStandPoseEntry,
    pub RightLeg: ArmorStandPoseEntry,
}

pub type ArmorStandPoseEntry = [FloatTag; 3];

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct Fireball {
    #[serde(flatten)]
    pub entity_like: EntityLike,
    #[serde(flatten)]
    pub projectile_like: ProjectileLike,
    #[serde(flatten)]
    pub thrown_item_like: ThrownItemLike,
    #[serde(flatten)]
    pub fireball_like: FireballLike,
    pub ExplosionPower: ByteTag,
}

#[derive(Serialize, Deserialize)]
pub struct WitherSkull {
    #[serde(flatten)]
    pub entity_like: EntityLike,
    #[serde(flatten)]
    pub projectile_like: ProjectileLike,
    #[serde(flatten)]
    pub thrown_item_like: FireballLike,
    pub dangerous: BooleanTag, // might want to be optional <https://minecraft.wiki/w/Wither#cite_ref-11>
}

#[derive(Serialize, Deserialize)]
pub struct DragonFireball {
    #[serde(flatten)]
    pub entity_like: EntityLike,
    #[serde(flatten)]
    pub projectile_like: ProjectileLike,
    #[serde(flatten)]
    pub thrown_item_like: FireballLike,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct ShulkerBullet {
    #[serde(flatten)]
    pub entity_like: EntityLike,
    #[serde(flatten)]
    pub projectile_like: ProjectileLike,
    pub Steps: IntTag,
    pub Target: IntArrayTag, // `UUIDLike`, `IntArrayTag<[number, number, number, number]>`
    pub TXD: DoubleTag,
    pub TYD: DoubleTag,
    pub TZD: DoubleTag,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct EndCrystal {
    #[serde(flatten)]
    pub entity_like: EntityLike,
    pub BeamTarget: EndCrystalBeamTarget,
    pub ShowBottom: BooleanTag,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct EndCrystalBeamTarget {
    pub X: IntTag,
    pub Y: IntTag,
    pub Z: IntTag,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct EvokerFangs {
    #[serde(flatten)]
    pub entity_like: EntityLike,
    pub Owner: IntArrayTag, // `UUIDLike`
    pub Warmup: IntTag,
}

#[derive(Serialize, Deserialize)]
pub struct Marker {
    #[serde(flatten)]
    pub entity_like: EntityLike,
    pub data: (), // `unknown`/`any` // <https://minecraft.wiki/w/Marker#Entity_data>
}

#[derive(Serialize, Deserialize)]
pub struct ItemDisplay {
    #[serde(flatten)]
    pub entity_like: EntityLike,
    #[serde(flatten)]
    pub display_like: DisplayLike,
    pub item_display: StringTag<ItemDisplayModel>,
}

#[allow(non_camel_case_types)]
#[derive(Serialize, Deserialize)]
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

#[derive(Serialize, Deserialize)]
pub struct BlockDisplay {
    #[serde(flatten)]
    pub entity_like: EntityLike,
    #[serde(flatten)]
    pub display_like: DisplayLike,
    pub block_state: BlockState,
}

#[derive(Serialize, Deserialize)]
pub struct TextDisplay {
    #[serde(flatten)]
    pub entity_like: EntityLike,
    #[serde(flatten)]
    pub display_like: DisplayLike,
    pub alignment: StringTag<TextDisplayAlignment>,
    pub background: IntTag,
    pub default_background: BooleanTag,
    pub line_width: IntTag,
    pub see_through: BooleanTag,
    pub shadow: BooleanTag,
    pub text: StringTag, // raw JSON text <https://minecraft.wiki/w/Raw_JSON_text_format>
    pub text_opacity: ByteTag,
}

#[allow(non_camel_case_types)]
#[derive(Serialize, Deserialize)]
pub enum TextDisplayAlignment {
    center,
    left,
    right,
}

#[derive(Serialize, Deserialize)]
pub struct Interaction {
    #[serde(flatten)]
    pub entity_like: EntityLike,
    pub width: FloatTag,
    pub height: FloatTag,
    pub response: BooleanTag,
    pub attack: InteractionEvent,
    pub interaction: InteractionEvent,
}

#[derive(Serialize, Deserialize)]
pub struct InteractionEvent {
    pub player: IntArrayTag, // `UUIDLike`
    pub timestamp: LongTag,
}

#[allow(non_camel_case_types)]
#[derive(Serialize, Deserialize)]
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
