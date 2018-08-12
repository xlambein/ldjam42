extern crate ggez;
extern crate nalgebra;
use ggez::*;
use ggez::event::{self, Keycode, Mod, MouseButton, MouseState};
use ggez::graphics::{DrawMode, DrawParam, Rect, Color, Point2, Vector2};
use nalgebra::{Real};

type Matrix2 = nalgebra::Matrix2<f32>;

extern crate rand;
use rand::prelude::*;

extern crate hsluv;

const G: f32 = 1.;

struct CelestialBody {
    pub pos: Point2,
    pub vel: Vector2,
    pub rad: f32,
    pub mass: f32,
    pub color: Color,
}

impl CelestialBody {
    fn apply_gravity(&mut self, other: &CelestialBody, seconds: f32) {
        let r = other.pos - self.pos;
        // Newton's formula for gravity
        // The body's own mass isn't in here, because we want the acceleration and not the force
        let acc = G * other.mass / r.norm_squared() * r.normalize();
        self.vel += seconds * acc;
    }

    fn update(&mut self, seconds: f32) -> GameResult<()> {
        self.pos += self.vel * seconds;
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::set_color(ctx, self.color)?;
        graphics::circle(
            ctx,
            DrawMode::Fill,
            self.pos,
            self.rad, 1.0)?;
        Ok(())
    }

    // Create a planet orbitting a star at a given location
    fn planet(sun: &CelestialBody, pos: Point2, clockwise: bool, rad: f32, mass: f32, color: Color) -> Self {
        let r = sun.pos - pos;
        // Velocity is perpendicular to radial vector
        let v = if clockwise {
            Matrix2::new(0., 1., -1., 0.)
        } else {
            Matrix2::new(0., -1., 1., 0.)
        } * r.normalize();
        let vel = (G * sun.mass / r.norm()).sqrt() * v;

        CelestialBody {
            pos,
            vel,
            rad,
            mass,
            color,
        }
    }
}

struct MainState {
    bodies: Vec<CelestialBody>,
    camera: DrawParam,
}

const STAR_RADIUS_RANGE: [f32; 2] = [100., 200.];
const STAR_DENSITY_RANGE: [f32; 2] = [0.01, 0.05];

const ROCKY_PLANET_RADIUS_RANGE: [f32; 2] = [5., 20.];
const ROCKY_PLANET_DENSITY_RANGE: [f32; 2] = [0.1, 1.];
const ROCKY_PLANET_ORBIT_RANGE: [f32; 2] = [500., 2000.];

const GAS_GIANT_PLANET_RADIUS_RANGE: [f32; 2] = [20., 50.];
const GAS_GIANT_PLANET_DENSITY_RANGE: [f32; 2] = [0.02, 0.1];
const GAS_GIANT_PLANET_ORBIT_RANGE: [f32; 2] = [2000., 4000.];

fn random_star() -> CelestialBody {
    let mut rng = thread_rng();

    let rad = rng.gen_range(STAR_RADIUS_RANGE[0], STAR_RADIUS_RANGE[1]);

    let density = rng.gen_range(STAR_DENSITY_RANGE[0], STAR_DENSITY_RANGE[1]);
    let volume = 2./3. * f32::two_pi() * rad.powi(3);
    let mass = volume * density;

    let (r, g, b) = hsluv::hsluv_to_rgb((70., 100., 90.));
    let color = [r as f32, g as f32, b as f32, 1.].into();

    CelestialBody {
        pos: Point2::new(0., 0.),
        vel: Vector2::new(0., 0.),
        rad,
        mass,
        color,
    }
}

fn random_rocky_planet(sun: &CelestialBody) -> CelestialBody {
    let mut rng = thread_rng();

    let orbit = rng.gen_range(ROCKY_PLANET_ORBIT_RANGE[0], ROCKY_PLANET_ORBIT_RANGE[1]);
    let angle = rng.gen_range(0., f32::two_pi());
    let pos = sun.pos + Vector2::new(orbit * angle.cos(), orbit * angle.sin());

    let clockwise = rng.gen();

    let rad = rng.gen_range(ROCKY_PLANET_RADIUS_RANGE[0], ROCKY_PLANET_RADIUS_RANGE[1]);

    let density = rng.gen_range(ROCKY_PLANET_DENSITY_RANGE[0], ROCKY_PLANET_DENSITY_RANGE[1]);
    let volume = 2./3. * f32::two_pi() * rad.powi(3);
    let mass = volume * density;

    let (r, g, b) = hsluv::hsluv_to_rgb((rng.gen_range(0., 150.), 68., 40.));
    let color = [r as f32, g as f32, b as f32, 1.].into();

    CelestialBody::planet(sun, pos, clockwise, rad, mass, color)
}

fn random_gas_giant_planet(sun: &CelestialBody) -> CelestialBody {
    let mut rng = thread_rng();

    let orbit = rng.gen_range(GAS_GIANT_PLANET_ORBIT_RANGE[0], GAS_GIANT_PLANET_ORBIT_RANGE[1]);
    let angle = rng.gen_range(0., f32::two_pi());
    let pos = sun.pos + Vector2::new(orbit * angle.cos(), orbit * angle.sin());

    let clockwise = rng.gen();

    let rad = rng.gen_range(GAS_GIANT_PLANET_RADIUS_RANGE[0], GAS_GIANT_PLANET_RADIUS_RANGE[1]);

    let density = rng.gen_range(GAS_GIANT_PLANET_DENSITY_RANGE[0], GAS_GIANT_PLANET_DENSITY_RANGE[1]);
    let volume = 2./3. * f32::two_pi() * rad.powi(3);
    let mass = volume * density;

    let (r, g, b) = hsluv::hsluv_to_rgb((rng.gen_range(150., 320.), 68., 40.));
    let color = [r as f32, g as f32, b as f32, 1.].into();

    CelestialBody::planet(sun, pos, clockwise, rad, mass, color)
}

impl MainState {
    fn new(_ctx: &mut Context) -> GameResult<MainState> {
        let sun = random_star();
        /*CelestialBody {
            pos: Point2::new(0., 0.),
            vel: Vector2::new(0., 0.),
            rad: 100.,
            mass: 100000.,
            color: [0.95, 0.8, 0.1, 1.].into(),
        };*/

        let mut bodies = vec![];
        /*let mut bodies = vec![
            CelestialBody::planet(&sun, Point2::new(500., 0.), false, 5., 100., [0.6, 0.15, 0.1, 1.].into()),
            CelestialBody::planet(&sun, Point2::new(-1000., 0.), true, 20., 1000., [0.4, 0.3, 0.7, 1.].into()),
            CelestialBody::planet(&sun, Point2::new(0., -4000.), false, 50., 10_000., [0.4, 0.6, 0.1, 1.].into()),
        ];*/
        for _ in 0..4 {
            bodies.push(random_rocky_planet(&sun));
        }
        for _ in 0..4 {
            bodies.push(random_gas_giant_planet(&sun));
        }
        bodies.push(sun);

        let camera = DrawParam {
            src: Rect::one(),
            dest: Point2::origin(),
            rotation: 0.0,
            scale: Point2::new(1.0, 1.0),
            offset: Point2::new(0.5, 0.5),
            shear: Point2::new(0.0, 0.0),
            color: None,
        };

        let s = MainState {
            bodies,
            camera,
        };
        Ok(s)
    }
}

impl event::EventHandler for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        const DESIRED_FPS: u32 = 60;

        while timer::check_update_time(ctx, DESIRED_FPS) {
            let seconds = 1. / (DESIRED_FPS as f32);

            for i in 0..self.bodies.len() {
                let (body, rest) = self.bodies.split_at_mut(i+1);
                let body = body.last_mut().unwrap();
                for other in rest {
                    body.apply_gravity(other, seconds);
                    other.apply_gravity(body, seconds);
                }
                body.update(seconds)?;
            }
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx);

        graphics::push_transform(ctx, Some(self.camera.into_matrix()));
        graphics::apply_transformations(ctx)?;

        for body in &mut self.bodies {
            body.draw(ctx)?;
        }

        graphics::present(ctx);

        timer::yield_now();
        Ok(())
    }

    fn mouse_button_down_event(&mut self, _ctx: &mut Context, button: MouseButton, x: i32, y: i32) {
        //self.mouse_down = true;
        println!("Mouse button pressed: {:?}, x: {}, y: {}", button, x, y);
    }

    fn mouse_button_up_event(&mut self, _ctx: &mut Context, button: MouseButton, x: i32, y: i32) {
        //self.mouse_down = false;
        println!("Mouse button released: {:?}, x: {}, y: {}", button, x, y);
    }

    fn mouse_motion_event(
        &mut self,
        _ctx: &mut Context,
        _state: MouseState,
        x: i32,
        y: i32,
        xrel: i32,
        yrel: i32,
    ) {
        //let (w, h) = graphics::get_size(ctx);
        //self.camera.dest = Point2::new((x - w as i32/2) as f32, (y - h as i32/2) as f32);
        self.camera.dest = Point2::new(x as f32, y as f32);

        println!(
            "Mouse motion, x: {}, y: {}, relative x: {}, relative y: {}",
            x, y, xrel, yrel
        );
    }

    fn mouse_wheel_event(&mut self, _ctx: &mut Context, x: i32, y: i32) {
        if y > 0 {
            self.camera.scale *= 1.1;
        } else if y < 0 {
            self.camera.scale /= 1.1;
        }

        println!("Mousewheel event, x: {}, y: {}", x, y);
    }

    fn key_down_event(&mut self, _ctx: &mut Context, keycode: Keycode, keymod: Mod, repeat: bool) {
        println!(
            "Key pressed: {:?}, modifier {:?}, repeat: {}",
            keycode, keymod, repeat
        );
    }

    fn key_up_event(&mut self, _ctx: &mut Context, keycode: Keycode, keymod: Mod, repeat: bool) {
        println!(
            "Key released: {:?}, modifier {:?}, repeat: {}",
            keycode, keymod, repeat
        );
    }
}

use std::fs::File;

pub fn main() {
    let mut config_file = File::open("config.toml").unwrap();
    let c = conf::Conf::from_toml_file(&mut config_file).unwrap();
    let ctx = &mut Context::load_from_conf("super_simple", "ggez", c).unwrap();
    let state = &mut MainState::new(ctx).unwrap();
    event::run(ctx, state).unwrap();
}

