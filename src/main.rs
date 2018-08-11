extern crate ggez;
use ggez::*;
use ggez::graphics::{DrawMode, Point2, Vector2};

const G: f32 = 1.;

struct MainState {
    sun_pos: Point2,
    sun_vel: Vector2,
    sun_rad: f32,
    sun_mass: f32,

    planet_pos: Point2,
    planet_vel: Vector2,
    planet_rad: f32,
    planet_mass: f32,
}

impl MainState {
    fn new(_ctx: &mut Context) -> GameResult<MainState> {
        let s = MainState {
            sun_pos: Point2::new(400., 300.),
            sun_vel: Vector2::new(0., 0.),
            sun_rad: 50.,
            sun_mass: 10000.,

            planet_pos: Point2::new(200., 300.),
            planet_vel: Vector2::new(0., 50.),
            planet_rad: 20.,
            planet_mass: 1.,
        };
        Ok(s)
    }
}

impl event::EventHandler for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        const DESIRED_FPS: u32 = 60;

        while timer::check_update_time(ctx, DESIRED_FPS) {
            let seconds = 1. / (DESIRED_FPS as f32);

            let r = self.sun_pos - self.planet_pos;
            self.planet_vel += G * self.planet_mass * self.sun_mass / r.norm_squared() * r.normalize();

            let r = self.planet_pos - self.sun_pos;
            self.sun_vel += G * self.planet_mass * self.sun_mass / r.norm_squared() * r.normalize();

            self.sun_pos += self.sun_vel * seconds;
            self.planet_pos += self.planet_vel * seconds;
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx);

        graphics::circle(
            ctx,
            DrawMode::Fill,
            self.sun_pos,
            self.sun_rad, 2.0)?;

        graphics::circle(
            ctx,
            DrawMode::Fill,
            self.planet_pos,
            self.planet_rad, 2.0)?;

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

