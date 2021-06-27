use crate::map::{tiles::TILE_LOW_MASK, MapPosition, TileMap, TileType};

/// Count the number of single tile houses in a residential zone.
///
/// See zone.cpp::doFreePop in the original codebase.
pub fn count_free_population(map: &TileMap, at: &MapPosition) -> u16 {
    let mut count = 0;
    let (house_type_min, house_type_max) = (
        TileType::House.to_u16().unwrap(),
        TileType::Hhthr.to_u16().unwrap(),
    );

    for x in (at.get_x() - 1)..(at.get_x() + 1) {
        for y in (at.get_y() - 1)..(at.get_y() + 1) {
            if let Some(tile) = map.get_tile_at(&(x, y).into()) {
                let tile_value = tile.get_raw() & TILE_LOW_MASK;
                if tile_value >= house_type_min && tile_value <= house_type_max {
                    count += 1;
                }
            }
        }
    }

    count
}

/// Returns the population of a residential zone center tile of type:
/// (ResidentialZoneBase, ResidentialZoneBase+9, ..., HOSPITAL - 9).
///
/// Parameter: `tile` center tile of a residential zone.
/// Returns: Population of the residential zone (16, 24, 32, 40, 16, ..., 40).
pub fn get_residential_zone_population(tile_value: u16) -> u16 {
    let zone_base_raw = TileType::ResidentialZoneBase.to_u16().unwrap();

    let cz_den = ((tile_value - zone_base_raw) / 9) % 4;
    cz_den * 8 + 16
}

/// Returns the population of a commercial zone center tile.
pub fn get_commercial_zone_population(tile_value: u16) -> u16 {
    if tile_value == TileType::CommercialClr.to_u16().unwrap() {
        0
    } else {
        let cz_den = ((tile_value - TileType::CommercialZoneBase.to_u16().unwrap()) / 9) % 5 + 1;
        cz_den
    }
}

/// Returns the population of an industrial zone center tile.
pub fn get_industrial_zone_population(tile_value: u16) -> u16 {
    if tile_value == TileType::IndustrialClr.to_u16().unwrap() {
        0
    } else {
        let cz_den = (((tile_value - TileType::IndustrialZoneBase.to_u16().unwrap()) / 9) % 4) + 1;
        cz_den
    }
}
