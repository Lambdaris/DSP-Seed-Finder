use crate::data::rule::{Rule, Condition};
use crate::data::star::Star;
use crate::data::planet::Planet;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RuleSatelliteCount {
    #[serde(skip)]
    pub evaluated: bool,
    pub condition: Condition,
}

impl Rule for RuleSatelliteCount {
    fn on_planets_created(&mut self, _: &Star, planets: &Vec<Planet>) -> Option<bool> {
        self.evaluated = true;
        let count = planets.iter().filter(|planet| planet.orbit_around != 0).count();
        Some(self.condition.eval(count as f32))
    }
    fn is_evaluated(&self) -> bool {
        self.evaluated
    }
    fn reset(&mut self) {
        self.evaluated = false;
    }
}
