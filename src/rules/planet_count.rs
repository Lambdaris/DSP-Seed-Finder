use crate::data::rule::Condition;
use crate::data::rule::Rule;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RulePlanetCount {
    #[serde(default)]
    pub exclude_giant: bool,
    pub condition: Condition,
}

impl Rule for RulePlanetCount {
    fn get_priority(&self) -> i32 {
        30
    }
    fn evaluate(
        &self,
        galaxy: &crate::data::galaxy::Galaxy,
        evaluation: &crate::data::rule::Evaluaton,
    ) -> Vec<usize> {
        let mut result: Vec<usize> = vec![];
        for (index, sp) in galaxy.stars.iter().take(evaluation.get_len()).enumerate() {
            if evaluation.is_known(index) {
                continue;
            }
            let planets = sp.get_planets();
            let len = if self.exclude_giant {
                planets.filter(|planet| !planet.is_gas_giant()).count()
            } else {
                planets.count()
            };
            if self.condition.eval(len as f32) {
                result.push(index)
            }
        }
        result
    }
}
