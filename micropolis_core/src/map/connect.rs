use super::{
    tiles::TILE_BULL_BIT, tiles::TILE_BURN_BIT, tiles::TILE_BURN_BULL_CONDUCT_BIT,
    tiles::TILE_CONDUCT_BIT, tiles::TILE_LOW_MASK, tools::ConnectTileCommand, tools::ToolEffects,
    tools::ToolResult, MapPosition, Tile, TileMap, TileType,
};

const ROADS_TABLE: [TileType; 16] = [
    TileType::Roads,
    TileType::Roads2,
    TileType::Roads,
    TileType::Roads3,
    TileType::Roads2,
    TileType::Roads2,
    TileType::Roads4,
    TileType::Roads8,
    TileType::Roads,
    TileType::Roads6,
    TileType::Roads,
    TileType::Roads7,
    TileType::Roads5,
    TileType::Roads10,
    TileType::Roads9,
    TileType::Intersection,
];
const RAILS_TABLE: [TileType; 16] = [
    TileType::LhRail,
    TileType::LvRail,
    TileType::LhRail,
    TileType::LvRail2,
    TileType::LvRail,
    TileType::LvRail,
    TileType::LvRail3,
    TileType::LvRail7,
    TileType::LhRail,
    TileType::LvRail5,
    TileType::LhRail,
    TileType::LvRail6,
    TileType::LvRail4,
    TileType::LvRail9,
    TileType::LvRail8,
    TileType::LvRail10,
];
const WIRES_TABLE: [TileType; 16] = [
    TileType::LhPower,
    TileType::LvPower,
    TileType::LhPower,
    TileType::LvPower2,
    TileType::LvPower,
    TileType::LvPower,
    TileType::LvPower3,
    TileType::LvPower7,
    TileType::LhPower,
    TileType::LvPower5,
    TileType::LhPower,
    TileType::LvPower6,
    TileType::LvPower4,
    TileType::LvPower9,
    TileType::LvPower8,
    TileType::LvPower10,
];

pub struct TileMapConnector;

impl TileMapConnector {
    /// Perform the command, fix wire/road/rail/zone connections around it,
    /// and return the modifications in an effects structure.
    pub fn connect_tile(
        map: &TileMap,
        position: &MapPosition,
        command: &ConnectTileCommand,
        effects: ToolEffects,
        auto_bulldoze: bool,
    ) -> Result<ToolResult, String> {
        if !map.in_bounds(position) {
            return Ok(ToolResult::Failed);
        }

        // auto-bulldoze if appropriate
        effects = match *command {
            ConnectTileCommand::Road | ConnectTileCommand::Rail | ConnectTileCommand::Wire => {
                if auto_bulldoze {
                    let mut tile_raw = effects
                        .get_map_value_at(map, position)
                        .ok_or(format!(
                            "TileMapConnector.connect_tile cannot read tile value at {}",
                            position
                        ))?
                        .get_raw();
                    if tile_raw & TILE_BULL_BIT != 0x00 {
                        tile_raw &= TILE_LOW_MASK;
                        tile_raw = Self::neutralize_road(tile_raw);
                        if (tile_raw >= TileType::TINYEXP.to_u16().unwrap()
                            && tile_raw <= TileType::LASTTINYEXP.to_u16().unwrap())
                            || (tile_raw < TileType::HorizontalBridge.to_u16().unwrap()
                                && tile_raw != TileType::Dirt.to_u16().unwrap())
                        {
                            effects
                                .add_cost(1)
                                .add_modification(position, Tile::from_type(TileType::Dirt)?)
                        } else {
                            effects
                        }
                    } else {
                        effects
                    }
                } else {
                    effects
                }
            }
            _ => effects,
        };

        // perform the command
        match *command {
            ConnectTileCommand::Fix => Ok(ToolResult::Succeeded(Self::fix_zone(
                map, position, effects,
            )?)),
            ConnectTileCommand::Bulldoze => {
                match Self::bulldoze_tile(map, position, effects, auto_bulldoze)? {
                    ToolResult::Succeeded(e) => effects = e,
                    other => return Ok(other),
                }
                Ok(ToolResult::Succeeded(Self::fix_zone(
                    map, position, effects,
                )?))
            }
            ConnectTileCommand::Road => todo!(),
            ConnectTileCommand::Rail => todo!(),
            ConnectTileCommand::Wire => todo!(),
        }
    }

    /// Bulldoze a tile by making it either a river or dirt depending on context.
    fn bulldoze_tile(
        map: &TileMap,
        position: &MapPosition,
        effects: ToolEffects,
        auto_bulldoze: bool,
    ) -> Result<ToolResult, String> {
        let mut tile_raw = effects
            .get_map_value_at(map, position)
            .ok_or(format!(
                "TileMapConnector::bulldoze_tile cannot get effects tile value at {}",
                position
            ))?
            .get_raw();
        if tile_raw & TILE_BULL_BIT == 0x00 {
            return Ok(ToolResult::Failed); // not bulldozeable
        }

        tile_raw &= TILE_LOW_MASK;
        tile_raw = Self::neutralize_road(tile_raw);

        Ok(ToolResult::Succeeded(
            effects.add_cost(1).add_modification(
                position,
                if [
                    TileType::HorizontalBridge,
                    TileType::VerticalBridge,
                    TileType::VerticalBridgeOpened,
                    TileType::HorizontalBridgeOpened,
                    TileType::HBRDG0,
                    TileType::HBRDG1,
                    TileType::HBRDG2,
                    TileType::HBRDG3,
                    TileType::VBRDG0,
                    TileType::VBRDG1,
                    TileType::VBRDG2,
                    TileType::VBRDG3,
                    TileType::HorizontalPower,
                    TileType::VerticalPower,
                    TileType::UnderwaterHorizontalRail,
                    TileType::UnderwaterVerticalRail,
                ]
                .iter()
                .map(|t| t.to_u16().unwrap())
                .collect::<Vec<u16>>()
                .contains(&tile_raw)
                {
                    Tile::from_type(TileType::River)?
                } else {
                    Tile::from_type(TileType::Dirt)?
                },
            ),
        ))
    }

    /// Lay down a road, and update the roads connections around it.
    fn lay_down_road(
        map: &TileMap,
        position: &MapPosition,
        effects: ToolEffects,
    ) -> Result<ToolResult, String> {
        let mut cost: u32 = 0;
        let (x, y) = (position.x, position.y);
        let (world_width, world_heigth) = map.bounds().get_tuple();
        let mut tile_raw = effects
            .get_map_value_at(map, position)
            .ok_or(format!(
                "TileMapConnector::lay_down_road cannot get effects tile value at {}",
                position
            ))?
            .get_raw();
        match tile_raw {
            t if t == TileType::Dirt.to_u16().unwrap() => {
                effects = effects.add_modification(
                    position,
                    Tile::from_raw(
                        TileType::Roads.to_u16().unwrap() | TILE_BULL_BIT | TILE_BURN_BIT,
                    )?,
                );
            }
            t if [
                TileType::River, // road on water
                TileType::RiverEdge,
                TileType::Channel, // check how to build bridges, if possible
            ]
            .iter()
            .map(|tile_type| tile_type.to_u16().unwrap())
            .collect::<Vec<u16>>()
            .contains(&t) =>
            {
                cost = 50;

                if x < world_width - 1 {
                    tile_raw = Self::neutralize_road(
                        effects
                            .get_map_tile_at(map, &position.with_x_offset(1))
                            .ok_or(format!(
                        "TileMapConnector::fix_single_tile cannot read effects map tile value at {}",
                        position.with_x_offset(1)
                    ))?
                            .get_raw(),
                    );
                    if tile_raw == TileType::VerticalRailRoad.to_u16().unwrap()
                        || tile_raw == TileType::HorizontalBridge.to_u16().unwrap()
                        || (tile_raw >= TileType::Roads.to_u16().unwrap()
                            && tile_raw <= TileType::HorizontalRoadPower.to_u16().unwrap())
                    {
                    }
                }
            }
            _ => return Ok(ToolResult::Failed),
        }

        Ok(ToolResult::Succeeded(effects.add_cost(cost)))
    }

    /// Update connections (roads/rails/wire) to a zone.
    fn fix_zone(
        map: &TileMap,
        position: &MapPosition,
        effects: ToolEffects,
    ) -> Result<ToolEffects, String> {
        effects = Self::fix_single_tile(map, position, effects)?;
        let (x, y) = (position.x, position.y);
        let (world_width, world_heigth) = map.bounds().get_tuple();

        if y > 0 {
            effects = Self::fix_single_tile(map, &position.with_y_offset(-1), effects)?;
        }
        if x < world_width - 1 {
            effects = Self::fix_single_tile(map, &position.with_x_offset(1), effects)?;
        }
        if y < world_heigth - 1 {
            effects = Self::fix_single_tile(map, &position.with_y_offset(1), effects)?;
        }
        if x > 0 {
            effects = Self::fix_single_tile(map, &position.with_x_offset(-1), effects)?;
        }

        Ok(effects)
    }

    /// Fix road, rails and wire connections at a given tile.
    fn fix_single_tile(
        map: &TileMap,
        position: &MapPosition,
        effects: ToolEffects,
    ) -> Result<ToolEffects, String> {
        let mut adjacent_tiles: usize = 0;
        let (x, y) = (position.x, position.y);
        let (world_width, world_heigth) = map.bounds().get_tuple();
        let mut tile_raw = Self::neutralize_road(
            effects
                .get_map_tile_at(map, position)
                .ok_or(format!(
                    "TileMapConnector::fix_single_tile cannot read effects map tile value at {}",
                    position
                ))?
                .get_raw(),
        );

        fn compute_tile(
            table: &[TileType; 16],
            adjacent: usize,
            flags: u16,
        ) -> Result<Tile, String> {
            Tile::from_raw(table.get(adjacent).unwrap().to_u16().unwrap() | flags)
        }

        // cleanup road
        if tile_raw >= TileType::Roads.to_u16().unwrap()
            && tile_raw <= TileType::Intersection.to_u16().unwrap()
        {
            if y > 0 {
                tile_raw = Self::neutralize_road(
                    effects
                        .get_map_tile_at(map, &position.with_y_offset(-1))
                        .ok_or(format!(
                    "TileMapConnector::fix_single_tile cannot read effects map tile value at {}",
                    position.with_y_offset(-1)
                ))?
                        .get_raw(),
                );
                if (tile_raw == TileType::HorizontalRailRoad.to_u16().unwrap()
                    || (tile_raw >= TileType::HorizontalBridge.to_u16().unwrap()
                        && tile_raw <= TileType::VerticalRoadPower.to_u16().unwrap()))
                    && tile_raw != TileType::HorizontalRoadPower.to_u16().unwrap()
                    && tile_raw != TileType::VerticalRailRoad.to_u16().unwrap()
                    && tile_raw != TileType::HorizontalBridge.to_u16().unwrap()
                {
                    adjacent_tiles |= 0b00000001;
                }
            }

            if x < world_width - 1 {
                tile_raw = Self::neutralize_road(
                    effects
                        .get_map_tile_at(map, &position.with_x_offset(1))
                        .ok_or(format!(
                    "TileMapConnector::fix_single_tile cannot read effects map tile value at {}",
                    position.with_x_offset(1)
                ))?
                        .get_raw(),
                );
                if (tile_raw == TileType::VerticalRailRoad.to_u16().unwrap()
                    || (tile_raw >= TileType::HorizontalBridge.to_u16().unwrap()
                        && tile_raw <= TileType::VerticalRoadPower.to_u16().unwrap()))
                    && tile_raw != TileType::VerticalRoadPower.to_u16().unwrap()
                    && tile_raw != TileType::HorizontalRailRoad.to_u16().unwrap()
                    && tile_raw != TileType::VerticalBridge.to_u16().unwrap()
                {
                    adjacent_tiles |= 0b00000010;
                }
            }

            if y < world_heigth - 1 {
                tile_raw = Self::neutralize_road(
                    effects
                        .get_map_tile_at(map, &position.with_y_offset(1))
                        .ok_or(format!(
                    "TileMapConnector::fix_single_tile cannot read effects map tile value at {}",
                    position.with_y_offset(1)
                ))?
                        .get_raw(),
                );
                if (tile_raw == TileType::HorizontalRailRoad.to_u16().unwrap()
                    || (tile_raw >= TileType::HorizontalBridge.to_u16().unwrap()
                        && tile_raw <= TileType::VerticalRoadPower.to_u16().unwrap()))
                    && tile_raw != TileType::HorizontalRoadPower.to_u16().unwrap()
                    && tile_raw != TileType::VerticalRailRoad.to_u16().unwrap()
                    && tile_raw != TileType::HorizontalBridge.to_u16().unwrap()
                {
                    adjacent_tiles |= 0b00000100;
                }
            }

            if x > 0 {
                tile_raw = Self::neutralize_road(
                    effects
                        .get_map_tile_at(map, &position.with_x_offset(-1))
                        .ok_or(format!(
                    "TileMapConnector::fix_single_tile cannot read effects map tile value at {}",
                    position.with_x_offset(-1)
                ))?
                        .get_raw(),
                );
                if (tile_raw == TileType::VerticalRailRoad.to_u16().unwrap()
                    || (tile_raw >= TileType::HorizontalBridge.to_u16().unwrap()
                        && tile_raw <= TileType::VerticalRoadPower.to_u16().unwrap()))
                    && tile_raw != TileType::VerticalRoadPower.to_u16().unwrap()
                    && tile_raw != TileType::HorizontalRailRoad.to_u16().unwrap()
                    && tile_raw != TileType::VerticalBridge.to_u16().unwrap()
                {
                    adjacent_tiles |= 0b00001000;
                }
            }

            Ok(effects.add_modification(
                position,
                compute_tile(&ROADS_TABLE, adjacent_tiles, TILE_BULL_BIT | TILE_BURN_BIT)?,
            ))
        } else if tile_raw >= TileType::LhRail.to_u16().unwrap()
            && tile_raw <= TileType::LvRail10.to_u16().unwrap()
        // cleanup rail
        {
            if y > 0 {
                tile_raw = Self::neutralize_road(
                    effects
                        .get_map_tile_at(map, &position.with_y_offset(-1))
                        .ok_or(format!(
                    "TileMapConnector::fix_single_tile cannot read effects map tile value at {}",
                    position.with_y_offset(-1)
                ))?
                        .get_raw(),
                );
                if tile_raw >= TileType::RailHorizontalPowerVertical.to_u16().unwrap()
                    && tile_raw <= TileType::VerticalRailRoad.to_u16().unwrap()
                    && tile_raw != TileType::RailHorizontalPowerVertical.to_u16().unwrap()
                    && tile_raw != TileType::HorizontalRailRoad.to_u16().unwrap()
                    && tile_raw != TileType::UnderwaterHorizontalRail.to_u16().unwrap()
                {
                    adjacent_tiles |= 0b00000001;
                }
            }

            if x < world_width - 1 {
                tile_raw = Self::neutralize_road(
                    effects
                        .get_map_tile_at(map, &position.with_x_offset(1))
                        .ok_or(format!(
                    "TileMapConnector::fix_single_tile cannot read effects map tile value at {}",
                    position.with_x_offset(1)
                ))?
                        .get_raw(),
                );
                if tile_raw >= TileType::RailHorizontalPowerVertical.to_u16().unwrap()
                    && tile_raw <= TileType::VerticalRailRoad.to_u16().unwrap()
                    && tile_raw != TileType::RailVerticalPowerHorizontal.to_u16().unwrap()
                    && tile_raw != TileType::VerticalRailRoad.to_u16().unwrap()
                    && tile_raw != TileType::UnderwaterVerticalRail.to_u16().unwrap()
                {
                    adjacent_tiles |= 0b00000010;
                }
            }

            if y < world_heigth - 1 {
                tile_raw = Self::neutralize_road(
                    effects
                        .get_map_tile_at(map, &position.with_y_offset(1))
                        .ok_or(format!(
                    "TileMapConnector::fix_single_tile cannot read effects map tile value at {}",
                    position.with_y_offset(1)
                ))?
                        .get_raw(),
                );
                if tile_raw >= TileType::RailHorizontalPowerVertical.to_u16().unwrap()
                    && tile_raw <= TileType::VerticalRailRoad.to_u16().unwrap()
                    && tile_raw != TileType::RailHorizontalPowerVertical.to_u16().unwrap()
                    && tile_raw != TileType::HorizontalRailRoad.to_u16().unwrap()
                    && tile_raw != TileType::UnderwaterHorizontalRail.to_u16().unwrap()
                {
                    adjacent_tiles |= 0b00000100;
                }
            }

            if x > 0 {
                tile_raw = Self::neutralize_road(
                    effects
                        .get_map_tile_at(map, &position.with_x_offset(-1))
                        .ok_or(format!(
                    "TileMapConnector::fix_single_tile cannot read effects map tile value at {}",
                    position.with_x_offset(-1)
                ))?
                        .get_raw(),
                );
                if tile_raw >= TileType::RailHorizontalPowerVertical.to_u16().unwrap()
                    && tile_raw <= TileType::VerticalRailRoad.to_u16().unwrap()
                    && tile_raw != TileType::RailVerticalPowerHorizontal.to_u16().unwrap()
                    && tile_raw != TileType::VerticalRailRoad.to_u16().unwrap()
                    && tile_raw != TileType::UnderwaterVerticalRail.to_u16().unwrap()
                {
                    adjacent_tiles |= 0b00001000;
                }
            }

            Ok(effects.add_modification(
                position,
                compute_tile(&RAILS_TABLE, adjacent_tiles, TILE_BULL_BIT | TILE_BURN_BIT)?,
            ))
        } else if tile_raw >= TileType::LhPower.to_u16().unwrap()
            && tile_raw <= TileType::LvPower10.to_u16().unwrap()
        // cleanup wire
        {
            if y > 0 {
                tile_raw = effects
                    .get_map_tile_at(map, &position.with_y_offset(-1))
                    .ok_or(format!(
                    "TileMapConnector::fix_single_tile cannot read effects map tile value at {}",
                    position.with_y_offset(-1)
                ))?
                    .get_raw();
                if tile_raw & TILE_CONDUCT_BIT != 0x00 {
                    tile_raw &= TILE_LOW_MASK;
                    tile_raw = Self::neutralize_road(tile_raw);
                    if ![
                        TileType::VerticalPower,
                        TileType::VerticalRoadPower,
                        TileType::RailVerticalPowerHorizontal,
                    ]
                    .iter()
                    .map(|t| t.to_u16().unwrap())
                    .collect::<Vec<u16>>()
                    .contains(&tile_raw)
                    {
                        adjacent_tiles |= 0b00000001;
                    }
                }
            }

            if x < world_width - 1 {
                tile_raw = Self::neutralize_road(
                    effects
                        .get_map_tile_at(map, &position.with_x_offset(1))
                        .ok_or(format!(
                    "TileMapConnector::fix_single_tile cannot read effects map tile value at {}",
                    position.with_x_offset(1)
                ))?
                        .get_raw(),
                );
                if tile_raw & TILE_CONDUCT_BIT != 0x00 {
                    tile_raw &= TILE_LOW_MASK;
                    tile_raw = Self::neutralize_road(tile_raw);
                    if ![
                        TileType::HorizontalPower,
                        TileType::HorizontalRoadPower,
                        TileType::RailHorizontalPowerVertical,
                    ]
                    .iter()
                    .map(|t| t.to_u16().unwrap())
                    .collect::<Vec<u16>>()
                    .contains(&tile_raw)
                    {
                        adjacent_tiles |= 0b00000010;
                    }
                }
            }

            if y < world_heigth - 1 {
                tile_raw = Self::neutralize_road(
                    effects
                        .get_map_tile_at(map, &position.with_y_offset(1))
                        .ok_or(format!(
                    "TileMapConnector::fix_single_tile cannot read effects map tile value at {}",
                    position.with_y_offset(1)
                ))?
                        .get_raw(),
                );
                if tile_raw & TILE_CONDUCT_BIT != 0x00 {
                    tile_raw &= TILE_LOW_MASK;
                    tile_raw = Self::neutralize_road(tile_raw);
                    if ![
                        TileType::VerticalPower,
                        TileType::VerticalRoadPower,
                        TileType::RailVerticalPowerHorizontal,
                    ]
                    .iter()
                    .map(|t| t.to_u16().unwrap())
                    .collect::<Vec<u16>>()
                    .contains(&tile_raw)
                    {
                        adjacent_tiles |= 0b00000100;
                    }
                }
            }

            if x > 0 {
                tile_raw = Self::neutralize_road(
                    effects
                        .get_map_tile_at(map, &position.with_x_offset(-1))
                        .ok_or(format!(
                    "TileMapConnector::fix_single_tile cannot read effects map tile value at {}",
                    position.with_x_offset(-1)
                ))?
                        .get_raw(),
                );
                if tile_raw & TILE_CONDUCT_BIT != 0x00 {
                    tile_raw &= TILE_LOW_MASK;
                    tile_raw = Self::neutralize_road(tile_raw);
                    if ![
                        TileType::HorizontalPower,
                        TileType::VerticalRoadPower,
                        TileType::RailVerticalPowerHorizontal,
                    ]
                    .iter()
                    .map(|t| t.to_u16().unwrap())
                    .collect::<Vec<u16>>()
                    .contains(&tile_raw)
                    {
                        adjacent_tiles |= 0b00001000;
                    }
                }
            }

            Ok(effects.add_modification(
                position,
                compute_tile(&WIRES_TABLE, adjacent_tiles, TILE_BURN_BULL_CONDUCT_BIT)?,
            ))
        } else {
            Ok(effects)
        }
    }

    /// Returns the given raw tile type with the road removed.
    fn neutralize_road(tile_raw: u16) -> u16 {
        if tile_raw >= 64 && tile_raw <= 207 {
            (tile_raw & 0x000F) + 64
        } else {
            tile_raw
        }
    }
}
