extern crate ggez;
extern crate nalgebra;
use ggez::*;
use ggez::graphics::{DrawMode, Mesh, Color, Point2, Vector2};

type Matrix2 = nalgebra::Matrix2<f32>;

const G: f32 = 1.;

pub struct CelestialBody {
    pub pos: Point2,
    pub vel: Vector2,
    pub rad: f32,
    pub mass: f32,
    pub color: Color,
}

impl CelestialBody {

    pub fn apply_gravity(&mut self, other: &CelestialBody, seconds: f32) {
        let r = other.pos - self.pos;
        // Newton's formula for gravity
        // The body's own mass isn't in here, because we want the acceleration and not the force
        let acc = G * other.mass / r.norm_squared() * r.normalize();
        self.vel += seconds * acc;
    }

    pub fn update(&mut self, seconds: f32) -> GameResult<()> {
        self.pos += self.vel * seconds;
        Ok(())
    }

    pub fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::set_color(ctx, self.color)?;
        graphics::circle(
            ctx,
            DrawMode::Fill,
            self.pos,
            self.rad, 1.0)?;
        Ok(())
    }

    // Create a planet orbitting a star at a given location
    pub fn planet(sun: &CelestialBody, pos: Point2, clockwise: bool, rad: f32, mass: f32, color: Color) -> Self {
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

pub struct Spaceship {
    pub pos: Point2,
    pub vel: Vector2,
    pub rot: f32,
    pub mass: f32,
}

const SPACESHIP_HEIGHT: f32 = 10.;

impl Spaceship {

    pub fn apply_gravity(&mut self, other: &CelestialBody, seconds: f32) {
        let r = other.pos - self.pos;
        // Newton's formula for gravity
        // The body's own mass isn't in here, because we want the acceleration and not the force
        let acc = G * other.mass / r.norm_squared() * r.normalize();
        self.vel += seconds * acc;
    }

    pub fn update(&mut self, seconds: f32) -> GameResult<()> {
        self.pos += self.vel * seconds;
        Ok(())
    }

    pub fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        let mesh = Mesh::new_polygon(
            ctx,
            DrawMode::Fill,
            &[
                Point2::new(-SPACESHIP_HEIGHT/3., -SPACESHIP_HEIGHT/2.),
                Point2::new(2.*SPACESHIP_HEIGHT/3., 0.),
                Point2::new(-SPACESHIP_HEIGHT/3., SPACESHIP_HEIGHT/2.),
                Point2::new(-SPACESHIP_HEIGHT/3., -SPACESHIP_HEIGHT/2.)
            ]
        )?;

        graphics::set_color(ctx, [0.5, 0.6, 0.7, 1.].into())?;
        graphics::draw(
            ctx,
            &mesh,
            self.pos,
            self.rot)?;

        Ok(())
    }

}

