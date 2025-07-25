// export type Entity<K extends keyof EntityNameMap = keyof EntityNameMap> = EntityNameMap[K];

// export type EntityNameMap = {
//   // placeholder
//   [K in EntityResource]: EntityLike<K>;
// };

use serde::{Deserialize, Serialize};

use crate::nbt::tag::StringTag;

// <EntityID extends string>
#[derive(Serialize, Deserialize)]
pub struct EntityLike {
    id: StringTag, // Should this instead be `StringTag<EntityResource>`, and or generic?
}

#[allow(non_camel_case_types)]
#[derive(Serialize, Deserialize)]
pub enum EntityResource {
    AreaEffectCloud,
    ArmorStand,
    Arrow,
    Bat,
    Blaze,
    Boat,
    CaveSpider,
    Chicken,
    Cow,
    Creeper,
    DragonFireball,
    EnderCrystal,
    EnderDragon,
    Enderman,
    Endermite,
    EntityHorse,
    EyeOfEnderSignal,
    FallingSand,
    Fireball,
    FireworksRocketEntity,
    Ghast,
    Giant,
    Guardian,
    Item,
    ItemFrame,
    LavaSlime,
    LeashKnot,
    Minecart,
    MinecartChest,
    MinecartFurnace,
    MinecartHopper,
    MinecartRideable,
    MinecartSpawner,
    MinecartTNT,
    MushroomCow,
    Ozelot,
    Painting,
    Pig,
    PigZombie,
    PrimedTnt,
    Rabbit,
    Sheep,
    ShulkerBullet,
    Silverfish,
    Skeleton,
    Slime,
    SmallFireball,
    SnowMan,
    Snowball,
    Spider,
    Squid,
    ThrownEgg,
    ThrownEnderpearl,
    ThrownExpBottle,
    ThrownPotion,
    Villager,
    VillagerGolem,
    Witch,
    Wither,
    WitherBoss,
    WitherSkull,
    Wolf,
    XPOrb,
    Zombie,
    area_effect_cloud,
    armor_stand,
    arrow,
    bat,
    blaze,
    boat,
    cave_spider,
    chest_minecart,
    chicken,
    cow,
    creeper,
    dolphin,
    donkey,
    dragon_fireball,
    drowned,
    egg,
    elder_guardian,
    ender_crystal,
    ender_dragon,
    ender_pearl,
    enderman,
    endermite,
    evocation_fangs,
    evocation_illager,
    eye_of_ender_signal,
    falling_block,
    fireball,
    fireworks_rocket,
    fish,
    furnace_minecart,
    ghast,
    giant,
    guardian,
    hopper_minecart,
    horse,
    husk,
    item,
    item_frame,
    leash_knot,
    llama,
    llama_spit,
    magma_cube,
    minecart,
    mooshroom,
    mule,
    ocelot,
    painting,
    parrot,
    phantom,
    pig,
    polar_bear,
    potion,
    pufferfish,
    rabbit,
    salmon,
    sheep,
    shulker,
    shulker_bullet,
    silverfish,
    skeleton,
    skeleton_horse,
    slime,
    small_fireball,
    snowball,
    snowman,
    spawner_minecart,
    spider,
    squid,
    stray,
    tnt,
    tnt_minecart,
    trident,
    tropicalfish,
    turtle,
    vex,
    villager,
    villager_golem,
    vindication_illager,
    witch,
    wither,
    wither_skeleton,
    wither_skull,
    wolf,
    xp_bottle,
    xp_orb,
    zombie,
    zombie_horse,
    zombie_pigman,
    zombie_villager,
}
