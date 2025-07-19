use crate::{
    convert::convert_to::{ConvertTo, TryConvertTo},
    schema::{java::v1_20::entity as java, pi::v0_1_1::entity as pi},
};

impl ConvertTo<pi::Player> for java::Player {
    fn convert(&self) -> pi::Player {
        pi::Player {
            entity_like: self.entity_like.convert(),
            mob_like: self.mob_like.convert(),
            Armor: self.mob_like.ArmorItems.map(|item| item.try_convert()),
            Dimension: self.Dimension.convert(),
            Inventory: self.Inventory.iter().map(|item| item.convert()).collect(),
            Score: self.Score,
            Sleeping: self.SleepTimer == 0,
            SleepTimer: self.SleepTimer,
            // https://minecraft.wiki/w/Player.dat_format#NBT_Structure
            // https://minecraft.wiki/w/Bedrock_Edition_level_format/History#NBT_Structure
            SpawnX: self.SpawnX.unwrap_or(0),
            SpawnY: self.SpawnY.unwrap_or(64),
            SpawnZ: self.SpawnZ.unwrap_or(0),
            abilities: self.abilities.convert(),
        }
    }
}
