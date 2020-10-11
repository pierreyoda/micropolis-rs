use crate::map::{Map, MapClusteringStrategy, MapRectangle};

pub type TrafficDensityMap = Map<u8>;

impl TrafficDensityMap {
    pub fn density_map_with_dimensions(dimensions: &MapRectangle, default_value: u8) -> Self {
        TrafficDensityMap::with_data(
            vec![vec![default_value; dimensions.get_height()]; dimensions.get_width()],
            MapClusteringStrategy::BlockSize2,
        )
    }
}

/// Traffic simulation.
pub struct CityTraffic {
    density_map: TrafficDensityMap,
}
