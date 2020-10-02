use crate::map::MapPosition;

pub enum CityQuery {
    PopulationDensity,
    LandValue,
    CrimeRate,
    Pollution,
    GrowthRate,
}

pub struct CityReportBuilder;

impl CityReportBuilder {
    // TODO: query cost
    pub fn query_tile(&self, category: &CityQuery, at: &MapPosition) -> usize {
        todo!()
    }

    pub fn tile_status(&self, at: &MapPosition) {
        todo!()
    }
}
