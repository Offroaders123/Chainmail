use crate::{
    convert::convert_to::{ConvertTo, TryConvertTo},
    schema::{
        java::v1_20::level::{self as java, CustomBossEvents, DataPacks},
        pi::v0_1_1::level as pi,
    },
};

impl ConvertTo<java::LevelDat> for pi::LevelDat {
    fn convert(&self) -> java::LevelDat {
        java::LevelDat {
            Data: self.convert(),
        }
    }
}

impl ConvertTo<java::LevelDatData> for pi::LevelDat {
    fn convert(&self) -> java::LevelDatData {
        java::LevelDatData {
            allowCommands: false,
            BorderCenterX: 0.0,
            BorderCenterY: 0.0,
            BorderDamagePerBlock: 0.2,
            BorderSize: 60000000.0,
            BorderSafeZone: 5.0,
            BorderSizeLerpTarget: 60000000.0,
            BorderSizeLerpTime: 0,
            BorderWarningBlocks: 5.0,
            BorderWarningTime: 15.0,
            clearWeatherTime: 0,
            CustomBossEvents: CustomBossEvents { ..Default::default() },
            DataPacks: DataPacks { ..Default::default() },
            DataVersion: self.DataVersion, // also a very technical upgrade
            DayTime: self.dayCycleStopTime.unwrap_or(0),
            Difficulty: self.Difficulty,
            DifficultyLocked: self.DifficultyLocked,
            DimensionData: self.DimensionData,
            GameRules: self.GameRules,
            WorldGenSettings: self.WorldGenSettings,
            GameType: self.GameType.convert(),
            generatorName: java::GeneratorName::default,
            generatorOptions: None, // these are unique to 1.15 and below, likely will remove these aliases
            generatorVersion: None, // these are unique to 1.15 and below, likely will remove these aliases
            hardcore: false,
            initialized: true,
            LastPlayed: self.LastPlayed,
            LevelName: self.LevelName,
            MapFeatures: true,
            Player: self.Player.convert(),
            raining: false,
            rainTime: 18000, // Midpoint of normal range (15 min)
            RandomSeed: self.RandomSeed,
            SizeOnDisk: self.SizeOnDisk,
            SpawnX: self.SpawnX,
            SpawnY: self.SpawnY,
            SpawnZ: self.SpawnZ,
            thundering: false,
            thunderTime: 8000, // Midpoint of thunder range (~6.5 min)
            Time: self.Time,
            version: self.version, // needs very technical handling https://minecraft.wiki/w/Java_Edition_level_format#level.dat_format
            Version: self.Version, // needs very technical handling
            WanderingTraderId: self.WanderingTraderId,
            WanderingTraderSpawnChance: self.WanderingTraderSpawnChance,
            WanderingTraderSpawnDelay: self.WanderingTraderSpawnDelay,
            WasModded: false,
        }
    }
}
