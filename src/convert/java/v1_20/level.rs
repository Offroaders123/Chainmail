use crate::{
    convert::convert_to::ConvertTo,
    nbt::tag::BooleanTag,
    schema::{java::v1_20::level as java, pi::v0_1_1::level as pi},
};

impl ConvertTo<pi::LevelDat> for java::LevelDat {
    fn convert(&self) -> pi::LevelDat {
        pi::LevelDat {
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

impl ConvertTo<pi::GameMode> for java::GameType {
    fn convert(&self) -> pi::GameMode {
        match self {
            Self::Survival => pi::GameMode::Survival,
            Self::Creative => pi::GameMode::Creative,
            Self::Adventure => pi::GameMode::Survival,
            Self::Spectator => pi::GameMode::Creative,
        }
    }
}

impl ConvertTo<pi::SpawnMobs> for BooleanTag {
    fn convert(&self) -> pi::SpawnMobs {
        match self {
            true => pi::SpawnMobs::True,
            false => pi::SpawnMobs::False,
        }
    }
}
