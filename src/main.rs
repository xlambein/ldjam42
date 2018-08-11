extern crate ggez;
extern crate nalgebra;
use ggez::*;
use ggez::graphics::{DrawMode, Color, Point2, Vector2};
use nalgebra::{Real};

type Matrix2 = nalgebra::Matrix2<f32>;

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

    fn planet(sun: &CelestialBody, pos: Point2, clockwise: bool, rad: f32, mass: f32, color: Color) -> Self {
        let r = sun.pos - pos;
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
}

impl MainState {
    fn new(_ctx: &mut Context) -> GameResult<MainState> {
        let sun = CelestialBody {
            pos: Point2::new(400., 300.),
            vel: Vector2::new(0., 0.),
            rad: 50.,
            mass: 1000000.,
            color: [0.95, 0.8, 0.1, 1.].into(),
        };

        let mut bodies = vec![
            CelestialBody::planet(&sun, Point2::new(500., 300.), false, 10., 1., [0.6, 0.15, 0.1, 1.].into()),
            CelestialBody::planet(&sun, Point2::new(200., 300.), true, 20., 1., [0.4, 0.3, 0.7, 1.].into()),
            CelestialBody::planet(&sun, Point2::new(600., 50.), false, 40., 1., [0.4, 0.6, 0.1, 1.].into()),
        ];
        bodies.push(sun);

        let s = MainState {
            bodies
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

        for body in &mut self.bodies {
            body.draw(ctx)?;
        }

        graphics::present(ctx);
        Ok(())
    }
}

pub fn main() {
    let c = conf::Conf::new();
    let ctx = &mut Context::load_from_conf("super_simple", "ggez", c).unwrap();
    let state = &mut MainState::new(ctx).unwrap();
    event::run(ctx, state).unwrap();
}

