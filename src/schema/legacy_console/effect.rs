use serde::{Deserialize, Serialize};

#[allow(non_camel_case_types)]
#[derive(Serialize, Deserialize)]
pub enum EffectResource {
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
    conduit_power,
    slow_falling,
    // do any of these ones following exist on LCE? Maybe PS4...
    // dolphins_grace = 30, // probably not this one, since would've been with the Aquatic features
    // bad_omen = 31,
    // hero_of_the_village = 32
}
