extern crate piston_window;

use piston_window::*;

mod application;

use application::Application;

fn main() {
    let mut window: PistonWindow = WindowSettings::new("Chess", [1400, 1000])
        .exit_on_esc(true)
        .resizable(false)
        .vsync(true)
        .build()
        .unwrap();

    let mut app = Application::new(&mut window.create_texture_context());

    app.run(&mut window);
}
