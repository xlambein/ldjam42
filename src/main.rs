extern crate ggez;
extern crate nalgebra;
extern crate rand;
extern crate hsluv;

use ggez::*;
use ggez::event::{self, Keycode, Mod, MouseButton, MouseState};
use ggez::graphics::{DrawParam, Rect, Point2, Vector2};
use nalgebra::{Real};

use std::fs::File;

mod entities;
mod star_system_gen;

use entities::{CelestialObject, Spaceship};
use star_system_gen::*;


struct MainState {
    bodies: Vec<CelestialObject>,
    player: Spaceship,
    camera: DrawParam,

    mouse: Point2,
    mouse_down: bool,
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
        graphics::apply_transformations(ctx)?;

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

