extern crate ggez;
extern crate nalgebra;
use ggez::*;
use ggez::graphics::{DrawMode, Mesh, Rect, Color, Point2, Vector2};

type Matrix2 = nalgebra::Matrix2<f32>;

pub const G: f32 = 1.;

pub struct Body {
    // TODO add forces
    pub pos: Point2,
    pub vel: Vector2,
    pub mass: f32,
}

impl Body {

    pub fn apply_gravity(&mut self, other: &Body, seconds: f32) {
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

    pub fn new(pos: Point2, vel: Vector2, mass: f32) -> Self {
        Body { pos, vel, mass }
    }

    pub fn new_in_orbit(parent: &Body, pos: Point2, clockwise: bool, mass: f32) -> Self {
        let r = parent.pos - pos;
        // Velocity is perpendicular to radial vector
        let v = if clockwise {
            Matrix2::new(0., 1., -1., 0.)
        } else {
            Matrix2::new(0., -1., 1., 0.)
        } * r.normalize();
        let vel = (G * parent.mass / r.norm()).sqrt() * v;

        Body::new(pos, vel, mass)
    }

}

pub struct CelestialObject {
    pub body: Body,
    pub rad: f32,
    pub color: Color,
}

impl CelestialObject {

    pub fn update(&mut self, seconds: f32) -> GameResult<()> {
        self.body.update(seconds)?;

        Ok(())
    }

    pub fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::set_color(ctx, self.color)?;
        graphics::circle(
            ctx,
            DrawMode::Fill,
            self.body.pos,
            self.rad, 1.0)?;
        Ok(())
    }

    // Create a planet orbitting a star at a given location
    pub fn new_planet_in_orbit(
        sun: &CelestialObject,
        pos: Point2,
        clockwise: bool,
        rad: f32,
        mass: f32,
        color: Color
    ) -> Self {
        CelestialObject {
            body: Body::new_in_orbit(&sun.body, pos, clockwise, mass),
            rad,
            color,
        }
    }

}

pub struct Spaceship {
    pub body: Body,
    pub rot: f32,
}

const SPACESHIP_HEIGHT: f32 = 1.;

impl Spaceship {

    pub fn update(&mut self, seconds: f32) -> GameResult<()> {
        self.body.update(seconds)?;

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
            self.body.pos,
            self.rot)?;

        Ok(())
    }

    pub fn draw_ui(&mut self, ctx: &mut Context) -> GameResult<()> {
        let height = 10.;
        let mesh = Mesh::new_polygon(
            ctx,
            DrawMode::Fill,
            &[
                Point2::new(-height/3., -height/2.),
                Point2::new(2.*height/3., 0.),
                Point2::new(-height/3., height/2.),
                Point2::new(-height/3., -height/2.)
            ]
        )?;

        let Rect { x, y, w, h } = graphics::get_screen_coordinates(ctx);

        graphics::set_color(ctx, [0.5, 0.6, 0.7, 0.7].into())?;
        graphics::draw(
            ctx,
            &mesh,
            Point2::new(x + w/2., y + h/2.),
            self.rot)?;

        Ok(())
    }

    pub fn new_in_orbit(
        sun: &CelestialObject,
        pos: Point2,
        clockwise: bool,
        mass: f32
    ) -> Self {
        Spaceship {
            body: Body::new_in_orbit(&sun.body, pos, clockwise, mass),
            rot: 0.,
        }
    }

}

