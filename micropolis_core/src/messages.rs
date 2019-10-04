mod parser;

use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;

use parser::parse_messages_resource;

pub type ParsedMessagesResource = Vec<String>;

pub struct MessagesStorage {
    score_card_strings: ParsedMessagesResource,
    game_messages: ParsedMessagesResource,
}

impl MessagesStorage {
    pub fn get_score_card_string(&self, id: ScoreCardString) -> Option<&String> {
        self.score_card_strings.get(id as usize)
    }

    pub fn get_game_message(&self, id: GameMessage) -> Option<&String> {
        match id as usize {
            0 => None,
            index => self.game_messages.get(index - 1),
        }
    }

    pub fn load() -> Result<Self, String> {
        Ok(MessagesStorage {
            score_card_strings: parse_messages_resource(20, Self::load_resource_file("stri.202")?)?,
            game_messages: parse_messages_resource(49, Self::load_resource_file("stri.301")?)?,
        })
    }

    fn load_resource_file(name: &'static str) -> Result<BufReader<File>, String> {
        let mut filepath = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        filepath.pop();
        filepath.push(format!("res/{}", name));
        Ok(BufReader::new(File::open(&filepath).map_err(|err| {
            format!("load_resource_file({}) error: {}", filepath.display(), err)
        })?))
    }
}

/// String literals displayed in the score card and
/// corresponding to the "stri.202" data resource.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum ScoreCardString {
    /// Low
    PopulationdensityLow = 0,
    /// Medium
    PopulationdensityMedium = 1,
    /// High
    PopulationdensityHigh = 2,
    /// Very High
    PopulationdensityVeryhigh = 3,

    /// Slum
    LandvalueSlum = 4,
    /// Lower Class
    LandvalueLowerClass = 5,
    /// Middle Class
    LandvalueMiddleClass = 6,
    /// High
    LandvalueHighClass = 7,

    /// Safe
    CrimeNone = 8,
    /// Light
    CrimeLight = 9,
    /// Moderate
    CrimeModerate = 10,
    /// Dangerous
    CrimeDangerous = 11,

    /// None
    PollutionNone = 12,
    /// Moderate
    PollutionModerate = 13,
    /// Heavy,
    PollutionHeavy = 14,
    /// Very Heavy
    PollutionVeryHeavy = 15,

    /// Declining
    GrowrateDeclining = 16,
    /// Stable
    GrowrateStable = 17,
    /// Slow Growth
    GrowrateSlowGrowth = 18,
    /// Fast Growth
    GrowrateFastGrowth = 19,
}

pub enum GameMessage {
    /// More residential zones needed.
    MessageNeedMoreResidential = 1,
    /// More commercial zones needed.
    MessageNeedMoreCommercial = 2,
    /// More industrial zones needed.
    MessageNeedMoreIndustrial = 3,
    /// More roads required.
    MessageNeedMoreRoads = 4,
    /// 5: Inadequate rail system.
    MessageNeedMoreRails = 5,
    /// Build a Power Plant.
    MessageNeedElectricity = 6,
    /// Residents demand a Stadium.
    MessageNeedStadium = 7,
    /// Industry requires a Sea Port.
    MessageNeedSeaport = 8,
    /// Commerce requires an Airport.
    MessageNeedAirport = 9,
    /// 10: Pollution very high.
    MessageHighPollution = 10,
    /// Crime very high.
    MessageHighCrime = 11,
    /// Frequent traffic jams reported.
    MessageTrafficJams = 12,
    /// Citizens demand a Fire Department.
    MessageNeedFireStation = 13,
    /// Citizens demand a Police Department.
    MessageNeedPoliceStation = 14,
    /// 15: Blackouts reported. Check power map.
    MessageBlackoutsReported = 15,
    /// Citizens upset. The tax rate is too high.
    MessageTaxTooHigh = 16,
    /// Roads deteriorating, due to lack of funds.
    MessageRoadNeedsFunding = 17,
    /// Fire departments need funding.
    MessageFireStationNeedsFunding = 18,
    /// Police departments need funding.
    MessagePoliceNeedsFunding = 19,
    /// 20: Fire reported !
    MessageFireReported = 20,
    /// A Monster has been sighted !!
    MessageMonsterSighted = 21,
    /// Tornado reported !!
    MessageTornadoSighted = 22,
    /// Major earthquake reported !!!
    MessageEarthquake = 23,
    /// A plane has crashed !
    MessagePlaneCrashed = 24,
    /// 25: Shipwreck reported !
    MessageShipCrashed = 25,
    /// A train crashed !
    MessageTrainCrashed = 26,
    /// A helicopter crashed !
    MessageHelicopterCrashed = 27,
    /// Unemployment rate is high.
    MessageHighUnemployment = 28,
    /// YOUR CITY HAS GONE BROKE!
    MessageNoMoney = 29,
    /// 30: Firebombing reported !
    MessageFirebombing = 30,
    /// Need more parks.
    MessageNeedMoreParks = 31,
    /// Explosion detected !
    MessageExplosionReported = 32,
    /// Insufficient funds to build that.
    MessageNotEnoughFunds = 33,
    /// Area must be bulldozed first.
    MessageBulldozeAreaFirst = 34,
    /// 35: Population has reached 2,000.
    MessageReachedTown = 35,
    /// Population has reached 10,000.
    MessageReachedCity = 36,
    /// Population has reached 50,000.
    MessageReachedCapital = 37,
    /// Population has reached 100,000.
    MessageReachedMetropolis = 38,
    /// Population has reached 500,000.
    MessageReachedMegalopolis = 39,
    /// 40: Brownouts, build another Power Plant.
    MessageNotEnoughPower = 40,
    /// Heavy Traffic reported.
    MessageHeavyTraffic = 41,
    /// Flooding reported !!
    MessageFloodingReported = 42,
    /// A Nuclear Meltdown has occurred !!!
    MessageNuclearMeltdown = 43,
    /// They're rioting in the streets !!
    MessageRiotsReported = 44,
    /// 45: Started a New City.
    MessageStartedNewCity = 45,
    /// Restored a Saved City.
    MessageLoadedSavedCity = 46,
    /// You won the scenario
    MessageScenarioWon = 47,
    /// You lose the scenario
    MessageScenarioLost = 48,
    /// About micropolis.
    MessageAboutMicropolis = 49,
    /// 50: Dullsville scenario.
    MessageScenarioDullsville = 50,
    /// San Francisco scenario.
    MessageScenarioSanFrancisco = 51,
    /// Hamburg scenario.
    MessageScenarioHamburg = 52,
    /// Bern scenario.
    MessageScenarioBern = 53,
    /// Tokyo scenario.
    MessageScenarioTokyo = 54,
    /// 55: Detroit scenario.
    MessageScenarioDetroit = 55,
    /// Boston scenario.
    MessageScenarioBoston = 56,
    /// 57: Rio de Janeiro scenario.
    MessageScenarioRioDeJaneiro = 57,
    /// Last valid message
    MessageLast,
}

#[cfg(test)]
mod tests {
    use super::{GameMessage, MessagesStorage, ScoreCardString};

    #[test]
    fn test_string_literals_loading() {
        let storage = MessagesStorage::load().expect("loading error");
        assert_eq!(
            storage.get_game_message(GameMessage::MessageReachedCapital),
            Some(&"Population has reached 50,000.".to_string())
        );
        assert_eq!(
            storage.get_score_card_string(ScoreCardString::GrowrateSlowGrowth),
            Some(&"Slow Growth".to_string())
        )
    }
}
