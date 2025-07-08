use crate::{
    convert::convert_to::ConvertTo,
    nbt::tag::BooleanTag,
    schema::{java::v1_20 as java, pi::v0_1_1 as pi},
};

impl ConvertTo<pi::level::LevelDat> for java::level::LevelDat {
    fn convert(&self) -> pi::level::LevelDat {
        pi::level::LevelDat {
            GameType: self.Data.GameType.convert(),
            LastPlayed: self.Data.LastPlayed,
            LevelName: self.Data.LevelName,
            Platform: 2, // This is the value I observed in my own world, this might change, not for certain. Could be an enum!
            Player: self.Data.Player.convert(),
            RandomSeed: self.Data.RandomSeed,
            SizeOnDisk: self.Data.SizeOnDisk,
            SpawnX: self.Data.SpawnX,
            SpawnY: self.Data.SpawnY,
            SpawnZ: self.Data.SpawnZ,
            StorageVersion: self.Data.StorageVersion,
            Time: self.Data.Time,
            dayCycleStopTime: self.Data.DayTime,
            spawnMobs: self.Data.GameRules.doMobSpawning.convert(),
        }
    }
}

impl ConvertTo<pi::level::GameMode> for java::level::GameType {
    fn convert(&self) -> pi::level::GameMode {
        match self {
            java::level::GameType::Survival => pi::level::GameMode::Survival,
            java::level::GameType::Creative => pi::level::GameMode::Creative,
            java::level::GameType::Adventure => pi::level::GameMode::Survival,
            java::level::GameType::Spectator => pi::level::GameMode::Creative,
        }
    }
}

impl ConvertTo<pi::entity::Player> for java::entity::Player {
    fn convert(&self) -> pi::entity::Player {
        pi::entity::Player {}
    }
}

impl ConvertTo<pi::level::SpawnMobs> for BooleanTag {
    fn convert(&self) -> pi::level::SpawnMobs {
        match self {
            true => pi::level::SpawnMobs::True,
            false => pi::level::SpawnMobs::False,
        }
    }
}
