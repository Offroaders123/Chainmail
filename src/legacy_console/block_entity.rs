// export type BlockEntity<K extends keyof BlockEntityNameMap = keyof BlockEntityNameMap> = BlockEntityNameMap[K];

// export type BlockEntityNameMap = {
//   // temp, also unsure of all of the values here yet
//   [K in BlockEntityResource]: BlockEntityLike<K>;
// };

use serde::{Deserialize, Serialize};

use crate::nbt::tag::StringTag;

// <BlockEntityID extends string>
#[derive(Serialize, Deserialize)]
pub struct BlockEntityLike {
    id: StringTag, // should this be `StringTag<BlockEntityResource>`, and or generic?
}

#[allow(non_camel_case_types)]
#[derive(Serialize, Deserialize)]
pub enum BlockEntityResource {
    Airportal,
    Banner,
    Beacon,
    Cauldron,
    Chest,
    Comparator,
    DLDetector,
    Dropper,
    EnchantTable,
    EndGateway,
    EnderChest,
    FlowerPot,
    Furnace,
    Hopper,
    MobSpawner,
    Music,
    Piston,
    RecordPlayer,
    Sign,
    Skull,
    Trap,
    banner,
    beacon,
    bed,
    brewing_stand,
    cauldron,
    chest,
    comparator,
    conduit,
    daylight_detector,
    dispenser,
    dropper,
    enchanting_table,
    end_gateway,
    end_portal,
    ender_Chest,
    ender_chest,
    flower_pot,
    furnace,
    hopper,
    jukebox,
    mob_spawner,
    noteblock,
    piston,
    shulker_box,
    sign,
    skull,
}
