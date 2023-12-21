use super::enums::{PlanetType, SpectrType, StarType, ThemeDistribute, VeinType};
use super::galaxy::Galaxy;
use super::game_desc::GameDesc;
use super::planet::Planet;
use super::random::DspRandom;
use super::star::Star;
use super::theme_proto::{ThemeProto, THEME_PROTOS};
use super::vein::Vein;

pub fn create_planet(
    galaxy: &Galaxy,
    star: &Star,
    index: i32,
    orbit_around_planet: Option<&Planet>,
    orbit_around: i32,
    orbit_index: i32,
    number: i32,
    gas_giant: bool,
    habitable_count: &mut i32,
    used_theme_ids: &mut Vec<i32>,
    info_seed: i32,
    gen_seed: i32,
) -> Planet {
    let mut planet = Planet::new();
    let mut rand = DspRandom::new(info_seed);
    planet.index = index;
    planet.seed = gen_seed;
    planet.info_seed = info_seed;
    planet.orbit_around = orbit_around;
    planet.orbit_index = orbit_index;
    planet.number = number;
    planet.id = star.astro_id() + index + 1;
    planet.name = star.name.clone() + " " + ROMAN[index as usize];

    let num3 = rand.next_f64();
    let num4 = rand.next_f64();
    let num5 = rand.next_f64();
    let num6 = rand.next_f64();
    let num7 = rand.next_f64();
    let num8 = rand.next_f64();
    let num9 = rand.next_f64();
    let num10 = rand.next_f64();
    let num11 = rand.next_f64();
    let num12 = rand.next_f64();
    let num13 = rand.next_f64();
    let num14 = rand.next_f64();
    let rand1 = rand.next_f64();
    let num15 = rand.next_f64();
    rand.next_f64();
    rand.next_f64();
    rand.next_f64();
    let theme_seed = rand.next();
    planet.theme_seed = theme_seed;
    let a = 1.2_f32.powf((num3 * (num4 - 0.5) * 0.5) as f32);
    let f1 = if let Some(orbit_planet) = orbit_around_planet {
        (((1600.0 * (orbit_index as f64) + 200.0)
            * (star.orbit_scaler.powf(0.3) as f64)
            * ((a + (1.0 - a) * 0.5) as f64)
            + (orbit_planet.real_radius() as f64))
            / 40000.0) as f32
    } else {
        let b = ORBIT_RADIUS[orbit_index as usize] * star.orbit_scaler;
        let num16 = (((a - 1.0) as f64) / (b.max(1.0) as f64) + 1.0) as f32;
        b * num16
    };

    planet.orbit_radius = f1;
    planet.orbit_inclination = (num5 * 16.0 - 8.0) as f32;
    if orbit_around > 0 {
        planet.orbit_inclination *= 2.2;
    }
    planet.orbit_longitude = (num6 * 360.0) as f32;
    if star.star_type == StarType::NeutronStar {
        if planet.orbit_inclination > 0.0 {
            planet.orbit_inclination += 3.0;
        } else {
            planet.orbit_inclination -= 3.0;
        }
    }

    planet.orbital_period = (39.4784176043574 * (f1 as f64) * (f1 as f64) * (f1 as f64)
        / (if orbit_around > 0 {
            1.08308421068537e-08
        } else {
            1.35385519905204e-06 * (star.mass as f64)
        }))
    .sqrt();

    planet.orbit_phase = (num7 * 360.0) as f32;

    if num15 < 0.0399999991059303 {
        planet.obliquity = (num8 * (num9 - 0.5) * 39.9) as f32;
        if planet.obliquity < 0.0 {
            planet.obliquity -= 70.0;
        } else {
            planet.obliquity += 70.0;
        }
    } else if num15 < 0.100000001490116 {
        planet.obliquity = (num8 * (num9 - 0.5) * 80.0) as f32;
        if planet.obliquity < 0.0 {
            planet.obliquity -= 30.0;
        } else {
            planet.obliquity += 30.0;
        }
    } else {
        planet.obliquity = (num8 * (num9 - 0.5) * 60.0) as f32;
    }

    planet.rotation_period = (num10 * num11 * 1000.0 + 400.0)
        * (if orbit_around == 0 {
            f1.powf(0.25) as f64
        } else {
            1.0
        })
        * (if gas_giant { 0.200000002980232 } else { 1.0 });

    if !gas_giant {
        planet.rotation_period *= match star.star_type {
            StarType::WhiteDwarf => 0.5,
            StarType::NeutronStar => 0.200000002980232,
            StarType::BlackHole => 0.150000005960464,
            _ => 1.0,
        };
    }
    planet.rotation_phase = (num12 * 360.0) as f32;
    planet.sun_distance = if let Some(orbit_planet) = orbit_around_planet {
        orbit_planet.orbit_radius
    } else {
        planet.orbit_radius
    };

    let num17 = if let Some(orbit_planet) = orbit_around_planet {
        orbit_planet.orbital_period
    } else {
        planet.orbital_period
    };

    planet.rotation_period = 1.0 / (1.0 / num17 + 1.0 / planet.rotation_period);
    if orbit_around == 0 && orbit_index <= 4 && !gas_giant {
        if num15 > 0.959999978542328 {
            planet.obliquity *= 0.01;
            planet.rotation_period = planet.orbital_period;
        } else if num15 > 0.930000007152557 {
            planet.obliquity *= 0.1;
            planet.rotation_period = planet.orbital_period * 0.5;
        } else if num15 > 0.899999976158142 {
            planet.obliquity *= 0.2;
            planet.rotation_period = planet.orbital_period * 0.25;
        }
    }

    if num15 > 0.85 && num15 <= 0.9 {
        planet.rotation_period = -planet.rotation_period;
    }

    let habitable_radius = star.habitable_radius;

    if gas_giant {
        planet.planet_type = PlanetType::Gas;
        planet.radius = 80.0;
        planet.scale = 10.0;
        planet.habitable_bias = 100.0;
    } else {
        let num18 = ((galaxy.star_count as f32) * 0.29).ceil().max(11.0);
        let num19 = (num18 as f64) - (*habitable_count as f64);
        let num20 = (galaxy.star_count - star.index) as f32;
        let sun_distance = planet.sun_distance;
        let (num21, f2) = if habitable_radius > 0.0 && sun_distance > 0.0 {
            let f2 = sun_distance / habitable_radius;
            (f2.ln().abs(), f2)
        } else {
            (1000.0, 1000.0)
        };
        let num22 = habitable_radius.sqrt().clamp(1.0, 2.0) - 0.04;
        let num23 = num20 as f64;
        let a = (num19 / num23) as f32;
        let num24 = (a + (0.35 - a) * 0.5).clamp(0.08, 0.8);
        planet.habitable_bias = num21 * num22;
        planet.temperature_bias =
            (1.20000004768372 / ((f2 + 0.200000002980232) as f64) - 1.0) as f32;
        let num25 = (planet.habitable_bias / num24)
            .clamp(0.0, 1.1)
            .powf(num24 * 10.0);
        if (num13 > (num25 as f64) && star.index > 0)
            || (orbit_around > 0 && orbit_index == 1 && star.index == 0)
        {
            planet.planet_type = PlanetType::Ocean;
            *habitable_count += 1;
        } else if f2 < 0.833333015441895 {
            let num26 = ((f2 as f64) * 2.5 - 0.850000023841858).max(0.15);
            planet.planet_type = if num14 >= num26 {
                PlanetType::Vocano
            } else {
                PlanetType::Desert
            };
        } else if f2 < 1.20000004768372 {
            planet.planet_type = PlanetType::Desert;
        } else {
            let num27 = 0.899999976158142 / (f2 as f64) - 0.100000001490116;
            planet.planet_type = if num14 >= num27 {
                PlanetType::Ice
            } else {
                PlanetType::Desert
            };
        }
    }

    set_planet_theme(&mut planet, star, used_theme_ids, rand1);

    planet
}

fn set_planet_theme(planet: &mut Planet, star: &Star, used_theme_ids: &mut Vec<i32>, rand1: f64) {
    let mut potential_themes: Vec<&'static ThemeProto> = vec![];
    let unused_themes: Vec<&'static ThemeProto> = THEME_PROTOS
        .iter()
        .filter(|&theme| !used_theme_ids.contains(&theme.id))
        .collect();
    for theme in &unused_themes {
        let mut flag1 = false;
        if star.index == 0 && planet.planet_type == PlanetType::Ocean {
            if theme.distribute == ThemeDistribute::Birth {
                flag1 = true;
            }
        } else {
            let flag2 = if theme.temperature.abs() < 0.5 && theme.planet_type == PlanetType::Desert
            {
                (planet.temperature_bias.abs() as f64)
                    < (theme.temperature.abs() as f64) + 0.100000001490116
            } else {
                (theme.temperature as f64) * (planet.temperature_bias as f64) >= -0.100000001490116
            };
            if (theme.planet_type == planet.planet_type) & flag2 {
                if star.index == 0 {
                    if theme.distribute == ThemeDistribute::Default {
                        flag1 = true;
                    }
                } else if theme.distribute == ThemeDistribute::Default
                    || theme.distribute == ThemeDistribute::Interstellar
                {
                    flag1 = true;
                }
            }
        }
        if flag1 {
            potential_themes.push(theme);
        }
    }
    if potential_themes.is_empty() {
        for theme in &unused_themes {
            if theme.planet_type == PlanetType::Desert {
                potential_themes.push(theme);
            }
        }
    }
    if potential_themes.is_empty() {
        for theme in &*THEME_PROTOS {
            if theme.planet_type == PlanetType::Desert {
                potential_themes.push(theme);
            }
        }
    }
    let theme_proto = potential_themes
        [((rand1 * (potential_themes.len() as f64)) as usize) % potential_themes.len()];

    planet.theme_proto = theme_proto;
    planet.planet_type = theme_proto.planet_type;
}

pub fn generate_gases(planet: &mut Planet, star: &Star, game_desc: &GameDesc) {
    let gas_coef = game_desc.gas_coef();

    let mut rand = DspRandom::new(planet.theme_seed);

    for (item, speed) in planet
        .theme_proto
        .gas_items
        .iter()
        .zip(planet.theme_proto.gas_speeds.iter())
    {
        let num2 = speed * (rand.next_f32() * 0.190909147262573 + 0.909090876579285) * gas_coef;
        planet
            .gases
            .push((*item, num2 * star.resource_coef.powf(0.3)))
    }
}

pub fn generate_veins(planet: &mut Planet, star: &Star, game_desc: &GameDesc) {
    let mut rand1 = DspRandom::new(planet.seed);
    rand1.next_f64();
    rand1.next_f64();
    rand1.next_f64();
    rand1.next_f64();
    rand1.next_f64();
    rand1.next_f64();
    let theme_proto = planet.theme_proto;
    let mut num_array_1: Vec<i32> = (0..15_i32)
        .map(|i| *theme_proto.vein_spot.get((i - 1) as usize).unwrap_or(&0))
        .collect();
    let mut num_array_2: Vec<f32> = (0..15_i32)
        .map(|i| *theme_proto.vein_count.get((i - 1) as usize).unwrap_or(&0.0))
        .collect();
    let mut num_array_3: Vec<f32> = (0..15_i32)
        .map(|i| {
            *theme_proto
                .vein_opacity
                .get((i - 1) as usize)
                .unwrap_or(&0.0)
        })
        .collect();

    let mut add_until = |i: &mut i32, t: f64| {
        for _ in 1..12 {
            if rand1.next_f64() >= t {
                break;
            }
            *i += 1;
        }
    };

    let p: f32 = match star.star_type {
        StarType::MainSeqStar => match star.spectr {
            SpectrType::M => 2.5,
            SpectrType::G => 0.7,
            SpectrType::F => 0.6,
            SpectrType::B => 0.4,
            SpectrType::O => 1.6,
            _ => 1.0,
        },
        StarType::GiantStar => 2.5,
        StarType::WhiteDwarf => {
            num_array_1[9] += 2;
            add_until(num_array_1.get_mut(9).unwrap(), 0.449999988079071);
            num_array_2[9] = 0.7;
            num_array_3[9] = 1.0;
            num_array_1[10] += 2;
            add_until(num_array_1.get_mut(10).unwrap(), 0.449999988079071);
            num_array_2[10] = 0.7;
            num_array_3[10] = 1.0;
            num_array_1[12] += 1;
            add_until(num_array_1.get_mut(12).unwrap(), 0.5);
            num_array_2[12] = 0.7;
            num_array_3[12] = 0.3;
            3.5
        }
        StarType::NeutronStar => {
            num_array_1[14] += 1;
            add_until(num_array_1.get_mut(14).unwrap(), 0.649999976158142);
            num_array_2[14] = 0.7;
            num_array_3[14] = 0.3;
            4.5
        }
        StarType::BlackHole => {
            num_array_1[14] += 1;
            add_until(num_array_1.get_mut(14).unwrap(), 0.649999976158142);
            num_array_2[14] = 0.7;
            num_array_3[14] = 0.3;
            5.0
        }
    };
    let is_rare_resource = game_desc.is_rare_resource();
    let mut f = star.resource_coef;
    if planet.is_birth {
        f *= 0.6666667;
    } else if is_rare_resource {
        if f > 1.0 {
            f = f.powf(0.8)
        }
        f *= 0.7;
    }

    for (index1, rare_vein_ref) in theme_proto.rare_veins.iter().enumerate() {
        let rare_vein = *rare_vein_ref as usize;
        let num2 = theme_proto.rare_settings[index1 * 4 + (if star.index == 0 { 0 } else { 1 })];
        let rare_setting_1 = theme_proto.rare_settings[index1 * 4 + 2];
        let rare_setting_2 = theme_proto.rare_settings[index1 * 4 + 3];
        let num4 = 1.0 - (1.0 - num2).powf(p);
        let num5 = 1.0 - (1.0 - rare_setting_2).powf(p);
        if rand1.next_f64() < (num4 as f64) {
            num_array_1[rare_vein] += 1;
            num_array_2[rare_vein] = num5;
            num_array_3[rare_vein] = num5;
            for _ in 1..12 {
                if rand1.next_f64() >= (rare_setting_1 as f64) {
                    break;
                }
                num_array_1[rare_vein] += 1;
            }
        }
    }

    let mut output: Vec<Vein> = vec![];
    for index3 in 1..15 {
        let num8 = num_array_1[index3 as usize];
        if num8 > 0 {
            let vein_type: VeinType = unsafe { ::std::mem::transmute(index3) };
            let mut vein = Vein::new();
            vein.vein_type = vein_type;
            vein.min_group = num8 - 1;
            vein.max_group = num8 + 1;
            if vein_type == VeinType::Oil {
                vein.min_patch = 1;
                vein.max_patch = 1;
            } else {
                let num12 = num_array_2[index3 as usize];
                vein.min_patch = (num12 * 20.0).round() as i32;
                vein.max_patch = (num12 * 24.0).round() as i32;
            }
            let num16 = if vein_type == VeinType::Oil {
                f.powf(0.5)
            } else {
                f
            };
            let num17 = ((num_array_3[index3 as usize] * 100000.0 * num16).round() as i32).max(20);
            let num18 = if num17 < 16000 {
                ((num17 as f32) * (15.0 / 16.0)).floor() as i32
            } else {
                15000
            };

            let map_amount = |amount: i32| -> i32 {
                let x1 = ((amount as f32) * 1.1).round();
                let x2 = (if vein_type == VeinType::Oil {
                    x1 * game_desc.oil_amount_multipler()
                } else if game_desc.is_infinite_resource() {
                    1000000000.0
                } else {
                    x1 * game_desc.resource_multiplier
                })
                .round() as i32;
                x2.max(1)
            };

            vein.min_amount = map_amount(num17 - num18);
            vein.max_amount = map_amount(num17 + num18);
            output.push(vein);
        }
    }

    planet.veins = output;
}

const ROMAN: &'static [&'static str] = &[
    "I", "II", "III", "IV", "V", "VI", "VII", "VIII", "IX", "X", "XI", "XII", "XIII", "XIV", "XV",
    "XVI", "XVII", "XVIII", "XIX", "XX",
];

const ORBIT_RADIUS: &'static [f32] = &[
    0.0, 0.4, 0.7, 1.0, 1.4, 1.9, 2.5, 3.3, 4.3, 5.5, 6.9, 8.4, 10.0, 11.7, 13.5, 15.4, 17.5,
];
