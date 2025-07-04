use crate::nbt::tag::{IntTag, LongTag};
use crate::pi::v0_1_1::entity::Player;

#[allow(non_snake_case)]
pub struct Level {
    Data: Data,
}

#[allow(non_snake_case)]
pub struct Data {
    LastPlayed: LongTag,
    SizeOnDisk: LongTag,
    RandomSeed: LongTag,
    SpawnX: IntTag,
    SpawnY: IntTag,
    SpawnZ: IntTag,
    Time: LongTag,
    Player: Player,
}
