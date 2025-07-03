use injectables::{inject_fields, injectable};

use crate::nbt::tag::{BooleanTag, ByteTag, FloatTag, IntTag, ListTag, ShortTag};
use crate::pi::v0_1_1::{dimension::DimensionID, item::Item};

pub enum Entity {
  Particle(Particle),
  Player(Player),
  TripodCamera(TripodCamera),
  CameraEntity(CameraEntity),
  Chicken(Chicken),
  Cow(Cow),
  Pig(Pig),
  Sheep(Sheep),
  Zombie(Zombie),
  Creeper(Creeper),
  Skeleton(Skeleton),
  Spider(Spider),
  PigZombie(PigZombie),
  ItemEntity(ItemEntity),
  PrimedTNT(PrimedTNT),
  FallingTile(FallingTile),
  Arrow(Arrow),
  Snowball(Snowball),
  ThrownEgg(ThrownEgg),
  Painting(Painting),
}

#[injectable]
pub struct EntityLike /*<EntityID extends number | undefined>*/ {
  id: IntTag<EntityResource>, // id: EntityID extends number ? IntTag<EntityID> : EntityID;
  Pos: [FloatTag; 3],
  Motion: [FloatTag; 3], // doesn't seem to mention optional
  Rotation: [FloatTag; 2],
  FallDistance: FloatTag,
  Fire: ShortTag,
  Air: ShortTag,
  OnGround: BooleanTag,
}

#[injectable]
pub struct MobLike {
  AttackTime: ShortTag,
  DeathTime: ShortTag,
  Health: ShortTag,
  HurtTime: ShortTag,
}

#[injectable]
pub struct AnimalLike {
  Age: IntTag,
}

#[inject_fields(EntityLike)]
pub struct Particle /*extends EntityLike<EntityResource.Particle>*/ {}

#[inject_fields(EntityLike, MobLike)]
pub struct Player /*extends EntityLike<undefined>, MobLike*/ {
  Armor: ListTag<Item>, // no slot tag actually, ignore. // with slot tag, right? docs doesn't say here. and are these optional? it's actually `[Item, Item, Item, Item]`
  Dimension: IntTag<DimensionID>,
  Inventory: ListTag<Item>, // slot tag for sure
  Score: IntTag,
  Sleeping: BooleanTag,
  SleepTimer: ShortTag,
  SpawnX: IntTag,
  SpawnY: IntTag,
  SpawnZ: IntTag,
  abilities: PlayerAbilities,
}

pub struct PlayerAbilities {
  mayfly: BooleanTag,
  flying: BooleanTag,
  invulnerable: BooleanTag,
  mayBuild: BooleanTag,
  instabuild: BooleanTag,
}

#[inject_fields(EntityLike)]
pub struct TripodCamera /*extends EntityLike<EntityResource.TripodCamera>*/ {}

#[inject_fields(EntityLike)]
pub struct CameraEntity /*extends EntityLike<EntityResource.CameraEntity>*/ {}

#[inject_fields(EntityLike, MobLike, AnimalLike)]
pub struct Chicken /*extends EntityLike<EntityResource.Chicken>, MobLike, AnimalLike*/ {}

#[inject_fields(EntityLike, MobLike, AnimalLike)]
pub struct Cow /*extends EntityLike<EntityResource.Cow>, MobLike, AnimalLike*/ {}

#[inject_fields(EntityLike, MobLike, AnimalLike)]
pub struct Pig /*extends EntityLike<EntityResource.Pig>, MobLike, AnimalLike*/ {}

#[inject_fields(EntityLike, MobLike, AnimalLike)]
pub struct Sheep /*extends EntityLike<EntityResource.Sheep>, MobLike, AnimalLike*/ {
  Sheared: BooleanTag,
  Color: ByteTag<SheepWoolColor>,
}

pub enum SheepWoolColor {
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
  Black
}

#[inject_fields(EntityLike, MobLike)]
pub struct Zombie /*extends EntityLike<EntityResource.Zombie>, MobLike*/ {}

#[inject_fields(EntityLike, MobLike)]
pub struct Creeper /*extends EntityLike<EntityResource.Creeper>, MobLike*/ {}

#[inject_fields(EntityLike, MobLike)]
pub struct Skeleton /*extends EntityLike<EntityResource.Skeleton>, MobLike*/ {}

#[inject_fields(EntityLike, MobLike)]
pub struct Spider /*extends EntityLike<EntityResource.Spider>, MobLike*/ {}

#[inject_fields(EntityLike, MobLike)]
pub struct PigZombie /*extends EntityLike<EntityResource.PigZombie>, MobLike*/ {}

#[inject_fields(EntityLike)]
pub struct ItemEntity /*extends EntityLike<EntityResource.ItemEntity>*/ {
  Health: ShortTag,
  Age: ShortTag,
  Item: Item,
}

#[inject_fields(EntityLike)]
pub struct PrimedTNT /*extends EntityLike<EntityResource.PrimedTnt>*/ {}

#[inject_fields(EntityLike)]
pub struct FallingTile /*extends EntityLike<EntityResource.FallingTile>*/ {}

#[inject_fields(EntityLike)]
pub struct Arrow /*extends EntityLike<EntityResource.Arrow>*/ {}

#[inject_fields(EntityLike)]
pub struct Snowball /*extends EntityLike<EntityResource.Snowball>*/ {}

#[inject_fields(EntityLike)]
pub struct ThrownEgg /*extends EntityLike<EntityResource.ThrownEgg>*/ {}

#[inject_fields(EntityLike)]
pub struct Painting /*extends EntityLike<EntityResource.Painting>*/ {}

pub enum EntityResource { // these first four are all actually `0`, this needs to be fixed with a `Into<u8>` block, or something similar.
  Particle = 0,
  Player = 01,
  TripodCamera = 02,
  CameraEntity = 03,
  // <Animals>
  Chicken = 10,
  Cow,
  Pig,
  Sheep,
  // <Monsters>
  Zombie = 32,
  Creeper,
  Skeleton,
  Spider,
  PigZombie,
  // </Monsters>
  ItemEntity = 64,
  PrimedTnt,
  FallingTile,
  Arrow = 80,
  Snowball,
  ThrownEgg,
  Painting,
}
