use serde::{Deserialize, Serialize};

use crate::nbt::tag::{BooleanTag, IntTag, StringTag};

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct Block {
    name: StringTag<BlockResource>,
    states: BlockState,
    version: IntTag,
}

#[allow(non_camel_case_types)]
#[derive(Serialize, Deserialize)]
pub enum BlockState {
    acacia_button(AcaciaButton),
    acacia_door(AcaciaDoor),
    acacia_double_slab(AcaciaDoubleSlab),
    acacia_fence(AcaciaFence),
    acacia_fence_gate(AcaciaFenceGate),
    acacia_hanging_sign(AcaciaHangingSign),
    acacia_leaves(AcaciaLeaves),
    acacia_log(AcaciaLog),
    acacia_planks(AcaciaPlanks),
    acacia_pressure_plate(AcaciaPressurePlate),
    acacia_slab(AcaciaSlab),
    acacia_stairs(AcaciaStairs),
    acacia_standing_sign(AcaciaStandingSign),
    acacia_trapdoor(AcaciaTrapdoor),
    acacia_wall_sign(AcaciaWallSign),
    acacia_wood(AcaciaWood),
    activator_rail(ActivatorRail),
    air(Air),
    allow(Allow),
    amethyst_block(AmethystBlock),
    amethyst_cluster(AmethystCluster),
    ancient_debris(AncientDebris),
    andesite(Andesite),
    andesite_stairs(AndesiteStairs),
    anvil(Anvil),
    azalea(Azalea),
    azalea_leaves(AzaleaLeaves),
    azalea_leaves_flowered(AzaleaLeavesFlowered),
    bamboo(Bamboo),
    bamboo_block(BambooBlock),
    bamboo_button(BambooButton),
    bamboo_door(BambooDoor),
    bamboo_double_slab(BambooDoubleSlab),
    bamboo_fence(BambooFence),
    bamboo_fence_gate(BambooFenceGate),
    bamboo_hanging_sign(BambooHangingSign),
    bamboo_mosaic(BambooMosaic),
    bamboo_mosaic_double_slab(BambooMosaicDoubleSlab),
    bamboo_mosaic_slab(BambooMosaicSlab),
    bamboo_mosaic_stairs(BambooMosaicStairs),
    bamboo_planks(BambooPlanks),
    bamboo_pressure_plate(BambooPressurePlate),
    bamboo_sapling(BambooSapling),
    bamboo_slab(BambooSlab),
    bamboo_stairs(BambooStairs),
    bamboo_standing_sign(BambooStandingSign),
    bamboo_trapdoor(BambooTrapdoor),
    bamboo_wall_sign(BambooWallSign),
    barrel(Barrel),
    barrier(Barrier),
    basalt(Basalt),
    beacon(Beacon),
    bed(Bed),
    bedrock(Bedrock),
    bee_nest(BeeNest),
    beehive(Beehive),
    beetroot(Beetroot),
    bell(Bell),
    big_dripleaf(BigDripleaf),
    birch_button(BirchButton),
    birch_door(BirchDoor),
    birch_double_slab(BirchDoubleSlab),
    birch_fence(BirchFence),
    birch_fence_gate(BirchFenceGate),
    birch_hanging_sign(BirchHangingSign),
    birch_leaves(BirchLeaves),
    birch_log(BirchLog),
    birch_planks(BirchPlanks),
    birch_pressure_plate(BirchPressurePlate),
    birch_slab(BirchSlab),
    birch_stairs(BirchStairs),
    birch_standing_sign(BirchStandingSign),
    birch_trapdoor(BirchTrapdoor),
    birch_wall_sign(BirchWallSign),
    birch_wood(BirchWood),
    black_candle(BlackCandle),
    black_candle_cake(BlackCandleCake),
    black_carpet(BlackCarpet),
    black_concrete(BlackConcrete),
    black_concrete_powder(BlackConcretePowder),
    black_glazed_terracotta(BlackGlazedTerracotta),
    black_shulker_box(BlackShulkerBox),
    black_stained_glass(BlackStainedGlass),
    black_stained_glass_pane(BlackStainedGlassPane),
    black_terracotta(BlackTerracotta),
    black_wool(BlackWool),
    blackstone(Blackstone),
    blackstone_double_slab(BlackstoneDoubleSlab),
    blackstone_slab(BlackstoneSlab),
    blackstone_stairs(BlackstoneStairs),
    blackstone_wall(BlackstoneWall),
    blast_furnace(BlastFurnace),
    blue_candle(BlueCandle),
    blue_candle_cake(BlueCandleCake),
    blue_carpet(BlueCarpet),
    blue_concrete(BlueConcrete),
    blue_concrete_powder(BlueConcretePowder),
    blue_glazed_terracotta(BlueGlazedTerracotta),
    blue_ice(BlueIce),
    blue_shulker_box(BlueShulkerBox),
    blue_stained_glass(BlueStainedGlass),
    blue_stained_glass_pane(BlueStainedGlassPane),
    blue_terracotta(BlueTerracotta),
    blue_wool(BlueWool),
    bone_block(BoneBlock),
    bookshelf(Bookshelf),
    border_block(BorderBlock),
    brain_coral(BrainCoral),
    brewing_stand(BrewingStand),
    brick_block(BrickBlock),
    brick_stairs(BrickStairs),
    brown_candle(BrownCandle),
    brown_candle_cake(BrownCandleCake),
    brown_carpet(BrownCarpet),
    brown_concrete(BrownConcrete),
    brown_concrete_powder(BrownConcretePowder),
    brown_glazed_terracotta(BrownGlazedTerracotta),
    brown_mushroom(BrownMushroom),
    brown_mushroom_block(BrownMushroomBlock),
    brown_shulker_box(BrownShulkerBox),
    brown_stained_glass(BrownStainedGlass),
    brown_stained_glass_pane(BrownStainedGlassPane),
    brown_terracotta(BrownTerracotta),
    brown_wool(BrownWool),
    bubble_column(BubbleColumn),
    bubble_coral(BubbleCoral),
    budding_amethyst(BuddingAmethyst),
    cactus(Cactus),
    cake(Cake),
    calcite(Calcite),
    calibrated_sculk_sensor(CalibratedSculkSensor),
    camera(Camera),
    campfire(Campfire),
    candle(Candle),
    candle_cake(CandleCake),
    carrots(Carrots),
    cartography_table(CartographyTable),
    carved_pumpkin(CarvedPumpkin),
    cauldron(Cauldron),
    cave_vines(CaveVines),
    cave_vines_body_with_berries(CaveVinesBodyWithBerries),
    cave_vines_head_with_berries(CaveVinesHeadWithBerries),
    chain(Chain),
    chain_command_block(ChainCommandBlock),
    chemical_heat(ChemicalHeat),
    chemistry_table(ChemistryTable),
    cherry_button(CherryButton),
    cherry_door(CherryDoor),
    cherry_double_slab(CherryDoubleSlab),
    cherry_fence(CherryFence),
    cherry_fence_gate(CherryFenceGate),
    cherry_hanging_sign(CherryHangingSign),
    cherry_leaves(CherryLeaves),
    cherry_log(CherryLog),
    cherry_planks(CherryPlanks),
    cherry_pressure_plate(CherryPressurePlate),
    cherry_sapling(CherrySapling),
    cherry_slab(CherrySlab),
    cherry_stairs(CherryStairs),
    cherry_standing_sign(CherryStandingSign),
    cherry_trapdoor(CherryTrapdoor),
    cherry_wall_sign(CherryWallSign),
    cherry_wood(CherryWood),
    chest(Chest),
    chiseled_bookshelf(ChiseledBookshelf),
    chiseled_copper(ChiseledCopper),
    chiseled_deepslate(ChiseledDeepslate),
    chiseled_nether_bricks(ChiseledNetherBricks),
    chiseled_polished_blackstone(ChiseledPolishedBlackstone),
    chiseled_tuff(ChiseledTuff),
    chiseled_tuff_bricks(ChiseledTuffBricks),
    chorus_flower(ChorusFlower),
    chorus_plant(ChorusPlant),
    clay(Clay),
    client_request_placeholder_block(ClientRequestPlaceholderBlock),
    coal_block(CoalBlock),
    coal_ore(CoalOre),
    cobbled_deepslate(CobbledDeepslate),
    cobbled_deepslate_double_slab(CobbledDeepslateDoubleSlab),
    cobbled_deepslate_slab(CobbledDeepslateSlab),
    cobbled_deepslate_stairs(CobbledDeepslateStairs),
    cobbled_deepslate_wall(CobbledDeepslateWall),
    cobblestone(Cobblestone),
    cobblestone_wall(CobblestoneWall),
    cocoa(Cocoa),
    colored_torch_bp(ColoredTorchBp),
    colored_torch_rg(ColoredTorchRg),
    command_block(CommandBlock),
    composter(Composter),
    conduit(Conduit),
    copper_block(CopperBlock),
    copper_bulb(CopperBulb),
    copper_door(CopperDoor),
    copper_grate(CopperGrate),
    copper_ore(CopperOre),
    copper_trapdoor(CopperTrapdoor),
    coral_block(CoralBlock),
    coral_fan(CoralFan),
    coral_fan_dead(CoralFanDead),
    coral_fan_hang(CoralFanHang),
    coral_fan_hang2(CoralFanHang2),
    coral_fan_hang3(CoralFanHang3),
    cracked_deepslate_bricks(CrackedDeepslateBricks),
    cracked_deepslate_tiles(CrackedDeepslateTiles),
    cracked_nether_bricks(CrackedNetherBricks),
    cracked_polished_blackstone_bricks(CrackedPolishedBlackstoneBricks),
    crafter(Crafter),
    crafting_table(CraftingTable),
    crimson_button(CrimsonButton),
    crimson_door(CrimsonDoor),
    crimson_double_slab(CrimsonDoubleSlab),
    crimson_fence(CrimsonFence),
    crimson_fence_gate(CrimsonFenceGate),
    crimson_fungus(CrimsonFungus),
    crimson_hanging_sign(CrimsonHangingSign),
    crimson_hyphae(CrimsonHyphae),
    crimson_nylium(CrimsonNylium),
    crimson_planks(CrimsonPlanks),
    crimson_pressure_plate(CrimsonPressurePlate),
    crimson_roots(CrimsonRoots),
    crimson_slab(CrimsonSlab),
    crimson_stairs(CrimsonStairs),
    crimson_standing_sign(CrimsonStandingSign),
    crimson_stem(CrimsonStem),
    crimson_trapdoor(CrimsonTrapdoor),
    crimson_wall_sign(CrimsonWallSign),
    crying_obsidian(CryingObsidian),
    cut_copper(CutCopper),
    cut_copper_slab(CutCopperSlab),
    cut_copper_stairs(CutCopperStairs),
    cyan_candle(CyanCandle),
    cyan_candle_cake(CyanCandleCake),
    cyan_carpet(CyanCarpet),
    cyan_concrete(CyanConcrete),
    cyan_concrete_powder(CyanConcretePowder),
    cyan_glazed_terracotta(CyanGlazedTerracotta),
    cyan_shulker_box(CyanShulkerBox),
    cyan_stained_glass(CyanStainedGlass),
    cyan_stained_glass_pane(CyanStainedGlassPane),
    cyan_terracotta(CyanTerracotta),
    cyan_wool(CyanWool),
    dark_oak_button(DarkOakButton),
    dark_oak_door(DarkOakDoor),
    dark_oak_double_slab(DarkOakDoubleSlab),
    dark_oak_fence(DarkOakFence),
    dark_oak_fence_gate(DarkOakFenceGate),
    dark_oak_hanging_sign(DarkOakHangingSign),
    dark_oak_leaves(DarkOakLeaves),
    dark_oak_log(DarkOakLog),
    dark_oak_planks(DarkOakPlanks),
    dark_oak_pressure_plate(DarkOakPressurePlate),
    dark_oak_slab(DarkOakSlab),
    dark_oak_stairs(DarkOakStairs),
    dark_oak_trapdoor(DarkOakTrapdoor),
    dark_oak_wood(DarkOakWood),
    dark_prismarine_stairs(DarkPrismarineStairs),
    darkoak_standing_sign(DarkOakStandingSign),
    darkoak_wall_sign(DarkOakWallSign),
    daylight_detector(DaylightDetector),
    daylight_detector_inverted(DaylightDetectorInverted),
    dead_brain_coral(DeadBrainCoral),
    dead_bubble_coral(DeadBubbleCoral),
    dead_fire_coral(DeadFireCoral),
    dead_horn_coral(DeadHornCoral),
    dead_tube_coral(DeadTubeCoral),
    deadbush(DeadBush),
    decorated_pot(DecoratedPot),
    deepslate(Deepslate),
    deepslate_brick_double_slab(DeepslateBrickDoubleSlab),
    deepslate_brick_slab(DeepslateBrickSlab),
    deepslate_brick_stairs(DeepslateBrickStairs),
    deepslate_brick_wall(DeepslateBrickWall),
    deepslate_bricks(DeepslateBricks),
    deepslate_coal_ore(DeepslateCoalOre),
    deepslate_copper_ore(DeepslateCopperOre),
    deepslate_diamond_ore(DeepslateDiamondOre),
    deepslate_emerald_ore(DeepslateEmeraldOre),
    deepslate_gold_ore(DeepslateGoldOre),
    deepslate_iron_ore(DeepslateIronOre),
    deepslate_lapis_ore(DeepslateLapisOre),
    deepslate_redstone_ore(DeepslateRedstoneOre),
    deepslate_tile_double_slab(DeepslateTileDoubleSlab),
    deepslate_tile_slab(DeepslateTileSlab),
    deepslate_tile_stairs(DeepslateTileStairs),
    deepslate_tile_wall(DeepslateTileWall),
    deepslate_tiles(DeepslateTiles),
    deny(Deny),
    detector_rail(DetectorRail),
    diamond_block(DiamondBlock),
    diamond_ore(DiamondOre),
    diorite(Diorite),
    diorite_stairs(DioriteStairs),
    dirt(Dirt),
    dirt_with_roots(DirtWithRoots),
    dispenser(Dispenser),
    double_cut_copper_slab(DoubleCutCopperSlab),
    double_plant(DoublePlant),
    double_stone_block_slab(DoubleStoneBlockSlab),
    double_stone_block_slab2(DoubleStoneBlockSlab2),
    double_stone_block_slab3(DoubleStoneBlockSlab3),
    double_stone_block_slab4(DoubleStoneBlockSlab4),
    dragon_egg(DragonEgg),
    dried_kelp_block(DriedKelpBlock),
    dripstone_block(DripstoneBlock),
    dropper(Dropper),
    element_0(Element0),
    element_1(Element1),
    element_10(Element10),
    element_100(Element100),
    element_101(Element101),
    element_102(Element102),
    element_103(Element103),
    element_104(Element104),
    element_105(Element105),
    element_106(Element106),
    element_107(Element107),
    element_108(Element108),
    element_109(Element109),
    element_11(Element11),
    element_110(Element110),
    element_111(Element111),
    element_112(Element112),
    element_113(Element113),
    element_114(Element114),
    element_115(Element115),
    element_116(Element116),
    element_117(Element117),
    element_118(Element118),
    element_12(Element12),
    element_13(Element13),
    element_14(Element14),
    element_15(Element15),
    element_16(Element16),
    element_17(Element17),
    element_18(Element18),
    element_19(Element19),
    element_2(Element2),
    element_20(Element20),
    element_21(Element21),
    element_22(Element22),
    element_23(Element23),
    element_24(Element24),
    element_25(Element25),
    element_26(Element26),
    element_27(Element27),
    element_28(Element28),
    element_29(Element29),
    element_3(Element3),
    element_30(Element30),
    element_31(Element31),
    element_32(Element32),
    element_33(Element33),
    element_34(Element34),
    element_35(Element35),
    element_36(Element36),
    element_37(Element37),
    element_38(Element38),
    element_39(Element39),
    element_4(Element4),
    element_40(Element40),
    element_41(Element41),
    element_42(Element42),
    element_43(Element43),
    element_44(Element44),
    element_45(Element45),
    element_46(Element46),
    element_47(Element47),
    element_48(Element48),
    element_49(Element49),
    element_5(Element5),
    element_50(Element50),
    element_51(Element51),
    element_52(Element52),
    element_53(Element53),
    element_54(Element54),
    element_55(Element55),
    element_56(Element56),
    element_57(Element57),
    element_58(Element58),
    element_59(Element59),
    element_6(Element6),
    element_60(Element60),
    element_61(Element61),
    element_62(Element62),
    element_63(Element63),
    element_64(Element64),
    element_65(Element65),
    element_66(Element66),
    element_67(Element67),
    element_68(Element68),
    element_69(Element69),
    element_7(Element7),
    element_70(Element70),
    element_71(Element71),
    element_72(Element72),
    element_73(Element73),
    element_74(Element74),
    element_75(Element75),
    element_76(Element76),
    element_77(Element77),
    element_78(Element78),
    element_79(Element79),
    element_8(Element8),
    element_80(Element80),
    element_81(Element81),
    element_82(Element82),
    element_83(Element83),
    element_84(Element84),
    element_85(Element85),
    element_86(Element86),
    element_87(Element87),
    element_88(Element88),
    element_89(Element89),
    element_9(Element9),
    element_90(Element90),
    element_91(Element91),
    element_92(Element92),
    element_93(Element93),
    element_94(Element94),
    element_95(Element95),
    element_96(Element96),
    element_97(Element97),
    element_98(Element98),
    element_99(Element99),
    emerald_block(EmeraldBlock),
    emerald_ore(EmeraldOre),
    enchanting_table(EnchantingTable),
    end_brick_stairs(EndBrickStairs),
    end_bricks(EndBricks),
    end_gateway(EndGateway),
    end_portal(EndPortal),
    end_portal_frame(EndPortalFrame),
    end_rod(EndRod),
    end_stone(EndStone),
    ender_chest(EnderChest),
    exposed_chiseled_copper(ExposedChiseledCopper),
    exposed_copper(ExposedCopper),
    exposed_copper_bulb(ExposedCopperBulb),
    exposed_copper_door(ExposedCopperDoor),
    exposed_copper_grate(ExposedCopperGrate),
    exposed_copper_trapdoor(ExposedCopperTrapdoor),
    exposed_cut_copper(ExposedCutCopper),
    exposed_cut_copper_slab(ExposedCutCopperSlab),
    exposed_cut_copper_stairs(ExposedCutCopperStairs),
    exposed_double_cut_copper_slab(ExposedDoubleCutCopperSlab),
    farmland(Farmland),
    fence_gate(FenceGate),
    fire(Fire),
    fire_coral(FireCoral),
    fletching_table(FletchingTable),
    flower_pot(FlowerPot),
    flowering_azalea(FloweringAzalea),
    flowing_lava(FlowingLava),
    flowing_water(FlowingWater),
    frame(Frame),
    frog_spawn(FrogSpawn),
    frosted_ice(FrostedIce),
    furnace(Furnace),
    gilded_blackstone(GildedBlackstone),
    glass(Glass),
    glass_pane(GlassPane),
    glow_frame(GlowFrame),
    glow_lichen(GlowLichen),
    glowingobsidian(GlowingObsidian),
    glowstone(Glowstone),
    gold_block(GoldBlock),
    gold_ore(GoldOre),
    golden_rail(GoldenRail),
    granite(Granite),
    granite_stairs(GraniteStairs),
    grass_block(GrassBlock),
    grass_path(GrassPath),
    gravel(Gravel),
    gray_candle(GrayCandle),
    gray_candle_cake(GrayCandleCake),
    gray_carpet(GrayCarpet),
    gray_concrete(GrayConcrete),
    gray_concrete_powder(GrayConcretePowder),
    gray_glazed_terracotta(GrayGlazedTerracotta),
    gray_shulker_box(GrayShulkerBox),
    gray_stained_glass(GrayStainedGlass),
    gray_stained_glass_pane(GrayStainedGlassPane),
    gray_terracotta(GrayTerracotta),
    gray_wool(GrayWool),
    green_candle(GreenCandle),
    green_candle_cake(GreenCandleCake),
    green_carpet(GreenCarpet),
    green_concrete(GreenConcrete),
    green_concrete_powder(GreenConcretePowder),
    green_glazed_terracotta(GreenGlazedTerracotta),
    green_shulker_box(GreenShulkerBox),
    green_stained_glass(GreenStainedGlass),
    green_stained_glass_pane(GreenStainedGlassPane),
    green_terracotta(GreenTerracotta),
    green_wool(GreenWool),
    grindstone(Grindstone),
    hanging_roots(HangingRoots),
    hard_black_stained_glass(HardBlackStainedGlass),
    hard_black_stained_glass_pane(HardBlackStainedGlassPane),
    hard_blue_stained_glass(HardBlueStainedGlass),
    hard_blue_stained_glass_pane(HardBlueStainedGlassPane),
    hard_brown_stained_glass(HardBrownStainedGlass),
    hard_brown_stained_glass_pane(HardBrownStainedGlassPane),
    hard_cyan_stained_glass(HardCyanStainedGlass),
    hard_cyan_stained_glass_pane(HardCyanStainedGlassPane),
    hard_glass(HardGlass),
    hard_glass_pane(HardGlassPane),
    hard_gray_stained_glass(HardGrayStainedGlass),
    hard_gray_stained_glass_pane(HardGrayStainedGlassPane),
    hard_green_stained_glass(HardGreenStainedGlass),
    hard_green_stained_glass_pane(HardGreenStainedGlassPane),
    hard_light_blue_stained_glass(HardLightBlueStainedGlass),
    hard_light_blue_stained_glass_pane(HardLightBlueStainedGlassPane),
    hard_light_gray_stained_glass(HardLightGrayStainedGlass),
    hard_light_gray_stained_glass_pane(HardLightGrayStainedGlassPane),
    hard_lime_stained_glass(HardLimeStainedGlass),
    hard_lime_stained_glass_pane(HardLimeStainedGlassPane),
    hard_magenta_stained_glass(HardMagentaStainedGlass),
    hard_magenta_stained_glass_pane(HardMagentaStainedGlassPane),
    hard_orange_stained_glass(HardOrangeStainedGlass),
    hard_orange_stained_glass_pane(HardOrangeStainedGlassPane),
    hard_pink_stained_glass(HardPinkStainedGlass),
    hard_pink_stained_glass_pane(HardPinkStainedGlassPane),
    hard_purple_stained_glass(HardPurpleStainedGlass),
    hard_purple_stained_glass_pane(HardPurpleStainedGlassPane),
    hard_red_stained_glass(HardRedStainedGlass),
    hard_red_stained_glass_pane(HardRedStainedGlassPane),
    hard_white_stained_glass(HardWhiteStainedGlass),
    hard_white_stained_glass_pane(HardWhiteStainedGlassPane),
    hard_yellow_stained_glass(HardYellowStainedGlass),
    hard_yellow_stained_glass_pane(HardYellowStainedGlassPane),
    hardened_clay(HardenedClay),
    hay_block(HayBlock),
    heavy_weighted_pressure_plate(HeavyWeightedPressurePlate),
    honey_block(HoneyBlock),
    honeycomb_block(HoneycombBlock),
    hopper(Hopper),
    horn_coral(HornCoral),
    ice(Ice),
    infested_deepslate(InfestedDeepslate),
    info_update(InfoUpdate),
    info_update2(InfoUpdate2),
    invisible_bedrock(InvisibleBedrock),
    iron_bars(IronBars),
    iron_block(IronBlock),
    iron_door(IronDoor),
    iron_ore(IronOre),
    iron_trapdoor(IronTrapdoor),
    jigsaw(Jigsaw),
    jukebox(Jukebox),
    jungle_button(JungleButton),
    jungle_door(JungleDoor),
    jungle_double_slab(JungleDoubleSlab),
    jungle_fence(JungleFence),
    jungle_fence_gate(JungleFenceGate),
    jungle_hanging_sign(JungleHangingSign),
    jungle_leaves(JungleLeaves),
    jungle_log(JungleLog),
    jungle_planks(JunglePlanks),
    jungle_pressure_plate(JunglePressurePlate),
    jungle_slab(JungleSlab),
    jungle_stairs(JungleStairs),
    jungle_standing_sign(JungleStandingSign),
    jungle_trapdoor(JungleTrapdoor),
    jungle_wall_sign(JungleWallSign),
    jungle_wood(JungleWood),
    kelp(Kelp),
    ladder(Ladder),
    lantern(Lantern),
    lapis_block(LapisBlock),
    lapis_ore(LapisOre),
    large_amethyst_bud(LargeAmethystBud),
    lava(Lava),
    lectern(Lectern),
    lever(Lever),
    light_block(LightBlock),
    light_blue_candle(LightBlueCandle),
    light_blue_candle_cake(LightBlueCandleCake),
    light_blue_carpet(LightBlueCarpet),
    light_blue_concrete(LightBlueConcrete),
    light_blue_concrete_powder(LightBlueConcretePowder),
    light_blue_glazed_terracotta(LightBlueGlazedTerracotta),
    light_blue_shulker_box(LightBlueShulkerBox),
    light_blue_stained_glass(LightBlueStainedGlass),
    light_blue_stained_glass_pane(LightBlueStainedGlassPane),
    light_blue_terracotta(LightBlueTerracotta),
    light_blue_wool(LightBlueWool),
    light_gray_candle(LightGrayCandle),
    light_gray_candle_cake(LightGrayCandleCake),
    light_gray_carpet(LightGrayCarpet),
    light_gray_concrete(LightGrayConcrete),
    light_gray_concrete_powder(LightGrayConcretePowder),
    light_gray_shulker_box(LightGrayShulkerBox),
    light_gray_stained_glass(LightGrayStainedGlass),
    light_gray_stained_glass_pane(LightGrayStainedGlassPane),
    light_gray_terracotta(LightGrayTerracotta),
    light_gray_wool(LightGrayWool),
    light_weighted_pressure_plate(LightWeightedPressurePlate),
    lightning_rod(LightningRod),
    lime_candle(LimeCandle),
    lime_candle_cake(LimeCandleCake),
    lime_carpet(LimeCarpet),
    lime_concrete(LimeConcrete),
    lime_concrete_powder(LimeConcretePowder),
    lime_glazed_terracotta(LimeGlazedTerracotta),
    lime_shulker_box(LimeShulkerBox),
    lime_stained_glass(LimeStainedGlass),
    lime_stained_glass_pane(LimeStainedGlassPane),
    lime_terracotta(LimeTerracotta),
    lime_wool(LimeWool),
    lit_blast_furnace(LitBlastFurnace),
    lit_deepslate_redstone_ore(LitDeepslateRedstoneOre),
    lit_furnace(LitFurnace),
    lit_pumpkin(LitPumpkin),
    lit_redstone_lamp(LitRedstoneLamp),
    lit_redstone_ore(LitRedstoneOre),
    lit_smoker(LitSmoker),
    lodestone(Lodestone),
    loom(Loom),
    magenta_candle(MagentaCandle),
    magenta_candle_cake(MagentaCandleCake),
    magenta_carpet(MagentaCarpet),
    magenta_concrete(MagentaConcrete),
    magenta_concrete_powder(MagentaConcretePowder),
    magenta_glazed_terracotta(MagentaGlazedTerracotta),
    magenta_shulker_box(MagentaShulkerBox),
    magenta_stained_glass(MagentaStainedGlass),
    magenta_stained_glass_pane(MagentaStainedGlassPane),
    magenta_terracotta(MagentaTerracotta),
    magenta_wool(MagentaWool),
    magma(Magma),
    mangrove_button(MangroveButton),
    mangrove_door(MangroveDoor),
    mangrove_double_slab(MangroveDoubleSlab),
    mangrove_fence(MangroveFence),
    mangrove_fence_gate(MangroveFenceGate),
    mangrove_hanging_sign(MangroveHangingSign),
    mangrove_leaves(MangroveLeaves),
    mangrove_log(MangroveLog),
    mangrove_planks(MangrovePlanks),
    mangrove_pressure_plate(MangrovePressurePlate),
    mangrove_propagule(MangrovePropagule),
    mangrove_roots(MangroveRoots),
    mangrove_slab(MangroveSlab),
    mangrove_stairs(MangroveStairs),
    mangrove_standing_sign(MangroveStandingSign),
    mangrove_trapdoor(MangroveTrapdoor),
    mangrove_wall_sign(MangroveWallSign),
    mangrove_wood(MangroveWood),
    medium_amethyst_bud(MediumAmethystBud),
    melon_block(MelonBlock),
    melon_stem(MelonStem),
    mob_spawner(MobSpawner),
    monster_egg(MonsterEgg),
    moss_block(MossBlock),
    moss_carpet(MossCarpet),
    mossy_cobblestone(MossyCobblestone),
    mossy_cobblestone_stairs(MossyCobblestoneStairs),
    mossy_stone_brick_stairs(MossyStoneBrickStairs),
    moving_block(MovingBlock),
    mud(Mud),
    mud_brick_double_slab(MudBrickDoubleSlab),
    mud_brick_slab(MudBrickSlab),
    mud_brick_stairs(MudBrickStairs),
    mud_brick_wall(MudBrickWall),
    mud_bricks(MudBricks),
    muddy_mangrove_roots(MuddyMangroveRoots),
    mycelium(Mycelium),
    nether_brick(NetherBrick),
    nether_brick_fence(NetherBrickFence),
    nether_brick_stairs(NetherBrickStairs),
    nether_gold_ore(NetherGoldOre),
    nether_sprouts(NetherSprouts),
    nether_wart(NetherWart),
    nether_wart_block(NetherWartBlock),
    netherite_block(NetheriteBlock),
    netherrack(Netherrack),
    netherreactor(NetherReactorCore),
    normal_stone_stairs(NormalStoneStairs),
    noteblock(NoteBlock),
    oak_double_slab(OakDoubleSlab),
    oak_fence(OakFence),
    oak_hanging_sign(OakHangingSign),
    oak_leaves(OakLeaves),
    oak_log(OakLog),
    oak_planks(OakPlanks),
    oak_slab(OakSlab),
    oak_stairs(OakStairs),
    oak_wood(OakWood),
    observer(Observer),
    obsidian(Obsidian),
    ochre_froglight(OchreFroglight),
    orange_candle(OrangeCandle),
    orange_candle_cake(OrangeCandleCake),
    orange_carpet(OrangeCarpet),
    orange_concrete(OrangeConcrete),
    orange_concrete_powder(OrangeConcretePowder),
    orange_glazed_terracotta(OrangeGlazedTerracotta),
    orange_shulker_box(OrangeShulkerBox),
    orange_stained_glass(OrangeStainedGlass),
    orange_stained_glass_pane(OrangeStainedGlassPane),
    orange_terracotta(OrangeTerracotta),
    orange_wool(OrangeWool),
    oxidized_chiseled_copper(OxidizedChiseledCopper),
    oxidized_copper(OxidizedCopper),
    oxidized_copper_bulb(OxidizedCopperBulb),
    oxidized_copper_door(OxidizedCopperDoor),
    oxidized_copper_grate(OxidizedCopperGrate),
    oxidized_copper_trapdoor(OxidizedCopperTrapdoor),
    oxidized_cut_copper(OxidizedCutCopper),
    oxidized_cut_copper_slab(OxidizedCutCopperSlab),
    oxidized_cut_copper_stairs(OxidizedCutCopperStairs),
    oxidized_double_cut_copper_slab(OxidizedDoubleCutCopperSlab),
    packed_ice(PackedIce),
    packed_mud(PackedMud),
    pearlescent_froglight(PearlescentFroglight),
    pink_candle(PinkCandle),
    pink_candle_cake(PinkCandleCake),
    pink_carpet(PinkCarpet),
    pink_concrete(PinkConcrete),
    pink_concrete_powder(PinkConcretePowder),
    pink_glazed_terracotta(PinkGlazedTerracotta),
    pink_petals(PinkPetals),
    pink_shulker_box(PinkShulkerBox),
    pink_stained_glass(PinkStainedGlass),
    pink_stained_glass_pane(PinkStainedGlassPane),
    pink_terracotta(PinkTerracotta),
    pink_wool(PinkWool),
    piston(Piston),
    piston_arm_collision(PistonArmCollision),
    pitcher_crop(PitcherCrop),
    pitcher_plant(PitcherPlant),
    podzol(Podzol),
    pointed_dripstone(PointedDripstone),
    polished_andesite(PolishedAndesite),
    polished_andesite_stairs(PolishedAndesiteStairs),
    polished_basalt(PolishedBasalt),
    polished_blackstone(PolishedBlackstone),
    polished_blackstone_brick_double_slab(PolishedBlackstoneBrickDoubleSlab),
    polished_blackstone_brick_slab(PolishedBlackstoneBrickSlab),
    polished_blackstone_brick_stairs(PolishedBlackstoneBrickStairs),
    polished_blackstone_brick_wall(PolishedBlackstoneBrickWall),
    polished_blackstone_bricks(PolishedBlackstoneBricks),
    polished_blackstone_button(PolishedBlackstoneButton),
    polished_blackstone_double_slab(PolishedBlackstoneDoubleSlab),
    polished_blackstone_pressure_plate(PolishedBlackstonePressurePlate),
    polished_blackstone_slab(PolishedBlackstoneSlab),
    polished_blackstone_stairs(PolishedBlackstoneStairs),
    polished_blackstone_wall(PolishedBlackstoneWall),
    polished_deepslate(PolishedDeepslate),
    polished_deepslate_double_slab(PolishedDeepslateDoubleSlab),
    polished_deepslate_slab(PolishedDeepslateSlab),
    polished_deepslate_stairs(PolishedDeepslateStairs),
    polished_deepslate_wall(PolishedDeepslateWall),
    polished_diorite(PolishedDiorite),
    polished_diorite_stairs(PolishedDioriteStairs),
    polished_granite(PolishedGranite),
    polished_granite_stairs(PolishedGraniteStairs),
    polished_tuff(PolishedTuff),
    polished_tuff_double_slab(PolishedTuffDoubleSlab),
    polished_tuff_slab(PolishedTuffSlab),
    polished_tuff_stairs(PolishedTuffStairs),
    polished_tuff_wall(PolishedTuffWall),
    portal(Portal),
    potatoes(Potatoes),
    powder_snow(PowderSnow),
    powered_comparator(PoweredComparator),
    powered_repeater(PoweredRepeater),
    prismarine(Prismarine),
    prismarine_bricks_stairs(PrismarineBricksStairs),
    prismarine_stairs(PrismarineStairs),
    pumpkin(Pumpkin),
    pumpkin_stem(PumpkinStem),
    purple_candle(PurpleCandle),
    purple_candle_cake(PurpleCandleCake),
    purple_carpet(PurpleCarpet),
    purple_concrete(PurpleConcrete),
    purple_concrete_powder(PurpleConcretePowder),
    purple_glazed_terracotta(PurpleGlazedTerracotta),
    purple_shulker_box(PurpleShulkerBox),
    purple_stained_glass(PurpleStainedGlass),
    purple_stained_glass_pane(PurpleStainedGlassPane),
    purple_terracotta(PurpleTerracotta),
    purple_wool(PurpleWool),
    purpur_block(PurpurBlock),
    purpur_stairs(PurpurStairs),
    quartz_block(QuartzBlock),
    quartz_bricks(QuartzBricks),
    quartz_ore(QuartzOre),
    quartz_stairs(QuartzStairs),
    rail(Rail),
    raw_copper_block(RawCopperBlock),
    raw_gold_block(RawGoldBlock),
    raw_iron_block(RawIronBlock),
    red_candle(RedCandle),
    red_candle_cake(RedCandleCake),
    red_carpet(RedCarpet),
    red_concrete(RedConcrete),
    red_concrete_powder(RedConcretePowder),
    red_flower(RedFlower),
    red_glazed_terracotta(RedGlazedTerracotta),
    red_mushroom(RedMushroom),
    red_mushroom_block(RedMushroomBlock),
    red_nether_brick(RedNetherBrick),
    red_nether_brick_stairs(RedNetherBrickStairs),
    red_sandstone(RedSandstone),
    red_sandstone_stairs(RedSandstoneStairs),
    red_shulker_box(RedShulkerBox),
    red_stained_glass(RedStainedGlass),
    red_stained_glass_pane(RedStainedGlassPane),
    red_terracotta(RedTerracotta),
    red_wool(RedWool),
    redstone_block(RedstoneBlock),
    redstone_lamp(RedstoneLamp),
    redstone_ore(RedstoneOre),
    redstone_torch(RedstoneTorch),
    redstone_wire(RedstoneWire),
    reeds(Reeds),
    reinforced_deepslate(ReinforcedDeepslate),
    repeating_command_block(RepeatingCommandBlock),
    reserved6(Reserved6),
    respawn_anchor(RespawnAnchor),
    sand(Sand),
    sandstone(Sandstone),
    sandstone_stairs(SandstoneStairs),
    sapling(Sapling),
    scaffolding(Scaffolding),
    sculk(Sculk),
    sculk_catalyst(SculkCatalyst),
    sculk_sensor(SculkSensor),
    sculk_shrieker(SculkShrieker),
    sculk_vein(SculkVein),
    sea_lantern(SeaLantern),
    sea_pickle(SeaPickle),
    seagrass(Seagrass),
    shroomlight(Shroomlight),
    silver_glazed_terracotta(SilverGlazedTerracotta),
    skull(Skull),
    slime(Slime),
    small_amethyst_bud(SmallAmethystBud),
    small_dripleaf_block(SmallDripleafBlock),
    smithing_table(SmithingTable),
    smoker(Smoker),
    smooth_basalt(SmoothBasalt),
    smooth_quartz_stairs(SmoothQuartzStairs),
    smooth_red_sandstone_stairs(SmoothRedSandstoneStairs),
    smooth_sandstone_stairs(SmoothSandstoneStairs),
    smooth_stone(SmoothStone),
    sniffer_egg(SnifferEgg),
    snow(Snow),
    snow_layer(SnowLayer),
    soul_campfire(SoulCampfire),
    soul_fire(SoulFire),
    soul_lantern(SoulLantern),
    soul_sand(SoulSand),
    soul_soil(SoulSoil),
    soul_torch(SoulTorch),
    sponge(Sponge),
    spore_blossom(SporeBlossom),
    spruce_button(SpruceButton),
    spruce_door(SpruceDoor),
    spruce_double_slab(SpruceDoubleSlab),
    spruce_fence(SpruceFence),
    spruce_fence_gate(SpruceFenceGate),
    spruce_hanging_sign(SpruceHangingSign),
    spruce_leaves(SpruceLeaves),
    spruce_log(SpruceLog),
    spruce_planks(SprucePlanks),
    spruce_pressure_plate(SprucePressurePlate),
    spruce_slab(SpruceSlab),
    spruce_stairs(SpruceStairs),
    spruce_standing_sign(SpruceStandingSign),
    spruce_trapdoor(SpruceTrapdoor),
    spruce_wall_sign(SpruceWallSign),
    spruce_wood(SpruceWood),
    standing_banner(StandingBanner),
    standing_sign(StandingSign),
    sticky_piston(StickyPiston),
    sticky_piston_arm_collision(StickyPistonArmCollision),
    stone(Stone),
    stone_block_slab(StoneBlockSlab),
    stone_block_slab2(StoneBlockSlab2),
    stone_block_slab3(StoneBlockSlab3),
    stone_block_slab4(StoneBlockSlab4),
    stone_brick_stairs(StoneBrickStairs),
    stone_button(StoneButton),
    stone_pressure_plate(StonePressurePlate),
    stone_stairs(StoneStairs),
    stonebrick(StoneBrick),
    stonecutter(Stonecutter),
    stonecutter_block(StonecutterBlock),
    stripped_acacia_log(StrippedAcaciaLog),
    stripped_acacia_wood(StrippedAcaciaWood),
    stripped_bamboo_block(StrippedBambooBlock),
    stripped_birch_log(StrippedBirchLog),
    stripped_birch_wood(StrippedBirchWood),
    stripped_cherry_log(StrippedCherryLog),
    stripped_cherry_wood(StrippedCherryWood),
    stripped_crimson_hyphae(StrippedCrimsonHyphae),
    stripped_crimson_stem(StrippedCrimsonStem),
    stripped_dark_oak_log(StrippedDarkOakLog),
    stripped_dark_oak_wood(StrippedDarkOakWood),
    stripped_jungle_log(StrippedJungleLog),
    stripped_jungle_wood(StrippedJungleWood),
    stripped_mangrove_log(StrippedMangroveLog),
    stripped_mangrove_wood(StrippedMangroveWood),
    stripped_oak_log(StrippedOakLog),
    stripped_oak_wood(StrippedOakWood),
    stripped_spruce_log(StrippedSpruceLog),
    stripped_spruce_wood(StrippedSpruceWood),
    stripped_warped_hyphae(StrippedWarpedHyphae),
    stripped_warped_stem(StrippedWarpedStem),
    structure_block(StructureBlock),
    structure_void(StructureVoid),
    suspicious_gravel(SuspiciousGravel),
    suspicious_sand(SuspiciousSand),
    sweet_berry_bush(SweetBerryBush),
    tallgrass(Tallgrass),
    target(Target),
    tinted_glass(TintedGlass),
    tnt(TNT),
    torch(Torch),
    torchflower(Torchflower),
    torchflower_crop(TorchflowerCrop),
    trapdoor(Trapdoor),
    trapped_chest(TrappedChest),
    trial_spawner(TrialSpawner),
    trip_wire(TripWire),
    tripwire_hook(TripwireHook),
    tube_coral(TubeCoral),
    tuff(Tuff),
    tuff_brick_double_slab(TuffBrickDoubleSlab),
    tuff_brick_slab(TuffBrickSlab),
    tuff_brick_stairs(TuffBrickStairs),
    tuff_brick_wall(TuffBrickWall),
    tuff_bricks(TuffBricks),
    tuff_double_slab(TuffDoubleSlab),
    tuff_slab(TuffSlab),
    tuff_stairs(TuffStairs),
    tuff_wall(TuffWall),
    turtle_egg(TurtleEgg),
    twisting_vines(TwistingVines),
    underwater_torch(UnderwaterTorch),
    undyed_shulker_box(UndyedShulkerBox),
    unknown(Unknown),
    unlit_redstone_torch(UnlitRedstoneTorch),
    unpowered_comparator(UnpoweredComparator),
    unpowered_repeater(UnpoweredRepeater),
    vault(Vault),
    verdant_froglight(VerdantFroglight),
    vine(Vine),
    wall_banner(WallBanner),
    wall_sign(WallSign),
    warped_button(WarpedButton),
    warped_door(WarpedDoor),
    warped_double_slab(WarpedDoubleSlab),
    warped_fence(WarpedFence),
    warped_fence_gate(WarpedFenceGate),
    warped_fungus(WarpedFungus),
    warped_hanging_sign(WarpedHangingSign),
    warped_hyphae(WarpedHyphae),
    warped_nylium(WarpedNylium),
    warped_planks(WarpedPlanks),
    warped_pressure_plate(WarpedPressurePlate),
    warped_roots(WarpedRoots),
    warped_slab(WarpedSlab),
    warped_stairs(WarpedStairs),
    warped_standing_sign(WarpedStandingSign),
    warped_stem(WarpedStem),
    warped_trapdoor(WarpedTrapdoor),
    warped_wall_sign(WarpedWallSign),
    warped_wart_block(WarpedWartBlock),
    water(Water),
    waterlily(Waterlily),
    waxed_chiseled_copper(WaxedChiseledCopper),
    waxed_copper(WaxedCopper),
    waxed_copper_bulb(WaxedCopperBulb),
    waxed_copper_door(WaxedCopperDoor),
    waxed_copper_grate(WaxedCopperGrate),
    waxed_copper_trapdoor(WaxedCopperTrapdoor),
    waxed_cut_copper(WaxedCutCopper),
    waxed_cut_copper_slab(WaxedCutCopperSlab),
    waxed_cut_copper_stairs(WaxedCutCopperStairs),
    waxed_double_cut_copper_slab(WaxedDoubleCutCopperSlab),
    waxed_exposed_chiseled_copper(WaxedExposedChiseledCopper),
    waxed_exposed_copper(WaxedExposedCopper),
    waxed_exposed_copper_bulb(WaxedExposedCopperBulb),
    waxed_exposed_copper_door(WaxedExposedCopperDoor),
    waxed_exposed_copper_grate(WaxedExposedCopperGrate),
    waxed_exposed_copper_trapdoor(WaxedExposedCopperTrapdoor),
    waxed_exposed_cut_copper(WaxedExposedCutCopper),
    waxed_exposed_cut_copper_slab(WaxedExposedCutCopperSlab),
    waxed_exposed_cut_copper_stairs(WaxedExposedCutCopperStairs),
    waxed_exposed_double_cut_copper_slab(WaxedExposedDoubleCutCopperSlab),
    waxed_oxidized_chiseled_copper(WaxedOxidizedChiseledCopper),
    waxed_oxidized_copper(WaxedOxidizedCopper),
    waxed_oxidized_copper_bulb(WaxedOxidizedCopperBulb),
    waxed_oxidized_copper_door(WaxedOxidizedCopperDoor),
    waxed_oxidized_copper_grate(WaxedOxidizedCopperGrate),
    waxed_oxidized_copper_trapdoor(WaxedOxidizedCopperTrapdoor),
    waxed_oxidized_cut_copper(WaxedOxidizedCutCopper),
    waxed_oxidized_cut_copper_slab(WaxedOxidizedCutCopperSlab),
    waxed_oxidized_cut_copper_stairs(WaxedOxidizedCutCopperStairs),
    waxed_oxidized_double_cut_copper_slab(WaxedOxidizedDoubleCutCopperSlab),
    waxed_weathered_chiseled_copper(WaxedWeatheredChiseledCopper),
    waxed_weathered_copper(WaxedWeatheredCopper),
    waxed_weathered_copper_bulb(WaxedWeatheredCopperBulb),
    waxed_weathered_copper_door(WaxedWeatheredCopperDoor),
    waxed_weathered_copper_grate(WaxedWeatheredCopperGrate),
    waxed_weathered_copper_trapdoor(WaxedWeatheredCopperTrapdoor),
    waxed_weathered_cut_copper(WaxedWeatheredCutCopper),
    waxed_weathered_cut_copper_slab(WaxedWeatheredCutCopperSlab),
    waxed_weathered_cut_copper_stairs(WaxedWeatheredCutCopperStairs),
    waxed_weathered_double_cut_copper_slab(WaxedWeatheredDoubleCutCopperSlab),
    weathered_chiseled_copper(WeatheredChiseledCopper),
    weathered_copper(WeatheredCopper),
    weathered_copper_bulb(WeatheredCopperBulb),
    weathered_copper_door(WeatheredCopperDoor),
    weathered_copper_grate(WeatheredCopperGrate),
    weathered_copper_trapdoor(WeatheredCopperTrapdoor),
    weathered_cut_copper(WeatheredCutCopper),
    weathered_cut_copper_slab(WeatheredCutCopperSlab),
    weathered_cut_copper_stairs(WeatheredCutCopperStairs),
    weathered_double_cut_copper_slab(WeatheredDoubleCutCopperSlab),
    web(Web),
    weeping_vines(WeepingVines),
    wheat(Wheat),
    white_candle(WhiteCandle),
    white_candle_cake(WhiteCandleCake),
    white_carpet(WhiteCarpet),
    white_concrete(WhiteConcrete),
    white_concrete_powder(WhiteConcretePowder),
    white_glazed_terracotta(WhiteGlazedTerracotta),
    white_shulker_box(WhiteShulkerBox),
    white_stained_glass(WhiteStainedGlass),
    white_stained_glass_pane(WhiteStainedGlassPane),
    white_terracotta(WhiteTerracotta),
    white_wool(WhiteWool),
    wither_rose(WitherRose),
    wooden_button(WoodenButton),
    wooden_door(WoodenDoor),
    wooden_pressure_plate(WoodenPressurePlate),
    yellow_candle(YellowCandle),
    yellow_candle_cake(YellowCandleCake),
    yellow_carpet(YellowCarpet),
    yellow_concrete(YellowConcrete),
    yellow_concrete_powder(YellowConcretePowder),
    yellow_flower(YellowFlower),
    yellow_glazed_terracotta(YellowGlazedTerracotta),
    yellow_shulker_box(YellowShulkerBox),
    yellow_stained_glass(YellowStainedGlass),
    yellow_stained_glass_pane(YellowStainedGlassPane),
    yellow_terracotta(YellowTerracotta),
    yellow_wool(YellowWool),
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct AcaciaButton {
    button_like: ButtonLike,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct AcaciaDoor {
    door_like: DoorLike,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct AcaciaDoubleSlab {
    double_slab_like: DoubleSlabLike,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct AcaciaFence {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct AcaciaFenceGate {
    fence_gate_like: FenceGateLike,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct AcaciaHangingSign {
    hanging_sign_like: HangingSignLike,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct AcaciaLeaves {
    leaves_like: LeavesLike,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct AcaciaLog {
    pillar_like: PillarLike,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct AcaciaPlanks {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct AcaciaPressurePlate {
    powerable: Powerable,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct AcaciaSlab {
    double_slab_like: DoubleSlabLike,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct AcaciaStairs {
    stairs_like: StairsLike,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct AcaciaStandingSign {
    sign_like: SignLike,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct AcaciaTrapdoor {
    trapdoor_like: TrapdoorLike,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct AcaciaWallSign {
    wall_sign_like: WallSignLike,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct AcaciaWood {
    pillar_like: PillarLike,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct ActivatorRail {
    rail_data_bit: BooleanTag,
    rail_direction: IntTag<ActivatorRailDirection>,
}

#[allow(non_camel_case_types)]
#[derive(Serialize, Deserialize)]
pub enum ActivatorRailDirection {
    Zero = 0,
    One,
    Two,
    Three,
    Four,
    Five,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct Air {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct Allow {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct AmethystBlock {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct AmethystCluster {
    #[serde(rename = "minecraft:block_face")]
    block_face: StringTag<AmethystClusterBlockFace>,
}

#[allow(non_camel_case_types)]
#[derive(Serialize, Deserialize)]
pub enum AmethystClusterBlockFace {
    down,
    up,
    north,
    south,
    west,
    east,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct AncientDebris {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct Andesite {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct AndesiteStairs {
    stairs_like: StairsLike,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct Anvil {
    cardinal_direction_like: CardinalDirectionLike,
    damage: StringTag<AnvilDamage>,
}

#[allow(non_camel_case_types)]
#[derive(Serialize, Deserialize)]
pub enum AnvilDamage {
    undamaged,
    slightly_damaged,
    very_damaged,
    broken,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct Azalea {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct AzaleaLeaves {
    leaves_like: LeavesLike,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct AzaleaLeavesFlowered {
    leaves_like: LeavesLike,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct Bamboo {
    age_bit: BooleanTag,
    bamboo_leaf_size: StringTag<BambooLeafSize>,
    bamboo_stalk_thickness: StringTag<BambooStalkThickness>,
}

#[allow(non_camel_case_types)]
#[derive(Serialize, Deserialize)]
pub enum BambooLeafSize {
    no_leaves,
    small_leaves,
    large_leaves,
}

#[allow(non_camel_case_types)]
#[derive(Serialize, Deserialize)]
pub enum BambooStalkThickness {
    thin,
    thick,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct BambooBlock {
    pillar_like: PillarLike,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct BambooButton {
    button_like: ButtonLike,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct BambooDoor {
    door_like: DoorLike,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct BambooDoubleSlab {
    double_slab_like: DoubleSlabLike,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct BambooFence {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct BambooFenceGate {
    fence_gate_like: FenceGateLike,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct BambooHangingSign {
    hanging_sign_like: HangingSignLike,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct BambooMosaic {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct BambooMosaicDoubleSlab {
    double_slab_like: DoubleSlabLike,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct BambooMosaicSlab {
    double_slab_like: DoubleSlabLike,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct BambooMosaicStairs {
    stairs_like: StairsLike,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct BambooPlanks {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct BambooPressurePlate {
    powerable: Powerable,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct BambooSapling {
    age_bit: BooleanTag,
    // what's the dealio with this guy?
    sapling_type: StringTag<BambooSaplingType>,
}

// what's the dealio with this guy?
#[allow(non_camel_case_types)]
#[derive(Serialize, Deserialize)]
pub enum BambooSaplingType {
    oak,
    spruce,
    birch,
    jungle,
    acacia,
    dark_oak,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct BambooSlab {
    double_slab_like: DoubleSlabLike,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct BambooStairs {
    stairs_like: StairsLike,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct BambooStandingSign {
    sign_like: SignLike,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct BambooTrapdoor {
    trapdoor_like: TrapdoorLike,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct BambooWallSign {
    wall_sign_like: WallSignLike,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct Barrel {
    facing_direction_like: FacingDirectionLike,
    open_bit: BooleanTag,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct Barrier {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct Basalt {
    pillar_like: PillarLike,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct Beacon {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct Bed {
    direction_like: DirectionLike,
    head_piece_bit: BooleanTag,
    occupied_bit: BooleanTag,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct Bedrock {
    infiniburn_bit: BooleanTag,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct BeeNest {
    beehive_like: BeehiveLike,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct Beehive {
    beehive_like: BeehiveLike,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct BeehiveLike {
    direction_like: DirectionLike,
    honey_level: IntTag<BeehiveHoneyLevel>,
}

#[allow(non_camel_case_types)]
#[derive(Serialize, Deserialize)]
pub enum BeehiveHoneyLevel {
    Zero = 0,
    One,
    Two,
    Three,
    Four,
    Five,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct Beetroot {
    growth: IntTag<BeetrootGrowth>,
}

#[allow(non_camel_case_types)]
#[derive(Serialize, Deserialize)]
pub enum BeetrootGrowth {
    Zero = 0,
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct Bell {
    direction_like: DirectionLike,
    attachment: StringTag<BellAttachment>,
    toggle_bit: BooleanTag,
}

#[allow(non_camel_case_types)]
#[derive(Serialize, Deserialize)]
pub enum BellAttachment {
    standing,
    hanging,
    side,
    multiple,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct BigDripleaf {
    cardinal_direction_like: CardinalDirectionLike,
    big_dripleaf_head: BooleanTag,
    big_dripleaf_tilt: StringTag<BigDripleafTilt>,
}

#[allow(non_camel_case_types)]
#[derive(Serialize, Deserialize)]
pub enum BigDripleafTilt {
    none,
    unstable,
    partial_tilt,
    full_tilt,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct BirchButton {
    button_like: ButtonLike,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct BirchDoor {
    door_like: DoorLike,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct BirchDoubleSlab {
    double_slab_like: DoubleSlabLike,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct BirchFence {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct BirchFenceGate {
    fence_gate_like: FenceGateLike,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct BirchHangingSign {
    hanging_sign_like: HangingSignLike,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct BirchLeaves {
    leaves_like: LeavesLike,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct BirchLog {
    pillar_like: PillarLike,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct BirchPlanks {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct BirchPressurePlate {
    powerable: Powerable,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct BirchSlab {
    double_slab_like: DoubleSlabLike,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct BirchStairs {
    stairs_like: StairsLike,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct BirchStandingSign {
    sign_like: SignLike,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct BirchTrapdoor {
    trapdoor_like: TrapdoorLike,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct BirchWallSign {
    wall_sign_like: WallSignLike,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct BirchWood {
    pillar_like: PillarLike,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct BlackCandle {
    candle_like: CandleLike,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct BlackCandleCake {
    light_like: LightLike,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct BlackCarpet {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct BlackConcrete {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct BlackConcretePowder {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct BlackGlazedTerracotta {
    glazed_terracotta_like: GlazedTerracottaLike,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct BlackShulkerBox {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct BlackStainedGlass {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct BlackStainedGlassPane {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct BlackTerracotta {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct BlackWool {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct Blackstone {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct BlackstoneDoubleSlab {
    double_slab_like: DoubleSlabLike,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct BlackstoneSlab {
    double_slab_like: DoubleSlabLike,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct BlackstoneStairs {
    stairs_like: StairsLike,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct BlackstoneWall {
    wall_like: WallLike,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct BlastFurnace {
    cardinal_direction_like: CardinalDirectionLike,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct BlueCandle {
    candle_like: CandleLike,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct BlueCandleCake {
    light_like: LightLike,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct BlueCarpet {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct BlueConcrete {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct BlueConcretePowder {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct BlueGlazedTerracotta {
    glazed_terracotta_like: GlazedTerracottaLike,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct BlueIce {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct BlueShulkerBox {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct BlueStainedGlass {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct BlueStainedGlassPane {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct BlueTerracotta {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct BlueWool {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct BoneBlock {
    pillar_like: PillarLike,
    deprecated: IntTag<BoneBlockDeprecated>,
}

#[allow(non_camel_case_types)]
#[derive(Serialize, Deserialize)]
pub enum BoneBlockDeprecated {
    Zero = 0,
    One,
    Two,
    Three,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct Bookshelf {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct BorderBlock {
    wall_like: WallLike,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct BrainCoral {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct BrewingStand {
    brewing_stand_slot_a_bit: BooleanTag,
    brewing_stand_slot_b_bit: BooleanTag,
    brewing_stand_slot_c_bit: BooleanTag,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct BrickBlock {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct BrickStairs {
    stairs_like: StairsLike,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct BrownCandle {
    candle_like: CandleLike,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct BrownCandleCake {
    light_like: LightLike,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct BrownCarpet {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct BrownConcrete {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct BrownConcretePowder {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct BrownGlazedTerracotta {
    glazed_terracotta_like: GlazedTerracottaLike,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct BrownMushroom {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct BrownMushroomBlock {
    huge_mushroom_bits: IntTag<HugeMushroomFaces>,
}

#[allow(non_camel_case_types)]
#[derive(Serialize, Deserialize)]
pub enum HugeMushroomFaces {
    Zero = 0,
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Eleven,
    Twelve,
    Thirteen,
    Fourteen,
    Fifteen,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct BrownShulkerBox {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct BrownStainedGlass {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct BrownStainedGlassPane {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct BrownTerracotta {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct BrownWool {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct BubbleColumn {
    drag_down: BooleanTag,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct BubbleCoral {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct BuddingAmethyst {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct Cactus {
    age: IntTag<CactusAge>,
}

#[allow(non_camel_case_types)]
#[derive(Serialize, Deserialize)]
pub enum CactusAge {
    Zero = 0,
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Eleven,
    Twelve,
    Thirteen,
    Fourteen,
    Fifteen,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct Cake {
    bite_counter: IntTag<CakeBites>,
}

#[allow(non_camel_case_types)]
#[derive(Serialize, Deserialize)]
pub enum CakeBites {
    Zero = 0,
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct Calcite {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct CalibratedSculkSensor {
    cardinal_direction_like: CardinalDirectionLike,
    sculk_sensor_phase: IntTag<CalibratedSculkSensorPhase>,
}

#[allow(non_camel_case_types)]
#[derive(Serialize, Deserialize)]
pub enum CalibratedSculkSensorPhase {
    Zero = 0,
    One,
    Two,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct Camera {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct Campfire {
    cardinal_direction_like: CardinalDirectionLike,
    extinguished: BooleanTag,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct Candle {
    candle_like: CandleLike,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct CandleCake {
    light_like: LightLike,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct Carrots {
    growth: IntTag<CarrotsGrowth>,
}

#[allow(non_camel_case_types)]
#[derive(Serialize, Deserialize)]
pub enum CarrotsGrowth {
    Zero = 0,
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct CartographyTable {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct CarvedPumpkin {
    cardinal_direction_like: CardinalDirectionLike,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct Cauldron {
    cauldron_liquid: StringTag<CauldronLiquid>,
    fill_level: IntTag<CauldronFillLevel>,
}

#[allow(non_camel_case_types)]
#[derive(Serialize, Deserialize)]
pub enum CauldronLiquid {
    water,
    lava,
    powder_snow,
}

#[allow(non_camel_case_types)]
#[derive(Serialize, Deserialize)]
pub enum CauldronFillLevel {
    Zero = 0,
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct CaveVines {
    growing_plant_age: IntTag<GrowingPlantAge>,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct CaveVinesBodyWithBerries {
    growing_plant_age: IntTag<GrowingPlantAge>,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct CaveVinesHeadWithBerries {
    growing_plant_age: IntTag<GrowingPlantAge>,
}

#[allow(non_camel_case_types)]
#[derive(Serialize, Deserialize)]
pub enum GrowingPlantAge {
    Zero = 0,
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Eleven,
    Twelve,
    Thirteen,
    Fourteen,
    Fifteen,
    Sixteen,
    Seventeen,
    Eighteen,
    Nineteen,
    Twenty,
    TwentyOne,
    TwentyTwo,
    TwentyThree,
    TwentyFour,
    TwentyFive,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct Chain {
    pillar_like: PillarLike,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct ChainCommandBlock {
    facing_direction_like: FacingDirectionLike,
    conditional_bit: BooleanTag,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct ChemicalHeat {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct ChemistryTable {
    direction_like: DirectionLike,
    chemistry_table_type: StringTag<ChemistryTableType>,
}

#[allow(non_camel_case_types)]
#[derive(Serialize, Deserialize)]
pub enum ChemistryTableType {
    compound_creator,
    material_reducer,
    element_constructor,
    lab_table,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct CherryButton {
    button_like: ButtonLike,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct CherryDoor {
    door_like: DoorLike,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct CherryDoubleSlab {
    double_slab_like: DoubleSlabLike,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct CherryFence {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct CherryFenceGate {
    fence_gate_like: FenceGateLike,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct CherryHangingSign {
    hanging_sign_like: HangingSignLike,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct CherryLeaves {
    leaves_like: LeavesLike,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct CherryLog {
    pillar_like: PillarLike,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct CherryPlanks {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct CherryPressurePlate {
    powerable: Powerable,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct CherrySapling {
    age_bit: BooleanTag,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct CherrySlab {
    double_slab_like: DoubleSlabLike,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct CherryStairs {
    stairs_like: StairsLike,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct CherryStandingSign {
    sign_like: SignLike,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct CherryTrapdoor {
    trapdoor_like: TrapdoorLike,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct CherryWallSign {
    wall_sign_like: WallSignLike,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct CherryWood {
    pillar_like: PillarLike,
    stripped_bit: BooleanTag,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct Chest {
    cardinal_direction_like: CardinalDirectionLike,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct ChiseledBookshelf {
    direction_like: DirectionLike,
    books_stored: IntTag<ChiseledBookshelfBooksStored>,
}

#[allow(non_camel_case_types)]
#[derive(Serialize, Deserialize)]
pub enum ChiseledBookshelfBooksStored {
    Zero = 0,
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Eleven,
    Twelve,
    Thirteen,
    Fourteen,
    Fifteen,
    Sixteen,
    Seventeen,
    Eighteen,
    Nineteen,
    Twenty,
    TwentyOne,
    TwentyTwo,
    TwentyThree,
    TwentyFour,
    TwentyFive,
    TwentySix,
    TwentySeven,
    TwentyEight,
    TwentyNine,
    Thirty,
    ThirtyOne,
    ThirtyTwo,
    ThirtyThree,
    ThirtyFour,
    ThirtyFive,
    ThirtySix,
    ThirtySeven,
    ThirtyEight,
    ThirtyNine,
    Forty,
    FortyOne,
    FortyTwo,
    FortyThree,
    FortyFour,
    FortyFive,
    FortySix,
    FortySeven,
    FortyEight,
    FortyNine,
    Fifty,
    FiftyOne,
    FiftyTwo,
    FiftyThree,
    FiftyFour,
    FiftyFive,
    FiftySix,
    FiftySeven,
    FiftyEight,
    FiftyNine,
    Sixty,
    SixtyOne,
    SixtyTwo,
    SixtyThree,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct ChiseledCopper {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct ChiseledDeepslate {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct ChiseledNetherBricks {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct ChiseledPolishedBlackstone {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct ChiseledTuff {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct ChiseledTuffBricks {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct ChorusFlower {
    age: IntTag<ChorusFlowerAge>,
}

#[allow(non_camel_case_types)]
#[derive(Serialize, Deserialize)]
pub enum ChorusFlowerAge {
    Zero = 0,
    One,
    Two,
    Three,
    Four,
    Five,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct ChorusPlant {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct Clay {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct ClientRequestPlaceholderBlock {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct CoalBlock {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct CoalOre {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct CobbledDeepslate {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct CobbledDeepslateDoubleSlab {
    double_slab_like: DoubleSlabLike,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct CobbledDeepslateSlab {
    double_slab_like: DoubleSlabLike,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct CobbledDeepslateStairs {
    stairs_like: StairsLike,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct CobbledDeepslateWall {
    wall_like: WallLike,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct Cobblestone {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct CobblestoneWall {
    wall_like: WallLike,
    wall_block_type: StringTag<WallBlockType>,
}

#[allow(non_camel_case_types)]
#[derive(Serialize, Deserialize)]
pub enum WallBlockType {
    cobblestone,
    mossy_cobblestone,
    granite,
    diorite,
    andesite,
    sandstone,
    brick,
    stone_brick,
    mossy_stone_brick,
    nether_brick,
    end_brick,
    prismarine,
    red_sandstone,
    red_nether_brick,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct Cocoa {
    direction_like: DirectionLike,
    age: IntTag<CocoaAge>,
}

#[allow(non_camel_case_types)]
#[derive(Serialize, Deserialize)]
pub enum CocoaAge {
    Zero = 0,
    One,
    Two,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct ColoredTorchBp {
    torch_like: TorchLike,
    color_bit: BooleanTag,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct ColoredTorchRg {
    torch_like: TorchLike,
    color_bit: BooleanTag,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct CommandBlock {
    facing_direction_like: FacingDirectionLike,
    conditional_bit: BooleanTag,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct Composter {
    composter_fill_level: IntTag<ComposterFillLevel>,
}

#[allow(non_camel_case_types)]
#[derive(Serialize, Deserialize)]
pub enum ComposterFillLevel {
    Zero = 0,
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct Conduit {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct CopperBlock {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct CopperBulb {
    light_like: LightLike,
    powered_bit: BooleanTag,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct CopperDoor {
    door_like: DoorLike,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct CopperGrate {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct CopperOre {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct CopperTrapdoor {
    trapdoor_like: TrapdoorLike,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct CoralBlock {
    coral_color: StringTag<CoralColor>,
    dead_bit: BooleanTag,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct CoralFan {
    coral_color: StringTag<CoralColor>,
    coral_fan_direction: IntTag<CoralFanDirection>,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct CoralFanDead {
    coral_color: StringTag<CoralColor>,
    coral_fan_direction: IntTag<CoralFanDirection>,
}

#[allow(non_camel_case_types)]
#[derive(Serialize, Deserialize)]
pub enum CoralColor {
    blue,
    pink,
    purple,
    red,
    yellow,
}

#[allow(non_camel_case_types)]
#[derive(Serialize, Deserialize)]
pub enum CoralFanDirection {
    Zero = 0,
    One,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct CoralFanHang {
    coral_direction: IntTag<Direction>,
    coral_hang_type_bit: BooleanTag,
    dead_bit: BooleanTag,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct CoralFanHang2 {
    coral_direction: IntTag<Direction>,
    coral_hang_type_bit: BooleanTag,
    dead_bit: BooleanTag,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct CoralFanHang3 {
    coral_direction: IntTag<Direction>,
    coral_hang_type_bit: BooleanTag,
    dead_bit: BooleanTag,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct CrackedDeepslateBricks {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct CrackedDeepslateTiles {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct CrackedNetherBricks {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct CrackedPolishedBlackstoneBricks {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct Crafter {
    crafting: BooleanTag,
    orientation: StringTag<CrafterOrientation>,
    triggered_bit: BooleanTag,
}

#[allow(non_camel_case_types)]
#[derive(Serialize, Deserialize)]
pub enum CrafterOrientation {
    down_east,
    down_north,
    down_south,
    down_west,
    up_east,
    up_north,
    up_south,
    up_west,
    west_up,
    east_up,
    north_up,
    south_up,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct CraftingTable {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct CrimsonButton {
    button_like: ButtonLike,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct CrimsonDoor {
    door_like: DoorLike,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct CrimsonDoubleSlab {
    double_slab_like: DoubleSlabLike,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct CrimsonFence {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct CrimsonFenceGate {
    fence_gate_like: FenceGateLike,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct CrimsonFungus {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct CrimsonHangingSign {
    hanging_sign_like: HangingSignLike,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct CrimsonHyphae {
    pillar_like: PillarLike,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct CrimsonNylium {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct CrimsonPlanks {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct CrimsonPressurePlate {
    powerable: Powerable,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct CrimsonRoots {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct CrimsonSlab {
    double_slab_like: DoubleSlabLike,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct CrimsonStairs {
    stairs_like: StairsLike,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct CrimsonStandingSign {
    sign_like: SignLike,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct CrimsonStem {
    pillar_like: PillarLike,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct CrimsonTrapdoor {
    trapdoor_like: TrapdoorLike,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct CrimsonWallSign {
    wall_sign_like: WallSignLike,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct CryingObsidian {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct CutCopper {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct CutCopperSlab {
    double_slab_like: DoubleSlabLike,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct CutCopperStairs {
    stairs_like: StairsLike,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct CyanCandle {
    candle_like: CandleLike,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct CyanCandleCake {
    light_like: LightLike,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct CyanCarpet {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct CyanConcrete {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct CyanConcretePowder {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct CyanGlazedTerracotta {
    glazed_terracotta_like: GlazedTerracottaLike,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct CyanShulkerBox {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct CyanStainedGlass {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct CyanStainedGlassPane {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct CyanTerracotta {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct CyanWool {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct DarkOakButton {
    button_like: ButtonLike,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct DarkOakDoor {
    door_like: DoorLike,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct DarkOakDoubleSlab {
    double_slab_like: DoubleSlabLike,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct DarkOakFence {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct DarkOakFenceGate {
    fence_gate_like: FenceGateLike,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct DarkOakHangingSign {
    hanging_sign_like: HangingSignLike,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct DarkOakLeaves {
    leaves_like: LeavesLike,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct DarkOakLog {
    pillar_like: PillarLike,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct DarkOakPlanks {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct DarkOakPressurePlate {
    powerable: Powerable,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct DarkOakSlab {
    double_slab_like: DoubleSlabLike,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct DarkOakStairs {
    stairs_like: StairsLike,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct DarkOakTrapdoor {
    trapdoor_like: TrapdoorLike,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct DarkOakWood {
    pillar_like: PillarLike,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct DarkPrismarineStairs {
    stairs_like: StairsLike,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct DarkOakStandingSign {
    sign_like: SignLike,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct DarkOakWallSign {
    wall_sign_like: WallSignLike,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct DaylightDetector {
    powerable: Powerable,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct DaylightDetectorInverted {
    powerable: Powerable,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct DeadBrainCoral {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct DeadBubbleCoral {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct DeadFireCoral {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct DeadHornCoral {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct DeadTubeCoral {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct DeadBush {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct DecoratedPot {
    direction_like: DirectionLike,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct Deepslate {
    pillar_like: PillarLike,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct DeepslateBrickDoubleSlab {
    double_slab_like: DoubleSlabLike,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct DeepslateBrickSlab {
    double_slab_like: DoubleSlabLike,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct DeepslateBrickStairs {
    stairs_like: StairsLike,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct DeepslateBrickWall {
    wall_like: WallLike,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct DeepslateBricks {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct DeepslateCoalOre {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct DeepslateCopperOre {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct DeepslateDiamondOre {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct DeepslateEmeraldOre {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct DeepslateGoldOre {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct DeepslateIronOre {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct DeepslateLapisOre {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct DeepslateRedstoneOre {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct DeepslateTileDoubleSlab {
    double_slab_like: DoubleSlabLike,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct DeepslateTileSlab {
    double_slab_like: DoubleSlabLike,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct DeepslateTileStairs {
    stairs_like: StairsLike,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct DeepslateTileWall {
    wall_like: WallLike,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct DeepslateTiles {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct Deny {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct DetectorRail {
    rail_data_bit: BooleanTag,
    rail_direction: IntTag<DetectorRailDirection>,
}

#[allow(non_camel_case_types)]
#[derive(Serialize, Deserialize)]
pub enum DetectorRailDirection {
    Zero = 0,
    One,
    Two,
    Three,
    Four,
    Five,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct DiamondBlock {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct DiamondOre {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct Diorite {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct DioriteStairs {
    stairs_like: StairsLike,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct Dirt {
    dirt_type: StringTag<DirtType>,
}

#[allow(non_camel_case_types)]
#[derive(Serialize, Deserialize)]
pub enum DirtType {
    normal,
    coarse,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct DirtWithRoots {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct Dispenser {
    facing_direction_like: FacingDirectionLike,
    triggered_bit: BooleanTag,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct DoubleCutCopperSlab {
    double_slab_like: DoubleSlabLike,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct DoublePlant {
    double_plant_type: StringTag<DoublePlantType>,
    upper_block_bit: BooleanTag,
}

#[allow(non_camel_case_types)]
#[derive(Serialize, Deserialize)]
pub enum DoublePlantType {
    sunflower,
    syringa,
    grass,
    fern,
    rose,
    paeonia,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct DoubleStoneBlockSlab {
    double_slab_like: DoubleSlabLike,
    stone_slab_type: StringTag<StoneSlabType>,
}

#[allow(non_camel_case_types)]
#[derive(Serialize, Deserialize)]
pub enum StoneSlabType {
    smooth_stone,
    sandstone,
    wood,
    cobblestone,
    brick,
    stone_brick,
    quartz,
    nether_brick,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct DoubleStoneBlockSlab2 {
    double_slab_like: DoubleSlabLike,
    stone_slab_type_2: StringTag<StoneSlabType2>,
}

#[allow(non_camel_case_types)]
#[derive(Serialize, Deserialize)]
pub enum StoneSlabType2 {
    red_sandstone,
    purpur,
    prismarine_rough,
    prismarine_dark,
    prismarine_brick,
    mossy_cobblestone,
    smooth_sandstone,
    red_nether_brick,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct DoubleStoneBlockSlab3 {
    double_slab_like: DoubleSlabLike,
    stone_slab_type_3: StringTag<StoneSlabType3>,
}

#[allow(non_camel_case_types)]
#[derive(Serialize, Deserialize)]
pub enum StoneSlabType3 {
    end_stone_brick,
    smooth_red_sandstone,
    polished_andesite,
    andesite,
    diorite,
    polished_diorite,
    granite,
    polished_granite,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct DoubleStoneBlockSlab4 {
    double_slab_like: DoubleSlabLike,
    stone_slab_type_4: StringTag<StoneSlabType4>,
}

#[allow(non_camel_case_types)]
#[derive(Serialize, Deserialize)]
pub enum StoneSlabType4 {
    mossy_stone_brick,
    smooth_quartz,
    stone,
    cut_sandstone,
    cut_red_sandstone,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct DragonEgg {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct DriedKelpBlock {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct DripstoneBlock {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct Dropper {
    facing_direction_like: FacingDirectionLike,
    triggered_bit: BooleanTag,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct Element0 {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct Element1 {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct Element10 {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct Element100 {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct Element101 {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct Element102 {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct Element103 {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct Element104 {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct Element105 {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct Element106 {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct Element107 {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct Element108 {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct Element109 {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct Element11 {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct Element110 {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct Element111 {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct Element112 {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct Element113 {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct Element114 {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct Element115 {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct Element116 {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct Element117 {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct Element118 {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct Element12 {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct Element13 {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct Element14 {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct Element15 {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct Element16 {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct Element17 {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct Element18 {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct Element19 {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct Element2 {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct Element20 {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct Element21 {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct Element22 {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct Element23 {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct Element24 {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct Element25 {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct Element26 {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct Element27 {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct Element28 {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct Element29 {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct Element3 {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct Element30 {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct Element31 {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct Element32 {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct Element33 {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct Element34 {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct Element35 {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct Element36 {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct Element37 {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct Element38 {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct Element39 {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct Element4 {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct Element40 {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct Element41 {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct Element42 {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct Element43 {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct Element44 {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct Element45 {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct Element46 {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct Element47 {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct Element48 {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct Element49 {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct Element5 {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct Element50 {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct Element51 {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct Element52 {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct Element53 {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct Element54 {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct Element55 {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct Element56 {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct Element57 {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct Element58 {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct Element59 {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct Element6 {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct Element60 {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct Element61 {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct Element62 {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct Element63 {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct Element64 {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct Element65 {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct Element66 {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct Element67 {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct Element68 {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct Element69 {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct Element7 {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct Element70 {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct Element71 {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct Element72 {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct Element73 {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct Element74 {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct Element75 {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct Element76 {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct Element77 {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct Element78 {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct Element79 {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct Element8 {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct Element80 {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct Element81 {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct Element82 {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct Element83 {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct Element84 {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct Element85 {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct Element86 {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct Element87 {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct Element88 {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct Element89 {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct Element9 {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct Element90 {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct Element91 {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct Element92 {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct Element93 {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct Element94 {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct Element95 {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct Element96 {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct Element97 {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct Element98 {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct Element99 {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct EmeraldBlock {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct EmeraldOre {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct EnchantingTable {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct EndBrickStairs {
    stairs_like: StairsLike,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct EndBricks {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct EndGateway {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct EndPortal {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct EndPortalFrame {
    cardinal_direction_like: CardinalDirectionLike,
    end_portal_eye_bit: BooleanTag,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct EndRod {
    facing_direction_like: FacingDirectionLike,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct EndStone {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct EnderChest {
    cardinal_direction_like: CardinalDirectionLike,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct ExposedChiseledCopper {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct ExposedCopper {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct ExposedCopperBulb {
    light_like: LightLike,
    powered_bit: BooleanTag,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct ExposedCopperDoor {
    door_like: DoorLike,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct ExposedCopperGrate {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct ExposedCopperTrapdoor {
    trapdoor_like: TrapdoorLike,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct ExposedCutCopper {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct ExposedCutCopperSlab {
    double_slab_like: DoubleSlabLike,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct ExposedCutCopperStairs {
    stairs_like: StairsLike,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct ExposedDoubleCutCopperSlab {
    double_slab_like: DoubleSlabLike,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct Farmland {
    moisturized_amount: IntTag<FarmlandMoisturizedAmount>,
}

#[allow(non_camel_case_types)]
#[derive(Serialize, Deserialize)]
pub enum FarmlandMoisturizedAmount {
    Zero = 0,
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct FenceGate {
    fence_gate_like: FenceGateLike,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct Fire {
    age: IntTag<FireAge>,
}

#[allow(non_camel_case_types)]
#[derive(Serialize, Deserialize)]
pub enum FireAge {
    Zero = 0,
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Eleven,
    Twelve,
    Thirteen,
    Fourteen,
    Fifteen,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct FireCoral {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct FletchingTable {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct FlowerPot {
    update_bit: BooleanTag,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct FloweringAzalea {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct FlowingLava {
    liquid_depth: IntTag<LiquidDepth>,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct FlowingWater {
    liquid_depth: IntTag<LiquidDepth>,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct Frame {
    facing_direction_like: FacingDirectionLike,
    item_frame_map_bit: BooleanTag,
    item_frame_photo_bit: BooleanTag,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct FrogSpawn {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct FrostedIce {
    age: IntTag<FrostedIceAge>,
}

// lol, the name 🦣
#[allow(non_camel_case_types)]
#[derive(Serialize, Deserialize)]
pub enum FrostedIceAge {
    Zero = 0,
    One,
    Two,
    Three,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct Furnace {
    cardinal_direction_like: CardinalDirectionLike,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct GildedBlackstone {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct Glass {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct GlassPane {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct GlowFrame {
    facing_direction_like: FacingDirectionLike,
    item_frame_map_bit: BooleanTag,
    item_frame_photo_bit: BooleanTag,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct GlowLichen {
    multi_face_direction_bits: IntTag<GlowLichenMultiFaceDirection>,
}

#[allow(non_camel_case_types)]
#[derive(Serialize, Deserialize)]
pub enum GlowLichenMultiFaceDirection {
    Zero = 0,
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Eleven,
    Twelve,
    Thirteen,
    Fourteen,
    Fifteen,
    Sixteen,
    Seventeen,
    Eighteen,
    Nineteen,
    Twenty,
    TwentyOne,
    TwentyTwo,
    TwentyThree,
    TwentyFour,
    TwentyFive,
    TwentySix,
    TwentySeven,
    TwentyEight,
    TwentyNine,
    Thirty,
    ThirtyOne,
    ThirtyTwo,
    ThirtyThree,
    ThirtyFour,
    ThirtyFive,
    ThirtySix,
    ThirtySeven,
    ThirtyEight,
    ThirtyNine,
    Forty,
    FortyOne,
    FortyTwo,
    FortyThree,
    FortyFour,
    FortyFive,
    FortySix,
    FortySeven,
    FortyEight,
    FortyNine,
    Fifty,
    FiftyOne,
    FiftyTwo,
    FiftyThree,
    FiftyFour,
    FiftyFive,
    FiftySix,
    FiftySeven,
    FiftyEight,
    FiftyNine,
    Sixty,
    SixtyOne,
    SixtyTwo,
    SixtyThree,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct GlowingObsidian {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct Glowstone {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct GoldBlock {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct GoldOre {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct GoldenRail {
    rail_data_bit: BooleanTag,
    rail_direction: IntTag<GoldenRailDirection>,
}

#[allow(non_camel_case_types)]
#[derive(Serialize, Deserialize)]
pub enum GoldenRailDirection {
    Zero = 0,
    One,
    Two,
    Three,
    Four,
    Five,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct Granite {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct GraniteStairs {
    stairs_like: StairsLike,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct GrassBlock {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct GrassPath {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct Gravel {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct GrayCandle {
    candle_like: CandleLike,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct GrayCandleCake {
    light_like: LightLike,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct GrayCarpet {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct GrayConcrete {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct GrayConcretePowder {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct GrayGlazedTerracotta {
    glazed_terracotta_like: GlazedTerracottaLike,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct GrayShulkerBox {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct GrayStainedGlass {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct GrayStainedGlassPane {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct GrayTerracotta {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct GrayWool {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct GreenCandle {
    candle_like: CandleLike,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct GreenCandleCake {
    light_like: LightLike,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct GreenCarpet {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct GreenConcrete {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct GreenConcretePowder {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct GreenGlazedTerracotta {
    glazed_terracotta_like: GlazedTerracottaLike,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct GreenShulkerBox {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct GreenStainedGlass {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct GreenStainedGlassPane {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct GreenTerracotta {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct GreenWool {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct Grindstone {
    direction_like: DirectionLike,
    attachment: StringTag<GrindstoneAttachment>,
}

#[allow(non_camel_case_types)]
#[derive(Serialize, Deserialize)]
pub enum GrindstoneAttachment {
    standing,
    hanging,
    side,
    multiple,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct HangingRoots {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct HardBlackStainedGlass {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct HardBlackStainedGlassPane {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct HardBlueStainedGlass {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct HardBlueStainedGlassPane {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct HardBrownStainedGlass {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct HardBrownStainedGlassPane {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct HardCyanStainedGlass {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct HardCyanStainedGlassPane {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct HardGlass {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct HardGlassPane {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct HardGrayStainedGlass {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct HardGrayStainedGlassPane {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct HardGreenStainedGlass {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct HardGreenStainedGlassPane {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct HardLightBlueStainedGlass {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct HardLightBlueStainedGlassPane {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct HardLightGrayStainedGlass {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct HardLightGrayStainedGlassPane {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct HardLimeStainedGlass {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct HardLimeStainedGlassPane {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct HardMagentaStainedGlass {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct HardMagentaStainedGlassPane {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct HardOrangeStainedGlass {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct HardOrangeStainedGlassPane {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct HardPinkStainedGlass {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct HardPinkStainedGlassPane {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct HardPurpleStainedGlass {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct HardPurpleStainedGlassPane {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct HardRedStainedGlass {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct HardRedStainedGlassPane {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct HardWhiteStainedGlass {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct HardWhiteStainedGlassPane {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct HardYellowStainedGlass {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct HardYellowStainedGlassPane {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct HardenedClay {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct HayBlock {
    pillar_like: PillarLike,
    deprecated: IntTag<HayBlockDeprecated>,
}

// old direction value?
#[allow(non_camel_case_types)]
#[derive(Serialize, Deserialize)]
pub enum HayBlockDeprecated {
    Zero = 0,
    One,
    Two,
    Three,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct HeavyWeightedPressurePlate {
    powerable: Powerable,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct HoneyBlock {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct HoneycombBlock {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct Hopper {
    facing_direction_like: FacingDirectionLike,
    toggle_bit: BooleanTag,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct HornCoral {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct Ice {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct InfestedDeepslate {
    pillar_like: PillarLike,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct InfoUpdate {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct InfoUpdate2 {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct InvisibleBedrock {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct IronBars {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct IronBlock {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct IronDoor {
    door_like: DoorLike,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct IronOre {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct IronTrapdoor {
    trapdoor_like: TrapdoorLike,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct Jigsaw {
    facing_direction_like: FacingDirectionLike,
    rotation: IntTag<JigsawRotation>,
}

#[allow(non_camel_case_types)]
#[derive(Serialize, Deserialize)]
pub enum JigsawRotation {
    Zero = 0,
    One,
    Two,
    Three,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct Jukebox {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct JungleButton {
    button_like: ButtonLike,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct JungleDoor {
    door_like: DoorLike,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct JungleDoubleSlab {
    double_slab_like: DoubleSlabLike,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct JungleFence {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct JungleFenceGate {
    fence_gate_like: FenceGateLike,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct JungleHangingSign {
    hanging_sign_like: HangingSignLike,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct JungleLeaves {
    leaves_like: LeavesLike,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct JungleLog {
    pillar_like: PillarLike,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct JunglePlanks {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct JunglePressurePlate {
    powerable: Powerable,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct JungleSlab {
    double_slab_like: DoubleSlabLike,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct JungleStairs {
    stairs_like: StairsLike,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct JungleStandingSign {
    sign_like: SignLike,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct JungleTrapdoor {
    trapdoor_like: TrapdoorLike,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct JungleWallSign {
    wall_sign_like: WallSignLike,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct JungleWood {
    pillar_like: PillarLike,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct Kelp {
    kelp_age: IntTag<KelpAge>,
}

#[allow(non_camel_case_types)]
#[derive(Serialize, Deserialize)]
pub enum KelpAge {
    Zero = 0,
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Eleven,
    Twelve,
    Thirteen,
    Fourteen,
    Fifteen,
    Sixteen,
    Seventeen,
    Eighteen,
    Nineteen,
    Twenty,
    TwentyOne,
    TwentyTwo,
    TwentyThree,
    TwentyFour,
    TwentyFive,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct Ladder {
    facing_direction_like: FacingDirectionLike,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct Lantern {
    hanging: BooleanTag,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct LapisBlock {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct LapisOre {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct LargeAmethystBud {
    #[serde(rename = "minecraft:block_face")]
    block_face: StringTag<LargeAmethystBudBlockFace>,
}

#[allow(non_camel_case_types)]
#[derive(Serialize, Deserialize)]
pub enum LargeAmethystBudBlockFace {
    down,
    up,
    north,
    south,
    west,
    east,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct Lava {
    liquid_depth: IntTag<LiquidDepth>,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct Lectern {
    cardinal_direction_like: CardinalDirectionLike,
    powered_bit: BooleanTag,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct Lever {
    lever_direction: StringTag<LeverDirection>,
    open_bit: BooleanTag,
}

#[allow(non_camel_case_types)]
#[derive(Serialize, Deserialize)]
pub enum LeverDirection {
    down_east_west,
    east,
    west,
    south,
    north,
    up_north_south,
    up_east_west,
    down_north_south,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct LightBlock {
    block_light_level: IntTag<LightBlockLightLevel>,
}

#[allow(non_camel_case_types)]
#[derive(Serialize, Deserialize)]
pub enum LightBlockLightLevel {
    Zero = 0,
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Eleven,
    Twelve,
    Thirteen,
    Fourteen,
    Fifteen,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct LightBlueCandle {
    candle_like: CandleLike,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct LightBlueCandleCake {
    light_like: LightLike,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct LightBlueCarpet {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct LightBlueConcrete {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct LightBlueConcretePowder {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct LightBlueGlazedTerracotta {
    glazed_terracotta_like: GlazedTerracottaLike,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct LightBlueShulkerBox {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct LightBlueStainedGlass {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct LightBlueStainedGlassPane {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct LightBlueTerracotta {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct LightBlueWool {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct LightGrayCandle {
    candle_like: CandleLike,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct LightGrayCandleCake {
    light_like: LightLike,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct LightGrayCarpet {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct LightGrayConcrete {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct LightGrayConcretePowder {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct LightGrayShulkerBox {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct LightGrayStainedGlass {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct LightGrayStainedGlassPane {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct LightGrayTerracotta {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct LightGrayWool {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct LightWeightedPressurePlate {
    powerable: Powerable,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct LightningRod {
    facing_direction_like: FacingDirectionLike,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct LimeCandle {
    candle_like: CandleLike,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct LimeCandleCake {
    light_like: LightLike,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct LimeCarpet {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct LimeConcrete {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct LimeConcretePowder {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct LimeGlazedTerracotta {
    glazed_terracotta_like: GlazedTerracottaLike,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct LimeShulkerBox {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct LimeStainedGlass {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct LimeStainedGlassPane {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct LimeTerracotta {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct LimeWool {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct LitBlastFurnace {
    cardinal_direction_like: CardinalDirectionLike,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct LitDeepslateRedstoneOre {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct LitFurnace {
    cardinal_direction_like: CardinalDirectionLike,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct LitPumpkin {
    cardinal_direction_like: CardinalDirectionLike,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct LitRedstoneLamp {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct LitRedstoneOre {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct LitSmoker {
    cardinal_direction_like: CardinalDirectionLike,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct Lodestone {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct Loom {
    direction_like: DirectionLike,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct MagentaCandle {
    candle_like: CandleLike,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct MagentaCandleCake {
    light_like: LightLike,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct MagentaCarpet {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct MagentaConcrete {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct MagentaConcretePowder {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct MagentaGlazedTerracotta {
    glazed_terracotta_like: GlazedTerracottaLike,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct MagentaShulkerBox {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct MagentaStainedGlass {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct MagentaStainedGlassPane {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct MagentaTerracotta {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct MagentaWool {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct Magma {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct MangroveButton {
    button_like: ButtonLike,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct MangroveDoor {
    door_like: DoorLike,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct MangroveDoubleSlab {
    double_slab_like: DoubleSlabLike,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct MangroveFence {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct MangroveFenceGate {
    fence_gate_like: FenceGateLike,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct MangroveHangingSign {
    hanging_sign_like: HangingSignLike,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct MangroveLeaves {
    leaves_like: LeavesLike,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct MangroveLog {
    pillar_like: PillarLike,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct MangrovePlanks {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct MangrovePressurePlate {
    powerable: Powerable,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct MangrovePropagule {
    hanging: BooleanTag,
    propagule_stage: IntTag<MangrovePropaguleStage>,
}

#[allow(non_camel_case_types)]
#[derive(Serialize, Deserialize)]
pub enum MangrovePropaguleStage {
    Zero = 0,
    One,
    Two,
    Three,
    Four,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct MangroveRoots {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct MangroveSlab {
    double_slab_like: DoubleSlabLike,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct MangroveStairs {
    stairs_like: StairsLike,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct MangroveStandingSign {
    sign_like: SignLike,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct MangroveTrapdoor {
    trapdoor_like: TrapdoorLike,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct MangroveWallSign {
    wall_sign_like: WallSignLike,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct MangroveWood {
    pillar_like: PillarLike,
    stripped_bit: BooleanTag,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct MediumAmethystBud {
    #[serde(rename = "minecraft:block_face")]
    block_face: StringTag<MediumAmethystBudBlockFace>,
}

#[allow(non_camel_case_types)]
#[derive(Serialize, Deserialize)]
pub enum MediumAmethystBudBlockFace {
    down,
    up,
    north,
    south,
    west,
    east,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct MelonBlock {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct MelonStem {
    facing_direction_like: FacingDirectionLike,
    growth: IntTag<MelonStemGrowth>,
}

#[allow(non_camel_case_types)]
#[derive(Serialize, Deserialize)]
pub enum MelonStemGrowth {
    Zero = 0,
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct MobSpawner {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct MonsterEgg {
    monster_egg_stone_type: StringTag<MonsterEggStoneType>,
}

#[allow(non_camel_case_types)]
#[derive(Serialize, Deserialize)]
pub enum MonsterEggStoneType {
    stone,
    cobblestone,
    stone_brick,
    mossy_stone_brick,
    cracked_stone_brick,
    chiseled_stone_brick,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct MossBlock {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct MossCarpet {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct MossyCobblestone {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct MossyCobblestoneStairs {
    stairs_like: StairsLike,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct MossyStoneBrickStairs {
    stairs_like: StairsLike,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct MovingBlock {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct Mud {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct MudBrickDoubleSlab {
    double_slab_like: DoubleSlabLike,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct MudBrickSlab {
    double_slab_like: DoubleSlabLike,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct MudBrickStairs {
    stairs_like: StairsLike,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct MudBrickWall {
    wall_like: WallLike,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct MudBricks {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct MuddyMangroveRoots {
    pillar_like: PillarLike,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct Mycelium {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct NetherBrick {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct NetherBrickFence {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct NetherBrickStairs {
    stairs_like: StairsLike,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct NetherGoldOre {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct NetherSprouts {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct NetherWart {
    age: IntTag<NetherWartAge>,
}

#[allow(non_camel_case_types)]
#[derive(Serialize, Deserialize)]
pub enum NetherWartAge {
    Zero = 0,
    One,
    Two,
    Three,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct NetherWartBlock {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct NetheriteBlock {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct Netherrack {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct NetherReactorCore {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct NormalStoneStairs {
    stairs_like: StairsLike,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct NoteBlock {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct OakDoubleSlab {
    double_slab_like: DoubleSlabLike,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct OakFence {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct OakHangingSign {
    hanging_sign_like: HangingSignLike,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct OakLeaves {
    leaves_like: LeavesLike,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct OakLog {
    pillar_like: PillarLike,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct OakPlanks {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct OakSlab {
    double_slab_like: DoubleSlabLike,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct OakStairs {
    stairs_like: StairsLike,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct OakWood {
    pillar_like: PillarLike,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct Observer {
    #[serde(rename = "minecraft:facing_direction")]
    facing_direction: StringTag<ObserverFacingDirection>,
    powered_bit: BooleanTag,
}

#[allow(non_camel_case_types)]
#[derive(Serialize, Deserialize)]
pub enum ObserverFacingDirection {
    down,
    up,
    north,
    south,
    west,
    east,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct Obsidian {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct OchreFroglight {
    pillar_like: PillarLike,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct OrangeCandle {
    candle_like: CandleLike,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct OrangeCandleCake {
    light_like: LightLike,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct OrangeCarpet {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct OrangeConcrete {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct OrangeConcretePowder {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct OrangeGlazedTerracotta {
    glazed_terracotta_like: GlazedTerracottaLike,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct OrangeShulkerBox {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct OrangeStainedGlass {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct OrangeStainedGlassPane {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct OrangeTerracotta {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct OrangeWool {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct OxidizedChiseledCopper {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct OxidizedCopper {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct OxidizedCopperBulb {
    light_like: LightLike,
    powered_bit: BooleanTag,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct OxidizedCopperDoor {
    door_like: DoorLike,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct OxidizedCopperGrate {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct OxidizedCopperTrapdoor {
    trapdoor_like: TrapdoorLike,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct OxidizedCutCopper {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct OxidizedCutCopperSlab {
    double_slab_like: DoubleSlabLike,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct OxidizedCutCopperStairs {
    stairs_like: StairsLike,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct OxidizedDoubleCutCopperSlab {
    double_slab_like: DoubleSlabLike,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct PackedIce {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct PackedMud {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct PearlescentFroglight {
    pillar_like: PillarLike,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct PinkCandle {
    candle_like: CandleLike,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct PinkCandleCake {
    light_like: LightLike,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct PinkCarpet {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct PinkConcrete {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct PinkConcretePowder {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct PinkGlazedTerracotta {
    glazed_terracotta_like: GlazedTerracottaLike,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct PinkPetals {
    cardinal_direction_like: CardinalDirectionLike,
    growth: IntTag<PinkPetalsGrowth>,
}

#[allow(non_camel_case_types)]
#[derive(Serialize, Deserialize)]
pub enum PinkPetalsGrowth {
    Zero = 0,
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct PinkShulkerBox {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct PinkStainedGlass {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct PinkStainedGlassPane {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct PinkTerracotta {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct PinkWool {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct Piston {
    facing_direction_like: FacingDirectionLike,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct PistonArmCollision {
    facing_direction_like: FacingDirectionLike,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct PitcherCrop {
    growth: IntTag<PitcherCropGrowth>,
    upper_block_bit: BooleanTag,
}

#[allow(non_camel_case_types)]
#[derive(Serialize, Deserialize)]
pub enum PitcherCropGrowth {
    Zero = 0,
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct PitcherPlant {
    upper_block_bit: BooleanTag,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct Podzol {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct PointedDripstone {
    dripstone_thickness: StringTag<PointedDripstoneThickness>,
    hanging: BooleanTag,
}

#[allow(non_camel_case_types)]
#[derive(Serialize, Deserialize)]
pub enum PointedDripstoneThickness {
    tip,
    frustum,
    middle,
    base,
    merge,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct PolishedAndesite {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct PolishedAndesiteStairs {
    stairs_like: StairsLike,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct PolishedBasalt {
    pillar_like: PillarLike,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct PolishedBlackstone {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct PolishedBlackstoneBrickDoubleSlab {
    double_slab_like: DoubleSlabLike,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct PolishedBlackstoneBrickSlab {
    double_slab_like: DoubleSlabLike,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct PolishedBlackstoneBrickStairs {
    stairs_like: StairsLike,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct PolishedBlackstoneBrickWall {
    wall_like: WallLike,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct PolishedBlackstoneBricks {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct PolishedBlackstoneButton {
    button_like: ButtonLike,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct PolishedBlackstoneDoubleSlab {
    double_slab_like: DoubleSlabLike,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct PolishedBlackstonePressurePlate {
    powerable: Powerable,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct PolishedBlackstoneSlab {
    double_slab_like: DoubleSlabLike,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct PolishedBlackstoneStairs {
    stairs_like: StairsLike,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct PolishedBlackstoneWall {
    wall_like: WallLike,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct PolishedDeepslate {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct PolishedDeepslateDoubleSlab {
    double_slab_like: DoubleSlabLike,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct PolishedDeepslateSlab {
    double_slab_like: DoubleSlabLike,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct PolishedDeepslateStairs {
    stairs_like: StairsLike,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct PolishedDeepslateWall {
    wall_like: WallLike,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct PolishedDiorite {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct PolishedDioriteStairs {
    stairs_like: StairsLike,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct PolishedGranite {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct PolishedGraniteStairs {
    stairs_like: StairsLike,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct PolishedTuff {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct PolishedTuffDoubleSlab {
    double_slab_like: DoubleSlabLike,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct PolishedTuffSlab {
    double_slab_like: DoubleSlabLike,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct PolishedTuffStairs {
    stairs_like: StairsLike,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct PolishedTuffWall {
    wall_like: WallLike,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct Portal {
    portal_axis: StringTag<PortalAxis>,
}

#[allow(non_camel_case_types)]
#[derive(Serialize, Deserialize)]
pub enum PortalAxis {
    unknown,
    x,
    z,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct Potatoes {
    growth: IntTag<PotatoesGrowth>,
}

#[allow(non_camel_case_types)]
#[derive(Serialize, Deserialize)]
pub enum PotatoesGrowth {
    Zero = 0,
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct PowderSnow {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct PoweredComparator {
    cardinal_direction_like: CardinalDirectionLike,
    output_lit_bit: BooleanTag,
    output_subtract_bit: BooleanTag,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct PoweredRepeater {
    cardinal_direction_like: CardinalDirectionLike,
    repeater_delay: IntTag<PoweredRepeaterDelay>,
}

#[allow(non_camel_case_types)]
#[derive(Serialize, Deserialize)]
pub enum PoweredRepeaterDelay {
    Zero = 0,
    One,
    Two,
    Three,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct Prismarine {
    prismarine_block_type: StringTag<PrismarineBlockType>,
}

#[allow(non_camel_case_types)]
#[derive(Serialize, Deserialize)]
pub enum PrismarineBlockType {
    default,
    dark,
    bricks,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct PrismarineBricksStairs {
    stairs_like: StairsLike,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct PrismarineStairs {
    stairs_like: StairsLike,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct Pumpkin {
    cardinal_direction_like: CardinalDirectionLike,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct PumpkinStem {
    facing_direction_like: FacingDirectionLike,
    growth: IntTag<PumpkinStepGrowth>,
}

#[allow(non_camel_case_types)]
#[derive(Serialize, Deserialize)]
pub enum PumpkinStepGrowth {
    Zero = 0,
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct PurpleCandle {
    candle_like: CandleLike,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct PurpleCandleCake {
    light_like: LightLike,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct PurpleCarpet {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct PurpleConcrete {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct PurpleConcretePowder {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct PurpleGlazedTerracotta {
    glazed_terracotta_like: GlazedTerracottaLike,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct PurpleShulkerBox {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct PurpleStainedGlass {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct PurpleStainedGlassPane {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct PurpleTerracotta {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct PurpleWool {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct PurpurBlock {
    pillar_like: PillarLike,
    chisel_type: StringTag<PurpurBlockChiselType>,
}

#[allow(non_camel_case_types)]
#[derive(Serialize, Deserialize)]
pub enum PurpurBlockChiselType {
    default,
    chiseled,
    lines,
    smooth,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct PurpurStairs {
    stairs_like: StairsLike,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct QuartzBlock {
    pillar_like: PillarLike,
    chisel_type: StringTag<QuartzBlockChiselType>,
}

#[allow(non_camel_case_types)]
#[derive(Serialize, Deserialize)]
pub enum QuartzBlockChiselType {
    default,
    chiseled,
    lines,
    smooth,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct QuartzBricks {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct QuartzOre {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct QuartzStairs {
    stairs_like: StairsLike,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct Rail {
    rail_direction: IntTag<RailDirection>,
}

#[allow(non_camel_case_types)]
#[derive(Serialize, Deserialize)]
pub enum RailDirection {
    Zero = 0,
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct RawCopperBlock {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct RawGoldBlock {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct RawIronBlock {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct RedCandle {
    candle_like: CandleLike,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct RedCandleCake {
    light_like: LightLike,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct RedCarpet {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct RedConcrete {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct RedConcretePowder {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct RedFlower {
    flower_type: StringTag<FlowerType>,
}

#[allow(non_camel_case_types)]
#[derive(Serialize, Deserialize)]
pub enum FlowerType {
    poppy,
    orchid,
    allium,
    houstonia,
    tulip_red,
    tulip_orange,
    tulip_white,
    tulip_pink,
    oxeye,
    cornflower,
    lily_of_the_valley,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct RedGlazedTerracotta {
    glazed_terracotta_like: GlazedTerracottaLike,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct RedMushroom {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct RedMushroomBlock {
    huge_mushroom_bits: IntTag<HugeMushroomFaces>,
}

#[allow(non_camel_case_types)]
#[derive(Serialize, Deserialize)]
pub enum HugeRedMushroomFaces {
    Zero = 0,
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Eleven,
    Twelve,
    Thirteen,
    Fourteen,
    Fifteen,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct RedNetherBrick {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct RedNetherBrickStairs {
    stairs_like: StairsLike,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct RedSandstone {
    sand_stone_type: StringTag<RedSandstoneType>,
}

#[allow(non_camel_case_types)]
#[derive(Serialize, Deserialize)]
pub enum RedSandstoneType {
    default,
    heiroglyphs,
    cut,
    smooth,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct RedSandstoneStairs {
    stairs_like: StairsLike,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct RedShulkerBox {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct RedStainedGlass {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct RedStainedGlassPane {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct RedTerracotta {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct RedWool {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct RedstoneBlock {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct RedstoneLamp {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct RedstoneOre {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct RedstoneTorch {
    torch_like: TorchLike,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct RedstoneWire {
    powerable: Powerable,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct Reeds {
    age: IntTag<ReedsAge>,
}

#[allow(non_camel_case_types)]
#[derive(Serialize, Deserialize)]
pub enum ReedsAge {
    Zero = 0,
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Eleven,
    Twelve,
    Thirteen,
    Fourteen,
    Fifteen,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct ReinforcedDeepslate {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct RepeatingCommandBlock {
    facing_direction_like: FacingDirectionLike,
    conditional_bit: BooleanTag,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct Reserved6 {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct RespawnAnchor {
    respawn_anchor_charge: IntTag<RespawnAnchorCharge>,
}

#[allow(non_camel_case_types)]
#[derive(Serialize, Deserialize)]
pub enum RespawnAnchorCharge {
    Zero = 0,
    One,
    Two,
    Three,
    Four,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct Sand {
    sand_type: StringTag<SandType>,
}

#[allow(non_camel_case_types)]
#[derive(Serialize, Deserialize)]
pub enum SandType {
    normal,
    red,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct Sandstone {
    sand_stone_type: StringTag<SandstoneType>,
}

#[allow(non_camel_case_types)]
#[derive(Serialize, Deserialize)]
pub enum SandstoneType {
    default,
    heiroglyphs,
    cut,
    smooth,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct SandstoneStairs {
    stairs_like: StairsLike,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct Sapling {
    age_bit: BooleanTag,
    sapling_type: StringTag<SaplingType>,
}

#[allow(non_camel_case_types)]
#[derive(Serialize, Deserialize)]
pub enum SaplingType {
    oak,
    spruce,
    birch,
    jungle,
    acacia,
    dark_oak,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct Scaffolding {
    stability: IntTag<ScaffoldingStability>,
    stability_check: BooleanTag,
}

#[allow(non_camel_case_types)]
#[derive(Serialize, Deserialize)]
pub enum ScaffoldingStability {
    Zero = 0,
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct Sculk {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct SculkCatalyst {
    bloom: BooleanTag,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct SculkSensor {
    sculk_sensor_phase: IntTag<SculkSensorPhase>,
}

#[allow(non_camel_case_types)]
#[derive(Serialize, Deserialize)]
pub enum SculkSensorPhase {
    Zero = 0,
    One,
    Two,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct SculkShrieker {
    active: BooleanTag,
    can_summon: BooleanTag,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct SculkVein {
    multi_face_direction_bits: IntTag<SculkVeinMultiFaceDirection>,
}

#[allow(non_camel_case_types)]
#[derive(Serialize, Deserialize)]
pub enum SculkVeinMultiFaceDirection {
    Zero = 0,
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Eleven,
    Twelve,
    Thirteen,
    Fourteen,
    Fifteen,
    Sixteen,
    Seventeen,
    Eighteen,
    Nineteen,
    Twenty,
    TwentyOne,
    TwentyTwo,
    TwentyThree,
    TwentyFour,
    TwentyFive,
    TwentySix,
    TwentySeven,
    TwentyEight,
    TwentyNine,
    Thirty,
    ThirtyOne,
    ThirtyTwo,
    ThirtyThree,
    ThirtyFour,
    ThirtyFive,
    ThirtySix,
    ThirtySeven,
    ThirtyEight,
    ThirtyNine,
    Forty,
    FortyOne,
    FortyTwo,
    FortyThree,
    FortyFour,
    FortyFive,
    FortySix,
    FortySeven,
    FortyEight,
    FortyNine,
    Fifty,
    FiftyOne,
    FiftyTwo,
    FiftyThree,
    FiftyFour,
    FiftyFive,
    FiftySix,
    FiftySeven,
    FiftyEight,
    FiftyNine,
    Sixty,
    SixtyOne,
    SixtyTwo,
    SixtyThree,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct SeaLantern {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct SeaPickle {
    cluster_count: IntTag<SeaPickleClusterCount>,
    dead_bit: BooleanTag,
}

#[allow(non_camel_case_types)]
#[derive(Serialize, Deserialize)]
pub enum SeaPickleClusterCount {
    Zero = 0,
    One,
    Two,
    Three,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct Seagrass {
    sea_grass_type: StringTag<SeagrassType>,
}

#[allow(non_camel_case_types)]
#[derive(Serialize, Deserialize)]
pub enum SeagrassType {
    default,
    double_top,
    double_bot,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct Shroomlight {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct SilverGlazedTerracotta {
    glazed_terracotta_like: GlazedTerracottaLike,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct Skull {
    facing_direction_like: FacingDirectionLike,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct Slime {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct SmallAmethystBud {
    #[serde(rename = "minecraft:block_face")]
    block_face: StringTag<SmallAmethystBudBlockFace>,
}

#[allow(non_camel_case_types)]
#[derive(Serialize, Deserialize)]
pub enum SmallAmethystBudBlockFace {
    down,
    up,
    north,
    south,
    west,
    east,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct SmallDripleafBlock {
    cardinal_direction_like: CardinalDirectionLike,
    upper_block_bit: BooleanTag,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct SmithingTable {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct Smoker {
    cardinal_direction_like: CardinalDirectionLike,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct SmoothBasalt {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct SmoothQuartzStairs {
    stairs_like: StairsLike,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct SmoothRedSandstoneStairs {
    stairs_like: StairsLike,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct SmoothSandstoneStairs {
    stairs_like: StairsLike,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct SmoothStone {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct SnifferEgg {
    cracked_state: StringTag<SnifferEggCrackedState>,
}

#[allow(non_camel_case_types)]
#[derive(Serialize, Deserialize)]
pub enum SnifferEggCrackedState {
    no_cracks,
    cracked,
    max_cracked,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct Snow {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct SnowLayer {
    covered_bit: BooleanTag,
    height: IntTag<SnowLayerHeight>,
}

#[allow(non_camel_case_types)]
#[derive(Serialize, Deserialize)]
pub enum SnowLayerHeight {
    Zero = 0,
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct SoulCampfire {
    cardinal_direction_like: CardinalDirectionLike,
    extinguished: BooleanTag,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct SoulFire {
    age: IntTag<SoulFireAge>,
}

#[allow(non_camel_case_types)]
#[derive(Serialize, Deserialize)]
pub enum SoulFireAge {
    Zero = 0,
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Eleven,
    Twelve,
    Thirteen,
    Fourteen,
    Fifteen,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct SoulLantern {
    hanging: BooleanTag,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct SoulSand {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct SoulSoil {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct SoulTorch {
    torch_like: TorchLike,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct Sponge {
    sponge_type: StringTag<SpongeType>,
}

#[allow(non_camel_case_types)]
#[derive(Serialize, Deserialize)]
pub enum SpongeType {
    dry,
    wet,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct SporeBlossom {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct SpruceButton {
    button_like: ButtonLike,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct SpruceDoor {
    door_like: DoorLike,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct SpruceDoubleSlab {
    double_slab_like: DoubleSlabLike,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct SpruceFence {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct SpruceFenceGate {
    fence_gate_like: FenceGateLike,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct SpruceHangingSign {
    hanging_sign_like: HangingSignLike,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct SpruceLeaves {
    leaves_like: LeavesLike,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct SpruceLog {
    pillar_like: PillarLike,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct SprucePlanks {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct SprucePressurePlate {
    powerable: Powerable,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct SpruceSlab {
    double_slab_like: DoubleSlabLike,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct SpruceStairs {
    stairs_like: StairsLike,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct SpruceStandingSign {
    sign_like: SignLike,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct SpruceTrapdoor {
    trapdoor_like: TrapdoorLike,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct SpruceWallSign {
    wall_sign_like: WallSignLike,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct SpruceWood {
    pillar_like: PillarLike,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct StandingBanner {
    sign_like: SignLike,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct StandingSign {
    sign_like: SignLike,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct StickyPiston {
    facing_direction_like: FacingDirectionLike,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct StickyPistonArmCollision {
    facing_direction_like: FacingDirectionLike,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct Stone {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct StoneBlockSlab {
    double_slab_like: DoubleSlabLike,
    stone_slab_type: StringTag<StoneBlockSlabType>,
}

#[allow(non_camel_case_types)]
#[derive(Serialize, Deserialize)]
pub enum StoneBlockSlabType {
    smooth_stone,
    sandstone,
    wood,
    cobblestone,
    brick,
    stone_brick,
    quartz,
    nether_brick,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct StoneBlockSlab2 {
    double_slab_like: DoubleSlabLike,
    stone_slab_type_2: StringTag<StoneBlockSlab2Type>,
}

#[allow(non_camel_case_types)]
#[derive(Serialize, Deserialize)]
pub enum StoneBlockSlab2Type {
    red_sandstone,
    purpur,
    prismarine_rough,
    prismarine_dark,
    prismarine_brick,
    mossy_cobblestone,
    smooth_sandstone,
    red_nether_brick,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct StoneBlockSlab3 {
    double_slab_like: DoubleSlabLike,
    stone_slab_type_3: StringTag<StoneBlockSlab3Type>,
}

#[allow(non_camel_case_types)]
#[derive(Serialize, Deserialize)]
pub enum StoneBlockSlab3Type {
    end_stone_brick,
    smooth_red_sandstone,
    polished_andesite,
    andesite,
    diorite,
    polished_diorite,
    granite,
    polished_granite,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct StoneBlockSlab4 {
    double_slab_like: DoubleSlabLike,
    stone_slab_type_4: StringTag<StoneBlockSlab4Type>,
}

#[allow(non_camel_case_types)]
#[derive(Serialize, Deserialize)]
pub enum StoneBlockSlab4Type {
    mossy_stone_brick,
    smooth_quartz,
    stone,
    cut_sandstone,
    cut_red_sandstone,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct StoneBrickStairs {
    stairs_like: StairsLike,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct StoneButton {
    button_like: ButtonLike,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct StonePressurePlate {
    powerable: Powerable,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct StoneStairs {
    stairs_like: StairsLike,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct StoneBrick {
    stone_brick_type: StringTag<StoneBrickType>,
}

#[allow(non_camel_case_types)]
#[derive(Serialize, Deserialize)]
pub enum StoneBrickType {
    default,
    mossy,
    cracked,
    chiseled,
    smooth,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct Stonecutter {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct StonecutterBlock {
    cardinal_direction_like: CardinalDirectionLike,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct StrippedAcaciaLog {
    pillar_like: PillarLike,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct StrippedAcaciaWood {
    pillar_like: PillarLike,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct StrippedBambooBlock {
    pillar_like: PillarLike,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct StrippedBirchLog {
    pillar_like: PillarLike,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct StrippedBirchWood {
    pillar_like: PillarLike,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct StrippedCherryLog {
    pillar_like: PillarLike,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct StrippedCherryWood {
    pillar_like: PillarLike,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct StrippedCrimsonHyphae {
    pillar_like: PillarLike,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct StrippedCrimsonStem {
    pillar_like: PillarLike,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct StrippedDarkOakLog {
    pillar_like: PillarLike,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct StrippedDarkOakWood {
    pillar_like: PillarLike,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct StrippedJungleLog {
    pillar_like: PillarLike,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct StrippedJungleWood {
    pillar_like: PillarLike,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct StrippedMangroveLog {
    pillar_like: PillarLike,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct StrippedMangroveWood {
    pillar_like: PillarLike,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct StrippedOakLog {
    pillar_like: PillarLike,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct StrippedOakWood {
    pillar_like: PillarLike,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct StrippedSpruceLog {
    pillar_like: PillarLike,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct StrippedSpruceWood {
    pillar_like: PillarLike,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct StrippedWarpedHyphae {
    pillar_like: PillarLike,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct StrippedWarpedStem {
    pillar_like: PillarLike,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct StructureBlock {
    structure_block_type: StringTag<StructureBlockType>,
}

#[allow(non_camel_case_types)]
#[derive(Serialize, Deserialize)]
pub enum StructureBlockType {
    data,
    save,
    load,
    corner,
    invalid,
    export,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct StructureVoid {
    structure_void_type: StringTag<StructureVoidType>,
}

#[allow(non_camel_case_types)]
#[derive(Serialize, Deserialize)]
pub enum StructureVoidType {
    void,
    air,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct SuspiciousGravel {
    brushed_progress: IntTag<BrushedProgress>,
    hanging: BooleanTag,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct SuspiciousSand {
    brushed_progress: IntTag<BrushedProgress>,
    hanging: BooleanTag,
}

#[allow(non_camel_case_types)]
#[derive(Serialize, Deserialize)]
pub enum BrushedProgress {
    Zero = 0,
    One,
    Two,
    Three,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct SweetBerryBush {
    growth: IntTag<SweetBerryBushGrowth>,
}

#[allow(non_camel_case_types)]
#[derive(Serialize, Deserialize)]
pub enum SweetBerryBushGrowth {
    Zero = 0,
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct Tallgrass {
    tall_grass_type: StringTag<TallgrassType>,
}

#[allow(non_camel_case_types)]
#[derive(Serialize, Deserialize)]
pub enum TallgrassType {
    default,
    tall,
    fern,
    snow,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct Target {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct TintedGlass {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct TNT {
    allow_underwater_bit: BooleanTag,
    explode_bit: BooleanTag,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct Torch {
    torch_like: TorchLike,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct Torchflower {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct TorchflowerCrop {
    growth: IntTag<TorchflowerGrowth>,
}

#[allow(non_camel_case_types)]
#[derive(Serialize, Deserialize)]
pub enum TorchflowerGrowth {
    Zero = 0,
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct Trapdoor {
    trapdoor_like: TrapdoorLike,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct TrappedChest {
    cardinal_direction_like: CardinalDirectionLike,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct TrialSpawner {
    trial_spawner_state: IntTag<TrialSpawnerState>,
}

#[allow(non_camel_case_types)]
#[derive(Serialize, Deserialize)]
pub enum TrialSpawnerState {
    Zero = 0,
    One,
    Two,
    Three,
    Four,
    Five,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct TripWire {
    attached_bit: BooleanTag,
    disarmed_bit: BooleanTag,
    powered_bit: BooleanTag,
    suspended_bit: BooleanTag,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct TripwireHook {
    direction_like: DirectionLike,
    attached_bit: BooleanTag,
    powered_bit: BooleanTag,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct TubeCoral {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct Tuff {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct TuffBrickDoubleSlab {
    double_slab_like: DoubleSlabLike,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct TuffBrickSlab {
    double_slab_like: DoubleSlabLike,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct TuffBrickStairs {
    stairs_like: StairsLike,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct TuffBrickWall {
    wall_like: WallLike,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct TuffBricks {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct TuffDoubleSlab {
    double_slab_like: DoubleSlabLike,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct TuffSlab {
    double_slab_like: DoubleSlabLike,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct TuffStairs {
    stairs_like: StairsLike,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct TuffWall {
    wall_like: WallLike,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct TurtleEgg {
    cracked_state: StringTag<TurtleEggCrackedState>,
    turtle_egg_count: StringTag<TurtleEggCount>,
}

#[allow(non_camel_case_types)]
#[derive(Serialize, Deserialize)]
pub enum TurtleEggCrackedState {
    no_cracks,
    cracked,
    max_cracked,
}

#[allow(non_camel_case_types)]
#[derive(Serialize, Deserialize)]
pub enum TurtleEggCount {
    one_egg,
    two_egg,
    three_egg,
    four_egg,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct TwistingVines {
    twisting_vines_age: IntTag<TwistingVinesAge>,
}

#[allow(non_camel_case_types)]
#[derive(Serialize, Deserialize)]
pub enum TwistingVinesAge {
    Zero = 0,
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Eleven,
    Twelve,
    Thirteen,
    Fourteen,
    Fifteen,
    Sixteen,
    Seventeen,
    Eighteen,
    Nineteen,
    Twenty,
    TwentyOne,
    TwentyTwo,
    TwentyThree,
    TwentyFour,
    TwentyFive,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct UnderwaterTorch {
    torch_like: TorchLike,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct UndyedShulkerBox {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct Unknown {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct UnlitRedstoneTorch {
    torch_like: TorchLike,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct UnpoweredComparator {
    cardinal_direction_like: CardinalDirectionLike,
    output_lit_bit: BooleanTag,
    output_subtract_bit: BooleanTag,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct UnpoweredRepeater {
    cardinal_direction_like: CardinalDirectionLike,
    repeater_delay: IntTag<RepeaterDelay>,
}

#[allow(non_camel_case_types)]
#[derive(Serialize, Deserialize)]
pub enum RepeaterDelay {
    Zero = 0,
    One,
    Two,
    Three,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct Vault {
    cardinal_direction_like: CardinalDirectionLike,
    vault_state: StringTag<VaultState>,
}

#[allow(non_camel_case_types)]
#[derive(Serialize, Deserialize)]
pub enum VaultState {
    inactive,
    active,
    unlocking,
    ejecting,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct VerdantFroglight {
    pillar_like: PillarLike,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct Vine {
    vine_direction_bits: IntTag<VineDirection>,
}

#[allow(non_camel_case_types)]
#[derive(Serialize, Deserialize)]
pub enum VineDirection {
    Zero = 0,
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Eleven,
    Twelve,
    Thirteen,
    Fourteen,
    Fifteen,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct WallBanner {
    facing_direction_like: FacingDirectionLike,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct WallSign {
    wall_sign_like: WallSignLike,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct WarpedButton {
    button_like: ButtonLike,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct WarpedDoor {
    door_like: DoorLike,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct WarpedDoubleSlab {
    double_slab_like: DoubleSlabLike,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct WarpedFence {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct WarpedFenceGate {
    fence_gate_like: FenceGateLike,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct WarpedFungus {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct WarpedHangingSign {
    hanging_sign_like: HangingSignLike,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct WarpedHyphae {
    pillar_like: PillarLike,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct WarpedNylium {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct WarpedPlanks {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct WarpedPressurePlate {
    powerable: Powerable,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct WarpedRoots {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct WarpedSlab {
    double_slab_like: DoubleSlabLike,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct WarpedStairs {
    stairs_like: StairsLike,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct WarpedStandingSign {
    sign_like: SignLike,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct WarpedStem {
    pillar_like: PillarLike,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct WarpedTrapdoor {
    trapdoor_like: TrapdoorLike,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct WarpedWallSign {
    wall_sign_like: WallSignLike,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct WarpedWartBlock {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct Water {
    liquid_depth: IntTag<LiquidDepth>,
}

#[allow(non_camel_case_types)]
#[derive(Serialize, Deserialize)]
pub enum LiquidDepth {
    Zero = 0,
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Eleven,
    Twelve,
    Thirteen,
    Fourteen,
    Fifteen,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct Waterlily {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct WaxedChiseledCopper {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct WaxedCopper {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct WaxedCopperBulb {
    light_like: LightLike,
    powered_bit: BooleanTag,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct WaxedCopperDoor {
    door_like: DoorLike,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct WaxedCopperGrate {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct WaxedCopperTrapdoor {
    trapdoor_like: TrapdoorLike,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct WaxedCutCopper {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct WaxedCutCopperSlab {
    double_slab_like: DoubleSlabLike,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct WaxedCutCopperStairs {
    stairs_like: StairsLike,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct WaxedDoubleCutCopperSlab {
    double_slab_like: DoubleSlabLike,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct WaxedExposedChiseledCopper {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct WaxedExposedCopper {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct WaxedExposedCopperBulb {
    light_like: LightLike,
    powered_bit: BooleanTag,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct WaxedExposedCopperDoor {
    door_like: DoorLike,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct WaxedExposedCopperGrate {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct WaxedExposedCopperTrapdoor {
    trapdoor_like: TrapdoorLike,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct WaxedExposedCutCopper {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct WaxedExposedCutCopperSlab {
    double_slab_like: DoubleSlabLike,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct WaxedExposedCutCopperStairs {
    stairs_like: StairsLike,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct WaxedExposedDoubleCutCopperSlab {
    double_slab_like: DoubleSlabLike,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct WaxedOxidizedChiseledCopper {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct WaxedOxidizedCopper {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct WaxedOxidizedCopperBulb {
    light_like: LightLike,
    powered_bit: BooleanTag,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct WaxedOxidizedCopperDoor {
    door_like: DoorLike,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct WaxedOxidizedCopperGrate {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct WaxedOxidizedCopperTrapdoor {
    trapdoor_like: TrapdoorLike,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct WaxedOxidizedCutCopper {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct WaxedOxidizedCutCopperSlab {
    double_slab_like: DoubleSlabLike,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct WaxedOxidizedCutCopperStairs {
    stairs_like: StairsLike,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct WaxedOxidizedDoubleCutCopperSlab {
    double_slab_like: DoubleSlabLike,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct WaxedWeatheredChiseledCopper {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct WaxedWeatheredCopper {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct WaxedWeatheredCopperBulb {
    light_like: LightLike,
    powered_bit: BooleanTag,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct WaxedWeatheredCopperDoor {
    door_like: DoorLike,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct WaxedWeatheredCopperGrate {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct WaxedWeatheredCopperTrapdoor {
    trapdoor_like: TrapdoorLike,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct WaxedWeatheredCutCopper {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct WaxedWeatheredCutCopperSlab {
    double_slab_like: DoubleSlabLike,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct WaxedWeatheredCutCopperStairs {
    stairs_like: StairsLike,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct WaxedWeatheredDoubleCutCopperSlab {
    double_slab_like: DoubleSlabLike,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct WeatheredChiseledCopper {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct WeatheredCopper {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct WeatheredCopperBulb {
    light_like: LightLike,
    powered_bit: BooleanTag,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct WeatheredCopperDoor {
    door_like: DoorLike,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct WeatheredCopperGrate {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct WeatheredCopperTrapdoor {
    trapdoor_like: TrapdoorLike,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct WeatheredCutCopper {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct WeatheredCutCopperSlab {
    double_slab_like: DoubleSlabLike,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct WeatheredCutCopperStairs {
    stairs_like: StairsLike,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct WeatheredDoubleCutCopperSlab {
    double_slab_like: DoubleSlabLike,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct Web {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct WeepingVines {
    weeping_vines_age: IntTag<WeepingVinesAge>,
}

#[allow(non_camel_case_types)]
#[derive(Serialize, Deserialize)]
pub enum WeepingVinesAge {
    Zero = 0,
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Eleven,
    Twelve,
    Thirteen,
    Fourteen,
    Fifteen,
    Sixteen,
    Seventeen,
    Eighteen,
    Nineteen,
    Twenty,
    TwentyOne,
    TwentyTwo,
    TwentyThree,
    TwentyFour,
    TwentyFive,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct Wheat {
    growth: IntTag<WheatGrowth>,
}

#[allow(non_camel_case_types)]
#[derive(Serialize, Deserialize)]
pub enum WheatGrowth {
    Zero = 0,
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct WhiteCandle {
    candle_like: CandleLike,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct WhiteCandleCake {
    light_like: LightLike,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct WhiteCarpet {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct WhiteConcrete {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct WhiteConcretePowder {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct WhiteGlazedTerracotta {
    glazed_terracotta_like: GlazedTerracottaLike,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct WhiteShulkerBox {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct WhiteStainedGlass {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct WhiteStainedGlassPane {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct WhiteTerracotta {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct WhiteWool {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct WitherRose {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct WoodenButton {
    button_like: ButtonLike,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct WoodenDoor {
    door_like: DoorLike,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct WoodenPressurePlate {
    powerable: Powerable,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct YellowCandle {
    candle_like: CandleLike,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct YellowCandleCake {
    light_like: LightLike,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct YellowCarpet {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct YellowConcrete {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct YellowConcretePowder {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct YellowFlower {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct YellowGlazedTerracotta {
    glazed_terracotta_like: GlazedTerracottaLike,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct YellowShulkerBox {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct YellowStainedGlass {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct YellowStainedGlassPane {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct YellowTerracotta {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct YellowWool {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct ButtonLike {
    facing_direction_like: FacingDirectionLike,
    button_pressed_bit: BooleanTag,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct DoubleSlabLike {
    #[serde(rename = "minecraft:vertical_half")]
    vertical_half: StringTag<DoubleSlabVerticalHalf>,
}

#[allow(non_camel_case_types)]
#[derive(Serialize, Deserialize)]
pub enum DoubleSlabVerticalHalf {
    bottom,
    top,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct CandleLike {
    light_like: LightLike,
    candles: IntTag<CandleCount>,
}

#[allow(non_camel_case_types)]
#[derive(Serialize, Deserialize)]
pub enum CandleCount {
    Zero = 0,
    One,
    Two,
    Three,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct CardinalDirectionLike {
    #[serde(rename = "minecraft:cardinal_direction")]
    cardinal_direction: StringTag<CardinalDirection>,
}

#[allow(non_camel_case_types)]
#[derive(Serialize, Deserialize)]
pub enum CardinalDirection {
    south,
    west,
    north,
    east,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct DirectionLike {
    direction: IntTag<Direction>,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct DoorLike {
    direction_like: DirectionLike,
    door_hinge_bit: BooleanTag,
    open_bit: BooleanTag,
    upper_block_bit: BooleanTag,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct FacingDirectionLike {
    facing_direction: IntTag<FacingDirection>,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct FenceGateLike {
    direction_like: DirectionLike,
    in_wall_bit: BooleanTag,
    open_bit: BooleanTag,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct GlazedTerracottaLike {
    facing_direction_like: FacingDirectionLike,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct HangingSignLike {
    facing_direction_like: FacingDirectionLike,
    attached_bit: BooleanTag,
    ground_sign_direction: IntTag<SignDirection>,
    hanging: BooleanTag,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct LeavesLike {
    persistent_bit: BooleanTag,
    update_bit: BooleanTag,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct LightLike {
    lit: BooleanTag,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct PillarLike {
    pillar_axis: StringTag<PillarAxis>,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct Powerable {
    redstone_signal: IntTag<RedstoneSignal>,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct SignLike {
    ground_sign_direction: IntTag<SignDirection>,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct StairsLike {
    upside_down_bit: BooleanTag,
    weirdo_direction: IntTag<Direction>,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct TorchLike {
    torch_facing_direction: StringTag<TorchFacingDirection>,
}

#[allow(non_camel_case_types)]
#[derive(Serialize, Deserialize)]
pub enum TorchFacingDirection {
    unknown,
    west,
    east,
    north,
    south,
    top,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct TrapdoorLike {
    direction_like: DirectionLike,
    open_bit: BooleanTag,
    upside_down_bit: BooleanTag,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct WallLike {
    wall_connection_type_east: StringTag<WallConnectionType>,
    wall_connection_type_north: StringTag<WallConnectionType>,
    wall_connection_type_south: StringTag<WallConnectionType>,
    wall_connection_type_west: StringTag<WallConnectionType>,
    wall_post_bit: BooleanTag,
}

#[allow(non_camel_case_types)]
#[derive(Serialize, Deserialize)]
pub enum WallConnectionType {
    none,
    short,
    tall,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct WallSignLike {
    facing_direction_like: FacingDirectionLike,
}

#[allow(non_camel_case_types)]
#[derive(Serialize, Deserialize)]
pub enum Direction {
    // I need to get the proper name values for this
    Zero = 0,
    One,
    Two,
    Three,
}

#[allow(non_camel_case_types)]
#[derive(Serialize, Deserialize)]
pub enum FacingDirection {
    // I need to get the proper name values for this
    Zero = 0,
    One,
    Two,
    Three,
    Four,
    Five,
}

#[allow(non_camel_case_types)]
#[derive(Serialize, Deserialize)]
pub enum PillarAxis {
    x,
    y,
    z,
}

#[allow(non_camel_case_types)]
#[derive(Serialize, Deserialize)]
pub enum RedstoneSignal {
    Zero = 0,
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Eleven,
    Twelve,
    Thirteen,
    Fourteen,
    Fifteen,
}

#[allow(non_camel_case_types)]
#[derive(Serialize, Deserialize)]
pub enum SignDirection {
    Zero = 0,
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Eleven,
    Twelve,
    Thirteen,
    Fourteen,
    Fifteen,
}

// Prefix with `minecraft:` when serialized
#[allow(non_camel_case_types)]
#[derive(Serialize, Deserialize)]
pub enum BlockResource {
    air,
    stone,
    grass,
    dirt,
    cobblestone,
    planks,
    sapling,
    bedrock,
    flowing_water,
    water,
    flowing_lava,
    lava,
    sand,
    gravel,
    gold_ore,
    iron_ore,
    coal_ore,
    log,
    leaves,
    sponge,
    glass,
    lapis_ore,
    lapis_block,
    dispenser,
    sandstone,
    noteblock,
    bed,
    golden_rail,
    detector_rail,
    sticky_piston,
    web,
    tallgrass,
    deadbush,
    piston,
    pistonarmcollision,
    wool,
    element_0,
    yellow_flower,
    red_flower,
    brown_mushroom,
    red_mushroom,
    gold_block,
    iron_block,
    double_stone_slab,
    stone_slab,
    brick_block,
    tnt,
    bookshelf,
    mossy_cobblestone,
    obsidian,
    torch,
    fire,
    mob_spawner,
    oak_stairs,
    chest,
    redstone_wire,
    diamond_ore,
    diamond_block,
    crafting_table,
    wheat,
    farmland,
    furnace,
    lit_furnace,
    standing_sign,
    wooden_door,
    ladder,
    rail,
    stone_stairs,
    wall_sign,
    lever,
    stone_pressure_plate,
    iron_door,
    wooden_pressure_plate,
    redstone_ore,
    lit_redstone_ore,
    unlit_redstone_torch,
    redstone_torch,
    stone_button,
    snow_layer,
    ice,
    snow,
    cactus,
    clay,
    reeds,
    jukebox,
    fence,
    pumpkin,
    netherrack,
    soul_sand,
    glowstone,
    portal,
    lit_pumpkin,
    cake,
    unpowered_repeater,
    powered_repeater,
    invisibleBedrock,
    trapdoor,
    monster_egg,
    stonebrick,
    brown_mushroom_block,
    red_mushroom_block,
    iron_bars,
    glass_pane,
    melon_block,
    pumpkin_stem,
    melon_stem,
    vine,
    fence_gate,
    brick_stairs,
    stone_brick_stairs,
    mycelium,
    waterlily,
    nether_brick,
    nether_brick_fence,
    nether_brick_stairs,
    nether_wart,
    enchanting_table,
    brewing_stand,
    cauldron,
    end_portal,
    end_portal_frame,
    end_stone,
    dragon_egg,
    redstone_lamp,
    lit_redstone_lamp,
    dropper,
    activator_rail,
    cocoa,
    sandstone_stairs,
    emerald_ore,
    ender_chest,
    tripwire_hook,
    tripwire,
    emerald_block,
    spruce_stairs,
    birch_stairs,
    jungle_stairs,
    command_block,
    beacon,
    cobblestone_wall,
    flower_pot,
    carrots,
    potatoes,
    wooden_button,
    skull,
    anvil,
    trapped_chest,
    light_weighted_pressure_plate,
    heavy_weighted_pressure_plate,
    unpowered_comparator,
    powered_comparator,
    daylight_detector,
    redstone_block,
    quartz_ore,
    hopper,
    quartz_block,
    quartz_stairs,
    double_wooden_slab,
    wooden_slab,
    stained_hardened_clay,
    stained_glass_pane,
    leaves2,
    log2,
    acacia_stairs,
    dark_oak_stairs,
    slime,
    iron_trapdoor,
    prismarine,
    sealantern,
    hay_block,
    carpet,
    hardened_clay,
    coal_block,
    packed_ice,
    double_plant,
    standing_banner,
    wall_banner,
    daylight_detector_inverted,
    red_sandstone,
    red_sandstone_stairs,
    double_stone_slab2,
    stone_slab2,
    spruce_fence_gate,
    birch_fence_gate,
    jungle_fence_gate,
    dark_oak_fence_gate,
    acacia_fence_gate,
    repeating_command_block,
    chain_command_block,
    hard_glass_pane,
    hard_stained_glass_pane,
    chemical_heat,
    spruce_door,
    birch_door,
    jungle_door,
    acacia_door,
    dark_oak_door,
    grass_path,
    frame,
    chorus_flower,
    purpur_block,
    colored_torch_rg,
    purpur_stairs,
    colored_torch_bp,
    undyed_shulker_box,
    end_bricks,
    frosted_ice,
    end_rod,
    end_gateway,
    allow,
    deny,
    border_block,
    magma,
    nether_wart_block,
    red_nether_brick,
    bone_block,
    structure_void,
    shulker_box,
    purple_glazed_terracotta,
    white_glazed_terracotta,
    orange_glazed_terracotta,
    magenta_glazed_terracotta,
    light_blue_glazed_terracotta,
    yellow_glazed_terracotta,
    lime_glazed_terracotta,
    pink_glazed_terracotta,
    gray_glazed_terracotta,
    silver_glazed_terracotta,
    cyan_glazed_terracotta,
    chalkboard,
    blue_glazed_terracotta,
    brown_glazed_terracotta,
    green_glazed_terracotta,
    red_glazed_terracotta,
    black_glazed_terracotta,
    concrete,
    concrete_powder,
    chemistry_table,
    underwater_torch,
    chorus_plant,
    stained_glass,
    camera,
    podzol,
    beetroot,
    stonecutter,
    glowingobsidian,
    netherreactor,
    info_update,
    info_update2,
    moving_block,
    observer,
    structure_block,
    hard_glass,
    hard_stained_glass,
    reserved6,
    prismarine_stairs,
    dark_prismarine_stairs,
    prismarine_bricks_stairs,
    stripped_spruce_log,
    stripped_birch_log,
    stripped_jungle_log,
    stripped_acacia_log,
    stripped_dark_oak_log,
    stripped_oak_log,
    blue_ice,
    element_1,
    element_2,
    element_3,
    element_4,
    element_5,
    element_6,
    element_7,
    element_8,
    element_9,
    element_10,
    element_11,
    element_12,
    element_13,
    element_14,
    element_15,
    element_16,
    element_17,
    element_18,
    element_19,
    element_20,
    element_21,
    element_22,
    element_23,
    element_24,
    element_25,
    element_26,
    element_27,
    element_28,
    element_29,
    element_30,
    element_31,
    element_32,
    element_33,
    element_34,
    element_35,
    element_36,
    element_37,
    element_38,
    element_39,
    element_40,
    element_41,
    element_42,
    element_43,
    element_44,
    element_45,
    element_46,
    element_47,
    element_48,
    element_49,
    element_50,
    element_51,
    element_52,
    element_53,
    element_54,
    element_55,
    element_56,
    element_57,
    element_58,
    element_59,
    element_60,
    element_61,
    element_62,
    element_63,
    element_64,
    element_65,
    element_66,
    element_67,
    element_68,
    element_69,
    element_70,
    element_71,
    element_72,
    element_73,
    element_74,
    element_75,
    element_76,
    element_77,
    element_78,
    element_79,
    element_80,
    element_81,
    element_82,
    element_83,
    element_84,
    element_85,
    element_86,
    element_87,
    element_88,
    element_89,
    element_90,
    element_91,
    element_92,
    element_93,
    element_94,
    element_95,
    element_96,
    element_97,
    element_98,
    element_99,
    element_100,
    element_101,
    element_102,
    element_103,
    element_104,
    element_105,
    element_106,
    element_107,
    element_108,
    element_109,
    element_110,
    element_111,
    element_112,
    element_113,
    element_114,
    element_115,
    element_116,
    element_117,
    element_118,
    seagrass,
    coral,
    coral_block,
    coral_fan,
    coral_fan_dead,
    coral_fan_hang,
    coral_fan_hang2,
    coral_fan_hang3,
    kelp,
    dried_kelp_block,
    acacia_button,
    birch_button,
    dark_oak_button,
    jungle_button,
    spruce_button,
    acacia_trapdoor,
    birch_trapdoor,
    dark_oak_trapdoor,
    jungle_trapdoor,
    spruce_trapdoor,
    acacia_pressure_plate,
    birch_pressure_plate,
    dark_oak_pressure_plate,
    jungle_pressure_plate,
    spruce_pressure_plate,
    carved_pumpkin,
    sea_pickle,
    conduit,
    turtle_egg,
    bubble_column,
    barrier,
    stone_slab3,
    bamboo,
    bamboo_sapling,
    scaffolding,
    stone_slab4,
    double_stone_slab3,
    double_stone_slab4,
    granite_stairs,
    diorite_stairs,
    andesite_stairs,
    polished_granite_stairs,
    polished_diorite_stairs,
    polished_andesite_stairs,
    mossy_stone_brick_stairs,
    smooth_red_sandstone_stairs,
    smooth_sandstone_stairs,
    end_brick_stairs,
    mossy_cobblestone_stairs,
    normal_stone_stairs,
    spruce_standing_sign,
    spruce_wall_sign,
    smooth_stone,
    red_nether_brick_stairs,
    smooth_quartz_stairs,
    birch_standing_sign,
    birch_wall_sign,
    jungle_standing_sign,
    jungle_wall_sign,
    acacia_standing_sign,
    acacia_wall_sign,
    darkoak_standing_sign,
    darkoak_wall_sign,
    lectern,
    grindstone,
    blast_furnace,
    stonecutter_block,
    smoker,
    lit_smoker,
    cartography_table,
    fletching_table,
    smithing_table,
    barrel,
    loom,
    bell,
    sweet_berry_bush,
    lantern,
    campfire,
    lava_cauldron,
    jigsaw,
    wood,
    composter,
    lit_blast_furnace,
    light_block,
    wither_rose,
    stickypistonarmcollision,
    bee_nest,
    beehive,
    honey_block,
    honeycomb_block,
    lodestone,
    crimson_roots,
    warped_roots,
    crimson_stem,
    warped_stem,
    warped_wart_block,
    crimson_fungus,
    warped_fungus,
    shroomlight,
    weeping_vines,
    crimson_nylium,
    warped_nylium,
    basalt,
    polished_basalt,
    soul_soil,
    soul_fire,
    nether_sprouts,
    target,
    stripped_crimson_stem,
    stripped_warped_stem,
    crimson_planks,
    warped_planks,
    crimson_door,
    warped_door,
    crimson_trapdoor,
    warped_trapdoor,
    crimson_standing_sign,
    warped_standing_sign,
    crimson_wall_sign,
    warped_wall_sign,
    crimson_stairs,
    warped_stairs,
    crimson_fence,
    warped_fence,
    crimson_fence_gate,
    warped_fence_gate,
    crimson_button,
    warped_button,
    crimson_pressure_plate,
    warped_pressure_plate,
    crimson_slab,
    warped_slab,
    crimson_double_slab,
    warped_double_slab,
    soul_torch,
    soul_lantern,
    netherite_block,
    ancient_debris,
    respawn_anchor,
    blackstone,
    polished_blackstone_bricks,
    polished_blackstone_brick_stairs,
    blackstone_stairs,
    blackstone_wall,
    polished_blackstone_brick_wall,
    chiseled_polished_blackstone,
    cracked_polished_blackstone_bricks,
    gilded_blackstone,
    blackstone_slab,
    blackstone_double_slab,
    polished_blackstone_brick_slab,
    polished_blackstone_brick_double_slab,
    chain,
    twisting_vines,
    nether_gold_ore,
    crying_obsidian,
    soul_campfire,
    polished_blackstone,
    polished_blackstone_stairs,
    polished_blackstone_slab,
    polished_blackstone_double_slab,
    polished_blackstone_pressure_plate,
    polished_blackstone_button,
    polished_blackstone_wall,
    warped_hyphae,
    crimson_hyphae,
    stripped_crimson_hyphae,
    stripped_warped_hyphae,
    chiseled_nether_bricks,
    cracked_nether_bricks,
    quartz_bricks,
    unknown,
    powder_snow,
    sculk_sensor,
    pointed_dripstone,
    copper_ore,
    lightning_rod,
    dripstone_block,
    dirt_with_roots,
    hanging_roots,
    moss_block,
    spore_blossom,
    cave_vines,
    big_dripleaf,
    azalea_leaves,
    azalea_leaves_flowered,
    calcite,
    amethyst_block,
    budding_amethyst,
    amethyst_cluster,
    large_amethyst_bud,
    medium_amethyst_bud,
    small_amethyst_bud,
    tuff,
    tinted_glass,
    moss_carpet,
    small_dripleaf_block,
    azalea,
    flowering_azalea,
    glow_frame,
    copper_block,
    exposed_copper,
    weathered_copper,
    oxidized_copper,
    waxed_copper,
    waxed_exposed_copper,
    waxed_weathered_copper,
    cut_copper,
    exposed_cut_copper,
    weathered_cut_copper,
    oxidized_cut_copper,
    waxed_cut_copper,
    waxed_exposed_cut_copper,
    waxed_weathered_cut_copper,
    cut_copper_stairs,
    exposed_cut_copper_stairs,
    weathered_cut_copper_stairs,
    oxidized_cut_copper_stairs,
    waxed_cut_copper_stairs,
    waxed_exposed_cut_copper_stairs,
    waxed_weathered_cut_copper_stairs,
    cut_copper_slab,
    exposed_cut_copper_slab,
    weathered_cut_copper_slab,
    oxidized_cut_copper_slab,
    waxed_cut_copper_slab,
    waxed_exposed_cut_copper_slab,
    waxed_weathered_cut_copper_slab,
    double_cut_copper_slab,
    exposed_double_cut_copper_slab,
    weathered_double_cut_copper_slab,
    oxidized_double_cut_copper_slab,
    waxed_double_cut_copper_slab,
    waxed_exposed_double_cut_copper_slab,
    waxed_weathered_double_cut_copper_slab,
    cave_vines_body_with_berries,
    cave_vines_head_with_berries,
    smooth_basalt,
    deepslate,
    cobbled_deepslate,
    cobbled_deepslate_slab,
    cobbled_deepslate_stairs,
    cobbled_deepslate_wall,
    polished_deepslate,
    polished_deepslate_slab,
    polished_deepslate_stairs,
    polished_deepslate_wall,
    deepslate_tiles,
    deepslate_tile_slab,
    deepslate_tile_stairs,
    deepslate_tile_wall,
    deepslate_bricks,
    deepslate_brick_slab,
    deepslate_brick_stairs,
    deepslate_brick_wall,
    chiseled_deepslate,
    cobbled_deepslate_double_slab,
    polished_deepslate_double_slab,
    deepslate_tile_double_slab,
    deepslate_brick_double_slab,
    deepslate_lapis_ore,
    deepslate_iron_ore,
    deepslate_gold_ore,
    deepslate_redstone_ore,
    lit_deepslate_redstone_ore,
    deepslate_diamond_ore,
    deepslate_coal_ore,
    deepslate_emerald_ore,
    deepslate_copper_ore,
    cracked_deepslate_tiles,
    cracked_deepslate_bricks,
    glow_lichen,
    candle,
    white_candle,
    orange_candle,
    magenta_candle,
    light_blue_candle,
    yellow_candle,
    lime_candle,
    pink_candle,
    gray_candle,
    light_gray_candle,
    cyan_candle,
    purple_candle,
    blue_candle,
    brown_candle,
    green_candle,
    red_candle,
    black_candle,
    candle_cake,
    white_candle_cake,
    orange_candle_cake,
    magenta_candle_cake,
    light_blue_candle_cake,
    yellow_candle_cake,
    lime_candle_cake,
    pink_candle_cake,
    gray_candle_cake,
    light_gray_candle_cake,
    cyan_candle_cake,
    purple_candle_cake,
    blue_candle_cake,
    brown_candle_cake,
    green_candle_cake,
    red_candle_cake,
    black_candle_cake,
    waxed_oxidized_copper,
    waxed_oxidized_cut_copper,
    waxed_oxidized_cut_copper_stairs,
    waxed_oxidized_cut_copper_slab,
    waxed_oxidized_double_cut_copper_slab,
    raw_iron_block,
    raw_copper_block,
    raw_gold_block,
    infested_deepslate,
    sculk,
    sculk_vein,
    sculk_catalyst,
    sculk_shrieker,
    client_request_placeholder_block,
    mysterious_frame,
    mysterious_frame_slot,
    frog_spawn,
    pearlescent_froglight,
    verdant_froglight,
    ochre_froglight,
    chiseled_bookshelf,
    suspicious_sand,
    suspicious_gravel,
    decorated_pot,
    compound_creator,
    element_constructor,
    lab_table,
    material_reducer,
}
