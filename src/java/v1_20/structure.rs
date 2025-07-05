use serde::{Deserialize, Serialize};

use crate::java::v1_20::{
    biome::BiomeResource,
    block::{BlockResource, BlockState},
};
use crate::nbt::tag::{BooleanTag, ByteTag, FloatTag, IntArrayTag, IntTag, ListTag, StringTag};

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct Structure {
    pub BB: IntArrayTag,
    pub biome: StringTag<BiomeResource>,
    pub Children: ListTag<StructurePiece>,
    pub ChunkX: IntTag,
    pub ChunkZ: IntTag,
    pub id: StringTag,
    pub Processed: ListTag<MonumentProcessed>,
    pub Valid: BooleanTag,
    // Need to implement the rest of the specific structures
    // [property: string]: any,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct StructurePiece {
    pub BB: IntArrayTag,
    pub BiomeType: StringTag<BiomeType>,
    pub C: ByteTag,
    pub CA: VillageBlock,
    pub CB: VillageBlock,
    pub CC: VillageBlock,
    pub CD: VillageBlock,
    pub Chest: BooleanTag,
    pub D: IntTag,
    pub Depth: IntTag,
    pub Entrances: ListTag<IntArrayTag>,
    pub EntryDoor: StringTag,
    pub GD: IntTag,
    pub hasPlacedChest0: BooleanTag,
    pub hasPlacedChest1: BooleanTag,
    pub hasPlacedChest2: BooleanTag,
    pub hasPlacedChest3: BooleanTag,
    pub Height: IntTag,
    pub HPos: IntTag,
    pub hps: BooleanTag,
    pub hr: BooleanTag,
    pub id: StringTag,
    pub integrity: FloatTag,
    pub isLarge: BooleanTag,
    pub junctions: ListTag<VillageJunction>,
    pub Left: BooleanTag,
    pub leftHigh: BooleanTag,
    pub leftLow: BooleanTag,
    pub Length: IntTag,
    pub Mob: BooleanTag,
    pub Num: IntTag,
    pub O: IntTag,
    pub placedHiddenCHest: BooleanTag,
    pub placedMainChest: BooleanTag,
    pub placedTrap1: BooleanTag,
    pub placedTrap2: BooleanTag,
    pub PosX: IntTag,
    pub PosY: IntTag,
    pub PosZ: IntTag,
    pub Right: BooleanTag,
    pub rightHigh: BooleanTag,
    pub rightLow: BooleanTag,
    pub Rot: StringTag<Rotation>,
    pub sc: BooleanTag,
    pub Seed: IntTag,
    pub Source: BooleanTag,
    pub Steps: IntTag,
    pub T: IntTag,
    pub Tall: BooleanTag,
    pub Template: StringTag,
    pub Terrace: BooleanTag,
    pub tf: BooleanTag,
    pub TPX: IntTag,
    pub TPY: IntTag,
    pub TPZ: IntTag,
    pub Type: ByteTag<StructureType>,
    pub Type__stronghold__: IntTag,
    pub VCount: IntTag,
    pub Width: IntTag,
    pub Witch: BooleanTag,
    pub Zombie: BooleanTag,
}

#[derive(Serialize, Deserialize)]
pub enum BiomeType {
    WARM,
    COLD,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct VillageBlock {
    pub Name: StringTag<BlockResource>,
    pub Properties: BlockState,
}

#[derive(Serialize, Deserialize)]
pub struct VillageJunction {
    pub source_x: IntTag,
    pub source_ground_y: IntTag,
    pub source_z: IntTag,
    pub delta_y: IntTag,
    pub dest_proj: StringTag<DestProj>,
}

#[allow(non_camel_case_types)]
#[derive(Serialize, Deserialize)]
pub enum Rotation {
    COUNTERCLOCKWISE_90,
    NONE,
    CLOCKWISE_90,
    CLOCKWISE_180,
}

// These are probably defined somewhere as to what these are
#[derive(Serialize, Deserialize)]
pub enum StructureType {
    Zero = 0,
    One,
    Two,
    Three,
}

#[allow(non_camel_case_types)]
#[derive(Serialize, Deserialize)]
pub enum DestProj {
    terrain_matching,
    rigid,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct MonumentProcessed {
    pub X: IntTag,
    pub Z: IntTag,
}

// Prefix with `minecraft:` when stringified!!
#[allow(non_camel_case_types)]
#[derive(Serialize, Deserialize)]
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
