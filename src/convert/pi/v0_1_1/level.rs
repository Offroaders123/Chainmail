use crate::{
    convert::convert_to::ConvertTo,
    schema::{java::v1_20::level as java, pi::v0_1_1::level as pi},
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
            allowCommands: self.allowCommands,
            BorderCenterX: self.BorderCenterX,
            BorderCenterY: self.BorderCenterY,
            BorderDamagePerBlock: self.BorderDamagePerBlock,
            BorderSize: self.BorderSize,
            BorderSafeZone: self.BorderSafeZone,
            BorderSizeLerpTarget: self.BorderSizeLerpTarget,
            BorderSizeLerpTime: self.BorderSizeLerpTime,
            BorderWarningBlocks: self.BorderWarningBlocks,
            BorderWarningTime: self.BorderWarningTime,
            clearWeatherTime: self.clearWeatherTime,
            CustomBossEvents: self.CustomBossEvents,
            DataPacks: self.DataPacks,
            DataVersion: self.DataVersion,
            DayTime: self.DayTime,
            Difficulty: self.Difficulty,
            DifficultyLocked: self.DifficultyLocked,
            DimensionData: self.DimensionData,
            GameRules: self.GameRules,
            WorldGenSettings: self.WorldGenSettings,
            GameType: self.GameType.convert(),
            generatorName: self.generatorName,
            generatorOptions: self.generatorOptions,
            generatorVersion: self.generatorVersion,
            hardcore: self.hardcore,
            initialized: self.initialized,
            LastPlayed: self.LastPlayed,
            LevelName: self.LevelName,
            MapFeatures: self.MapFeatures,
            Player: self.Player.convert(),
            raining: self.raining,
            rainTime: self.rainTime,
            RandomSeed: self.RandomSeed,
            SizeOnDisk: self.SizeOnDisk,
            SpawnX: self.SpawnX,
            SpawnY: self.SpawnY,
            SpawnZ: self.SpawnZ,
            thundering: self.thundering,
            thunderTime: self.thunderTime,
            Time: self.Time,
            version: self.version,
            Version: self.Version,
            WanderingTraderId: self.WanderingTraderId,
            WanderingTraderSpawnChance: self.WanderingTraderSpawnChance,
            WanderingTraderSpawnDelay: self.WanderingTraderSpawnDelay,
            WasModded: self.WasModded,
        }
    }
}
