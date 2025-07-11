use serde::{Deserialize, Serialize};

// These should be `minecraft:` prefixed when serialized
#[allow(non_camel_case_types)]
#[derive(Serialize, Deserialize)]
pub enum EnchantmentResource {
    protection,
    fire_protection,
    feather_falling,
    blast_protection,
    projectile_protection,
    thorns,
    respiration,
    depth_strider,
    aqua_affinity,
    sharpness,
    bane_of_arthropods,
    knockback,
    fire_aspect,
    looting,
    efficiency,
    silk_touch,
    unbreaking,
    fortune,
    power,
    punch,
    flame,
    infinity,
    luck_of_the_sea,
    lure,
    frost_walker,
    mending,
    binding_curse,
    vanishing_curse,
    impaling,
    riptide,
    loyalty,
    channeling,
    multishot,
    piercing,
    quick_charge,
    soul_speed,
    swift_sneak,
}
