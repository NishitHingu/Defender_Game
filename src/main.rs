pub mod geom;

extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use piston::{ReleaseEvent, PressEvent};
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderEvent, UpdateEvent};
use basic_game::config::GraphicsConfig;
use basic_game::App;

fn main() {
    // Create a new game and run it.
    let mut app = App::new(GraphicsConfig::new("space_game", 400.0, 400.0));

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut app.window.settings) {
        if let Some(args) = e.render_args() {
            app.render(&args);
        }

        if let Some(args) = e.update_args() {
            app.update(&args);
        }

        if let Some(args) = e.press_args() {
            app.input(&args, true);
        }

        if let Some(args) = e.release_args() {
            app.input(&args, false);
        }
    }
}
