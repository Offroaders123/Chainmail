use injectables::{inject_fields, injectable};

use crate::java::alpha::{
    block::BlockResource,
    dimension::DimensionID,
    item::{Item, SlottedItem},
};
use crate::nbt::tag::{
    BooleanTag, ByteTag, DoubleTag, FloatTag, IntTag, ListTag, ShortTag, StringTag,
};

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
#[injectable]
pub struct EntityLike {
    id: StringTag, // should this be generic to `EntityResource`?
    Pos: [DoubleTag; 3],
    Motion: [DoubleTag; 3],
    Rotation: [FloatTag; 2],
    FallDistance: FloatTag,
    Fire: ShortTag,
    Air: ShortTag,
    OnGround: BooleanTag,
}

#[allow(non_snake_case)]
#[injectable]
pub struct MobLike {
    AttackTime: ShortTag,
    DeathTime: ShortTag,
    Health: ShortTag,
    HurtTime: ShortTag,
}

#[allow(non_snake_case)]
#[injectable]
pub struct ProjectileLike {
    xTile: ShortTag,
    yTile: ShortTag,
    zTile: ShortTag,
    inTile: ByteTag, // boolean?
    shake: ByteTag,  // boolean?
    inGround: BooleanTag,
}

// needs to be 'subclassed' with the new interfaces
#[allow(non_snake_case)]
pub struct Player {
    Dimension: IntTag<DimensionID>,
    Pos: [DoubleTag; 3],
    Rotation: [FloatTag; 2],
    Motion: [DoubleTag; 3],
    OnGround: BooleanTag,
    FallDistance: FloatTag,
    Health: ShortTag,
    AttackTime: ShortTag,
    HurtTime: ShortTag,
    DeathTime: ShortTag,
    Air: ShortTag,
    Fire: ShortTag,
    Score: IntTag,
    Inventory: ListTag<SlottedItem>,
}

#[inject_fields(EntityLike, ProjectileLike)]
pub struct Arrow {}

#[inject_fields(EntityLike)]
pub struct Boat {}

#[inject_fields(EntityLike, MobLike)]
pub struct Chicken {}

#[inject_fields(EntityLike, MobLike)]
pub struct Cow {}

#[inject_fields(EntityLike, MobLike)]
pub struct Creeper {}

#[inject_fields(EntityLike, ProjectileLike)]
pub struct Egg {}

#[allow(non_snake_case)]
#[inject_fields(EntityLike)]
pub struct FallingSand {
    Tile: ByteTag<BlockResource>,
}

#[inject_fields(EntityLike, MobLike)]
pub struct Ghast {}

#[inject_fields(EntityLike, MobLike)]
pub struct Giant {}

#[allow(non_snake_case)]
#[inject_fields(EntityLike)]
pub struct ItemEntity {
    Health: ShortTag,
    Age: ShortTag,
    Item: Item,
}

#[allow(non_snake_case)]
#[inject_fields(EntityLike)]
pub struct Minecart {
    Type: ByteTag<MinecartVariant>,
    // Only for Furnace Minecart variant
    PushX: Option<DoubleTag>,
    PushZ: Option<DoubleTag>,
    Fuel: Option<ShortTag>,
    // Only for Chest Minecart variant
    Items: Option<ListTag<SlottedItem>>,
}

pub enum MinecartVariant {
    Empty = 0,
    Chest,
    Furnace,
}

#[allow(non_snake_case)]
#[inject_fields(EntityLike)]
pub struct Painting {
    Dir: ByteTag<PaintingDirection>,
    Motive: PaintingVariant,
    TileX: IntTag, // This isn't in Indev...? It appears to be in modern versions though, so..
    TileY: IntTag,
    TileZ: IntTag,
}

pub enum PaintingDirection {
    East = 0,
    North,
    West,
    South,
}

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
#[inject_fields(EntityLike, MobLike)]
pub struct Pig {
    Saddle: BooleanTag,
}

#[inject_fields(EntityLike, MobLike)]
pub struct PigZombie {}

#[allow(non_snake_case)]
#[inject_fields(EntityLike)]
pub struct PrimedTnt {
    Fuse: ByteTag,
}

#[allow(non_snake_case)]
#[inject_fields(EntityLike, MobLike)]
pub struct Sheep {
    Sheared: BooleanTag,
}

#[inject_fields(EntityLike, MobLike)]
pub struct Skeleton {}

#[allow(non_snake_case)]
#[inject_fields(EntityLike, MobLike)]
pub struct Slime {
    Size: IntTag,
}

#[inject_fields(EntityLike, ProjectileLike)]
pub struct Snowball {}

#[inject_fields(EntityLike, MobLike)]
pub struct Spider {}

#[inject_fields(EntityLike, MobLike)]
pub struct Zombie {}

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
