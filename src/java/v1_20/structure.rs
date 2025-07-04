use crate::java::v1_20::{
    biome::BiomeResource,
    block::{BlockResource, BlockState},
};
use crate::nbt::tag::{BooleanTag, ByteTag, FloatTag, IntArrayTag, IntTag, ListTag, StringTag};

#[allow(non_snake_case)]
pub struct Structure {
    BB: IntArrayTag,
    biome: StringTag<BiomeResource>,
    Children: ListTag<StructurePiece>,
    ChunkX: IntTag,
    ChunkZ: IntTag,
    id: StringTag,
    Processed: ListTag<MonumentProcessed>,
    Valid: BooleanTag,
    // Need to implement the rest of the specific structures
    // [property: string]: any,
}

#[allow(non_snake_case)]
pub struct StructurePiece {
    BB: IntArrayTag,
    BiomeType: StringTag<BiomeType>,
    C: ByteTag,
    CA: VillageBlock,
    CB: VillageBlock,
    CC: VillageBlock,
    CD: VillageBlock,
    Chest: BooleanTag,
    D: IntTag,
    Depth: IntTag,
    Entrances: ListTag<IntArrayTag>,
    EntryDoor: StringTag,
    GD: IntTag,
    hasPlacedChest0: BooleanTag,
    hasPlacedChest1: BooleanTag,
    hasPlacedChest2: BooleanTag,
    hasPlacedChest3: BooleanTag,
    Height: IntTag,
    HPos: IntTag,
    hps: BooleanTag,
    hr: BooleanTag,
    id: StringTag,
    integrity: FloatTag,
    isLarge: BooleanTag,
    junctions: ListTag<VillageJunction>,
    Left: BooleanTag,
    leftHigh: BooleanTag,
    leftLow: BooleanTag,
    Length: IntTag,
    Mob: BooleanTag,
    Num: IntTag,
    O: IntTag,
    placedHiddenCHest: BooleanTag,
    placedMainChest: BooleanTag,
    placedTrap1: BooleanTag,
    placedTrap2: BooleanTag,
    PosX: IntTag,
    PosY: IntTag,
    PosZ: IntTag,
    Right: BooleanTag,
    rightHigh: BooleanTag,
    rightLow: BooleanTag,
    Rot: StringTag<Rotation>,
    sc: BooleanTag,
    Seed: IntTag,
    Source: BooleanTag,
    Steps: IntTag,
    T: IntTag,
    Tall: BooleanTag,
    Template: StringTag,
    Terrace: BooleanTag,
    tf: BooleanTag,
    TPX: IntTag,
    TPY: IntTag,
    TPZ: IntTag,
    Type: ByteTag<StructureType>,
    Type__stronghold__: IntTag,
    VCount: IntTag,
    Width: IntTag,
    Witch: BooleanTag,
    Zombie: BooleanTag,
}

pub enum BiomeType {
    WARM,
    COLD,
}

#[allow(non_snake_case)]
pub struct VillageBlock {
    Name: StringTag<BlockResource>,
    Properties: BlockState,
}

pub struct VillageJunction {
    source_x: IntTag,
    source_ground_y: IntTag,
    source_z: IntTag,
    delta_y: IntTag,
    dest_proj: StringTag<DestProj>,
}

#[allow(non_camel_case_types)]
pub enum Rotation {
    COUNTERCLOCKWISE_90,
    NONE,
    CLOCKWISE_90,
    CLOCKWISE_180,
}

// These are probably defined somewhere as to what these are
pub enum StructureType {
    Zero = 0,
    One,
    Two,
    Three,
}

#[allow(non_camel_case_types)]
pub enum DestProj {
    terrain_matching,
    rigid,
}

#[allow(non_snake_case)]
pub struct MonumentProcessed {
    X: IntTag,
    Z: IntTag,
}

// Prefix with `minecraft:` when stringified!!
#[allow(non_camel_case_types)]
pub enum StructureResource {
    ancient_city,
    bastion_remnant,
    buried_treasure,
    end_city,
    fortress,
    mansion,
    mineshaft,
    mineshaft_mesa,
    monument,
    nether_fossil,
    ocean_ruin_cold,
    ocean_ruin_warm,
    pillager_outpost,
    ruined_portal,
    ruined_portal_desert,
    ruined_portal_jungle,
    ruined_portal_mountain,
    ruined_portal_nether,
    ruined_portal_ocean,
    ruined_portal_swamp,
    shipwreck,
    shipwreck_beached,
    stronghold,
    desert_pyramid,
    igloo,
    jungle_pyramid,
    swamp_hut,
    trail_ruins,
    village_desert,
    village_plains,
    village_savanna,
    village_snowy,
    village_taiga,
}
