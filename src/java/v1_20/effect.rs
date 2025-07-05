use serde::{Deserialize, Serialize};

use crate::nbt::tag::{BooleanTag, ByteTag, IntTag};

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct Effect {
    pub Ambient: BooleanTag,
    pub Amplifier: ByteTag,
    pub Duration: IntTag,
    pub HiddenEffect: Box<Effect>, // Might be optional?
    pub Id: EffectID, // I think this should be an enum instead, and it needs to be wrapped in an NBT tag type
    pub ShowIcon: BooleanTag,
    pub ShowParticles: BooleanTag,
}

#[allow(non_camel_case_types)]
#[derive(Serialize, Deserialize)]
pub enum EffectID {
    speed = 1,
    slowness,
    haste,
    mining_fatigue,
    strength,
    instant_health,
    instant_damage,
    jump_boost,
    nausea,
    regeneration,
    resistance,
    fire_resistance,
    water_breathing,
    invisibility,
    blindness,
    night_vision,
    hunger,
    weakness,
    poison,
    wither,
    health_boost,
    absorption,
    saturation,
    glowing,
    levitation,
    luck,
    unluck,
    slow_falling,
    conduit_power,
    dolphins_grace,
    bad_omen,
    hero_of_the_village,
    darkness,
}

#[allow(non_camel_case_types)]
pub enum EffectResource {
    absorption,
    bad_omen,
    blindness,
    conduit_power,
    dolphins_grace,
    fire_resistance,
    glowing,
    haste,
    health_boost,
    hero_of_the_village,
    hunger,
    instant_health,
    instant_damage,
    invisibility,
    jump_boost,
    levitation,
    luck,
    mining_fatigue,
    nausea,
    night_vision,
    poison,
    regeneration,
    resistance,
    saturation,
    slow_falling,
    slowness,
    speed,
    strength,
    unluck,
    water_breathing,
    weakness,
    wither,
    darkness,
}
