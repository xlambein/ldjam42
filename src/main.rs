extern crate ggez;
extern crate nalgebra;
extern crate rand;
extern crate hsluv;

use ggez::*;
use ggez::event::{self, Keycode, Mod, MouseButton, MouseState};
use ggez::graphics::{DrawParam, DrawMode, Mesh, Rect, Point2, Vector2, Matrix4};
use nalgebra::{Real};

use std::fs::File;

mod entities;
mod star_system_gen;

use entities::{G, CelestialObject, Spaceship};
use star_system_gen::*;


struct Orbit {
    pub body_center: Point2,
    pub e: Vector2,
    pub p: f32,
}

impl Orbit {

    fn new(body_center: Point2, e: Vector2, p: f32) -> Self {
        Orbit { body_center, e, p }
    }

    fn ellipse_axes(&self) -> Option<(f32, f32)> {
        let e2 = self.e.norm_squared();
        if e2 < 1. {
            let a = self.p / (1. - e2);
            let b = a * (1. - e2).sqrt();
            Some((a, b))
        } else {
            None
        }
    }

}

impl Default for Orbit {

    fn default() -> Self {
        Orbit::new(Point2::origin(), Vector2::x(), 0.)
    }
}

struct MainState {
    bodies: Vec<CelestialObject>,
    player: Spaceship,
    camera: DrawParam,

    mouse: Point2,
    mouse_down: bool,

    orbit: Orbit,
}

impl MainState {
    fn new(_ctx: &mut Context) -> GameResult<MainState> {
        let sun = random_star();

        let mut bodies = vec![];

        for _ in 0..4 {
            bodies.push(random_rocky_planet(&sun));
        }
        for _ in 0..4 {
            bodies.push(random_gas_giant_planet(&sun));
        }

        let player = Spaceship::new_in_orbit(
            &sun,
            Point2::new(250., 0.),
            true,
            1.
        );

        bodies.push(sun);

        let camera = DrawParam {
            src: Rect::one(),
            dest: Point2::origin(),
            rotation: 0.0,
            scale: Point2::new(1.0, 1.0),
            offset: Point2::new(0., 0.),
            shear: Point2::new(0.0, 0.0),
            color: None,
        };

        let s = MainState {
            bodies,
            player,
            camera,
            mouse: Point2::origin(),
            mouse_down: false,
            orbit: Orbit::default(),
        };
        Ok(s)
    }
}

fn mouse_to_screen_coordinates(ctx: &Context, x: i32, y: i32) -> (f32, f32) {
    let (w, h) = graphics::get_size(ctx);
    let screen = graphics::get_screen_coordinates(ctx);
    (screen.x + (x as f32 / w as f32) * screen.w, screen.y + (y as f32 / h as f32) * screen.h)
}

impl event::EventHandler for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        const DESIRED_FPS: u32 = 60;

        while timer::check_update_time(ctx, DESIRED_FPS) {
            let seconds = 1. / (DESIRED_FPS as f32);

            for i in 0..self.bodies.len() {
                let (object, rest) = self.bodies.split_at_mut(i+1);
                let object = object.last_mut().unwrap();
                for other in rest {
                    object.body.apply_gravity(&other.body, seconds);
                    other.body.apply_gravity(&object.body, seconds);
                }

                self.player.body.apply_gravity(&object.body, seconds);

                object.update(seconds)?;
            }

            let mut min_rad = std::f32::INFINITY;
            self.orbit = Orbit::default();
            for object in self.bodies.iter() {
                let body = &object.body;
                let player = &self.player.body;
                let r = player.pos - body.pos;
                let v = player.vel - body.vel;
                let h = r.perp(&v);
                let mu = G * body.mass;
                let e = Vector2::new(v.y * h, -v.x * h) / mu - r.normalize();

                if e.norm() < 1. && r.norm() < min_rad {
                    min_rad = r.norm();
                    self.orbit = Orbit::new(body.pos, e, h*h / mu);
                }
            }

            let mouse_rel = self.mouse - Point2::new(400., 300.);
            let angle = Real::atan2(mouse_rel.y, mouse_rel.x);
            self.player.rot = angle;

            if self.mouse_down {
                let ACC: f32 = 10.;
                self.player.body.vel +=
                    seconds * ACC * Vector2::new(angle.cos(), angle.sin());
            }

            self.player.update(seconds)?;

            self.camera.dest = -self.player.body.pos + Vector2::new(400., 300.);
            self.camera.offset = self.player.body.pos;
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx);

        graphics::push_transform(ctx, Some(self.camera.into_matrix()));
        //graphics::push_transform(ctx, Some(Matrix4::new_scaling(self.camera.scale.x)));
        //graphics::push_transform(ctx, None);
        graphics::apply_transformations(ctx)?;

        let scale = self.camera.scale.x;
        let inv_scale = Point2::new(1./scale, 1./scale);

        if let Some((a, b)) = self.orbit.ellipse_axes() {
            let c = (a*a - b*b).sqrt();
            let center = self.orbit.body_center - self.orbit.e.normalize() * c;

            graphics::set_color(ctx, [1.0, 0.0, 0.0, 1.].into())?;

            let ellipse = Mesh::new_ellipse(
                ctx,
                DrawMode::Line(2.),
                Point2::origin(),
                a*scale, b*scale,
                1.)?;

            // TODO idk why this works
            let angle = if self.orbit.e.y >= 0. {
                self.orbit.e.angle(&Vector2::x())
            } else {
                f32::pi() - self.orbit.e.angle(&Vector2::x())
            };

            graphics::draw_ex(
                ctx,
                &ellipse,
                DrawParam {
                    dest: center,
                    rotation: angle,
                    scale: inv_scale,
                    ..Default::default()
                })?;
        };

        //graphics::pop_transform(ctx);
        //graphics::apply_transformations(ctx)?;

        for body in &mut self.bodies {
            body.draw(ctx)?;
        }

        self.player.draw(ctx)?;

        graphics::pop_transform(ctx);
        graphics::apply_transformations(ctx)?;

        self.player.draw_ui(ctx)?;

        graphics::present(ctx);

        timer::yield_now();
        Ok(())
    }

    fn mouse_button_down_event(&mut self, _ctx: &mut Context, _button: MouseButton, _x: i32, _y: i32) {
        self.mouse_down = true;
    }

    fn mouse_button_up_event(&mut self, _ctx: &mut Context, _button: MouseButton, _x: i32, _y: i32) {
        self.mouse_down = false;
    }

    fn mouse_motion_event(
        &mut self,
        ctx: &mut Context,
        _state: MouseState,
        x: i32,
        y: i32,
        _xrel: i32,
        _yrel: i32,
    ) {
        let (x, y) = mouse_to_screen_coordinates(ctx, x, y);
        self.mouse = Point2::new(x, y);
    }

    fn mouse_wheel_event(&mut self, _ctx: &mut Context, _x: i32, y: i32) {
        if y > 0 {
            self.camera.scale *= 1.1;
        } else if y < 0 {
            self.camera.scale /= 1.1;
        }
    }

    /*fn key_down_event(&mut self, _ctx: &mut Context, keycode: Keycode, keymod: Mod, repeat: bool) {
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
    }*/
}

const SCALING_FACTOR: f32 = 2.;

pub fn main() {
    // TODO handle errors
    let mut config_file = File::open("config.toml").unwrap();
    let c = conf::Conf::from_toml_file(&mut config_file).unwrap();
    let ctx = &mut Context::load_from_conf("super_simple", "ggez", c).unwrap();
    let state = &mut MainState::new(ctx).unwrap();

    // High-DPI stuff
    // TODO pull that into a config file
    
    let (w, h) = graphics::get_size(ctx);
    graphics::set_resolution(ctx,
                             (SCALING_FACTOR * w as f32) as u32,
                             (SCALING_FACTOR * h as f32) as u32).unwrap();

    event::run(ctx, state).unwrap();
}

