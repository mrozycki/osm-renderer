#[deny(warnings)]
mod gfx;

use gfx::{Canvas, Color};

fn main() -> Result<(), std::io::Error> {
    let mut canvas = Canvas::new(1024, 768);

    gfx::shapes::rect(&mut canvas, 100, 200, 400, 300, Color::rgb(0, 0, 0));

    canvas.save_png("test.png")?;

    Ok(())
}
