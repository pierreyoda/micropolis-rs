use std::fmt;

use num_derive::{FromPrimitive, ToPrimitive};
use num_traits::{FromPrimitive as FromPrimitiveTrait, ToPrimitive as ToPrimitiveTrait};
use serde::Serialize;

pub const WOODS_LOW: u16 = TileType::TreeBase as u16;
pub const WOODS_HIGH: u16 = TileType::UnusedTrash2 as u16;

/// The type of a single tile on the map.
///
/// TODO: how to handle duplicates / ranges? => REFACTOR enum River(...), Road(...)
/// TODO: explicit names for all types
/// TODO: documentation
#[derive(Clone, Debug, PartialEq, Eq, Hash, FromPrimitive, ToPrimitive, Serialize)]
pub enum TileType {
    Invalid = -1,
    /// Clear tile.
    Dirt = 0,
    // tile 1 ?

    /* Water */
    River = 2,
    /// Edge of a river. Original name `REDGE`.
    RiverEdge = 3, // TODO: ?
    Channel = 4,
    FirstRiverEdge = 5,
    // tile 6 -- 19 ?
    LastRiverEdge = 20,
    /// First water tile.
    // WaterLow = TileType::River,
    /// Last Water tile (inclusive).
    // WaterHigh = TileType::LastRiverEdge,

    /* Woods */
    TreeBase = 21,
    // WoodsLow = TileType::TreeBase,
    LastTree = 36,
    Woods = 37,
    UnusedTrash1 = 38,
    UnusedTrash2 = 39,
    // WoodsHigh = TileType::UnusedTrash2,
    Woods2 = 40,
    Woods3 = 41,
    Woods4 = 42,
    Woods5 = 43,

    /* Rubble (4 tiles) */
    Rubble = 44,
    LastRubble = 47,

    Flood = 48,
    // tile 49, 50 ?
    LastFlood = 51,

    Radioactive = 52,

    UnusedTrash3 = 53,
    UnusedTrash4 = 54,
    UnusedTrash5 = 55,

    /* Fire animation (8 tiles) */
    Fire = 56,
    // FireBase = TileType::Fire,
    LastFire = 63,

    /* Roads */
    HorizontalBridge = 64,
    VerticalBridge = 65,
    Roads = 66,
    Roads2 = 67,
    Roads3 = 68,
    Roads4 = 69,
    Roads5 = 70,
    Roads6 = 71,
    Roads7 = 72,
    Roads8 = 73,
    Roads9 = 74,
    Roads10 = 75,
    Intersection = 76,
    HorizontalRoadPower = 77,
    VerticalRoadPower = 78,
    HorizontalBridgeOpened = 79,
    /// First tile with low traffic.
    LowTrafficBase = 80,
    // tile 81 -- 94 ?
    VerticalBridgeOpened = 95,
    // tile 96 -- 110 ?
    BRWXXX1 = 111,
    // tile 112 -- 142 ?
    BRWXXX3 = 143,
    /// First tile with high traffic.
    HighTrafficBase = 144,
    // tile 145 -- 158 ?
    BRWXXX4 = 159,
    // tile 160 -- 174 ?
    BRWXXX5 = 175,
    // tile 176 -- 190 ?
    BRWXXX6 = 191,
    // tile 192 -- 205 ?
    // RoadBase = TileType::HorizontalBridge,
    LastRoad = 206,

    /* Power lines */
    /// Underwater horizontal power line.
    HorizontalPower = 208,
    /// Underwater vertical power line.
    VerticalPower = 209,
    LhPower = 210,
    LvPower = 211,
    LvPower2 = 212,
    LvPower3 = 213,
    LvPower4 = 214,
    LvPower5 = 215,
    LvPower6 = 216,
    LvPower7 = 217,
    LvPower8 = 218,
    LvPower9 = 219,
    LvPower10 = 220,
    RailHorizontalPowerVertical = 221,
    RailVerticalPowerHorizontal = 222,
    // PowerBase = TileType::HorizontalPower,
    // LastPower = TileType::RailVerticalPowerHorizontal,
    UnusedTrash6 = 223,

    /* Rail */
    /// Horizontal underwater rail.
    UnderwaterHorizontalRail = 224,
    /// Vertical underwater rail.
    UnderwaterVerticalRail = 225,
    LhRail = 226,
    LvRail = 227,
    LvRail2 = 228,
    LvRail3 = 229,
    LvRail4 = 230,
    LvRail5 = 231,
    LvRail6 = 232,
    LvRail7 = 233,
    LvRail8 = 234,
    LvRail9 = 235,
    LvRail10 = 236,
    HorizontalRailRoad = 237,
    VerticalRailRoad = 238,
    // RailBase = TileType:::UnderwaterHorizontalRail,
    // LastRail = TileType::VerticalRailRoad,

    /* Residential zone */
    /// Empty residential tile 240-248.
    ResidentialBase = 240,
    /// Center tile of a 3x3 empty residential.
    FreeZoneCenter = 244,
    /// Single-tile houses, until 260.
    House = 249,
    // Lhthr = TileType::House, // 12 tiles house.
    Hhthr = 260,
    /// Center tile first 3x3 tile residential.
    ResidentialZoneBase = 265,

    HospitalBase = 405,
    /// Center tile of hospital (tiles 405-413).
    Hospital = 409,

    ChurchBase = 414,
    // Church0Base = TileType::ChurchBase, // numbered alias
    /// Center tile of church (tiles 414-422.
    Church = 418,
    // Church0 = TileType::Church, // number alias

    /* Commercial zone */
    /// Empty commercial, tiles 423-431.
    CommercialBase = 423,
    // tile 424 -- 426 ?
    CommercialClr = 427,
    // tile 428 -- 435 ?
    CommercialZoneBase = 436,
    // tile 437 -- 608 ?
    CommercialLast = 609,
    // tile 610, 611 ?

    /* Industrial zone */
    /// Top-left tile of empty industrial zone.
    IndustrialBase = 612,
    /// Center tile of empty industrial zone.
    IndustrialClr = 616,
    /// Last tile of empty industrial zone.
    LastIndustrial = 620,

    // Industrial zone population 0, value 0: 621 -- 629
    /// Top-left tile of first non-empty industry zone.
    Industrial1 = 621,
    /// Center tile of first non-empty industry zone.
    IndustrialZoneBase = 625,

    // Industrial zone population 1, value 0: 630 -- 638

    // Industrial zone population 2, value 0: 639 -- 647
    Industrial2 = 641,
    Industrial3 = 644,

    // Industrial zone population 3, value 0: 648 -- 656
    Industrial4 = 649,
    Industrial5 = 650,

    // Industrial zone population 0, value 1: 657 -- 665

    // Industrial zone population 1, value 1: 666 -- 674

    // Industrial zone population 2, value 1: 675 -- 683
    Industrial6 = 676,
    Industrial7 = 677,

    // Industrial zone population 3, value 1: 684 -- 692
    Industrial8 = 686,
    Industrial9 = 689,

    /* Seaport */
    /// Top-left tile of the seaport.
    PortBase = 693,
    /// Center tile of the seaport.
    Port = 698,
    /// Last tile of the seaport.
    LastPort = 708,

    AirportBase = 709,
    // tile 710
    Radar = 711,
    // tile 712 -- 715 ?
    Airport = 716,
    // tile 717 -- 744 ?

    /* Coal power plant (4x4) */
    /// First tile of coal power plant.
    CoalBase = 745,
    /// 'Center' tile of coal power plant.
    PowerPlant = 750,
    /// Last tile of coal power plant.
    LastPowerPlant = 760,

    /* Fire station (3x3) */
    /// First tile of fire station.
    FireStationBase = 761,
    /// 'Center tile' of fire station.
    /// 769 last tile for fire station.
    FireStation = 765,

    /* Police station */
    PoliceStationBase = 770,
    // tile 771 -- 773 ?
    PoliceStation = 774,
    // tile 775 -- 778 ?

    /* Stadium (4x4) */
    /// First tile stadium.
    StadiumBase = 779,
    /// 'Center tile' stadium.
    /// Last tile stadium 794.
    Stadium = 784,

    // tile 785 -- 799 ?
    FullStadium = 800,
    // tile 801 -- 810 ?

    /* Nuclear power plant (4x4). */
    /// First tile nuclear power plant.
    NuclearBase = 811,
    /// 'Center' tile nuclear power plant.
    Nuclear = 816,
    /// Also last tile nuclear power plant.
    LastZone = 826,

    LIGHTNINGBOLT = 827,
    HBRDG0 = 828, // draw bridge tiles (horz)
    HBRDG1 = 829,
    HBRDG2 = 830,
    HBRDG3 = 831,
    HBRDG_END = 832,
    // RADAR0 = 832,
    RADAR1 = 833,
    RADAR2 = 834,
    RADAR3 = 835,
    RADAR4 = 836,
    RADAR5 = 837,
    RADAR6 = 838,
    RADAR7 = 839,
    FOUNTAIN = 840,
    // tile 841 -- 843: fountain animation.
    INDBASE2 = 844,
    // TELEBASE = 844,
    // tile 845 -- 850 ?
    TELELAST = 851,
    SMOKEBase = 852,
    // tile 853 -- 859 ?
    TINYEXP = 860,
    // tile 861 -- 863 ?
    SOMETINYEXP = 864,
    // tile 865 -- 866 ?
    LASTTINYEXP = 867,
    // tile 868 -- 882 ?
    TINYEXPLAST = 883,
    // tile 884 -- 915 ?
    /// Chimney animation at coal power plant (2, 0).
    /// 919 last animation tile for chimney at coal power plant (2, 0).
    CoalSmoke1 = 916,
    /// Chimney animation at coal power plant (3, 0).
    /// 923 last animation tile for chimney at coal power plant (3, 0).
    CoalSmoke2 = 920,
    /// Chimney animation at coal power plant (2, 1).
    /// 927 last animation tile for chimney at coal power plant (2, 1).
    CoalSmoke3 = 924,
    /// Chimney animation at coal power plant (3, 1).
    /// 931 last animation tile for chimney at coal power plant (3, 1).
    CoalSmoke4 = 928,

    FootballGame1 = 932,
    // tile 933 -- 939 ?
    FootballGame2 = 940,
    // tile 941 -- 947 ?
    VBRDG0 = 948, // draw bridge tiles (vert)
    VBRDG1 = 949,
    VBRDG2 = 950,
    VBRDG3 = 951,

    NukeSwirl1 = 952,
    NukeSwirl2 = 953,
    NukeSwirl3 = 954,
    NukeSwirl4 = 955,

    // Tiles 956-959 unused (originally)
    // TileCount = 960,

    /* Extended zones: 956-1019 */
    Church1Base = 956,
    Church1 = 960,
    Church2Base = 965,
    Church2 = 969,
    Church3Base = 974,
    Church3 = 978,
    Church4Base = 983,
    Church4 = 987,
    Church5Base = 992,
    Church5 = 996,
    Church6Base = 1001,
    Church6 = 1005,
    Church7Base = 1010,
    Church7 = 1014,
    Church7Last = 1018,
    // TileCount = 1024,
}

impl fmt::Display for TileType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "TileType::{:?}", self)
    }
}

impl TileType {
    pub fn from_i16(value: i16) -> Option<Self> {
        FromPrimitiveTrait::from_i16(value)
    }

    pub fn from_u16(value: u16) -> Option<Self> {
        FromPrimitiveTrait::from_u16(value)
    }

    pub fn from_usize(value: usize) -> Option<Self> {
        FromPrimitiveTrait::from_usize(value)
    }

    pub fn to_i16(&self) -> Option<i16> {
        ToPrimitiveTrait::to_i16(self)
    }

    pub fn to_u16(&self) -> Option<u16> {
        match self {
            TileType::Invalid => None,
            _ => ToPrimitiveTrait::to_u16(self),
        }
    }

    pub fn to_usize(&self) -> Option<usize> {
        match self {
            TileType::Invalid => None,
            _ => ToPrimitiveTrait::to_usize(self),
        }
    }
}
