use glutin_window::GlutinWindow as Window;
use piston::window::{ Size, WindowSettings };
use opengl_graphics::{ GlGraphics, OpenGL };

pub mod font;
pub struct GraphicsConfig {
    // OpenGL drawing backend
    pub gl: GlGraphics,
    // Window
    pub settings: Window,
    // Window size
    pub size: Size,
}

impl GraphicsConfig {
    pub fn new(title: &'static str, width: f64, height: f64) -> GraphicsConfig {
        // Change this to OpenGL::V2_1 if not working.
        let opengl = OpenGL::V3_2;
        // Setup a new window
        let size = Size { width, height };
        let settings: Window = WindowSettings::new(title, [width, height])
            // Sets the OpenGL version
            .graphics_api(opengl)
            .exit_on_esc(true)
            .build()
            .unwrap();

        GraphicsConfig {
            gl: GlGraphics::new(opengl),
            settings,
            size
        }
    }
}