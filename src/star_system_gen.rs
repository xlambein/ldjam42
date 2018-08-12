use ggez::graphics::{Point2, Vector2};
use nalgebra::{Real};

use hsluv;
use rand::prelude::*;

use entities::{Body, CelestialObject};


const STAR_RADIUS_RANGE: [f32; 2] = [100., 200.];
const STAR_DENSITY_RANGE: [f32; 2] = [0.01, 0.05];

const ROCKY_PLANET_RADIUS_RANGE: [f32; 2] = [5., 20.];
const ROCKY_PLANET_DENSITY_RANGE: [f32; 2] = [0.1, 1.];
const ROCKY_PLANET_ORBIT_RANGE: [f32; 2] = [500., 2000.];

const GAS_GIANT_PLANET_RADIUS_RANGE: [f32; 2] = [20., 50.];
const GAS_GIANT_PLANET_DENSITY_RANGE: [f32; 2] = [0.02, 0.1];
const GAS_GIANT_PLANET_ORBIT_RANGE: [f32; 2] = [2000., 4000.];

pub fn random_star() -> CelestialObject {
    let mut rng = thread_rng();

    let rad = rng.gen_range(STAR_RADIUS_RANGE[0], STAR_RADIUS_RANGE[1]);

    let density = rng.gen_range(STAR_DENSITY_RANGE[0], STAR_DENSITY_RANGE[1]);
    let volume = 2./3. * f32::two_pi() * rad.powi(3);
    let mass = volume * density;

    let (r, g, b) = hsluv::hsluv_to_rgb((70., 100., 90.));
    let color = [r as f32, g as f32, b as f32, 1.].into();

    CelestialObject {
        body: Body::new(
            Point2::origin(),
            Vector2::zeros(),
            mass),
        rad,
        color,
    }
}

pub fn random_rocky_planet(sun: &CelestialObject) -> CelestialObject {
    let mut rng = thread_rng();

    let orbit = rng.gen_range(ROCKY_PLANET_ORBIT_RANGE[0], ROCKY_PLANET_ORBIT_RANGE[1]);
    let angle = rng.gen_range(0., f32::two_pi());
    let pos = sun.body.pos + Vector2::new(orbit * angle.cos(), orbit * angle.sin());

    let clockwise = rng.gen();

    let rad = rng.gen_range(ROCKY_PLANET_RADIUS_RANGE[0], ROCKY_PLANET_RADIUS_RANGE[1]);

    let density = rng.gen_range(ROCKY_PLANET_DENSITY_RANGE[0], ROCKY_PLANET_DENSITY_RANGE[1]);
    let volume = 2./3. * f32::two_pi() * rad.powi(3);
    let mass = volume * density;

    let (r, g, b) = hsluv::hsluv_to_rgb((rng.gen_range(0., 150.), 68., 40.));
    let color = [r as f32, g as f32, b as f32, 1.].into();

    CelestialObject::new_planet_in_orbit(sun, pos, clockwise, rad, mass, color)
}

pub fn random_gas_giant_planet(sun: &CelestialObject) -> CelestialObject {
    let mut rng = thread_rng();

    let orbit = rng.gen_range(GAS_GIANT_PLANET_ORBIT_RANGE[0], GAS_GIANT_PLANET_ORBIT_RANGE[1]);
    let angle = rng.gen_range(0., f32::two_pi());
    let pos = sun.body.pos + Vector2::new(orbit * angle.cos(), orbit * angle.sin());

    let clockwise = rng.gen();

    let rad = rng.gen_range(GAS_GIANT_PLANET_RADIUS_RANGE[0], GAS_GIANT_PLANET_RADIUS_RANGE[1]);

    let density = rng.gen_range(GAS_GIANT_PLANET_DENSITY_RANGE[0], GAS_GIANT_PLANET_DENSITY_RANGE[1]);
    let volume = 2./3. * f32::two_pi() * rad.powi(3);
    let mass = volume * density;

    let (r, g, b) = hsluv::hsluv_to_rgb((rng.gen_range(150., 320.), 68., 40.));
    let color = [r as f32, g as f32, b as f32, 1.].into();

    CelestialObject::new_planet_in_orbit(sun, pos, clockwise, rad, mass, color)
}

