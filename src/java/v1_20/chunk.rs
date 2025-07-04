use crate::java::v1_20::{
    biome::BiomeResource, block::BlockState, block_entity::BlockEntity, entity::Entity,
    structure::Structure, tile_tick::TileTick,
};
use crate::nbt::tag::{
    ByteArrayTag, ByteTag, CompoundTag, IntTag, ListTag, LongArrayTag, LongTag, ShortTag, StringTag,
};

#[allow(non_snake_case)]
pub struct Chunk {
    DataVersion: IntTag,
    xPos: IntTag,
    zPos: IntTag,
    yPos: IntTag,
    Status: StringTag,
    LastUpdate: LongTag,
    sections: ListTag<Section>,
    block_entities: ListTag<BlockEntity>,
    CarvingMasks: CarvingMasks,
    Heightmaps: Heightmaps,
    Lights: Lights,
    Entities: ListTag<Entity>,
    fluid_ticks: ListTag<TileTick>,
    block_ticks: ListTag<TileTick>,
    InhabitedTime: LongTag,
    PostProcessing: ListTag<ToBeTicked>,
    structures: Structures,
}

#[allow(non_snake_case)]
pub struct Section {
    Y: ByteTag,
    block_states: BlockStates,
    biomes: Biomes,
    BlockLight: ByteArrayTag,
    SkyLight: ByteArrayTag,
}

pub struct BlockStates {
    palette: ListTag<BlockState>,
    data: Option<LongArrayTag>,
}

pub struct Biomes {
    palette: ListTag<StringTag<BiomeResource>>,
    data: Option<LongArrayTag>,
}

#[allow(non_snake_case)]
pub struct CarvingMasks {
    AIR: ByteArrayTag,
    LIQUID: ByteArrayTag,
}

#[allow(non_snake_case)]
pub struct Heightmaps {
    MOTION_BLOCKING: LongArrayTag,
    MOTION_BLOCKING_NO_LEAVES: LongArrayTag,
    OCEAN_FLOOR: LongArrayTag,
    OCEAN_FLOOR_WG: LongArrayTag,
    WORLD_SURFACE: LongArrayTag,
    WORLD_SURFACE_WG: LongArrayTag,
}

pub type Lights = ListTag<ListTag<ShortTag>>;

pub type ToBeTicked = ListTag<ShortTag>;

#[allow(non_snake_case)]
pub struct Structures {
    References: References,
    starts: Starts,
}

pub type References = CompoundTag<Option<LongArrayTag>>; // { [K in StructureResource]: Option<LongArrayTag>, }

pub type Starts = CompoundTag<Option<Structure>>; // { [K in StructureResource]: Option<Structure>, }
