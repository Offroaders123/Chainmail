use serde::{Deserialize, Serialize};

use crate::java::v1_20::{
    biome::BiomeResource, block::BlockState, block_entity::BlockEntity, entity::Entity,
    structure::Structure, tile_tick::TileTick,
};
use crate::nbt::tag::{
    ByteArrayTag, ByteTag, CompoundTag, IntTag, ListTag, LongArrayTag, LongTag, ShortTag, StringTag,
};

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct Chunk {
    pub DataVersion: IntTag,
    pub xPos: IntTag,
    pub zPos: IntTag,
    pub yPos: IntTag,
    pub Status: StringTag,
    pub LastUpdate: LongTag,
    pub sections: ListTag<Section>,
    pub block_entities: ListTag<BlockEntity>,
    pub CarvingMasks: CarvingMasks,
    pub Heightmaps: Heightmaps,
    pub Lights: Lights,
    pub Entities: ListTag<Entity>,
    pub fluid_ticks: ListTag<TileTick>,
    pub block_ticks: ListTag<TileTick>,
    pub InhabitedTime: LongTag,
    pub PostProcessing: ListTag<ToBeTicked>,
    pub structures: Structures,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct Section {
    pub Y: ByteTag,
    pub block_states: BlockStates,
    pub biomes: Biomes,
    pub BlockLight: ByteArrayTag,
    pub SkyLight: ByteArrayTag,
}

#[derive(Serialize, Deserialize)]
pub struct BlockStates {
    pub palette: ListTag<BlockState>,
    pub data: Option<LongArrayTag>,
}

#[derive(Serialize, Deserialize)]
pub struct Biomes {
    pub palette: ListTag<StringTag<BiomeResource>>,
    pub data: Option<LongArrayTag>,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct CarvingMasks {
    pub AIR: ByteArrayTag,
    pub LIQUID: ByteArrayTag,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct Heightmaps {
    pub MOTION_BLOCKING: LongArrayTag,
    pub MOTION_BLOCKING_NO_LEAVES: LongArrayTag,
    pub OCEAN_FLOOR: LongArrayTag,
    pub OCEAN_FLOOR_WG: LongArrayTag,
    pub WORLD_SURFACE: LongArrayTag,
    pub WORLD_SURFACE_WG: LongArrayTag,
}

pub type Lights = ListTag<ListTag<ShortTag>>;

pub type ToBeTicked = ListTag<ShortTag>;

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct Structures {
    pub References: References,
    pub starts: Starts,
}

pub type References = CompoundTag<Option<LongArrayTag>>; // { [K in StructureResource]: Option<LongArrayTag>, }

pub type Starts = CompoundTag<Option<Structure>>; // { [K in StructureResource]: Option<Structure>, }
