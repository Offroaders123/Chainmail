use serde::{Deserialize, Serialize};

use crate::{
    bedrock::{
        block::Block,
        dimension::DimensionID,
        effect::{Effect, EffectResource},
        item::Item,
        level::{Abilities, GameType},
    },
    nbt::tag::{BooleanTag, ByteTag, FloatTag, IntTag, ListTag, LongTag, ShortTag, StringTag},
};

#[allow(non_camel_case_types)]
#[derive(Serialize, Deserialize)]
pub enum Entity {
    minecart(Minecart),
    villager(LegacyVillager),
    villager_v2(Villager),
    blaze(Blaze),
    cave_spider(CaveSpider),
    creeper(Creeper),
    elder_guardian(ElderGuardian),
    ender_dragon(EnderDragon),
    enderman(Enderman),
    endermite(Endermite),
    evocation_illager(Evoker),
    ghast(Ghast),
    guardian(Guardian),
    magma_cube(MagmaCube),
    phantom(Phantom),
    piglin(Piglin),
    piglin_brute(PiglinBrute),
    pillager(Pillager),
    ravager(Ravager),
    skeleton(Skeleton),
    slime(Slime),
    silverfish(Sliverfish),
    spider(Spider),
    stray(Stray),
    vex(Vex),
    warden(Warden),
    wither(Wither),
    wither_skeleton(WitherSkeleton),
    zoglin(Zoglin),
    chest_minecart(ChestMinecart),
    command_block_minecart(CommandBlockMinecart),
    hopper_minecart(HopperMinecart),
    tnt_minecart(TNTMinecart),
    tnt(TNT),
    drowned(Drowned),
    husk(Husk),
    vindicator(Vindicator),
    witch(Witch),
    zombie(Zombie),
    zombie_villager(LegacyZombieVillager),
    zombie_villager_v2(ZombieVillager),
    zombie_pigman(ZombifiedPiglin),
    wither_skull(WitherSkull),
    allay(Allay),
    area_effect_cloud(AreaEffectCloud),
    bee(Bee),
    camel(Camel),
    cat(Cat),
    chicken(Chicken),
    cow(Cow),
    donkey(Donkey),
    dolphin(Dolphin),
    egg(Egg),
    item(ItemEntity),
    ender_crystal(EnderCrystal),
    fox(Fox),
    frog(Frog),
    glow_squid(GlowSquid),
    goat(Goat),
    hoglin(Hoglin),
    horse(Horse),
    llama(Llama),
    llama_spit(LlamaSpit),
    mooshroom(Mooshroom),
    mule(Mule),
    ocelot(Ocelot),
    painting(Painting),
    panda(Panda),
    parrot(Parrot),
    pig(Pig),
    polar_bear(PolarBear),
    rabbit(Rabbit),
    sheep(Sheep),
    skeleton_horse(SkeletonHorse),
    squid(Squid),
    sniffer(Sniffer),
    snowball(Snowball),
    strider(Strider),
    trader_llama(TraderLlama),
    turtle(Turtle),
    wolf(Wolf),
    zombie_horse(ZombieHorse),
    armor_stand(ArmorStand),
    bat(Bat),
    tripod_camera(Camera),
    iron_golem(IronGolem),
    npc(NPC),
    player(Player),
    shulker(Shulker),
    shulker_bullet(ShulkerBullet),
    wandering_trader(WanderingTrader),
    axolotl(Axolotl),
    cod(Cod),
    pufferfish(Pufferfish),
    salmon(Salmon),
    tadpole(Tadpole),
    tropicalfish(TropicalFish),
    arrow(Arrow),
    thrown_trident(Trident),
    xp_orb(ExperienceOrb),
    xp_bottle(ExperiencePotion),
    splash_potion(SplashPotion),
    lingering_potion(LingeringPotion),
    ender_pearl(EnderPearl),
    falling_block(FallingBlock),
    fireball(Fireball),
    fireworks_rocket(FireworkRocket),
    fishing_hook(FishingBobber),
    chest_boat(ChestBoat),
    // I think these just aren't complete yet
    // agent(Agent),
    // armadillo(Armadillo),
    // balloon(Balloon),
    // boat(Boat),
    // bogged(Bogged),
    // breeze(Breeze),
    // breeze_wind_charge_projectile(BreezeWindChargeProjectile),
    // chalkboard(Chalkboard),
    // dragon_fireball(DragonFireball),
    // elder_guardian_ghost(ElderGuardianGhost),
    // evocation_fang(EvocationFang),
    // eye_of_ender_signal(EyeOfEnderSignal),
    // firefly(Firefly), // should I omit this?
    // ice_bomb(IceBomb),
    // leash_knot(LeashKnot),
    // lightning_bolt(LightningBolt),
    // moving_block(MovingBlock),
    // shield(Shield),
    // small_fireball(SmallFireball),
    // snow_golem(SnowGolem),
    // wind_charge_projectile(WindChargeProjectile),
    // wither_skull_dangerous(WitherSkullDangerous),
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct Minecart {
    // extends EntityLike<EntityResource.minecart>
    CustomDisplayTile: Option<BooleanTag>,
    DisplayBlock: Option<Block>,
    DisplayOffset: Option<IntTag>,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct ChestMinecart {
    // extends EntityLike<EntityResource.chest_minecart>, InventoryLike<Item[]>
    // 0-26 slots
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct CommandBlockMinecart {
    // extends Omit<EntityLike<EntityResource.command_block_minecart>, "CustomName">, InventoryLike<Item[]>, CommandBlockLike
    // how many slots again?
    CurrentTickCount: IntTag,
    Ticking: ByteTag, // boolean?
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct HopperMinecart {
    // extends EntityLike<EntityResource.hopper_minecart>, InventoryLike<[Item?, Item?, Item?, Item?, Item?]>
    // 0-4 slots
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct TNTMinecart {
    // extends EntityLike<EntityResource.tnt_minecart>, ExplodeLike
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct LegacyVillager {
    // extends EntityLike<EntityResource.villager>, MobLike, VillagerLike
}

// The overlap types here for villager trade keys is a bit borked at the moment, maybe they need to be a union type? I feel like one is the old version, and one is the new version. That's just a hunch though. I'll have to dig into some older worlds' data to see what shapes they use there.
#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct Villager {
    // extends EntityLike<EntityResource.villager_v2>, MobLike, VillagerLike, AgeableLike, DwellerLike, EconomyTradeTableLike, InventoryLike<Item[]>, Omit<TradeTableLike, "Willing" | "Offers">
    // how many slots again?
    HasResupplied: BooleanTag,
    IsInRaid: BooleanTag,
    ReactToBell: ByteTag, // boolean?
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct Blaze {
    // extends EntityLike<EntityResource.blaze>, MobLike, MonsterLike
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct CaveSpider {
    // extends EntityLike<EntityResource.cave_spider>, MobLike, MonsterLike
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct Creeper {
    // extends EntityLike<EntityResource.creeper>, MobLike, MonsterLike, ExplodeLike
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct TNT {
    // extends EntityLike<EntityResource.tnt>, ExplodeLike
}

// Same fields as `Guardian`, make a `GuardianLike`?
#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct ElderGuardian {
    // extends EntityLike<EntityResource.elder_guardian>, MobLike, MonsterLike, HomeLike, TimerLike
    Elder: BooleanTag,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct EnderDragon {
    // extends EntityLike<EntityResource.ender_dragon>, MobLike, MonsterLike
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct Enderman {
    // extends EntityLike<EntityResource.enderman>, MobLike, MonsterLike
    carriedBlock: Option<Block>, // I think this is optional
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct Endermite {
    // extends EntityLike<EntityResource.endermite>, MobLike, MonsterLike
    Lifetime: IntTag,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct Evoker {
    // extends EntityLike<EntityResource.evocation_illager>, MobLike, MonsterLike, DwellerLike
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct Ghast {
    // extends EntityLike<EntityResource.ghast>, MobLike, MonsterLike
}

// Same fields as `ElderGuardian`, make a `GuardianLike`?
#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct Guardian {
    // extends EntityLike<EntityResource.guardian>, MobLike, MonsterLike, HomeLike, TimerLike
    Elder: BooleanTag,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct Drowned {
    // extends EntityLike<EntityResource.drowned>, MobLike, MonsterLike, HumanoidMonsterLike
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct Husk {
    // extends EntityLike<EntityResource.husk>, MobLike, MonsterLike, HumanoidMonsterLike, TimerLike
}

// VindicationIllager? / `vindication_illager`
#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct Vindicator {
    // extends EntityLike<EntityResource.vindicator>, MobLike, MonsterLike, HumanoidMonsterLike, DwellerLike
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct Witch {
    // extends EntityLike<EntityResource.witch>, MobLike, MonsterLike, HumanoidMonsterLike, DwellerLike
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct Zombie {
    // extends EntityLike<EntityResource.zombie>, MobLike, MonsterLike, HumanoidMonsterLike
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct LegacyZombieVillager {
    // extends EntityLike<EntityResource.zombie_villager>, MobLike, MonsterLike, HumanoidMonsterLike
    SpawnedFromVillage: BooleanTag,
}
// I think these are missing villager traits
#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct ZombieVillager {
    // extends EntityLike<EntityResource.zombie_villager_v2>, MobLike, MonsterLike, HumanoidMonsterLike
    SpawnedFromVillage: BooleanTag,
}

// Zombie Pigman, kinda
#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct ZombifiedPiglin {
    // extends EntityLike<EntityResource.zombie_pigman>, MobLike, MonsterLike, HumanoidMonsterLike
    Anger: ShortTag,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct MagmaCube {
    // extends EntityLike<EntityResource.magma_cube>, MobLike, MonsterLike
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct Phantom {
    // extends EntityLike<EntityResource.phantom>, MobLike, MonsterLike
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct Piglin {
    // extends EntityLike<EntityResource.piglin>, MobLike, MonsterLike, TimerLike, InventoryLike<Item[]>
    // how many slots again?
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct PiglinBrute {
    // extends EntityLike<EntityResource.piglin_brute>, MobLike, MonsterLike, HomeLike, TimerLike
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct Pillager {
    // extends EntityLike<EntityResource.pillager>, MobLike, MonsterLike, DwellerLike
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct Ravager {
    // extends EntityLike<EntityResource.ravager>, MobLike, MonsterLike, DwellerLike, TimerLike
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct Skeleton {
    // extends EntityLike<EntityResource.skeleton>, MobLike, MonsterLike, TimerLike
    ItemInHand: Item, // optional? not totally sure, I haven't seen a skeleton without an item, in Bedrock specifically
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct Slime {
    // extends EntityLike<EntityResource.slime>, MobLike, MonsterLike
    Size: ByteTag,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct Sliverfish {
    // extends EntityLike<EntityResource.silverfish>, MobLike, MonsterLike
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct Spider {
    // extends EntityLike<EntityResource.spider>, MobLike, MonsterLike
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct Stray {
    // extends EntityLike<EntityResource.stray>, MobLike, MonsterLike
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct Vex {
    // extends EntityLike<EntityResource.vex>, MobLike, MonsterLike
    ItemInHand: Item, // optional? same thing as Skeleton, I've never seen one empty-handed
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct Warden {
    // extends EntityLike<EntityResource.warden>, MobLike, MonsterLike
    Nuisances: ListTag<WardenNuisance>,
    VibrationListener: WardenVibrationListener,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct WardenNuisance {
    ActorId: LongTag,
    Anger: IntTag,
    Priority: BooleanTag,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct WardenVibrationListener {
    event: IntTag,
    pending: WardenVibrationListenerPending, // unknown
    selector: (),                            // object; // unknown
    ticks: IntTag,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct WardenVibrationListenerPending {
    distance: FloatTag,
    source: LongTag,
    vibration: IntTag,
    x: IntTag,
    y: IntTag,
    z: IntTag,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct Wither {
    // extends EntityLike<EntityResource.witch>, MobLike, MonsterLike
    AirAttack: ByteTag, // bool?
    dyingFrames: IntTag,
    firerate: FloatTag,
    Invul: IntTag,
    lastHealthInterval: IntTag,
    maxHealth: IntTag,
    oldSwellAmount: FloatTag,
    overlayAlpha: FloatTag,
    Phase: IntTag,
    ShieldHealth: IntTag,
    SpawningFrames: IntTag,
    swellAmount: FloatTag,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct WitherSkeleton {
    // extends EntityLike<EntityResource.wither_skeleton>, MobLike, MonsterLike
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct WitherSkull {
    // extends EntityLike<EntityResource.wither_skull>, ExplodeLike, ProjectileLike
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct Zoglin {
    // extends EntityLike<EntityResource.zoglin>, MobLike, MonsterLike
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct Allay {
    // extends EntityLike<EntityResource.allay>, MobLike
    AllayDuplicationCooldown: LongTag,
    VibrationListener: AllayVibrationListener,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct AllayVibrationListener {
    event: IntTag,                          // Probably a union type
    pending: AllayVibrationListenerPending, // unknown
    selector: (),                           // {}; // unknown
    ticks: IntTag,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct AllayVibrationListenerPending {
    distance: FloatTag,
    source: LongTag,
    vibration: IntTag,
    x: IntTag,
    y: IntTag,
    z: IntTag,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct AreaEffectCloud {
    // extends EntityLike<EntityResource.area_effect_cloud>
    Duration: IntTag,
    DurationOnUse: IntTag,
    InitialRadius: FloatTag,
    mobEffects: ListTag<Effect>,
    OwnerID: LongTag,
    ParticleColor: IntTag, // Union? Or color created by HEX code?
    ParticleId: IntTag,    // `ParticleID` (resource)? This is not `EffectID`
    PickupCount: IntTag,
    PotionId: ShortTag, // `PotionID`
    Radius: FloatTag,
    RadiusChangeOnPickup: FloatTag,
    RadiusOnUse: FloatTag,
    RadiusPerTick: FloatTag,
    ReapplicationDelay: IntTag,
    SpawnTick: LongTag,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct Bee {
    // extends EntityLike<EntityResource.bee>, MobLike, AgeableLike, BreedableLike, HomeLike, TimerLike
    properties: BeeProperties,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct BeeProperties {
    #[serde(rename = "minecraft:has_nectar")]
    has_nectar: BooleanTag,
}

// like `Horse`, should I make a `HorseLike`?
#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct Camel {
    // extends EntityLike<EntityResource.camel>, MobLike, AgeableLike, BreedableLike, InventoryLike<Item[]>
    // is all of `InventoryLike` unused? camels can't have chest inventory items
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct Cat {
    // extends EntityLike<EntityResource.cat>, MobLike, AgeableLike, BreedableLike, BreedableLike, DwellerLike
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct Chicken {
    // extends EntityLike<EntityResource.chicken>, MobLike, AgeableLike, BreedableLike
    entries: ListTag<ChickenEntry>,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct ChickenEntry {
    SpawnTimer: IntTag,
    StopSpawning: ByteTag, // boolean?
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct Cow {
    // extends EntityLike<EntityResource.cow>, MobLike, AgeableLike, BreedableLike
}

// like `Horse`, should I make a `HorseLike`?
#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct Donkey {
    // extends EntityLike<EntityResource.donkey>, MobLike, AgeableLike, BreedableLike, InventoryLike<Item[]>
    // how many slots again?
    Temper: IntTag,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct Dolphin {
    // extends EntityLike<EntityResource.dolphin>, MobLike, AgeableLike, DryingOutTimerLike
    BribeTime: IntTag,
    DamageTime: Option<ShortTag>,
    TicksRemainingUntilDryOut: IntTag,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct Egg {
    // extends EntityLike<EntityResource.egg>, ProjectileLike
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct ItemEntity {
    // extends EntityLike<EntityResource.item>
    Age: ShortTag,
    Health: ShortTag,
    Item: Item,
    OwnerID: Option<LongTag>,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct EnderCrystal {
    // extends EntityLike<EntityResource.ender_crystal>, ExplodeLike
    BlockTargetX: Option<IntTag>,
    BlockTargetY: Option<IntTag>,
    BlockTargetZ: Option<IntTag>,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct Fox {
    // extends EntityLike<EntityResource.fox>, MobLike, AgeableLike, BreedableLike
    TrustedPlayersAmount: IntTag,
    // Not quite sure how to type this
    // "TrustedPlayer${number}": LongTag, // <------------
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct Frog {
    // extends EntityLike<EntityResource.frog>, MobLike, BreedableLike
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct GlowSquid {
    // extends EntityLike<EntityResource.glow_squid>, MobLike
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct Goat {
    // extends EntityLike<EntityResource.goat>, MobLike, AgeableLike, BreedableLike, GeneticsLike
    GoatHornCount: IntTag,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct Hoglin {
    // extends EntityLike<EntityResource.hoglin>, MobLike, AgeableLike, BreedableLike, TimerLike
}

// ~~horse v2 as well~~, wait, those are just the texture names, not the IDs themselves
// like `Donkey`, should I make a `HorseLike`?
#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct Horse {
    // extends EntityLike<EntityResource.horse>, MobLike, AgeableLike, BreedableLike, InventoryLike<Item[]>
    // how many slots again?
    Temper: IntTag,
}

// like `Horse`, should I make a `HorseLike`?
#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct Llama {
    // extends EntityLike<EntityResource.llama>, MobLike, AgeableLike, BreedableLike, InventoryLike<Item[]>
    // how many slots again?
    Temper: IntTag,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct LlamaSpit {
    // extends EntityLike<EntityResource.llama_spit>, ProjectileLike
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct Mooshroom {
    // extends EntityLike<EntityResource.mooshroom>, MobLike, AgeableLike, BreedableLike
}

// like `Horse`, should I make a `HorseLike`?
#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct Mule {
    // extends EntityLike<EntityResource.mule>, MobLike, AgeableLike, InventoryLike<Item[]>
    // how many slots again?
    Temper: IntTag,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct Ocelot {
    // extends EntityLike<EntityResource.ocelot>, MobLike, AgeableLike, BreedableLike
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct Painting {
    // extends EntityLike<EntityResource.painting>
    Dir: ByteTag<PaintingDirection>,
    Direction: ByteTag,        // unknown what the difference between these two are
    Motive: Option<StringTag>, // `PaintingResource`
}

#[allow(non_camel_case_types)]
#[derive(Serialize, Deserialize)]
pub enum PaintingDirection {
    // this could be renamed eventually
    Zero = 0,
    One,
    Two,
    Three,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct Panda {
    // extends EntityLike<EntityResource.panda>, MobLike, AgeableLike, BreedableLike, GeneticsLike, InventoryLike<Item[]>
    // how many slots again?
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct Parrot {
    // extends EntityLike<EntityResource.parrot>, MobLike
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct Pig {
    // extends EntityLike<EntityResource.pig>, MobLike, AgeableLike, BreedableLike
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct PolarBear {
    // extends EntityLike<EntityResource.polar_bear>, MobLike, AgeableLike
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct Rabbit {
    // extends EntityLike<EntityResource.rabbit>, MobLike, AgeableLike, BreedableLike
    CarrotsEaten: IntTag,
    MoreCarrotTicks: IntTag,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct Sheep {
    // extends EntityLike<EntityResource.sheep>, MobLike, AgeableLike, BreedableLike
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct SkeletonHorse {
    // extends EntityLike<EntityResource.skeleton_horse>, MobLike, AgeableLike
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct Squid {
    // extends EntityLike<EntityResource.squid>, MobLike
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct Sniffer {
    // extends EntityLike<EntityResource.sniffer>, MobLike, AgeableLike, BreedableLike
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct Snowball {
    // extends EntityLike<EntityResource.snowball>, ProjectileLike
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct Strider {
    // extends EntityLike<EntityResource.strider>, MobLike, AgeableLike, BreedableLike
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct TraderLlama {
    // extends EntityLike<EntityResource.trader_llama>, MobLike
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct Turtle {
    // extends EntityLike<EntityResource.turtle>, MobLike, AgeableLike, BreedableLike, HomeLike
    IsPregnant: BooleanTag,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct Wolf {
    // extends EntityLike<EntityResource.wolf>, MobLike, AgeableLike, BreedableLike
    // v1.20.50
    // properties: {
    //   "minecraft:has_armor": BooleanTag,
    //   "minecraft:is_armorable": BooleanTag,
    // },
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct ZombieHorse {
    // extends EntityLike<EntityResource.zombie_horse>, MobLike, AgeableLike
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct ArmorStand {
    // extends EntityLike<EntityResource.armor_stand>, MobLike
    Pose: ArmorStandPose,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct ArmorStandPose {
    LastSignal: IntTag, // union?
    PoseIndex: IntTag,  // union?
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct Bat {
    // extends EntityLike<EntityResource.bat>, MobLike
    BatFlags: BooleanTag,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct Camera {
    // extends EntityLike<EntityResource.tripod_camera>, MobLike
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct IronGolem {
    // extends EntityLike<EntityResource.iron_golem>, MobLike, DwellerLike
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct NPC {
    // extends EntityLike<EntityResource.npc>, MobLike
    Actions: Option<StringTag>,
    InteractiveText: Option<StringTag>,
    PlayerSceneMapping: Option<ListTag<NPCPlayerSceneMap>>,
    RawtextName: Option<StringTag>,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct NPCPlayerSceneMap {
    PlayerID: LongTag,
    SceneName: StringTag,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct Player {
    // extends EntityLike<EntityResource.player>, MobLike, AbilitiesLike
    AgentID: Option<LongTag>,
    DimensionId: IntTag<DimensionID>,
    EnchantmentSeed: IntTag,
    EnderChestInventory: ListTag<Item>,  // 0-26, with slot tag
    fogCommandStack: ListTag<StringTag>, // `FogResource`? I think the fogs can be player-made though, like scoreboards? Maybe not,..
    format_version: StringTag,
    HasSeenCredits: BooleanTag,
    Inventory: ListTag<Item>, // 0-35, with slot tag
    LeftShoulderRiderID: Option<LongTag>,
    MapIndex: IntTag,
    PlayerGameMode: IntTag<GameType>,
    PlayerLevel: IntTag,
    PlayerLevelProgress: FloatTag,
    PlayerUIItems: ListTag<Item>, // unknown
    recipe_unlocking: PlayerRecipeUnlocking,
    RideID: Option<LongTag>,
    RightShoulderRiderID: Option<LongTag>,
    SelectedContainerId: IntTag, // optional?* I don't think so, but I don't fully know what this is for, so I'm gonna leave it for now
    SelectedInventorySlot: IntTag, // 0-8? I don't remember the hotbar number indices, might be wrong there
    Sleeping: BooleanTag,
    SleepTimer: ShortTag,
    Sneaking: BooleanTag,
    SpawnBlockPositionX: IntTag,
    SpawnBlockPositionY: IntTag,
    SpawnBlockPositionZ: IntTag,
    SpawnDimension: IntTag<DimensionID>,
    SpawnX: IntTag,
    SpawnY: IntTag,
    SpawnZ: IntTag,
    TimeSinceRest: IntTag,
    WardenThreatDecreaseTimer: IntTag,
    WardenThreatLevel: IntTag<WardenThreatLevel>,
    WardenThreatLevelIncreaseCooldown: IntTag,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct PlayerRecipeUnlocking {
    unlocked_recipes: ListTag<StringTag>, // `Item["id"]`, or rather actually, `RecipeResource`
    used_contexts: IntTag, // maybe a union type of different crafting interface types
}

#[allow(non_camel_case_types)]
#[derive(Serialize, Deserialize)]
pub enum WardenThreatLevel {
    Zero = 0,
    One,
    Two,
    Three,
    Four,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct Shulker {
    // extends EntityLike<EntityResource.shulker>, MobLike
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct ShulkerBullet {
    // extends EntityLike<EntityResource.shulker_bullet>, MobLike, ProjectileLike
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct WanderingTrader {
    // extends EntityLike<EntityResource.wandering_trader>, MobLike, EconomyTradeTableLike, TimerLike
    // This is essentially the same as `Chicken`, it can be abstracted into it's own type
    entries: ListTag<WanderingTraderEntry>,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct WanderingTraderEntry {
    SpawnTimer: IntTag,
    StopSpawning: ByteTag, // boolean?
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct Axolotl {
    // extends EntityLike<EntityResource.axolotl>, MobLike, AgeableLike, BreedableLike, DryingOutTimerLike
    DamageTime: Option<ShortTag>,
    TicksRemainingUntilDryOut: IntTag,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct Cod {
    // extends EntityLike<EntityResource.cod>, MobLike
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct Pufferfish {
    // extends EntityLike<EntityResource.pufferfish>, MobLike, TimerLike
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct Salmon {
    // extends EntityLike<EntityResource.salmon>, MobLike
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct Tadpole {
    // extends EntityLike<EntityResource.tadpole>, MobLike, AgeableLike
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct TropicalFish {
    // extends EntityLike<EntityResource.tropicalfish>, MobLike
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct Arrow {
    // extends EntityLike<EntityResource.arrow>, ProjectileLike, ArrowLike
    auxValue: ByteTag<ArrowType>,
    enchantFlame: ByteTag<ArrowEnchantFlame>,
    enchantInfinity: ByteTag<ArrowEnchantInfinity>,
    enchantPower: ByteTag<ArrowEnchantPower>,
    enchantPunch: ByteTag<ArrowEnchantPunch>,
    mobEffects: ListTag<Effect>,
}

#[allow(non_camel_case_types)]
#[derive(Serialize, Deserialize)]
pub enum ArrowEnchantFlame {
    Zero = 0,
    One,
}

#[allow(non_camel_case_types)]
#[derive(Serialize, Deserialize)]
pub enum ArrowEnchantInfinity {
    Zero = 0,
    One,
}

#[allow(non_camel_case_types)]
#[derive(Serialize, Deserialize)]
pub enum ArrowEnchantPower {
    Zero = 0,
    One,
    Two,
    Three,
    Four,
    Five,
}

#[allow(non_camel_case_types)]
#[derive(Serialize, Deserialize)]
pub enum ArrowEnchantPunch {
    Zero = 0,
    One,
    Two,
}

#[allow(non_camel_case_types)]
#[derive(Serialize, Deserialize)]
pub enum ArrowType {
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
    TwentyOne,
    TwentyTwo,
    TwentyThree,
    TwentyFour,
    TwentyFive,
    TwentySix,
    TwentySeven,
    TwentyEight,
    TwentyNine,
    Thirty,
    ThirtyOne,
    ThirtyTwo,
    ThirtyThree,
    ThirtyFour,
    ThirtyFive,
    ThirtySix,
    ThirtySeven,
    ThirtyEight,
    ThirtyNine,
    Forty,
    FortyOne,
    FortyTwo,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct Trident {
    // extends EntityLike<EntityResource.thrown_trident>, ArrowLike, ProjectileLike
    favoredSlot: IntTag, // union range of hotbar slots? or can it be any slot technically?
    Trident: Item,       // `Item<"trident">`
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct ExperienceOrb {
    // extends EntityLike<EntityResource.xp_bottle>, ThrowableLike
    Age: ShortTag,
    #[serde(rename = "experience value")]
    experience_value: IntTag,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct ExperiencePotion {
    // extends EntityLike<EntityResource.xp_bottle>, ThrowableLike, ProjectileLike
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct SplashPotion {
    // extends EntityLike<EntityResource.splash_potion>, ThrowableLike, ProjectileLike
    PotionId: ShortTag<EffectResource>,
}
// `PotionLike`? ^^vv^^vv this is `Thrown Potion` from the wiki
#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct LingeringPotion {
    // extends EntityLike<EntityResource.lingering_potion>, ThrowableLike, ProjectileLike
    PotionId: ShortTag<EffectResource>,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct EnderPearl {
    // extends EntityLike<EntityResource.ender_pearl>, ThrowableLike, ProjectileLike
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct FallingBlock {
    // extends EntityLike<EntityResource.falling_block>
    FallingBlock: Block,
    Time: ByteTag,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct Fireball {
    // extends EntityLike<EntityResource.fireball>, ExplodeLike, ProjectileLike
    Direction: [FloatTag; 3],
    inGround: ByteTag, // boolean?
    power: [FloatTag; 3],
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct FireworkRocket {
    // extends EntityLike<EntityResource.fireworks_rocket>
    // I feel like this is missing metadata about what kind of firework shape it is, I should demo this
    Life: IntTag,
    LifeTime: IntTag,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct FishingBobber {
    // extends EntityLike<EntityResource.fishing_hook>, ProjectileLike
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct ChestBoat {
    // extends EntityLike<EntityResource.chest_boat>, InventoryLike<Item[]>
    // 0-26 slots
}

// pub struct Agent {}

// pub struct Armadillo {}

// pub struct Balloon {}

// pub struct Boat {}

// pub struct Bogged {}

// pub struct Breeze {}

// pub struct BreezeWindChargeProjectile {}

// pub struct Chalkboard {}

// pub struct DragonFireball {}

// pub struct ElderGuardianGhost {}

// pub struct EvocationFang {}

// pub struct EyeOfEnderSignal {}

// should I omit this?
// pub struct Firefly {}

// pub struct IceBomb {}

// pub struct LeashKnot {}

// pub struct LightningBolt {}

// pub struct MovingBlock {}

// pub struct Shield {}

// pub struct SmallFireball {}

// pub struct SnowGolem {}

// pub struct WindChargeProjectile {}

// pub struct WitherSkullDangerous {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct VillagerLike {
    Willing: BooleanTag,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct MonsterLike {
    SpawnedByNight: BooleanTag,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct HumanoidMonsterLike {
    ItemInHand: Option<Item>, // double-check this is correct, once `Item` is implemented
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct MobLike {
    ActiveEffects: Option<ListTag<Effect>>,
    Air: ShortTag,
    Armor: [Item; 4], // these actually appear to not be optional, and they are just Air items by default when there is no value
    AttackTime: ShortTag,
    Attributes: ListTag<Attribute>,
    BodyRot: Option<FloatTag>,
    boundX: IntTag,
    boundY: IntTag,
    boundZ: IntTag,
    canPickupItems: BooleanTag,
    Dead: BooleanTag,
    DeathTime: ShortTag,
    hasBoundOrigin: BooleanTag,
    hasSetCanPickupItems: BooleanTag, // if `canPickupItems` was set by the game
    HurtTime: ShortTag,
    LeasherID: LongTag,
    limitedLife: LongTag,
    Mainhand: [Item; 1], // I don't think this one is optional, and it defaults to Air if nothing is being held.
    NaturalSpawn: BooleanTag,
    Offhand: [Item; 1], // Same as the `Mainhand`
    persistingOffers: Option<PersistingOffers>,
    persistingRiches: Option<IntTag>,
    Surface: BooleanTag,
    TargetCaptainID: Option<LongTag>,
    TargetID: LongTag, // optional I think? the wiki doesn't say* Not totally sure but I think not, looks like it's set to -1 otherwise.
    TradeExperience: Option<IntTag>,
    TraderTier: Option<IntTag>, // union? 'trade tier of this trader entity'. Maybe it's just a value though.
    WantsToBeJockey: Option<ByteTag>, // guessing it's a boolean
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct Attribute {
    Base: FloatTag,
    Current: FloatTag,
    DefaultMax: FloatTag,
    DefaultMin: FloatTag,
    Max: FloatTag,
    Min: FloatTag,
    Modifiers: Option<ListTag<AttributeModifier>>,
    Name: StringTag, // `AttributeResource`
    TemporalBuffs: ListTag<TemporalBuff>,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct AttributeModifier {
    Amount: FloatTag,
    Name: StringTag, // `ModifierResource`
    Operand: IntTag,
    Operation: IntTag,
    UUIDLeast: LongTag,
    UUIDMost: LongTag,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct TemporalBuff {
    Amount: FloatTag,
    Duration: IntTag,
    LifeTime: IntTag,
    Type: IntTag, // probably a union, not documented
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct PersistingOffers {} // not documented

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct AbilitiesLike {
    abilities: Abilities,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct ArrowLike {
    isCreative: BooleanTag,
    OwnerID: LongTag,
    player: BooleanTag,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct ProjectileLike {
    TargetID: LongTag,
    StuckToBlockPos: [IntTag; 3],
    CollisionPos: [FloatTag; 3],
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct ThrowableLike {
    inGround: BooleanTag, // boolean?
    OwnerID: LongTag,
    shake: BooleanTag, // boolean?
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct AgeableLike {
    Age: IntTag,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct BreedableLike {
    InLove: IntTag,
    LoveCause: LongTag,
    BreedCooldown: IntTag,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct DryingOutTimerLike {
    CompleteTick: LongTag,
    State: IntTag<DryingOutTimerState>, // is a boolean, but Bedrock be Bedrock
}

#[allow(non_camel_case_types)]
#[derive(Serialize, Deserialize)]
pub enum DryingOutTimerState {
    Zero = 0,
    One,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct HomeLike {
    HomePos: [FloatTag; 3],
    HomeDimensionId: IntTag<DimensionID>,
}

// <Items extends (Item | undefined)[]>
#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct InventoryLike {
    ChestItems: ListTag<Option<Item>>, // with slot tag
    InventoryVersion: StringTag,
    LootTable: Option<StringTag>,  // optional like JE?
    LootTableSeed: Option<IntTag>, // optional like JE?
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct TimerLike {
    TimeStamp: LongTag,
    HasExecuted: ByteTag, // boolean?
    CountTime: IntTag,    // deprecated..
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct DwellerLike {
    DwellingUniqueID: StringTag,
    RewardPlayersOnFirstFounding: ByteTag, // boolean?
    PreferredProfession: StringTag,        // hmm? *lol, didn't mean for that to be a pun hahaha*
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct EconomyTradeTableLike {
    Riches: IntTag,
    Offers: Option<ListTag<TradeOffer>>,
    ConvertedFromVillagerV1: Option<BooleanTag>,
    TradeTablePath: Option<StringTag>,
    LowTierCuredDiscount: Option<IntTag>,
    HighTierCuredDiscount: Option<IntTag>,
    NearbyCuredDiscount: Option<IntTag>,
    NearbyCuredDiscountTimeStamp: Option<IntTag>,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct TradeOffer {
    Recipes: ListTag<TradeRecipe>,
    TierExpRequirements: ListTag<TradeOfferExpTier>,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct TradeRecipe {
    buyA: Item,
    buyB: Option<Item>,
    sell: Item,
    tier: IntTag, // union?
    uses: IntTag,
    maxUses: IntTag,
    traderExp: IntTag,
    rewardExp: BooleanTag,
    demand: IntTag,
    buyCountA: IntTag,
    buyCountB: IntTag,
    priceMultiplierA: FloatTag,
    priceMultiplierB: FloatTag,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct TradeOfferExpTier {
    // This would be a type `[`${number}`]`
    #[serde(rename = "<tier_level_num>")]
    tier_level_num: IntTag,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct TradeTableLike {
    sizeOfTradeFirstTimeVector: IntTag,
    FirstTimeTrade: Option<IntTag>,
    TradeTier: IntTag,
    Riches: IntTag,
    Willing: ByteTag,            // boolean?
    Offers: Option<ListTag<()>>, // object[]; // unknown
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct ExplodeLike {
    // optional..?
    Fuse: Option<ByteTag>,
    IsFuseLit: Option<ByteTag>,
    AllowUnderwater: ByteTag,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct GeneticsLike {
    GeneArray: ListTag<GeneticsGene>,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct GeneticsGene {
    // I think these possible could be typed as unions, is for Goat & friends' breeding types
    HiddenAllele: IntTag,
    MainAllele: IntTag,
}

// <EntityID extends string>
#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct EntityLike {
    Chested: BooleanTag,
    Color: ByteTag,
    Color2: ByteTag,
    CustomName: Option<StringTag>,
    CustomNameVisible: Option<BooleanTag>,
    definitions: ListTag<StringTag>, // needs more type info, I think this is a `DefinitionResource` kind of thing
    FallDistance: FloatTag,
    Fire: ShortTag,
    identifier: StringTag, // `StringTag<EntityResource>` or, what do you think? // `${EntityID}`,
    internalComponents: EntityInternalComponents,
    Invulnerable: BooleanTag,
    IsAngry: BooleanTag,
    IsAutonomous: BooleanTag,
    IsBaby: BooleanTag,
    IsEating: BooleanTag,
    IsGliding: BooleanTag,
    IsGlobal: BooleanTag,
    IsIllagerCaptain: BooleanTag,
    IsOrphaned: BooleanTag,
    IsOutOfControl: BooleanTag,
    IsRoaring: BooleanTag,
    IsScared: BooleanTag,
    IsStunned: BooleanTag,
    IsSwimming: BooleanTag,
    IsTamed: BooleanTag,
    IsTrusting: BooleanTag,
    LastDimensionId: Option<IntTag<DimensionID>>,
    LinksTag: Option<EntityLinksTag>,
    LootDropped: BooleanTag,
    MarkVariant: IntTag,
    Motion: Option<[FloatTag; 3]>,
    OnGround: BooleanTag,
    OwnerNew: LongTag,
    Persistent: BooleanTag,
    PortalCooldown: IntTag,
    Pos: [FloatTag; 3],
    Rotation: [FloatTag; 2],
    Saddled: BooleanTag,
    Sheared: BooleanTag,
    ShowBottom: BooleanTag,
    Sitting: BooleanTag,
    SkinID: IntTag,
    Strength: IntTag,
    StrengthMax: IntTag,
    Tags: ListTag<StringTag>, // Scoreboard tags
    UniqueID: LongTag,
    Variant: IntTag,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct EntityInternalComponents {
    EntityStorageKeyComponent: EntityStorageKeyComponent,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct EntityStorageKeyComponent {
    StorageKey: StringTag, // This is a reference to the entity's Actor storage key.
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct EntityLinksTag {
    entityId: LongTag,
    LinkID: IntTag,
}

// Prefix these with `minecraft:` when serialized
#[allow(non_camel_case_types)]
#[derive(Serialize, Deserialize)]
pub enum EntityResource {
    item,
    xp_orb,
    tnt,
    falling_block,
    moving_block,
    armor_stand,
    xp_bottle,
    eye_of_ender_signal,
    ender_crystal,
    fireworks_rocket,
    thrown_trident,
    shulker_bullet,
    fishing_hook,
    dragon_fireball,
    arrow,
    snowball,
    egg,
    painting,
    minecart,
    fireball,
    splash_potion,
    ender_pearl,
    leash_knot,
    wither_skull,
    boat,
    wither_skull_dangerous,
    lightning_bolt,
    small_fireball,
    area_effect_cloud,
    hopper_minecart,
    tnt_minecart,
    chest_minecart,
    command_block_minecart,
    lingering_potion,
    llama_spit,
    evocation_fang,
    ice_bomb,
    balloon,
    chest_boat,
    zombie,
    creeper,
    skeleton,
    spider,
    zombie_pigman,
    slime,
    enderman,
    silverfish,
    cave_spider,
    ghast,
    magma_cube,
    blaze,
    zombie_villager,
    witch,
    stray,
    husk,
    wither_skeleton,
    guardian,
    elder_guardian,
    wither,
    ender_dragon,
    shulker,
    endermite,
    vindicator,
    phantom,
    ravager,
    evocation_illager,
    vex,
    drowned,
    pillager,
    zombie_villager_v2,
    piglin,
    hoglin,
    zoglin,
    piglin_brute,
    warden,
    chicken,
    cow,
    pig,
    sheep,
    wolf,
    villager,
    mooshroom,
    squid,
    glow_squid,
    rabbit,
    bat,
    iron_golem,
    snow_golem,
    ocelot,
    horse,
    donkey,
    mule,
    skeleton_horse,
    zombie_horse,
    polar_bear,
    llama,
    trader_llama,
    parrot,
    dolphin,
    turtle,
    cat,
    pufferfish,
    salmon,
    tropicalfish,
    cod,
    panda,
    villager_v2,
    wandering_trader,
    fox,
    bee,
    strider,
    goat,
    axolotl,
    frog,
    tadpole,
    allay,
    player,
    shield,
    elder_guardian_ghost,
    npc,
    agent,
    tripod_camera,
    chalkboard,
    sniffer,
    camel,
}
