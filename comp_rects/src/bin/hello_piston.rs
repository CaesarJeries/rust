extern crate piston_window;

use piston_window::*;


fn main() {
    let mut window: PistonWindow = WindowSettings::new("Hello Piston!", (1920, 1080))
        .exit_on_esc(true)
        .build()
        .unwrap_or_else(|e| { panic!("Failed to build PistonWindow: {}", e) });
    while let Some(e) = window.next() {
        window.draw_2d(&e, |_context, g, _device| {
            clear([0.5, 1.0, 0.5, 1.0], g);
        });
    }
}
