use serde::{Deserialize, Serialize};

#[allow(non_camel_case_types)]
#[derive(Serialize, Deserialize)]
pub enum EnchantmentResource {
    protection = 0,
    fire_protection,
    feather_falling,
    blast_protection,
    projectile_protection,
    respiration,
    aqua_affinity,
    thorns,
    depth_strider,
    frost_walker,
    binding_curse,
    sharpness = 16,
    smite,
    bane_of_arthropods,
    knockback,
    fire_aspect,
    looting,
    efficiency = 32,
    silk_touch,
    unbreaking,
    fortune,
    power = 48,
    punch,
    flame,
    infinity,
    luck_of_the_sea = 61,
    lure,
    mending = 70,
    vanishing_curse,
    impaling = 80,
    riptide,
    loyalty,
    channeling,
}
