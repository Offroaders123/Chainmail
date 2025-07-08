use serde::{Deserialize, Serialize};

use crate::{
    nbt::tag::{IntTag, StringTag},
    schema::java::v1_20::block::BlockResource,
};

#[derive(Serialize, Deserialize)]
pub struct TileTick {
    pub i: StringTag<TileTickResource>,
    pub p: IntTag,
    pub t: IntTag,
    pub x: IntTag,
    pub y: IntTag,
    pub z: IntTag,
}

// Need to check if these actually can be inherited like this; I'm pretty sure they can, because they're just a collection of existing Block values?
#[allow(non_camel_case_types)]
#[derive(Serialize, Deserialize)]
pub enum TileTickResource {
    repeater = BlockResource::repeater as isize,
    redstone_torch = BlockResource::redstone_torch as isize,
    redstone_wall_torch = BlockResource::redstone_wall_torch as isize,
    comparator = BlockResource::comparator as isize,
    observer = BlockResource::observer as isize,
    dispenser = BlockResource::dispenser as isize,
    dropper = BlockResource::dropper as isize,
    chain_command_block = BlockResource::chain_command_block as isize,
    command_block = BlockResource::command_block as isize,
    repeating_command_block = BlockResource::repeating_command_block as isize,
    water = BlockResource::water as isize,
    lava = BlockResource::lava as isize,
    red_sand = BlockResource::red_sand as isize,
    sand = BlockResource::sand as isize,
    suspicious_sand = BlockResource::suspicious_sand as isize,
    acacia_button = BlockResource::acacia_button as isize,
    birch_button = BlockResource::birch_button as isize,
    crimson_button = BlockResource::crimson_button as isize,
    dark_oak_button = BlockResource::dark_oak_button as isize,
    jungle_button = BlockResource::jungle_button as isize,
    oak_button = BlockResource::oak_button as isize,
    polished_blackstone_button = BlockResource::polished_blackstone_button as isize,
    spruce_button = BlockResource::spruce_button as isize,
    stone_button = BlockResource::stone_button as isize,
    warped_button = BlockResource::warped_button as isize,
    mangrove_button = BlockResource::mangrove_button as isize,
    bamboo_button = BlockResource::bamboo_button as isize,
    cherry_button = BlockResource::cherry_button as isize,
    acacia_pressure_plate = BlockResource::acacia_pressure_plate as isize,
    birch_pressure_plate = BlockResource::birch_pressure_plate as isize,
    crimson_pressure_plate = BlockResource::crimson_pressure_plate as isize,
    dark_oak_pressure_plate = BlockResource::dark_oak_pressure_plate as isize,
    heavy_weighted_pressure_plate = BlockResource::heavy_weighted_pressure_plate as isize,
    jungle_pressure_plate = BlockResource::jungle_pressure_plate as isize,
    light_weighted_pressure_plate = BlockResource::light_weighted_pressure_plate as isize,
    oak_pressure_plate = BlockResource::oak_pressure_plate as isize,
    polished_blackstone_pressure_plate = BlockResource::polished_blackstone_pressure_plate as isize,
    spruce_pressure_plate = BlockResource::spruce_pressure_plate as isize,
    stone_pressure_plate = BlockResource::stone_pressure_plate as isize,
    warped_pressure_plate = BlockResource::warped_pressure_plate as isize,
    mangrove_pressure_plate = BlockResource::mangrove_pressure_plate as isize,
    bamboo_pressure_plate = BlockResource::bamboo_pressure_plate as isize,
    cherry_pressure_plate = BlockResource::cherry_pressure_plate as isize,
    detector_rail = BlockResource::detector_rail as isize,
    tripwire_hook = BlockResource::tripwire_hook as isize,
    redstone_lamp = BlockResource::redstone_lamp as isize,
}
