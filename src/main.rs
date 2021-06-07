#[deny(warnings)]
mod gfx;

use std::f64::consts::PI;

use gfx::{Canvas, Color};

fn main() -> Result<(), std::io::Error> {
    let mut canvas = Canvas::new(800, 800);

    for alpha in 0..90 {
        let alpha = alpha as f64 / 360.0 * 8.0 * PI;
        gfx::shapes::line(
            &mut canvas,
            (400, 400),
            (
                (400.0 + 200.0 * alpha.sin()) as i32,
                (400.0 + 200.0 * alpha.cos()) as i32,
            ),
            Color::rgb(0, 0, 0),
        );
    }

    canvas.save_png("test.png")?;

    Ok(())
}
