use crate::{
    convert::convert_to::ConvertTo,
    schema::{java::v1_20::entity as java, pi::v0_1_1::entity as pi},
};

impl ConvertTo<pi::Player> for java::Player {
    fn convert(&self) -> pi::Player {
        pi::Player {
            entity_like: self.entity_like.convert(),
            mob_like: self.mob_like.convert(),
            Armor: self.Armor,
            Dimension: self.Dimension.convert(),
            Inventory: self.Inventory.convert(),
            Score: self.Score,
            Sleeping: self.Sleeping,
            SleepTimer: self.SleepTimer,
            SpawnX: self.SpawnX,
            SpawnY: self.SpawnY,
            SpawnZ: self.SpawnZ,
            abilities: self.abilities.convert(),
        }
    }
}
