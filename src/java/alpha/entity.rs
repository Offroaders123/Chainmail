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
pub struct MobLike {
    pub AttackTime: ShortTag,
    pub DeathTime: ShortTag,
    pub Health: ShortTag,
    pub HurtTime: ShortTag,
}

#[allow(non_snake_case)]
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

pub struct Arrow {
    pub entity_like: EntityLike,
    pub projectile_like: ProjectileLike,
}

pub struct Boat {
    pub entity_like: EntityLike,
}

pub struct Chicken {
    pub entity_like: EntityLike,
    pub mob_like: MobLike,
}

pub struct Cow {
    pub entity_like: EntityLike,
    pub mob_like: MobLike,
}

pub struct Creeper {
    pub entity_like: EntityLike,
    pub mob_like: MobLike,
}

pub struct Egg {
    pub entity_like: EntityLike,
    pub projectile_like: ProjectileLike,
}

#[allow(non_snake_case)]
pub struct FallingSand {
    pub entity_like: EntityLike,
    pub Tile: ByteTag<BlockResource>,
}

pub struct Ghast {
    pub entity_like: EntityLike,
    pub mob_like: MobLike,
}

pub struct Giant {
    pub entity_like: EntityLike,
    pub mob_like: MobLike,
}

#[allow(non_snake_case)]
pub struct ItemEntity {
    pub entity_like: EntityLike,
    pub Health: ShortTag,
    pub Age: ShortTag,
    pub Item: Item,
}

#[allow(non_snake_case)]
pub struct Minecart {
    pub entity_like: EntityLike,
    pub Type: ByteTag<MinecartVariant>,
    // Only for Furnace Minecart variant
    pub PushX: Option<DoubleTag>,
    pub PushZ: Option<DoubleTag>,
    pub Fuel: Option<ShortTag>,
    // Only for Chest Minecart variant
    pub Items: Option<ListTag<SlottedItem>>,
}

pub enum MinecartVariant {
    Empty = 0,
    Chest,
    Furnace,
}

#[allow(non_snake_case)]
pub struct Painting {
    pub entity_like: EntityLike,
    pub Dir: ByteTag<PaintingDirection>,
    pub Motive: PaintingVariant,
    pub TileX: IntTag, // This isn't in Indev...? It appears to be in modern versions though, so..
    pub TileY: IntTag,
    pub TileZ: IntTag,
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
pub struct Pig {
    pub entity_like: EntityLike,
    pub mob_like: MobLike,
    pub Saddle: BooleanTag,
}

pub struct PigZombie {
    pub entity_like: EntityLike,
    pub mob_like: MobLike,
}

#[allow(non_snake_case)]
pub struct PrimedTnt {
    pub entity_like: EntityLike,
    pub Fuse: ByteTag,
}

#[allow(non_snake_case)]
pub struct Sheep {
    pub entity_like: EntityLike,
    pub mob_like: MobLike,
    pub Sheared: BooleanTag,
}

pub struct Skeleton {
    pub entity_like: EntityLike,
    pub mob_like: MobLike,
}

#[allow(non_snake_case)]
pub struct Slime {
    pub entity_like: EntityLike,
    pub mob_like: MobLike,
    pub Size: IntTag,
}

pub struct Snowball {
    pub entity_like: EntityLike,
    pub projectile_like: ProjectileLike,
}

pub struct Spider {
    pub entity_like: EntityLike,
    pub mob_like: MobLike,
}

pub struct Zombie {
    pub entity_like: EntityLike,
    pub mob_like: MobLike,
}

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
