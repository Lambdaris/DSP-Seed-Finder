use serde::Serialize;

use super::enums::PlanetType;
use super::theme_proto::{ThemeProto, DEFAULT_THEME_PROTO};
use super::vein::Vein;

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Planet {
    pub index: i32,
    #[serde(skip_serializing)]
    pub seed: i32,
    #[serde(skip_serializing)]
    pub info_seed: i32,
    #[serde(skip_serializing)]
    pub theme_seed: i32,
    pub orbit_around: i32,
    pub orbit_index: i32,
    #[serde(skip_serializing)]
    pub number: i32,
    pub id: i32,
    #[serde(skip_serializing)]
    pub radius: f32,
    #[serde(skip_serializing)]
    pub scale: f32,
    pub orbit_radius: f32,
    pub orbit_inclination: f32,
    pub orbit_longitude: f32,
    pub orbital_period: f64,
    pub orbit_phase: f32,
    pub obliquity: f32,
    pub rotation_period: f64,
    pub rotation_phase: f32,
    pub sun_distance: f32,
    pub planet_type: PlanetType,
    pub habitable_bias: f32,
    pub temperature_bias: f32,
    pub luminosity: f32,
    pub theme_proto: &'static ThemeProto,
    #[serde(skip_serializing)]
    pub theme_rand1: f64,
    pub veins: Vec<Vein>,
    pub gases: Vec<(i32, f32)>,
}

impl Default for Planet {
    fn default() -> Self {
        Self {
            index: 0,
            seed: 0,
            info_seed: 0,
            theme_seed: 0,
            orbit_around: 0,
            orbit_index: 0,
            number: 0,
            id: 0,
            radius: 200.0,
            scale: 1.0,
            orbit_radius: 0.0,
            orbit_inclination: 0.0,
            orbit_longitude: 0.0,
            orbital_period: 0.0,
            orbit_phase: 0.0,
            obliquity: 0.0,
            rotation_period: 0.0,
            rotation_phase: 0.0,
            sun_distance: 0.0,
            planet_type: PlanetType::None,
            habitable_bias: 0.0,
            temperature_bias: 0.0,
            luminosity: 0.0,
            theme_proto: DEFAULT_THEME_PROTO,
            theme_rand1: 0.0,
            veins: vec![],
            gases: vec![],
        }
    }
}

impl Planet {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn real_radius(&self) -> f32 {
        self.radius * self.scale
    }
}
