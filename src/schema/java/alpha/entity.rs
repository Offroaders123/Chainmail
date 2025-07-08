use serde::{Deserialize, Serialize};

use crate::java::alpha::{
    block::BlockResource,
    dimension::DimensionID,
    item::{Item, SlottedItem},
};
use crate::nbt::tag::{
    BooleanTag, ByteTag, DoubleTag, FloatTag, IntTag, ListTag, ShortTag, StringTag,
};

#[derive(Serialize, Deserialize)]
pub enum Entity {
    // Player(Player), // I'm speculating; not sure where this should be defined
    Arrow(Arrow),
    Boat(Boat),
    Chicken(Chicken),
    Cow(Cow),
    Creeper(Creeper),
    Egg(Egg),
    FallingSand(FallingSand),
    Ghast(Ghast),
    Giant(Giant),
    Item(ItemEntity),
    // Mob(Mob), // what's this? player?
    // Monster(Monster), // what's this too?
    Minecart(Minecart),
    Painting(Painting),
    Pig(Pig),
    PigZombie(PigZombie),
    PrimedTnt(PrimedTnt),
    Sheep(Sheep),
    Skeleton(Skeleton),
    Slime(Slime),
    Snowball(Snowball),
    Spider(Spider),
    Zombie(Zombie),
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct EntityLike {
    pub id: StringTag, // should this be generic to `EntityResource`?
    pub Pos: [DoubleTag; 3],
    pub Motion: [DoubleTag; 3],
    pub Rotation: [FloatTag; 2],
    pub FallDistance: FloatTag,
    pub Fire: ShortTag,
    pub Air: ShortTag,
    pub OnGround: BooleanTag,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct MobLike {
    pub AttackTime: ShortTag,
    pub DeathTime: ShortTag,
    pub Health: ShortTag,
    pub HurtTime: ShortTag,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct ProjectileLike {
    pub xTile: ShortTag,
    pub yTile: ShortTag,
    pub zTile: ShortTag,
    pub inTile: ByteTag, // boolean?
    pub shake: ByteTag,  // boolean?
    pub inGround: BooleanTag,
}

// needs to be 'subclassed' with the new interfaces
#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct Player {
    pub Dimension: IntTag<DimensionID>,
    pub Pos: [DoubleTag; 3],
    pub Rotation: [FloatTag; 2],
    pub Motion: [DoubleTag; 3],
    pub OnGround: BooleanTag,
    pub FallDistance: FloatTag,
    pub Health: ShortTag,
    pub AttackTime: ShortTag,
    pub HurtTime: ShortTag,
    pub DeathTime: ShortTag,
    pub Air: ShortTag,
    pub Fire: ShortTag,
    pub Score: IntTag,
    pub Inventory: ListTag<SlottedItem>,
}

#[derive(Serialize, Deserialize)]
pub struct Arrow {
    #[serde(flatten)]
    pub entity_like: EntityLike,
    #[serde(flatten)]
    pub projectile_like: ProjectileLike,
}

#[derive(Serialize, Deserialize)]
pub struct Boat {
    #[serde(flatten)]
    pub entity_like: EntityLike,
}

#[derive(Serialize, Deserialize)]
pub struct Chicken {
    #[serde(flatten)]
    pub entity_like: EntityLike,
    #[serde(flatten)]
    pub mob_like: MobLike,
}

#[derive(Serialize, Deserialize)]
pub struct Cow {
    #[serde(flatten)]
    pub entity_like: EntityLike,
    #[serde(flatten)]
    pub mob_like: MobLike,
}

#[derive(Serialize, Deserialize)]
pub struct Creeper {
    #[serde(flatten)]
    pub entity_like: EntityLike,
    #[serde(flatten)]
    pub mob_like: MobLike,
}

#[derive(Serialize, Deserialize)]
pub struct Egg {
    #[serde(flatten)]
    pub entity_like: EntityLike,
    #[serde(flatten)]
    pub projectile_like: ProjectileLike,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct FallingSand {
    #[serde(flatten)]
    pub entity_like: EntityLike,
    pub Tile: ByteTag<BlockResource>,
}

#[derive(Serialize, Deserialize)]
pub struct Ghast {
    #[serde(flatten)]
    pub entity_like: EntityLike,
    #[serde(flatten)]
    pub mob_like: MobLike,
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
pub struct ItemEntity {
    #[serde(flatten)]
    pub entity_like: EntityLike,
    pub Health: ShortTag,
    pub Age: ShortTag,
    pub Item: Item,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct Minecart {
    #[serde(flatten)]
    pub entity_like: EntityLike,
    pub Type: ByteTag<MinecartVariant>,
    // Only for Furnace Minecart variant
    pub PushX: Option<DoubleTag>,
    pub PushZ: Option<DoubleTag>,
    pub Fuel: Option<ShortTag>,
    // Only for Chest Minecart variant
    pub Items: Option<ListTag<SlottedItem>>,
}

#[derive(Serialize, Deserialize)]
pub enum MinecartVariant {
    Empty = 0,
    Chest,
    Furnace,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct Painting {
    #[serde(flatten)]
    pub entity_like: EntityLike,
    pub Dir: ByteTag<PaintingDirection>,
    pub Motive: StringTag<PaintingVariant>,
    pub TileX: IntTag, // This isn't in Indev...? It appears to be in modern versions though, so..
    pub TileY: IntTag,
    pub TileZ: IntTag,
}

#[derive(Serialize, Deserialize)]
pub enum PaintingDirection {
    East = 0,
    North,
    West,
    South,
}

#[derive(Serialize, Deserialize)]
pub enum PaintingVariant {
    Alban,
    Aztec,
    Aztec2,
    Bomb,
    Bust,
    Courbet,
    Kebab,
    Match,
    Plant,
    Pool,
    Sea,
    SkullAndRoses,
    Stage,
    Void,
    Wanderer,
    Wasteland,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct Pig {
    #[serde(flatten)]
    pub entity_like: EntityLike,
    #[serde(flatten)]
    pub mob_like: MobLike,
    pub Saddle: BooleanTag,
}

#[derive(Serialize, Deserialize)]
pub struct PigZombie {
    #[serde(flatten)]
    pub entity_like: EntityLike,
    #[serde(flatten)]
    pub mob_like: MobLike,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct PrimedTnt {
    #[serde(flatten)]
    pub entity_like: EntityLike,
    pub Fuse: ByteTag,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct Sheep {
    #[serde(flatten)]
    pub entity_like: EntityLike,
    #[serde(flatten)]
    pub mob_like: MobLike,
    pub Sheared: BooleanTag,
}

#[derive(Serialize, Deserialize)]
pub struct Skeleton {
    #[serde(flatten)]
    pub entity_like: EntityLike,
    #[serde(flatten)]
    pub mob_like: MobLike,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct Slime {
    #[serde(flatten)]
    pub entity_like: EntityLike,
    #[serde(flatten)]
    pub mob_like: MobLike,
    pub Size: IntTag,
}

#[derive(Serialize, Deserialize)]
pub struct Snowball {
    #[serde(flatten)]
    pub entity_like: EntityLike,
    #[serde(flatten)]
    pub projectile_like: ProjectileLike,
}

#[derive(Serialize, Deserialize)]
pub struct Spider {
    #[serde(flatten)]
    pub entity_like: EntityLike,
    #[serde(flatten)]
    pub mob_like: MobLike,
}

#[derive(Serialize, Deserialize)]
pub struct Zombie {
    #[serde(flatten)]
    pub entity_like: EntityLike,
    #[serde(flatten)]
    pub mob_like: MobLike,
}

#[derive(Serialize, Deserialize)]
pub enum EntityResource {
    // Player, // I'm speculating; unknown if this is defined here
    Arrow,
    Boat,
    Chicken,
    Cow,
    Creeper,
    Egg,
    FallingSand,
    Ghast,
    Giant,
    Item,
    // Mob, // what's this? player?
    // Monster, // what's this too?
    Minecart,
    Painting,
    Pig,
    PigZombie,
    PrimedTnt,
    Sheep,
    Skeleton,
    Slime,
    Snowball,
    Spider,
    Zombie,
}
