use serde::{Deserialize, Serialize};

// These should be `minecraft:` prefixed when serialized
#[allow(non_camel_case_types)]
#[derive(Serialize, Deserialize)]
pub enum StructureResource {
    ancientcity,
    ancient_city,
    bastionremnant,
    bastion_remnant,
    buriedtreasure,
    buried_treasure,
    endcity,
    end_city,
    fortress,
    mansion,
    mineshaft,
    monument,
    ruins,
    pillageroutpost,
    pillager_outpost,
    ruinedportal,
    ruined_portal,
    shipwreck,
    stronghold,
    temple,
    trail_ruins,
    village,
}
